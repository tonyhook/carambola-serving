mod client;
mod entity;
mod service;

use std::{collections::HashMap, fs::File, panic::AssertUnwindSafe, sync::OnceLock};

use axum::{extract::{Path, Query, State}, http::{HeaderMap, StatusCode}, routing::{get, post}, Json, Router};
use base64::prelude::*;
use futures::{future::join_all, FutureExt};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use tokio_cron_scheduler::{Job, JobScheduler};
use tower_http::compression::CompressionLayer;

type HmacSha1 = Hmac<Sha1>;

use client::*;
use entity::*;
use service::*;

#[derive(Serialize, Deserialize)]
pub struct EnvConfig {
    pub db_connection: String,

    pub performance_connection: String,
    pub flowcontrol_connection: String,
    pub idgenerator_connection: String,
    pub notification_connection: String,
    pub performance_interval: u32,

    pub serving_server: String,
    pub tracking_server: String,

    pub listen_address: String,
    pub listen_port: String,
}

impl EnvConfig {
    fn new() -> Self {
        let file = File::open("serving.yml").unwrap();
        serde_yaml::from_reader(file)
            .expect("serving.yml read failed!")
    }
}
static GLOBAL_CONFIG: OnceLock<EnvConfig> = OnceLock::new();

#[tokio::main]
async fn main() {
    match GLOBAL_CONFIG.set(EnvConfig::new()) {
        Ok(_) => (),
        Err(_) => panic!("Could not get configuration!"),
    }

    let database = Database::new(&GLOBAL_CONFIG.get().unwrap().db_connection);
    let cache = Cache::new(&GLOBAL_CONFIG.get().unwrap());

    let sched = JobScheduler::new().await.unwrap();
    let database_for_cron = database.clone();
    let _ = sched.add(
        Job::new("0/2 * * * * *", {
            move |_uuid, _lock| {
                database_for_cron.get_connections();
            }
        }).unwrap()
    ).await;
    sched.start().await.unwrap();

    let comression_layer: CompressionLayer = CompressionLayer::new()
        .br(true)
        .deflate(true)
        .gzip(true)
        .zstd(true);

    let app = Router::new()
        .route("/api/ps", post(handler))
        .route("/api/win/:connection_id/:request_id", get(win))
        .route("/api/lose/:connection_id/:request_id", get(lose))
        .layer(comression_layer)
        .with_state((database, cache));

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", GLOBAL_CONFIG.get().unwrap().listen_address, GLOBAL_CONFIG.get().unwrap().listen_port))
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn handler(
    State((database, cache)): State<(Database, Cache)>,
    headers: HeaderMap,
    Json(request): Json<Request>)
-> Result<Json<Response>, (StatusCode, String)> {
    // step 1: check protocol
    let version = match headers.get("x-carambola-version") {
        Some(version) => {
            match version.to_str() {
                Ok(version) => Ok(version),
                Err(_) => Err((StatusCode::BAD_REQUEST, "NO PROTOCOL IDENTIFICATION".to_string())),
            }
        }
        None => {
            Err((StatusCode::BAD_REQUEST, "NO PROTOCOL IDENTIFICATION".to_string()))
        }
    };

    match version {
        Ok(version) => {
            if !version.starts_with("1.") {
                cache.update_performance(-1, -1, PERFORMANCE_BAD_PROTOCOL_VER);
                return Err((StatusCode::BAD_REQUEST, "BAD PROTOCOL IDENTIFICATION".to_string()));
            }
        },
        Err(_) => {
            cache.update_performance(-1, -1, PERFORMANCE_NO_PROTOCOL);
            return Err((StatusCode::BAD_REQUEST, "BAD PROTOCOL IDENTIFICATION".to_string()));
        },
    }

    // step 2: check item
    if request.item.len() == 0 {
        cache.update_performance(-1, -1, PERFORMANCE_NO_ITEM);
        return Err((StatusCode::BAD_REQUEST, "NO ITEM".to_string()));
    }

    // step 3: check vendor port
    let tagid = &request.item[0].spec.tagid;
    let vendor_port;

    let vps = {
        let vpll = database.vpla.clone();
        let vpl = vpll.read().unwrap();

        vpl.clone()
    };
    if !vps.contains_key(tagid) {
        cache.update_performance(-1, -1, PERFORMANCE_NOT_REGISTERED);
        return Err((StatusCode::BAD_REQUEST, "NO VALID TAG ID".to_string()));
    } else {
        vendor_port = vps.get(tagid).unwrap();
    }

    // step 4: check connection
    let mut connections = {
        let cml = database.cma.clone();
        let cm = cml.read().unwrap();

        let connections = cm.get(&vendor_port.0);

        match connections {
            Some(c) => {
                c.clone()
            },
            None => {
                cache.update_performance(-1, vendor_port.0, PERFORMANCE_NO_MATCH_CONNECTION);
                return Err((StatusCode::BAD_REQUEST, "NO MATCHED CONNECTION".to_string()));
            },
        }
    };

    if connections.len() == 0 {
        cache.update_performance(-1, vendor_port.0, PERFORMANCE_NO_MATCH_CONNECTION);
        return Err((StatusCode::BAD_REQUEST, "NO MATCHED CONNECTION".to_string()));
    }

    // step 5: select connection
    let mut client_mode = PORT_TYPE_SHARE;

    match vendor_port.2 {
        PORT_TYPE_SHARE => {
            let mut high_priority = 0;
            let mut best_connection_id: Option<i32> = None;
            for connection in connections.iter() {
                if connection.client_mode == PORT_TYPE_SHARE && connection.priority > high_priority {
                    high_priority = connection.priority;
                    best_connection_id = Some(connection.id);
                }
            }

            match best_connection_id {
                Some(id) => {
                    connections.retain(|connection| connection.id == id);
                    client_mode = PORT_TYPE_SHARE;
                },
                None => {
                    connections.retain(|connection| connection.client_mode == PORT_TYPE_BIDDING);

                    if connections.len() == 0 {
                        cache.update_performance(-1, vendor_port.0, PERFORMANCE_NO_MATCH_CONNECTION);
                        return Err((StatusCode::BAD_REQUEST, "NO MATCHED CONNECTION".to_string()));
                    }

                    client_mode = PORT_TYPE_BIDDING;
                },
            }
        },
        PORT_TYPE_BIDDING => {
            connections.retain(|connection| connection.client_mode == PORT_TYPE_BIDDING);

            if connections.len() == 0 {
                cache.update_performance(-1, vendor_port.0, PERFORMANCE_NO_MATCH_CONNECTION);
                return Err((StatusCode::BAD_REQUEST, "NO MATCHED CONNECTION".to_string()));
            }

            client_mode = PORT_TYPE_BIDDING;
        },
        _ => (),
    }

    // step 6: send requests
    let mut requests = Vec::new();
    let mut request_connections = Vec::new();
    for connection in connections.iter() {
        let client_port = connection.client_port;

        // check qps limitation
        let q = cache.get_request_amount(client_port, vendor_port.0);

        if q >= connection.configuration.limit_request_frequency * 60 {
            cache.update_performance(client_port, vendor_port.0, PERFORMANCE_BEYOND_CLIENT_QPS);
            continue;
        }

        cache.set_request_amount(client_port, vendor_port.0);

        // build request
        let request = AssertUnwindSafe(query(&request, connection, &cache)).catch_unwind();

        requests.push(request);
        request_connections.push(connection);
    }

    let responses = join_all(requests).await;

    // step 7: collect responses
    let mut valid_responses = Vec::<(&Response, &Connection)>::new();
    let mut errors = Vec::<String>::new();

    for (i, response) in responses.iter().enumerate() {
        let connection = request_connections.get(i).unwrap();
        let client_port = connection.client_port;

        match response {
            Ok(Ok(response)) => {
                let mut asset = 0;

                match &response.seatbid {
                    Some(seatbid) => {
                        for seatbid1 in seatbid {
                            for bid in &seatbid1.bid {
                                match &bid.media.display.banner {
                                    Some(_) => {
                                        asset += 1;
                                    },
                                    None => (),
                                }
                                match &bid.media.display.native {
                                    Some(native) => {
                                        asset += native.asset.len();
                                    },
                                    None => (),
                                }
                            }
                        }
                    },
                    None => (),
                }

                if asset > 0 {
                    valid_responses.push((response, connection));
                } else {
                    cache.update_performance(client_port, vendor_port.0, PERFORMANCE_NOT_BIDDING);
                }
            },
            Ok(Err(result_message)) => {
                match result_message.code {
                    991 => {
                        cache.update_performance(client_port, vendor_port.0, PERFORMANCE_TIMEOUT);
                    },
                    992 => {
                        cache.update_performance(client_port, vendor_port.0, PERFORMANCE_REQUEST_FAILED);
                    },
                    993 => {
                        cache.update_performance(client_port, vendor_port.0, PERFORMANCE_NOT_BIDDING);
                    },
                    997 => {
                        cache.update_performance(client_port, vendor_port.0, PERFORMANCE_TRANS_FROM_FAILED);
                    },
                    998 => {
                        cache.update_performance(client_port, vendor_port.0, PERFORMANCE_TRANS_TO_FAILED);
                    },
                    999 => {
                        cache.update_performance(client_port, vendor_port.0, PERFORMANCE_UNKNOWN_CLIENT);
                    },
                    _ => {
                        cache.update_performance(client_port, vendor_port.0, PERFORMANCE_REQUEST_FAILED);
                    }
                }
                if result_message.message.len() > 0 {
                    errors.push(result_message.message.clone());
                }
            },
            Err(_) => {
                cache.update_performance(client_port, vendor_port.0, PERFORMANCE_REQUEST_FAILED);
            },
        }
    }

    if valid_responses.len() == 0 {
        cache.update_performance(-1, vendor_port.0, PERFORMANCE_NO_RESPONSE);
        if errors.len() == 0 {
            return Err((StatusCode::NO_CONTENT, "".to_string()));
        } else {
            return Err((StatusCode::BAD_REQUEST, errors.join(",")));
        }
    }

    // step 8: select best response
    let mut best_client_price = -1;
    let mut next_client_price = -1;
    let mut best_vendor_price = -1;
    let mut best_client: Option<(&Response, &Connection)> = Option::None;
    let mut best_burl: Option<Vec<String>> = Option::None;
    let mut best_lurl: Option<Vec<String>> = Option::None;

    match client_mode {
        PORT_TYPE_SHARE => {
            // generate price
            best_client = Some((valid_responses.get(0).unwrap().0, valid_responses.get(0).unwrap().1));
            best_client_price = best_client.unwrap().0.clone().seatbid.unwrap()[0].bid[0].price;
            best_vendor_price = Price::to_vendor(best_client.unwrap().1, Some(best_client_price));

            cache.update_performance(best_client.unwrap().1.client_port, vendor_port.0, PERFORMANCE_SHARE_SUCCESS);
            cache.update_performance(best_client.unwrap().1.client_port, vendor_port.0, PERFORMANCE_SHARE_OK);
        },
        PORT_TYPE_BIDDING => {
            // generate price
            for &(response, connection) in valid_responses.iter() {
                let bid = &response.clone().seatbid.unwrap()[0].bid[0];
                let price = bid.price;

                // invalid price
                if connection.default_price > price {
                    match &bid.lurl {
                        Some(lurl) => {
                            for url in lurl.iter() {
                                bidding_notify_lose(url.clone(), connection.default_price, 1, &"".to_string(), &"".to_string(), connection).await;
                            }
                        },
                        None => (),
                    }
                    cache.update_performance(connection.client_port, vendor_port.0, PERFORMANCE_BIDDING_INVALID);
                    continue;
                }

                if price > best_client_price {
                    next_client_price = best_client_price;
                    best_client_price = price;
                    best_client = Some((&response, &connection));
                } else if price > next_client_price {
                    next_client_price = price;
                }

                cache.update_performance(connection.client_port, vendor_port.0, PERFORMANCE_BIDDING_OK);
            }

            if best_client.is_none() {
                cache.update_performance(-1, vendor_port.0, PERFORMANCE_NO_RESPONSE);
                if errors.len() == 0 {
                    return Err((StatusCode::NO_CONTENT, "".to_string()));
                } else {
                    return Err((StatusCode::BAD_REQUEST, errors.join(",")));
                }
            }

            let best_reponse = best_client.unwrap().0;
            let best_seatbid = best_reponse.clone().seatbid.unwrap();
            let best_connection = best_client.unwrap().1;

            if next_client_price > 0 {
                best_client_price = next_client_price + 1;
            }
            best_vendor_price = Price::to_vendor(best_connection, Some(best_client_price));

            cache.update_performance(best_client.unwrap().1.client_port, vendor_port.0, PERFORMANCE_BIDDING_SUCCESS);

            // bidding lose notification
            for &(response, connection) in valid_responses.iter() {
                if connection.id != best_connection.id {
                    cache.update_performance(connection.client_port, vendor_port.0, PERFORMANCE_BIDDING_LOSE);

                    match &response.clone().seatbid.unwrap()[0].bid[0].lurl {
                        Some(lurl) => {
                            for url in lurl.iter() {
                                let lose_price = best_client_price;
                                bidding_notify_lose(url.clone(), lose_price, 2, &"".to_string(), &"".to_string(), connection).await;
                            }
                        },
                        None => (),
                    }
                }
            }

            if best_connection.vendor_mode == PORT_TYPE_SHARE {
                // bidding win notification directly
                cache.update_performance(best_connection.client_port, vendor_port.0, PERFORMANCE_BIDDING_WIN);

                match &best_seatbid[0].bid[0].burl {
                    Some(burl) => {
                        for url in burl.iter() {
                            bidding_notify_win(url.clone(), best_client_price, best_client_price - 1, &"".to_string(), best_connection).await;
                        }
                    },
                    None => (),
                }
            } else {
                let request_id = &best_seatbid[0].bid[0].id.clone().unwrap();

                match &best_seatbid[0].bid[0].burl {
                    Some(burl) => {
                        // map to client win notice urls
                        cache.set_notification_url(request_id, "win", burl);
                    },
                    None => (),
                }

                // setup platform win notice url
                let mut replaced_burl = Vec::<String>::new();
                replaced_burl.push(format!("{}/api/win/{}/{}?price=__WIN_PRICE__&next=__2ND_PRICE__", GLOBAL_CONFIG.get().unwrap().serving_server, best_connection.id, request_id));
                best_burl = Some(replaced_burl);

                match &best_seatbid[0].bid[0].lurl {
                    Some(lurl) => {
                        // map to client lose notice urls
                        cache.set_notification_url(request_id, "lose", lurl);
                    },
                    None => (),
                }

                // setup platform lose notice url
                let mut replaced_lurl = Vec::<String>::new();
                replaced_lurl.push(format!("{}/api/lose/{}/{}?price=__LOSE_PRICE__&reason=__LOSE_REASON__&adn=__LOSE_ADN_NAME__", GLOBAL_CONFIG.get().unwrap().serving_server, best_connection.id, request_id));
                best_lurl = Some(replaced_lurl);
            }
        },
        _ => (),
    }

    // step 9: update price and notification urls
    let mut final_response = best_client.unwrap().0.clone();
    let mut final_seatbid = final_response.seatbid.unwrap().clone();
    let final_connection = best_client.unwrap().1;
    if final_connection.vendor_mode == PORT_TYPE_SHARE {
        final_seatbid[0].bid[0].price = 0;
        final_seatbid[0].bid[0].burl = None;
        final_seatbid[0].bid[0].lurl = None;
    } else {
        final_seatbid[0].bid[0].price = best_vendor_price;
        final_seatbid[0].bid[0].burl = best_burl;
        final_seatbid[0].bid[0].lurl = best_lurl;
    }

    // step 10: update cost
    // save price early avoiding no win notice call
    let client_port = best_client.unwrap().1.client_port;
    cache.set_notification_cost(&final_seatbid[0].bid[0].id.clone().unwrap().as_str(), client_port, vendor_port.0, best_client_price, best_vendor_price);

    // step 11: update tracking
    let request_id = &final_seatbid[0].bid[0].id.clone().unwrap();
    let event = &mut final_seatbid[0].bid[0].media.display.event;
    event.push(Event {
        eventtype: 501,
        method: 1,
        url: format!("{}/v1/{}/{}", GLOBAL_CONFIG.get().unwrap().tracking_server, 501 << 22 | final_connection.id, request_id),
        header: None,
        content: None,
    });
    event.push(Event {
        eventtype: 502,
        method: 1,
        url: format!("{}/v1/{}/{}", GLOBAL_CONFIG.get().unwrap().tracking_server, 502 << 22 | final_connection.id, request_id),
        header: None,
        content: None,
    });
    for eventtype in 503..=507 {
        if event.iter().any(|event| event.eventtype == eventtype) {
            event.push(Event {
                eventtype,
                method: 1,
                url: format!("{}/v1/{}/{}", GLOBAL_CONFIG.get().unwrap().tracking_server, eventtype << 22 | final_connection.id, request_id),
                header: None,
                content: None,
            });
        }
    }

    // TODO: log transaction

    final_response.seatbid = Some(final_seatbid);
    Ok(Json(final_response))
}

async fn win(
    State((database, cache)): State<(Database, Cache)>,
    Path((connection_id, request_id)): Path<(i32, String)>,
    Query(query): Query<HashMap<String, String>>) {
    let connection = {
        let cll = database.cla.clone();
        let cl = cll.read().unwrap();
        match cl.get(&connection_id) {
            Some(connection) => Some(connection.clone()),
            None => None,
        }
    };

    match connection {
        Some(connection) => {
            let win_price_message = query.get("price");
            let next_price_message = query.get("next");

            match win_price_message {
                Some(win_price_message) => {
                    let win_price = decrypt(win_price_message, &connection.vendor_ekey, &connection.vendor_ikey);
                    match win_price {
                        Some(win_price) => {
                            let vendor_win_price = win_price;
                            let client_win_price = Price::to_client(&connection, Some(win_price));
                            let client_next_price = {
                                match next_price_message {
                                    Some(next_price_message) => {
                                        let next_price = decrypt(next_price_message, &connection.vendor_ekey, &connection.vendor_ikey);
                                        match next_price {
                                            Some(next_price) => {
                                                Price::to_client(&connection, Some(next_price))
                                            },
                                            None => {
                                                client_win_price - 1
                                            },
                                        }
                                    },
                                    None => {
                                        client_win_price - 1
                                    },
                                }
                            };

                            match cache.get_notification_url(&request_id, "win") {
                                Some(burl) => {
                                    if burl.len() > 0 {
                                        for url in burl.iter() {
                                            if url.len() > 0 {
                                                bidding_notify_win(url.clone(), client_win_price, client_next_price, &"".to_string(), &connection).await;
                                            }
                                        }
                                    }
                                },
                                None => (),
                            }

                            cache.update_performance(connection.client_port, connection.vendor_port, PERFORMANCE_BIDDING_WIN);

                            // update the final price from win notice
                            cache.set_notification_cost(request_id.as_str(), connection.client_port, connection.vendor_port, client_win_price, vendor_win_price);
                        },
                        None => (),
                    }
                },
                None => (),
            }
        },
        None => (),
    }
}

async fn lose(
    State((database, cache)): State<(Database, Cache)>,
    Path((connection_id, request_id)): Path<(i32, String)>,
    Query(query): Query<HashMap<String, String>>) {
    let connection = {
        let cll = database.cla.clone();
        let cl = cll.read().unwrap();
        match cl.get(&connection_id) {
            Some(connection) => Some(connection.clone()),
            None => None,
        }
    };

    match connection {
        Some(connection) => {
            let lose_price_message = query.get("price");
            let lose_reason_message = query.get("reason");
            let lose_adn_name_message = query.get("adn");

            let client_lose_price = {
                match lose_price_message {
                    Some(lose_price_message) => {
                        let lose_price = decrypt(lose_price_message, &connection.vendor_ekey, &connection.vendor_ikey);
                        match lose_price {
                            Some(lose_price) => {
                                Price::to_client(&connection, Some(lose_price))
                            },
                            None => 0,
                        }
                    },
                    None => 0,
                }
            };
            let lose_reason = {
                match lose_reason_message {
                    Some(lose_reason_message) => {
                        match lose_reason_message.parse::<i32>() {
                            Ok(lose_reason) => lose_reason,
                            Err(_) => 0,
                        }
                    },
                    None => 0,
                }
            };
            let lose_adn_name = {
                match lose_adn_name_message {
                    Some(lose_adn_name_message) => lose_adn_name_message.to_string(),
                    None => "other".to_string(),
                }
            };

            match cache.get_notification_url(&request_id, "lose") {
                Some(lurl) => {
                    for url in lurl.iter() {
                        if url.len() > 0 {
                            bidding_notify_lose(url.clone(), client_lose_price, lose_reason, &lose_adn_name, &"".to_string(), &connection).await;
                        }
                    }

                    cache.update_performance(connection.client_port, connection.vendor_port, PERFORMANCE_BIDDING_LOSE);
                },
                None => (),
            }
        },
        None => (),
    }
}

async fn query(request: &Request, connection: &Connection, cache: &Cache) -> Result<Response, ResultMessage> {
    match connection.client_code.as_str() {
        "dummy" => Dummy::request(request, connection, cache).await,
        "adwanji" => Adwanji::request(request, connection, cache).await,
        "fanglin" => Fanglin::request(request, connection, cache).await,
        "fwb" => Fwb::request(request, connection, cache).await,
        "mfocus" => Mfocus::request(request, connection, cache).await,
        "richmob" => Richmob::request(request, connection, cache).await,
        "yiba" => Yiba::request(request, connection, cache).await,
        &_ => Err(ResultMessage {
            code: 999,
            message: "unknown client code".to_string(),
        }),
    }
}

async fn bidding_notify_win(url: String, win_price: i32, next_price: i32, iv: &String, connection: &Connection) {
    match connection.client_code.as_str() {
        "dummy" => Dummy::bidding_notify_win(url, win_price, next_price, iv, connection).await,
        "adwanji" => Adwanji::bidding_notify_win(url, win_price, next_price, iv, connection).await,
        "fanglin" => Fanglin::bidding_notify_win(url, win_price, next_price, iv, connection).await,
        "fwb" => Fwb::bidding_notify_win(url, win_price, next_price, iv, connection).await,
        "mfocus" => Mfocus::bidding_notify_win(url, win_price, next_price, iv, connection).await,
        "richmob" => Richmob::bidding_notify_win(url, win_price, next_price, iv, connection).await,
        "yiba" => Yiba::bidding_notify_win(url, win_price, next_price, iv, connection).await,
        &_ => (),
    }
}

async fn bidding_notify_lose(url: String, lose_price: i32, lose_reason: i32, lose_adn_name: &String, iv: &String, connection: &Connection) {
    match connection.client_code.as_str() {
        "dummy" => Dummy::bidding_notify_lose(url, lose_price, lose_reason, lose_adn_name, iv, connection).await,
        "adwanji" => Adwanji::bidding_notify_lose(url, lose_price, lose_reason, lose_adn_name, iv, connection).await,
        "fanglin" => Fanglin::bidding_notify_lose(url, lose_price, lose_reason, lose_adn_name, iv, connection).await,
        "fwb" => Fwb::bidding_notify_lose(url, lose_price, lose_reason, lose_adn_name, iv, connection).await,
        "mfocus" => Mfocus::bidding_notify_lose(url, lose_price, lose_reason, lose_adn_name, iv, connection).await,
        "richmob" => Richmob::bidding_notify_lose(url, lose_price, lose_reason, lose_adn_name, iv, connection).await,
        "yiba" => Yiba::bidding_notify_lose(url, lose_price, lose_reason, lose_adn_name, iv, connection).await,
        &_ => (),
    }
}

fn decrypt(message: &String, ekey_base64: &String, ikey_base64: &String) -> Option<i32> {
    let mut ekey_base64 = ekey_base64.replace("-", "+").replace("_", "/");
    while ekey_base64.len() % 4 != 0 {
        ekey_base64.push_str("=");
    }
    let ekey = BASE64_STANDARD.decode(ekey_base64);
    let mut ikey_base64 = ikey_base64.replace("-", "+").replace("_", "/");
    while ikey_base64.len() % 4 != 0 {
        ikey_base64.push_str("=");
    }
    let ikey = BASE64_STANDARD.decode(ikey_base64);
    if ekey.is_err() || ikey.is_err() {
        return None;
    }
    let ekey = ekey.unwrap();
    let ikey = ikey.unwrap();

    let mut message_base64 = message.replace("-", "+").replace("_", "/");
    while message_base64.len() % 4 != 0 {
        message_base64.push_str("=");
    }
    let message_bytes = BASE64_STANDARD.decode(message_base64);
    if message_bytes.is_err() {
        return None;
    }
    let message_bytes = message_bytes.unwrap();
    if message_bytes.len() != 28 {
        return None;
    }

    let iv = message_bytes[0..16].to_vec();
    let p = message_bytes[16..24].to_vec();
    let sig = message_bytes[24..28].to_vec();

    let mut price_pad: Vec<u8> = [].to_vec();
    let mac_ekey = HmacSha1::new_from_slice(&ekey);
    match mac_ekey {
        Ok(mut mac) => {
            mac.update(&iv);
            price_pad = mac.finalize().into_bytes().to_vec();
        },
        Err(_) => (),
    }

    let price_bytes: Vec<u8> = p.iter()
        .zip(price_pad[0..8].iter())
        .map(|(&x1, &x2)| x1 ^ x2)
        .collect();
    let price = u64::from_be_bytes(price_bytes[0..8].try_into().unwrap());

    let mut conf_sig = [].to_vec();
    let mac_ikey = HmacSha1::new_from_slice(&ikey);
    match mac_ikey {
        Ok(mut mac) => {
            mac.update(&[price_bytes.to_vec(), iv.to_vec()].concat());
            conf_sig = mac.finalize().into_bytes().to_vec();
        },
        Err(_) => (),
    }

    let conf_sig: [u8; 4] = conf_sig[0..4].try_into().unwrap();

    if sig != conf_sig {
        return None;
    } else {
        return Some(price as i32);
    }
}
