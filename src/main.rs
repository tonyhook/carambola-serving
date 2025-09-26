mod client;
mod entity;
mod service;

use std::{collections::HashMap, fs::File, io::Read, panic::AssertUnwindSafe, sync::OnceLock, time::Duration};

use axum::{extract::{Path, Query, State}, http::{HeaderMap, StatusCode}, routing::{get, post}, Json, Router};
use base64::prelude::*;
use futures::{future::join_all, FutureExt};
use hmac::{Hmac, Mac};
use hyper_util::{rt::{TokioExecutor, TokioIo}, server};
use hyper::body::Incoming;
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use tokio_cron_scheduler::{Job, JobScheduler};
use tokio::time::sleep;
use tower_http::{compression::CompressionLayer, decompression::RequestDecompressionLayer};
use tower::Service;
use wildmatch::WildMatch;
use yaml_rust2::YamlLoader;

type HmacSha1 = Hmac<Sha1>;

use client::*;
use entity::*;
use service::*;

#[derive(Serialize, Deserialize)]
pub struct EnvConfig {
    pub db_connection: String,

    pub performance_connection_write: String,
    pub notification_connection_write: String,
    pub notification_connection_read: String,
    pub idgenerator_connection: String,
    pub trafficcontrol_connection_write: String,
    pub trafficcontrol_connection_read: String,
    pub antifraud_connection_write: String,
    pub antifraud_connection_read: String,

    pub performance_interval: u32,

    pub console_server: String,
    pub serving_server: String,
    pub tracking_server: String,

    pub listen_address: String,
    pub listen_port: String,
}

impl EnvConfig {
    fn new() -> Self {
        let mut file = File::open("serving.yml").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).expect("Failed to read serving.yml");

        let docs = YamlLoader::load_from_str(&buffer)
            .expect("serving.yml parsing failed!");
        let yaml = &docs[0];

        EnvConfig {
            db_connection: yaml["db_connection"].as_str().unwrap_or("").to_string(),

            performance_connection_write: yaml["performance_connection_write"].as_str().unwrap_or("").to_string(),
            notification_connection_write: yaml["notification_connection_write"].as_str().unwrap_or("").to_string(),
            notification_connection_read: yaml["notification_connection_read"].as_str().unwrap_or("").to_string(),
            idgenerator_connection: yaml["idgenerator_connection"].as_str().unwrap_or("").to_string(),
            trafficcontrol_connection_write: yaml["trafficcontrol_connection_write"].as_str().unwrap_or("").to_string(),
            trafficcontrol_connection_read: yaml["trafficcontrol_connection_read"].as_str().unwrap_or("").to_string(),
            antifraud_connection_write: yaml["antifraud_connection_write"].as_str().unwrap_or("").to_string(),
            antifraud_connection_read: yaml["antifraud_connection_read"].as_str().unwrap_or("").to_string(),

            performance_interval: yaml["performance_interval"].as_i64().unwrap_or(0) as u32,

            console_server: yaml["console_server"].as_str().unwrap_or("").to_string(),
            serving_server: yaml["serving_server"].as_str().unwrap_or("").to_string(),
            tracking_server: yaml["tracking_server"].as_str().unwrap_or("").to_string(),

            listen_address: yaml["listen_address"].as_str().unwrap_or("").to_string(),
            listen_port: yaml["listen_port"].as_i64().unwrap_or(0).to_string(),
        }
    }
}
static GLOBAL_CONFIG: OnceLock<EnvConfig> = OnceLock::new();
static NODE_ID: OnceLock<i32> = OnceLock::new();

#[tokio::main]
async fn main() {
    match GLOBAL_CONFIG.set(EnvConfig::new()) {
        Ok(_) => (),
        Err(_) => panic!("Could not get configuration!"),
    }

    let client = reqwest::ClientBuilder::new()
        .gzip(true)
        .no_brotli()
        .no_deflate()
        .build().unwrap();

    loop {
        let server_info = client.get(GLOBAL_CONFIG.get().unwrap().console_server.clone() + "/api/open/server")
            .header("Accept-Encoding", "gzip")
            .send().await;

        match server_info {
            Ok(response) => {
                match response.status() {
                    StatusCode::OK => {
                        let server_info = response.json::<Server>().await.unwrap();
                        let node_id = server_info.node;

                        match NODE_ID.set(node_id) {
                            Ok(_) => break,
                            Err(_) => (),
                        }
                    },
                    _ => (),
                }
            },
            Err(_) => (),
        }

        sleep(Duration::from_secs(1)).await;
    }

    println!("Serving node {} started.", NODE_ID.get().unwrap());

    let database = Database::new(&GLOBAL_CONFIG.get().unwrap().db_connection);
    let cache = Cache::new(&GLOBAL_CONFIG.get().unwrap());
    let traffic = Traffic::new(database.clone(), cache.clone());
    let pool = HttpPool::new();

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
    let decomression_layer: RequestDecompressionLayer = RequestDecompressionLayer::new()
        .br(true)
        .deflate(true)
        .gzip(true)
        .zstd(true);

    let app = Router::new()
        .route("/api/ps", post(handler))
        .route("/api/win/{connection_id}/{request_id}", get(win))
        .route("/api/lose/{connection_id}/{request_id}", get(lose))
        .layer(comression_layer)
        .layer(decomression_layer)
        .with_state((database, cache, traffic, pool));

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", GLOBAL_CONFIG.get().unwrap().listen_address, GLOBAL_CONFIG.get().unwrap().listen_port))
        .await
        .unwrap();

    loop {
        let (socket, _remote_addr) = listener.accept().await.unwrap();
        let tower_service = app.clone();

        tokio::spawn(async move {
            let socket = TokioIo::new(socket);
            let hyper_service = hyper::service::service_fn(move |request: hyper::Request<Incoming>| {
                tower_service.clone().call(request)
            });

            let _ = server::conn::auto::Builder::new(TokioExecutor::new())
                .http1()
                .keep_alive(true)
                .serve_connection(socket, hyper_service)
                .await;
        });
    }
}

async fn handler(
    State((database, cache, traffic, pool)): State<(Database, Cache, Traffic, HttpPool)>,
    headers: HeaderMap,
    Json(request): Json<Request>)
-> Result<Json<Response>, (StatusCode, String)> {
    if !traffic.pass_traffic_control(-1, -1, &"".to_string()) {
        cache.update_performance(-1, -1, &"".to_string(), PERFORMANCE_BEYOND_VENDOR_TRAFFIC_CONTROL);
        return Err((StatusCode::TOO_MANY_REQUESTS, "TRAFFIC CONTROL LIMITATION".to_string()));
    }

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
                cache.update_performance(-1, -1, &"".to_string(), PERFORMANCE_BAD_PROTOCOL_VER);
                return Err((StatusCode::BAD_REQUEST, "BAD PROTOCOL IDENTIFICATION".to_string()));
            }
        },
        Err(_) => {
            cache.update_performance(-1, -1, &"".to_string(), PERFORMANCE_NO_PROTOCOL);
            return Err((StatusCode::BAD_REQUEST, "BAD PROTOCOL IDENTIFICATION".to_string()));
        },
    }

    // step 2: check item
    if request.item.len() == 0 {
        cache.update_performance(-1, -1, &"".to_string(), PERFORMANCE_NO_ITEM);
        return Err((StatusCode::BAD_REQUEST, "NO ITEM".to_string()));
    }

    let mut bundle: String = "UNKNOWN".to_string();
    match &request.context.app {
        Some(app) => {
            bundle = app.name.clone();
        },
        None => (),
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
        cache.update_performance(-1, -1, &bundle, PERFORMANCE_NOT_REGISTERED);
        return Err((StatusCode::BAD_REQUEST, "NO VALID TAG ID".to_string()));
    } else {
        vendor_port = vps.get(tagid).unwrap();
    }

    // step 4: check vendor port anti fraud
    if !traffic.pass_anti_fraud(-1, &request) {
        cache.update_performance(-1, -1, &bundle, PERFORMANCE_VENDOR_ANTI_FRAUD);
        return Err((StatusCode::FORBIDDEN, "ANTI FRAUD VIOLATION".to_string()));
    }

    // step 5: check connection
    let mut connections = {
        let cml = database.cma.clone();
        let cm = cml.read().unwrap();

        let connections = cm.get(&vendor_port.0);

        match connections {
            Some(c) => {
                c.clone()
            },
            None => {
                cache.update_performance(-1, vendor_port.0, &bundle, PERFORMANCE_NO_MATCH_CONNECTION);
                return Err((StatusCode::BAD_REQUEST, "NO MATCHED CONNECTION".to_string()));
            },
        }
    };

    if connections.len() == 0 {
        cache.update_performance(-1, vendor_port.0, &bundle, PERFORMANCE_NO_MATCH_CONNECTION);
        return Err((StatusCode::BAD_REQUEST, "NO MATCHED CONNECTION".to_string()));
    }

    // step 6: select connection
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
                        cache.update_performance(-1, vendor_port.0, &bundle, PERFORMANCE_NO_MATCH_CONNECTION);
                        return Err((StatusCode::BAD_REQUEST, "NO MATCHED CONNECTION".to_string()));
                    }

                    client_mode = PORT_TYPE_BIDDING;
                },
            }
        },
        PORT_TYPE_BIDDING => {
            connections.retain(|connection| connection.client_mode == PORT_TYPE_BIDDING);

            if connections.len() == 0 {
                cache.update_performance(-1, vendor_port.0, &bundle, PERFORMANCE_NO_MATCH_CONNECTION);
                return Err((StatusCode::BAD_REQUEST, "NO MATCHED CONNECTION".to_string()));
            }

            client_mode = PORT_TYPE_BIDDING;
        },
        _ => (),
    }

    // step 7: send requests
    let mut requests = Vec::new();
    let mut request_connections = Vec::new();
    for connection in connections.iter() {
        let client_port = connection.client_port;

        // check traffic control
        if !traffic.pass_traffic_control(client_port, -1, &"".to_string()) {
            cache.update_performance(client_port, vendor_port.0, &bundle, PERFORMANCE_BEYOND_CLIENT_TRAFFIC_CONTROL);
            continue;
        }
        if !traffic.pass_traffic_control(client_port, vendor_port.0, &"".to_string()) {
            cache.update_performance(client_port, vendor_port.0, &bundle, PERFORMANCE_BEYOND_CLIENT_TRAFFIC_CONTROL);
            continue;
        }
        if !traffic.pass_traffic_control(client_port, vendor_port.0, &bundle) {
            cache.update_performance(client_port, vendor_port.0, &bundle, PERFORMANCE_BEYOND_CLIENT_TRAFFIC_CONTROL);
            continue;
        }

        // check key fields of connection
        if !match_key_field(&request, connection) {
            cache.update_performance(client_port, vendor_port.0, &bundle, PERFORMANCE_LOST_KEY_FIELD);
            continue;
        }

        // check client port anti fraud
        if !traffic.pass_anti_fraud(client_port, &request) {
            cache.update_performance(client_port, vendor_port.0, &bundle, PERFORMANCE_CLIENT_ANTI_FRAUD);
            continue;
        }

        // build request
        let request = AssertUnwindSafe(query(&request, connection, &pool, &cache)).catch_unwind();

        requests.push(request);
        request_connections.push(connection);
    }

    let responses = join_all(requests).await;

    // step 8: collect responses
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
                    cache.update_performance(client_port, vendor_port.0, &bundle, PERFORMANCE_NOT_BIDDING);
                }
            },
            Ok(Err(result_message)) => {
                match result_message.code {
                    991 => {
                        cache.update_performance(client_port, vendor_port.0, &bundle, PERFORMANCE_TIMEOUT);
                    },
                    992 => {
                        cache.update_performance(client_port, vendor_port.0, &bundle, PERFORMANCE_REQUEST_FAILED);
                    },
                    993 => {
                        cache.update_performance(client_port, vendor_port.0, &bundle, PERFORMANCE_NOT_BIDDING);
                    },
                    994 => {
                        cache.update_performance(client_port, vendor_port.0, &bundle, PERFORMANCE_REQUEST_REJECTED);
                    },
                    997 => {
                        cache.update_performance(client_port, vendor_port.0, &bundle, PERFORMANCE_TRANS_FROM_FAILED);
                    },
                    998 => {
                        cache.update_performance(client_port, vendor_port.0, &bundle, PERFORMANCE_TRANS_TO_FAILED);
                    },
                    999 => {
                        cache.update_performance(client_port, vendor_port.0, &bundle, PERFORMANCE_UNKNOWN_CLIENT);
                    },
                    _ => {
                        cache.update_performance(client_port, vendor_port.0, &bundle, PERFORMANCE_REQUEST_FAILED);
                    }
                }
                if result_message.message.len() > 0 {
                    errors.push(result_message.message.clone());
                }
            },
            Err(_) => {
                cache.update_performance(client_port, vendor_port.0, &bundle, PERFORMANCE_REQUEST_FAILED);
            },
        }
    }

    if valid_responses.len() == 0 {
        cache.update_performance(-1, vendor_port.0, &bundle, PERFORMANCE_NO_RESPONSE);
        if errors.len() == 0 {
            return Err((StatusCode::NO_CONTENT, "".to_string()));
        } else {
            return Err((StatusCode::BAD_REQUEST, errors.join(",")));
        }
    }

    // step 9: select best response
    let mut best_client_price = -1.0;
    let mut next_client_price = -1.0;
    let mut best_vendor_price = -1.0;
    let mut best_client: Option<(&Response, &Connection)> = Option::None;
    let mut best_burl: Option<Vec<String>> = Option::None;
    let mut best_lurl: Option<Vec<String>> = Option::None;

    match client_mode {
        PORT_TYPE_SHARE => {
            // generate price
            best_client = Some((valid_responses.get(0).unwrap().0, valid_responses.get(0).unwrap().1));
            best_client_price = best_client.unwrap().0.clone().seatbid.unwrap()[0].bid[0].price as f64;
            best_vendor_price = Price::to_vendor(best_client.unwrap().1, Some(best_client_price));

            cache.update_performance(best_client.unwrap().1.client_port, vendor_port.0, &bundle, PERFORMANCE_SHARE_SUCCESS);
            cache.update_performance(best_client.unwrap().1.client_port, vendor_port.0, &bundle, PERFORMANCE_SHARE_OK);
        },
        PORT_TYPE_BIDDING => {
            // generate price
            for &(response, connection) in valid_responses.iter() {
                let bid = &response.clone().seatbid.unwrap()[0].bid[0];
                let price = bid.price as f64;

                // invalid price
                if connection.default_price as f64 > price {
                    match &bid.lurl {
                        Some(lurl) => {
                            for url in lurl.iter() {
                                bidding_notify_lose(url.clone(), connection.default_price, 1, &"".to_string(), &"".to_string(), connection, &pool).await;
                            }
                        },
                        None => (),
                    }
                    cache.update_performance(connection.client_port, vendor_port.0, &bundle, PERFORMANCE_BIDDING_INVALID);
                    continue;
                }

                if price > best_client_price {
                    next_client_price = best_client_price;
                    best_client_price = price;
                    best_client = Some((&response, &connection));
                } else if price > next_client_price {
                    next_client_price = price;
                }

                cache.update_performance(connection.client_port, vendor_port.0, &bundle, PERFORMANCE_BIDDING_OK);
            }

            if best_client.is_none() {
                cache.update_performance(-1, vendor_port.0, &bundle, PERFORMANCE_NO_RESPONSE);
                if errors.len() == 0 {
                    return Err((StatusCode::NO_CONTENT, "".to_string()));
                } else {
                    return Err((StatusCode::BAD_REQUEST, errors.join(",")));
                }
            }

            let best_reponse = best_client.unwrap().0;
            let best_seatbid = best_reponse.clone().seatbid.unwrap();
            let best_connection = best_client.unwrap().1;

            if next_client_price > 0.0 {
                best_client_price = next_client_price + 1.0;
            }
            best_vendor_price = Price::to_vendor(best_connection, Some(best_client_price));

            cache.update_performance(best_client.unwrap().1.client_port, vendor_port.0, &bundle, PERFORMANCE_BIDDING_SUCCESS);

            // bidding lose notification
            for &(response, connection) in valid_responses.iter() {
                if connection.id != best_connection.id {
                    cache.update_performance(connection.client_port, vendor_port.0, &bundle, PERFORMANCE_BIDDING_LOSE);

                    match &response.clone().seatbid.unwrap()[0].bid[0].lurl {
                        Some(lurl) => {
                            for url in lurl.iter() {
                                let lose_price = best_client_price;
                                bidding_notify_lose(url.clone(), lose_price as i32, 2, &"".to_string(), &"".to_string(), connection, &pool).await;
                            }
                        },
                        None => (),
                    }
                }
            }

            if best_connection.vendor_mode == PORT_TYPE_SHARE {
                // bidding win notification directly
                cache.update_performance(best_connection.client_port, vendor_port.0, &bundle, PERFORMANCE_BIDDING_WIN);

                match &best_seatbid[0].bid[0].burl {
                    Some(burl) => {
                        for url in burl.iter() {
                            bidding_notify_win(url.clone(), best_client_price as i32, (best_client_price - 1.0) as i32, &"".to_string(), best_connection, &pool).await;
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

    // step 10: update price and notification urls
    let mut final_response = best_client.unwrap().0.clone();
    let mut final_seatbid = final_response.seatbid.unwrap().clone();
    let final_connection = best_client.unwrap().1;
    if final_connection.vendor_mode == PORT_TYPE_SHARE {
        final_seatbid[0].bid[0].price = 0;
        final_seatbid[0].bid[0].burl = None;
        final_seatbid[0].bid[0].lurl = None;
    } else {
        final_seatbid[0].bid[0].price = best_vendor_price as i32;
        final_seatbid[0].bid[0].burl = best_burl;
        final_seatbid[0].bid[0].lurl = best_lurl;
    }

    // step 11: prepare bundle name for tracking
    cache.set_bundle(&final_seatbid[0].bid[0].id.clone().unwrap(), &bundle);

    // step 12: update traffic control
    traffic.prepare_for_imp(final_connection.client_port, &final_seatbid[0].bid[0].id.clone().unwrap(), &request);

    // step 13: update cost
    // save price early avoiding no win notice call
    let client_port = best_client.unwrap().1.client_port;
    let upstream_price = Price::to_upstream(best_client.unwrap().1, Some(best_client_price));
    let rebate_price = Price::to_rebate(best_client.unwrap().1, Some(best_client_price));
    cache.set_notification_cost(&final_seatbid[0].bid[0].id.clone().unwrap(), client_port, vendor_port.0, best_client_price as i32, upstream_price, rebate_price, best_vendor_price);

    // step 14: update tracking
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

fn match_rule(request: &Request, entry: &Entry) -> bool {
    let identifiers = Identifiers::new(request);

    match entry {
        Entry::Rule(rule) => {
            let mut request_value = None;
            let mut request_value_type = "string";

            match rule.field.as_str() {
                "app#bundle" => {
                    match &request.context.app {
                        Some(app) => {
                            request_value = app.bundle.clone();
                            request_value_type = "string";
                        },
                        None => (),
                    }
                },
                "app#domain" => {
                    match &request.context.app {
                        Some(app) => {
                            request_value = app.domain.clone();
                            request_value_type = "string";
                        },
                        None => (),
                    }
                },
                "app#name" => {
                    match &request.context.app {
                        Some(app) => {
                            request_value = Some(app.name.clone());
                            request_value_type = "string";
                        },
                        None => (),
                    }
                },
                "app#paid" => {
                    match &request.context.app {
                        Some(app) => {
                            request_value = Some(app.paid.to_string());
                            request_value_type = "category";
                        },
                        None => (),
                    }
                },
                "app#storeid" => {
                    match &request.context.app {
                        Some(app) => {
                            request_value = app.storeid.clone();
                            request_value_type = "string";
                        },
                        None => (),
                    }
                },
                "app#storeurl" => {
                    match &request.context.app {
                        Some(app) => {
                            request_value = app.storeurl.clone();
                            request_value_type = "string";
                        },
                        None => (),
                    }
                },
                "app#ver" => {
                    match &request.context.app {
                        Some(app) => {
                            request_value = app.ver.clone();
                            request_value_type = "string";
                        },
                        None => (),
                    }
                },
                "device#app" => {
                    request_value = request.context.device.app.clone();
                    request_value_type = "string";
                },
                "device#bootmark" => {
                    request_value = request.context.device.bootmark.clone();
                    request_value_type = "string";
                },
                "device#boottime" => {
                    request_value = request.context.device.boottime.clone();
                    request_value_type = "string";
                },
                "device#brand" => {
                    request_value = request.context.device.brand.clone();
                    request_value_type = "string";
                },
                "device#carrier" => {
                    request_value = request.context.device.carrier.clone();
                    request_value_type = "category";
                },
                "device#contype" => {
                    request_value = match &request.context.device.contype {
                        Some(contype) => Some(contype.to_string()),
                        None => None,
                    };
                    request_value_type = "category";
                },
                "device#country" => {
                    request_value = request.context.device.country.clone();
                    request_value_type = "string";
                },
                "device#h" => {
                    request_value = match &request.context.device.h {
                        Some(h) => Some(h.to_string()),
                        None => None,
                    };
                    request_value_type = "number";
                },
                "device#hmsv" => {
                    request_value = request.context.device.hmsv.clone();
                    request_value_type = "string";
                },
                "device#hwmachine" => {
                    request_value = request.context.device.hwmachine.clone();
                    request_value_type = "string";
                },
                "device#hwmodel" => {
                    request_value = request.context.device.hwmodel.clone();
                    request_value_type = "string";
                },
                "device#hwname" => {
                    request_value = request.context.device.hwname.clone();
                    request_value_type = "string";
                },
                "device#hwv" => {
                    request_value = request.context.device.hwv.clone();
                    request_value_type = "string";
                },
                "device#inittime" => {
                    request_value = request.context.device.inittime.clone();
                    request_value_type = "string";
                },
                "device#ip" => {
                    request_value = request.context.device.ip.clone();
                    request_value_type = "string";
                },
                "device#ipv6" => {
                    request_value = request.context.device.ipv6.clone();
                    request_value_type = "string";
                },
                "device#lang" => {
                    request_value = request.context.device.lang.clone();
                    request_value_type = "string";
                },
                "device#lmt" => {
                    request_value = match &request.context.device.lmt {
                        Some(lmt) => Some(lmt.to_string()),
                        None => None,
                    };
                    request_value_type = "category";
                },
                "device#make" => {
                    request_value = request.context.device.make.clone();
                    request_value_type = "string";
                },
                "device#mntid" => {
                    request_value = request.context.device.mntid.clone();
                    request_value_type = "string";
                },
                "device#model" => {
                    request_value = request.context.device.model.clone();
                    request_value_type = "string";
                },
                "device#orientation" => {
                    request_value = match &request.context.device.orientation {
                        Some(orientation) => Some(orientation.to_string()),
                        None => None,
                    };
                    request_value_type = "category";
                },
                "device#os" => {
                    request_value = match &request.context.device.os {
                        Some(os) => Some(os.to_string()),
                        None => None,
                    };
                    request_value_type = "category";
                },
                "device#oslevel" => {
                    request_value = match &request.context.device.oslevel {
                        Some(oslevel) => Some(oslevel.to_string()),
                        None => None,
                    };
                    request_value_type = "number";
                },
                "device#osv" => {
                    request_value = request.context.device.osv.clone();
                    request_value_type = "string";
                },
                "device#ppi" => {
                    request_value = match &request.context.device.ppi {
                        Some(ppi) => Some(ppi.to_string()),
                        None => None,
                    };
                    request_value_type = "number";
                },
                "device#pxratio" => {
                    request_value = match &request.context.device.pxratio {
                        Some(pxratio) => Some(pxratio.to_string()),
                        None => None,
                    };
                    request_value_type = "number";
                },
                "device#romtime" => {
                    request_value = request.context.device.romtime.clone();
                    request_value_type = "string";
                },
                "device#romv" => {
                    request_value = request.context.device.romv.clone();
                    request_value_type = "string";
                },
                "device#size" => {
                    request_value = match &request.context.device.size {
                        Some(size) => Some(size.to_string()),
                        None => None,
                    };
                    request_value_type = "number";
                },
                "device#skan" => {
                    request_value = match &request.context.device.skan {
                        Some(skan) => Some(skan.join(",")),
                        None => None,
                    };
                    request_value_type = "string";
                },
                "device#storename" => {
                    request_value = request.context.device.storename.clone();
                    request_value_type = "string";
                },
                "device#storev" => {
                    request_value = request.context.device.storev.clone();
                    request_value_type = "string";
                },
                "device#sysbatterypower" => {
                    request_value = match &request.context.device.sysbatterypower {
                        Some(sysbatterypower) => Some(sysbatterypower.to_string()),
                        None => None,
                    };
                    request_value_type = "number";
                },
                "device#sysbatterystatus" => {
                    request_value = match &request.context.device.sysbatterystatus {
                        Some(sysbatterystatus) => Some(sysbatterystatus.to_string()),
                        None => None,
                    };
                    request_value_type = "category";
                },
                "device#syscpu" => {
                    request_value = match &request.context.device.syscpu {
                        Some(syscpu) => Some(syscpu.to_string()),
                        None => None,
                    };
                    request_value_type = "number";
                },
                "device#syscpufreq" => {
                    request_value = match &request.context.device.syscpufreq {
                        Some(syscpufreq) => Some(syscpufreq.to_string()),
                        None => None,
                    };
                    request_value_type = "number";
                },
                "device#sysdisksize" => {
                    request_value = match &request.context.device.sysdisksize {
                        Some(sysdisksize) => Some(sysdisksize.to_string()),
                        None => None,
                    };
                    request_value_type = "number";
                },
                "device#sysmemory" => {
                    request_value = match &request.context.device.sysmemory {
                        Some(sysmemory) => Some(sysmemory.to_string()),
                        None => None,
                    };
                    request_value_type = "number";
                },
                "device#timezone" => {
                    request_value = request.context.device.timezone.clone();
                    request_value_type = "string";
                },
                "device#type" => {
                    request_value = match &request.context.device.devicetype {
                        Some(devicetype) => Some(devicetype.to_string()),
                        None => None,
                    };
                    request_value_type = "category";
                },
                "device#ua" => {
                    request_value = Some(request.context.device.ua.clone());
                    request_value_type = "string";
                },
                "device#uiv" => {
                    request_value = request.context.device.uiv.clone();
                    request_value_type = "string";
                },
                "device#updatemark" => {
                    request_value = request.context.device.updatemark.clone();
                    request_value_type = "string";
                },
                "device#updatetime" => {
                    request_value = request.context.device.updatetime.clone();
                    request_value_type = "string";
                },
                "device#w" => {
                    request_value = match &request.context.device.w {
                        Some(w) => Some(w.to_string()),
                        None => None,
                    };
                    request_value_type = "number";
                },
                "device#xff" => {
                    request_value = request.context.device.xff.clone();
                    request_value_type = "string";
                },
                "id#501" => {
                    request_value = match identifiers.get_id(501, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    };
                    request_value_type = "string";
                },
                "id#502" => {
                    request_value = match identifiers.get_id(502, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    };
                    request_value_type = "string";
                },
                "id#503" => {
                    request_value = match identifiers.get_id(503, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    };
                    request_value_type = "string";
                },
                "id#504" => {
                    request_value = match identifiers.get_id(504, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    };
                    request_value_type = "string";
                },
                "id#505" => {
                    request_value = match identifiers.get_id(505, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    };
                    request_value_type = "string";
                },
                "id#506" => {
                    request_value = match identifiers.get_id(506, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    };
                    request_value_type = "string";
                },
                "id#507" => {
                    request_value = match identifiers.get_id(507, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    };
                    request_value_type = "string";
                },
                "id#508" => {
                    request_value = match identifiers.get_id(508, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    };
                    request_value_type = "string";
                },
                "id#509" => {
                    request_value = match identifiers.get_id(509, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    };
                    request_value_type = "string";
                },
                "id#510" => {
                    request_value = match identifiers.get_id(510, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    };
                    request_value_type = "string";
                },
                "id#511" => {
                    request_value = match identifiers.get_id(511, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    };
                    request_value_type = "string";
                },
                "id#512" => {
                    request_value = match identifiers.get_id(512, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    };
                    request_value_type = "string";
                },
                "id#513" => {
                    request_value = match identifiers.get_id(513, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    };
                    request_value_type = "string";
                },
                "id#514" => {
                    request_value = match identifiers.get_id(514, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    };
                    request_value_type = "string";
                },
                "id#515" => {
                    request_value = match identifiers.get_id(515, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    };
                    request_value_type = "string";
                },
                "id#516" => {
                    request_value = match identifiers.get_id(516, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    };
                    request_value_type = "string";
                },
                "id#517" => {
                    request_value = match identifiers.get_id(517, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    };
                    request_value_type = "string";
                },
                "id#518" => {
                    request_value = match identifiers.get_id(518, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    };
                    request_value_type = "string";
                },
                "id#519" => {
                    request_value = match identifiers.get_id(519, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    };
                    request_value_type = "string";
                },
                "id#520" => {
                    request_value = match identifiers.get_id(520, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    };
                    request_value_type = "string";
                },
                "id#521" => {
                    request_value = match identifiers.get_id(521, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    };
                    request_value_type = "string";
                },
                "id#522" => {
                    request_value = match identifiers.get_id(522, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    };
                    request_value_type = "string";
                },
                "id#523" => {
                    request_value = match identifiers.get_id(523, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    };
                    request_value_type = "string";
                },
                "id#524" => {
                    request_value = match identifiers.get_id(524, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    };
                    request_value_type = "string";
                },
                "id#525" => {
                    request_value = match identifiers.get_id(525, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    };
                    request_value_type = "string";
                },
                "id#526" => {
                    request_value = match identifiers.get_id(526, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    };
                    request_value_type = "string";
                },
                "id#527" => {
                    request_value = match identifiers.get_id(527, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    };
                    request_value_type = "string";
                },
                "id#528" => {
                    request_value = match identifiers.get_id(528, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    };
                    request_value_type = "string";
                },
                "id#513#2025" => {
                    let mut caid = match identifiers.get_id(513, 0) {
                        Some(uid) => {
                            match &uid.ver {
                                Some(ver) => {
                                    if ver.len() >= 4 && &ver[0..4] >= "2025" {
                                        Some(uid.id.clone())
                                    } else {
                                        None
                                    }
                                },
                                None => None,
                            }
                        },
                        None => None,
                    };
                    if caid.is_none() {
                        caid = match identifiers.get_id(513, 1) {
                            Some(uid) => {
                                match &uid.ver {
                                    Some(ver) => {
                                    if ver.len() >= 4 && &ver[0..4] >= "2025" {
                                            Some(uid.id.clone())
                                        } else {
                                            None
                                        }
                                    },
                                    None => None,
                                }
                            },
                            None => None,
                        };
                    }

                    request_value = caid;
                    request_value_type = "string";
                },
                _ => (),
            };

            match request_value_type {
                "string" => {
                    match rule.operator.as_str() {
                        "is not null" => {
                            match request_value {
                                Some(request_value) => {
                                    if request_value.len() > 0 {
                                        true
                                    } else {
                                        false
                                    }
                                }
                                None => false,
                            }
                        },
                        "=" => {
                            match request_value {
                                Some(request_value) => {
                                    match &rule.value {
                                        Some(Options::String(rule_value)) => {
                                            request_value.as_str() == rule_value.as_str()
                                        },
                                        _ => true,
                                    }
                                }
                                None => false,
                            }
                        },
                        "!=" => {
                            match request_value {
                                Some(request_value) => {
                                    match &rule.value {
                                        Some(Options::String(rule_value)) => {
                                            request_value.as_str() != rule_value.as_str()
                                        },
                                        _ => true,
                                    }
                                }
                                None => false,
                            }
                        },
                        "contains" => {
                            match request_value {
                                Some(request_value) => {
                                    match &rule.value {
                                        Some(Options::String(rule_value)) => {
                                            if rule_value.len() > 0 {
                                                request_value.contains(rule_value.as_str())
                                            } else {
                                                true
                                            }
                                        },
                                        _ => true,
                                    }
                                }
                                None => false,
                            }
                        },
                        "like" => {
                            match request_value {
                                Some(request_value) => {
                                    match &rule.value {
                                        Some(Options::String(rule_value)) => {
                                            if rule_value.len() > 0 {
                                                WildMatch::new(&rule_value).matches(request_value.as_str())
                                            } else {
                                                true
                                            }
                                        },
                                        _ => true,
                                    }
                                }
                                None => false,
                            }
                        },
                        _ => true,
                    }
                },
                "category" => {
                    match rule.operator.as_str() {
                        "is not null" => {
                            match request_value {
                                Some(request_value) => {
                                    if request_value.len() > 0 {
                                        true
                                    } else {
                                        false
                                    }
                                }
                                None => false,
                            }
                        },
                        "=" => {
                            match request_value {
                                Some(request_value) => {
                                    match &rule.value {
                                        Some(Options::String(rule_value)) => {
                                            request_value.as_str() == rule_value.as_str()
                                        },
                                        _ => true,
                                    }
                                }
                                None => false,
                            }
                        },
                        "!=" => {
                            match request_value {
                                Some(request_value) => {
                                    match &rule.value {
                                        Some(Options::String(rule_value)) => {
                                            request_value.as_str() != rule_value.as_str()
                                        },
                                        _ => true,
                                    }
                                }
                                None => false,
                            }
                        },
                        "in" => {
                            match request_value {
                                Some(request_value) => {
                                    match &rule.value {
                                        Some(Options::Category(rule_value)) => {
                                            if rule_value.len() > 0 {
                                                rule_value.contains(&request_value)
                                            } else {
                                                false
                                            }
                                        },
                                        _ => true,
                                    }
                                }
                                None => false,
                            }
                        },
                        "not in" => {
                            match request_value {
                                Some(request_value) => {
                                    match &rule.value {
                                        Some(Options::Category(rule_value)) => {
                                            if rule_value.len() > 0 {
                                                !rule_value.contains(&request_value)
                                            } else {
                                                true
                                            }
                                        },
                                        _ => true,
                                    }
                                }
                                None => false,
                            }
                        },
                        _ => true,
                    }
                },
                "number" => {
                    match rule.operator.as_str() {
                        "is not null" => {
                            match request_value {
                                Some(request_value) => {
                                    match request_value.parse::<f64>() {
                                        Ok(_) => {
                                            true
                                        },
                                        Err(_) => {
                                            false
                                        },
                                    }
                                }
                                None => false,
                            }
                        },
                        "=" => {
                            match request_value {
                                Some(request_value) => {
                                    match &rule.value {
                                        Some(Options::Number(rule_value)) => {
                                            match request_value.parse::<f64>() {
                                                Ok(request_value_n) => {
                                                    request_value_n == *rule_value
                                                },
                                                Err(_) => false,
                                            }
                                        },
                                        _ => true,
                                    }
                                }
                                None => false,
                            }
                        },
                        "!=" => {
                            match request_value {
                                Some(request_value) => {
                                    match &rule.value {
                                        Some(Options::Number(rule_value)) => {
                                            match request_value.parse::<f64>() {
                                                Ok(request_value_n) => {
                                                    request_value_n != *rule_value
                                                },
                                                Err(_) => false,
                                            }
                                        },
                                        _ => true,
                                    }
                                }
                                None => false,
                            }
                        },
                        ">" => {
                            match request_value {
                                Some(request_value) => {
                                    match &rule.value {
                                        Some(Options::Number(rule_value)) => {
                                            match request_value.parse::<f64>() {
                                                Ok(request_value_n) => {
                                                    request_value_n > *rule_value
                                                },
                                                Err(_) => false,
                                            }
                                        },
                                        _ => true,
                                    }
                                }
                                None => false,
                            }
                        },
                        ">=" => {
                            match request_value {
                                Some(request_value) => {
                                    match &rule.value {
                                        Some(Options::Number(rule_value)) => {
                                            match request_value.parse::<f64>() {
                                                Ok(request_value_n) => {
                                                    request_value_n >= *rule_value
                                                },
                                                Err(_) => false,
                                            }
                                        },
                                        _ => true,
                                    }
                                }
                                None => false,
                            }
                        },
                        "<" => {
                            match request_value {
                                Some(request_value) => {
                                    match &rule.value {
                                        Some(Options::Number(rule_value)) => {
                                            match request_value.parse::<f64>() {
                                                Ok(request_value_n) => {
                                                    request_value_n < *rule_value
                                                },
                                                Err(_) => false,
                                            }
                                        },
                                        _ => true,
                                    }
                                }
                                None => false,
                            }
                        },
                        "<=" => {
                            match request_value {
                                Some(request_value) => {
                                    match &rule.value {
                                        Some(Options::Number(rule_value)) => {
                                            match request_value.parse::<f64>() {
                                                Ok(request_value_n) => {
                                                    request_value_n <= *rule_value
                                                },
                                                Err(_) => false,
                                            }
                                        },
                                        _ => true,
                                    }
                                }
                                None => false,
                            }
                        },
                        _ => true,
                    }
                },
                "boolean" => {
                    match rule.operator.as_str() {
                        "is not null" => {
                            match request_value {
                                Some(request_value) => {
                                    match request_value.parse::<bool>() {
                                        Ok(_) => {
                                            true
                                        },
                                        Err(_) => {
                                            false
                                        },
                                    }
                                }
                                None => false,
                            }
                        },
                        "=" => {
                            match request_value {
                                Some(request_value) => {
                                    match &rule.value {
                                        Some(Options::Boolean(rule_value)) => {
                                            match request_value.parse::<bool>() {
                                                Ok(request_value_b) => {
                                                    request_value_b == *rule_value
                                                },
                                                Err(_) => false,
                                            }
                                        },
                                        _ => true,
                                    }
                                },
                                None => false,
                            }
                        },
                        _ => true,
                    }
                },
                _ => true,
            }
        },
        Entry::RuleSet(rule_set) => {
            match_rule_set(request, rule_set)
        },
    }
}

fn match_rule_set(request: &Request, rule_set: &RuleSet) -> bool {
    let mut result = true;

    match rule_set.condition.as_str() {
        "and" => {
            result = true;
            for rule in rule_set.rules.iter() {
                result &= match_rule(request, rule);
            }
        },
        "or" => {
            result = false;
            for rule in rule_set.rules.iter() {
                result |= match_rule(request, rule);
            }
        },
        _ => (),
    }

    result
}

fn match_key_field(request: &Request, connection: &Connection) -> bool {
    match &connection.filter {
        Some(filter) => {
            match_rule_set(request, filter)
        },
        None => {
            true
        },
    }
}

async fn win(
    State((database, cache, _traffic, pool)): State<(Database, Cache, Traffic, HttpPool)>,
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
                            let vendor_win_price = win_price as f64;
                            let client_win_price = Price::to_client(&connection, Some(win_price as f64)) as i32;
                            let client_next_price = {
                                match next_price_message {
                                    Some(next_price_message) => {
                                        let next_price = decrypt(next_price_message, &connection.vendor_ekey, &connection.vendor_ikey);
                                        match next_price {
                                            Some(next_price) => {
                                                Price::to_client(&connection, Some(next_price as f64)) as i32
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
                                                bidding_notify_win(url.clone(), client_win_price, client_next_price, &"".to_string(), &connection, &pool).await;
                                            }
                                        }
                                    }
                                },
                                None => (),
                            }

                            let bundle = cache.get_bundle(&request_id).unwrap_or("UNKNOWN".to_string());

                            cache.update_performance(connection.client_port, connection.vendor_port, &bundle, PERFORMANCE_BIDDING_WIN);

                            // update the final price from win notice
                            let upstream_price = Price::to_upstream(&connection, Some(client_win_price as f64));
                            let rebate_price = Price::to_rebate(&connection, Some(client_win_price as f64));
                            cache.set_notification_cost(&request_id, connection.client_port, connection.vendor_port, client_win_price, upstream_price, rebate_price, vendor_win_price);
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
    State((database, cache, _traffic, pool)): State<(Database, Cache, Traffic, HttpPool)>,
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
                                Price::to_client(&connection, Some(lose_price as f64)) as i32
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
                            bidding_notify_lose(url.clone(), client_lose_price, lose_reason, &lose_adn_name, &"".to_string(), &connection, &pool).await;
                        }
                    }

                    let bundle = cache.get_bundle(&request_id).unwrap_or("UNKNOWN".to_string());

                    cache.update_performance(connection.client_port, connection.vendor_port, &bundle, PERFORMANCE_BIDDING_LOSE);
                },
                None => (),
            }
        },
        None => (),
    }
}

async fn query(request: &Request, connection: &Connection, pool: &HttpPool, cache: &Cache) -> Result<Response, ResultMessage> {
    match connection.client_code.as_str() {
        "dummy" => Dummy::request(request, connection, pool, cache).await,
        "adwanji" => Adwanji::request(request, connection, pool, cache).await,
        "billowlink" => Billowlink::request(request, connection, pool, cache).await,
        "fanglin" => Fanglin::request(request, connection, pool, cache).await,
        "fwb" => Fwb::request(request, connection, pool, cache).await,
        "huichuan" => Huichuan::request(request, connection, pool, cache).await,
        "jmedium" => Jmedium::request(request, connection, pool, cache).await,
        "kaka" => Kaka::request(request, connection, pool, cache).await,
        "kkmh" => Kkmh::request(request, connection, pool, cache).await,
        "leidong" => Leidong::request(request, connection, pool, cache).await,
        "mfocus" => Mfocus::request(request, connection, pool, cache).await,
        "mobrtb" => Mobrtb::request(request, connection, pool, cache).await,
        "mygolbs" => Mygolbs::request(request, connection, pool, cache).await,
        "richmob" => Richmob::request(request, connection, pool, cache).await,
        "ruiang" => Ruiang::request(request, connection, pool, cache).await,
        "spinview" => Spinview::request(request, connection, pool, cache).await,
        "sweet" => Sweet::request(request, connection, pool, cache).await,
        "ustars" => Ustars::request(request, connection, pool, cache).await,
        "yiba" => Yiba::request(request, connection, pool, cache).await,
        "yiwei" => Yiwei::request(request, connection, pool, cache).await,
        "zhanqing" => Zhanqing::request(request, connection, pool, cache).await,
        &_ => Err(ResultMessage {
            code: 999,
            message: "unknown client code".to_string(),
        }),
    }
}

async fn bidding_notify_win(url: String, win_price: i32, next_price: i32, iv: &String, connection: &Connection, pool: &HttpPool) {
    match connection.client_code.as_str() {
        "dummy" => Dummy::bidding_notify_win(url, win_price, next_price, iv, connection, pool).await,
        "adwanji" => Adwanji::bidding_notify_win(url, win_price, next_price, iv, connection, pool).await,
        "billowlink" => Billowlink::bidding_notify_win(url, win_price, next_price, iv, connection, pool).await,
        "fanglin" => Fanglin::bidding_notify_win(url, win_price, next_price, iv, connection, pool).await,
        "fwb" => Fwb::bidding_notify_win(url, win_price, next_price, iv, connection, pool).await,
        "huichuan" => Huichuan::bidding_notify_win(url, win_price, next_price, iv, connection, pool).await,
        "jmedium" => Jmedium::bidding_notify_win(url, win_price, next_price, iv, connection, pool).await,
        "kaka" => Kaka::bidding_notify_win(url, win_price, next_price, iv, connection, pool).await,
        "kkmh" => Kkmh::bidding_notify_win(url, win_price, next_price, iv, connection, pool).await,
        "leidong" => Leidong::bidding_notify_win(url, win_price, next_price, iv, connection, pool).await,
        "mfocus" => Mfocus::bidding_notify_win(url, win_price, next_price, iv, connection, pool).await,
        "mobrtb" => Mobrtb::bidding_notify_win(url, win_price, next_price, iv, connection, pool).await,
        "mygolbs" => Mygolbs::bidding_notify_win(url, win_price, next_price, iv, connection, pool).await,
        "richmob" => Richmob::bidding_notify_win(url, win_price, next_price, iv, connection, pool).await,
        "ruiang" => Ruiang::bidding_notify_win(url, win_price, next_price, iv, connection, pool).await,
        "spinview" => Spinview::bidding_notify_win(url, win_price, next_price, iv, connection, pool).await,
        "sweet" => Sweet::bidding_notify_win(url, win_price, next_price, iv, connection, pool).await,
        "ustars" => Ustars::bidding_notify_win(url, win_price, next_price, iv, connection, pool).await,
        "yiba" => Yiba::bidding_notify_win(url, win_price, next_price, iv, connection, pool).await,
        "yiwei" => Yiwei::bidding_notify_win(url, win_price, next_price, iv, connection, pool).await,
        "zhanqing" => Zhanqing::bidding_notify_win(url, win_price, next_price, iv, connection, pool).await,
        &_ => (),
    }
}

async fn bidding_notify_lose(url: String, lose_price: i32, lose_reason: i32, lose_adn_name: &String, iv: &String, connection: &Connection, pool: &HttpPool) {
    match connection.client_code.as_str() {
        "dummy" => Dummy::bidding_notify_lose(url, lose_price, lose_reason, lose_adn_name, iv, connection, pool).await,
        "adwanji" => Adwanji::bidding_notify_lose(url, lose_price, lose_reason, lose_adn_name, iv, connection, pool).await,
        "billowlink" => Billowlink::bidding_notify_lose(url, lose_price, lose_reason, lose_adn_name, iv, connection, pool).await,
        "fanglin" => Fanglin::bidding_notify_lose(url, lose_price, lose_reason, lose_adn_name, iv, connection, pool).await,
        "fwb" => Fwb::bidding_notify_lose(url, lose_price, lose_reason, lose_adn_name, iv, connection, pool).await,
        "huichuan" => Huichuan::bidding_notify_lose(url, lose_price, lose_reason, lose_adn_name, iv, connection, pool).await,
        "jmedium" => Jmedium::bidding_notify_lose(url, lose_price, lose_reason, lose_adn_name, iv, connection, pool).await,
        "kaka" => Kaka::bidding_notify_lose(url, lose_price, lose_reason, lose_adn_name, iv, connection, pool).await,
        "kkmh" => Kkmh::bidding_notify_lose(url, lose_price, lose_reason, lose_adn_name, iv, connection, pool).await,
        "leidong" => Leidong::bidding_notify_lose(url, lose_price, lose_reason, lose_adn_name, iv, connection, pool).await,
        "mfocus" => Mfocus::bidding_notify_lose(url, lose_price, lose_reason, lose_adn_name, iv, connection, pool).await,
        "mobrtb" => Mobrtb::bidding_notify_lose(url, lose_price, lose_reason, lose_adn_name, iv, connection, pool).await,
        "mygolbs" => Mygolbs::bidding_notify_lose(url, lose_price, lose_reason, lose_adn_name, iv, connection, pool).await,
        "richmob" => Richmob::bidding_notify_lose(url, lose_price, lose_reason, lose_adn_name, iv, connection, pool).await,
        "ruiang" => Ruiang::bidding_notify_lose(url, lose_price, lose_reason, lose_adn_name, iv, connection, pool).await,
        "spinview" => Spinview::bidding_notify_lose(url, lose_price, lose_reason, lose_adn_name, iv, connection, pool).await,
        "sweet" => Sweet::bidding_notify_lose(url, lose_price, lose_reason, lose_adn_name, iv, connection, pool).await,
        "ustars" => Ustars::bidding_notify_lose(url, lose_price, lose_reason, lose_adn_name, iv, connection, pool).await,
        "yiba" => Yiba::bidding_notify_lose(url, lose_price, lose_reason, lose_adn_name, iv, connection, pool).await,
        "yiwei" => Yiwei::bidding_notify_lose(url, lose_price, lose_reason, lose_adn_name, iv, connection, pool).await,
        "zhanqing" => Zhanqing::bidding_notify_lose(url, lose_price, lose_reason, lose_adn_name, iv, connection, pool).await,
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
