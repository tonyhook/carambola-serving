use std::{io::Write, time::Duration};

use chrono::TimeZone;
use chrono_tz::Tz;
use flate2::{Compression, write::GzEncoder};

use crate::{protocol::*, Assets, Cache, Client, Connection, HttpPool, Identifiers, Price, ResultMessage};

pub mod adm;
pub mod app;
pub mod appinfo;
pub mod banner_format;
pub mod bid;
pub mod caid;
pub mod device_ext;
pub mod device;
pub mod geo;
pub mod imp_ext;
pub mod imp;
pub mod monitor;
pub mod native_format;
pub mod request;
pub mod response_wrapper;
pub mod response;
pub mod seatbid;
pub mod user;
pub mod video_format;
pub mod video;

pub use adm::KkmhAdm;
pub use app::KkmhApp;
pub use appinfo::KkmhAppinfo;
pub use banner_format::KkmhBannerFormat;
pub use bid::KkmhBid;
pub use caid::KkmhCaid;
pub use device_ext::KkmhDeviceExt;
pub use device::KkmhDevice;
pub use geo::KkmhGeo;
pub use imp_ext::KkmhImpExt;
pub use imp::KkmhImp;
pub use monitor::KkmhMonitor;
pub use native_format::KkmhNativeFormat;
pub use request::KkmhRequest;
pub use response_wrapper::KkmhResponseWrapper;
pub use response::KkmhResponse;
pub use seatbid::KkmhSeatbid;
pub use user::KkmhUser;
pub use video_format::KkmhVideoFormat;
pub use video::KkmhVideo;

pub struct Kkmh {

}

impl Client for Kkmh {

    async fn request(request: &Request, connection: &Connection, pool: &HttpPool, cache: &Cache) -> Result<Response, ResultMessage> {
        let request_id = cache.get_sequence();

        let mut assets = Assets::new(request);
        let identifiers = Identifiers::new(request);

        let request_kkmh = KkmhRequest {
            id: {
                request_id.to_string()
            },
            imps: {
                let mut imp_kkmh = KkmhImp {
                    id: request.item[0].id.clone(),
                    ba: None,
                    vd: None,
                    na: None,
                    tagid: connection.client_tag_id.clone(),
                    bidfloor: Some(Price::to_client(connection, request.item[0].flr.map(f64::from)) as i32),
                    ad_num: None,
                    ext: None,
                };

                let mut ad_types = vec![];

                let reward = request.item[0].spec.reward;
                let pos = request.item[0].spec.display.pos;
                let instl = request.item[0].spec.display.instl;

                match &request.item[0].spec.display.displayfmt {
                    Some(_displayfmt) => {
                        ad_types.push(
                            match instl {
                                0 => 2,
                                1 => 6,
                                _ => 0,
                            }
                        );
                    },
                    None => (),
                }

                match &request.item[0].spec.display.nativefmt {
                    Some(_nativefmt) => {
                        match pos {
                            Some(501) => {
                                ad_types.push(1);
                            },
                            _ => {
                                if assets.get_asset_size("video") > 0 {
                                    if reward > 0 {
                                        ad_types.push(3);
                                    } else {
                                        ad_types.push(4);
                                    }
                                }
                                if assets.get_asset_size("img") > 0 || assets.get_asset_size("thumb") > 0 {
                                    match instl {
                                        0 => {
                                            ad_types.push(2);
                                        },
                                        1 => {
                                            ad_types.push(6);
                                        },
                                        _ => {
                                            ad_types.push(0);
                                        },
                                    }
                                }
                            },
                        }
                    },
                    None => (),
                }

                imp_kkmh.ext = Some(KkmhImpExt {
                    inventory_types: None,
                    ad_types: Some(ad_types),
                    tag_name: None,
                });

                [imp_kkmh].to_vec()
            },
            app: KkmhApp {
                id: {
                    None
                },
                name: {
                    match &connection.client_media_appname {
                        Some(client_media_appname) => client_media_appname.clone(),
                        None => {
                            match &request.context.app {
                                Some(app) => app.name.clone(),
                                None => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.context.app is required for upstream".to_string(),
                                }),
                            }
                        },
                    }
                },
                bundle: {
                    match &connection.client_media_apppackage {
                        Some(client_media_apppackage) => client_media_apppackage.clone(),
                        None => {
                            match &request.context.app {
                                Some(app) => {
                                    match &app.bundle {
                                        Some(bundle) => bundle.clone(),
                                        None => return Err(ResultMessage {
                                            code: 998,
                                            message: "request.context.app.bundle is required for upstream".to_string(),
                                        }),
                                    }
                                }
                                None => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.context.app is required for upstream".to_string(),
                                }),
                            }
                        },
                    }
                },
                version: {
                    match &request.context.app {
                        Some(app) => {
                            match &app.ver {
                                Some(ver) => ver.clone(),
                                None => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.context.app.ver is required for upstream".to_string(),
                                }),
                            }
                        }
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.app is required for upstream".to_string(),
                        }),
                    }
                },
                keywords: {
                    None
                },
            },
            device: KkmhDevice {
                ua: {
                    request.context.device.ua.clone()
                },
                geo: {
                    match &request.context.device.geo {
                        Some(geo) => Some(KkmhGeo {
                            lat: {
                                match geo.lat {
                                    Some(lat) => lat,
                                    None => return Err(ResultMessage {
                                        code: 998,
                                        message: "request.context.device.geo.lat is required for upstream".to_string(),
                                    }),
                                }
                            },
                            lon: {
                                match geo.lon {
                                    Some(lon) => lon,
                                    None => return Err(ResultMessage {
                                        code: 998,
                                        message: "request.context.device.geo.lon is required for upstream".to_string(),
                                    }),
                                }
                            },
                            country: geo.country.clone(),
                            province: geo.province.clone(),
                            city: geo.city.clone(),
                        }),
                        None => None,
                    }
                },
                ip: {
                    match &request.context.device.ip {
                        Some(ip) => ip.clone(),
                        None => {
                            match &request.context.device.ipv6  {
                                Some(ipv6) => ipv6.clone(),
                                None => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.context.device.ip is required for upstream".to_string(),
                                }),
                            }
                        },
                    }
                },
                devt: {
                    match &request.context.device.devicetype {
                        Some(4) => 1,
                        Some(5) => 2,
                        Some(_) => 3,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.devicetype is required for upstream".to_string(),
                        }),
                    }
                },
                make: {
                    request.context.device.make.clone()
                },
                model: {
                    request.context.device.model.clone()
                },
                os: {
                    match &request.context.device.os {
                        Some(2) => "Android".to_string(),
                        Some(13) => "iOS".to_string(),
                        _ => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.os should be 2/13 for upstream".to_string(),
                        }),
                    }
                },
                osv: {
                    match &request.context.device.osv {
                        Some(osv) => osv.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.osv is required for upstream".to_string(),
                        }),
                    }
                },
                hwv: {
                    request.context.device.hwv.clone()
                },
                w: {
                    request.context.device.w.clone()
                },
                h: {
                    request.context.device.h.clone()
                },
                ppi: {
                    request.context.device.ppi.clone()
                },
                ct: {
                    match &request.context.device.contype {
                        Some(2) => Some(20),
                        Some(4) => Some(2),
                        Some(5) => Some(3),
                        Some(6) => Some(4),
                        Some(7) => Some(5),
                        Some(_) => Some(0),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.contype is required for upstream".to_string(),
                        }),
                    }
                },
                ca: {
                    match &request.context.device.carrier {
                        Some(carrier) => {
                            match carrier.as_str() {
                                "cmcc" => 1,
                                "unicom" => 3,
                                "telecom" => 4,
                                _ => 20,
                            }
                        },
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.carrier is required for upstream".to_string(),
                        }),
                    }

                },
                imei: {
                    match identifiers.get_id(501, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                imeimd5: {
                    match identifiers.get_id(502, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                oaid: {
                    match identifiers.get_id(505, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                idfa: {
                    match identifiers.get_id(507, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                idfamd5: {
                    match identifiers.get_id(508, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                paid: {
                    match identifiers.get_id(519, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                caids: {
                    match identifiers.get_ids(513) {
                        Some(uids) => {
                            let mut caids = vec![];
                            for uid in uids {
                                caids.push(KkmhCaid {
                                    caid: uid.id.clone(),
                                    caid_version: {
                                        match &uid.ver {
                                            Some(ver) => ver.clone(),
                                            None => "".to_string(),
                                        }
                                    }
                                });
                            }
                            Some(caids)
                        },
                        None => None,
                    }
                },
                mac: {
                    match identifiers.get_id(511, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                macmd5: {
                    match identifiers.get_id(512, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                boot_mark: {
                    request.context.device.bootmark.clone()
                },
                update_mark: {
                    request.context.device.updatemark.clone()
                },
                device_init_sec: {
                    match &request.context.device.inittime {
                        Some(inittime) => Some(inittime.split(".").nth(0).unwrap().to_string()),
                        None => {
                            match &request.context.device.birthtime {
                                Some(birthtime) => Some(birthtime.split(".").nth(0).unwrap().to_string()),
                                None => None,
                            }
                        },
                    }
                },
                device_start_sec: {
                    match &request.context.device.boottime {
                        Some(boottime) => Some(boottime.split(".").nth(0).unwrap().to_string()),
                        None => None,
                    }
                },
                device_name_md5: {
                    match identifiers.get_id(528, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                hardware_machine: {
                    request.context.device.hwmachine.clone()
                },
                physical_memory_byte: {
                    match request.context.device.sysmemory {
                        Some(sysmemory) => Some(sysmemory.to_string()),
                        None => None,
                    }
                },
                harddisk_size_byte: {
                    match &request.context.device.sysdisksize {
                        Some(sysdisksize) => Some(sysdisksize.to_string()),
                        None => None,
                    }
                },
                system_update_sec: {
                    match &request.context.device.updatetime {
                        Some(updatetime) => Some(updatetime.split(".").nth(0).unwrap().to_string()),
                        None => None,
                    }
                },
                hardware_model: {
                    request.context.device.hwmodel.clone()
                },
                language: {
                    request.context.device.lang.clone()
                },
                time_zone: {
                    match &request.context.device.timezone {
                        Some(timezone) => {
                            let tz: Result<Tz, chrono_tz::ParseError> = timezone.parse();
                            match tz {
                                Ok(tz) => {
                                    let t = tz.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
                                    let utc = chrono_tz::UTC.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
                                    Some((t.timestamp() - utc.timestamp()).to_string())
                                },
                                Err(_) => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.context.device.timezone is malformat for upstream, should be like Asia/Shanghai".to_string(),
                                }),
                            }
                        },
                        None => None,
                    }
                },
                ext: Some(KkmhDeviceExt {
                    android_id: {
                        match identifiers.get_id(509, 0) {
                            Some(uid) => Some(uid.id.clone()),
                            None => None,
                        }
                    },
                    android_id_md5: {
                        match identifiers.get_id(510, 0) {
                            Some(uid) => Some(uid.id.clone()),
                            None => None,
                        }
                    },
                    idfv: {
                        match identifiers.get_id(515, 0) {
                            Some(uid) => Some(uid.id.clone()),
                            None => None,
                        }
                    },
                    orientation: {
                        match request.context.device.orientation {
                            Some(orientation) => {
                                match orientation {
                                    501 => Some(1),
                                    502 => Some(2),
                                    _ => Some(0),
                                }
                            },
                            None => None,
                        }
                    },
                }),
            },
            user: Some(KkmhUser {
                id: {
                    None
                },
                yob: {
                    request.context.user.yob
                },
                gender: {
                    match &request.context.user.gender {
                        Some(gender) => {
                            match gender.as_str() {
                                "M" => Some(1),
                                "F" => Some(2),
                                "O" => Some(0),
                                _ => Some(0),
                            }
                        }
                        None => Some(0),
                    }
                },
                keywords: {
                    match &request.context.user.keywords {
                        Some(keywords) => Some(keywords.split(",").map(|s| s.to_string()).collect()),
                        None => None,
                    }
                },
                geo: {
                    None
                },
            }),
            tmax: None,
            support_https: {
                Some(1)
            },
        };

        let json_string = serde_json::to_vec(&request_kkmh).unwrap();
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&json_string).unwrap();
        let compressed_bytes = encoder.finish().unwrap();

        let response_kkmh: KkmhResponse;

        let client = {
            let pool_kkmh_lock = pool.pool_kkmh.clone();
            let pool_kkmh = pool_kkmh_lock.read().unwrap();
            pool_kkmh.clone()
        };
        let response_kkmh_raw = client.post("https://api.kkmh.com/ad/union/api/req/")
            .body(compressed_bytes)
            .header("Accept-Encoding", "gzip")
            .header("Connection", "keep-alive")
            .header("Content-Encoding", "gzip")
            .header("Content-Type", "application/json")
            .timeout(Duration::from_millis(connection.timeout))
            .send().await;

        match response_kkmh_raw {
            Ok(response_kkmh_raw) => {
                let status = response_kkmh_raw.status();
                if status != 200 {
                    return Err(ResultMessage {
                        code: 992,
                        message: {
                            match response_kkmh_raw.text().await {
                                Ok(text) => format!("upstream error {}: {}", status, text),
                                Err(_) => format!("upstream error {}", status),
                            }
                        },
                    });
                } else {
                    match response_kkmh_raw.text().await {
                        Ok(text) => {
                            match serde_json::from_str::<KkmhResponseWrapper>(&text) {
                                Ok(json) => {
                                    match json.code {
                                        200 => {
                                            match json.data {
                                                Some(data) => {
                                                    response_kkmh = data;
                                                },
                                                None => {
                                                    return Err(ResultMessage {
                                                        code: 993,
                                                        message: "upstream error: no data".to_string(),
                                                    });
                                                },
                                            }
                                        },
                                        204 => {
                                            return Err(ResultMessage {
                                                code: 993,
                                                message: "".to_string(),
                                            });
                                        },
                                        1002 => {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: format!("upstream error: {}", &json.message.unwrap_or("lost key parameter".to_string())),
                                            });
                                        },
                                        1003 => {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: format!("upstream error: {}", &json.message.unwrap_or("invalid value".to_string())),
                                            });
                                        },
                                        1004 => {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: format!("upstream error: {}", &json.message.unwrap_or("invalid input".to_string())),
                                            });
                                        },
                                        1005 => {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: format!("upstream error: {}", json.message.unwrap()),
                                            });
                                        },
                                        1006 => {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: "".to_string(),
                                            });
                                        },
                                        _ => {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: "upstream error: unknown code".to_string(),
                                            });
                                        },
                                    }
                                },
                                Err(error) => {
                                    return Err(ResultMessage {
                                        code: 997,
                                        message: error.to_string(),
                                    });
                                },
                            }
                        },
                        Err(error) => {
                            return Err(ResultMessage {
                                code: 992,
                                message: error.to_string(),
                            });
                        },
                    }
                }
            },
            Err(error) => {
                if error.is_timeout() {
                    return Err(ResultMessage {
                        code: 991,
                        message: format!("upstream request timeout"),
                    });
                } else {
                    return Err(ResultMessage {
                        code: 992,
                        message: format!("upstream request failed: {:?}", error),
                    });
                }
            }
        }

        let response = Response {
            id: request.id.clone(),
            nbr: None,
            seatbid: {
                let mut seatbids = vec![];

                for seatbid_kkmh in &response_kkmh.seatbids {
                    let mut bid = vec![];

                    let link_asset = LinkAsset {
                        linktype: {
                            match seatbid_kkmh.bids[0].interact_type {
                                0 => 1,
                                1 => 2,
                                2 => 3,
                                3 => 1,
                                _ => 1,
                            }
                        },
                        universallink: {
                            seatbid_kkmh.bids[0].universal_link.clone()
                        },
                        storeid: None,
                        deeplink: {
                            seatbid_kkmh.bids[0].deeplink.clone()
                        },
                        quickapplink: None,
                        wechatmppath: {
                            seatbid_kkmh.bids[0].mini_app_path.clone()
                        },
                        wechatmpid: {
                            seatbid_kkmh.bids[0].mini_app_name.clone()
                        },
                        marketurl: None,
                        downloadurl: {
                            match seatbid_kkmh.bids[0].interact_type {
                                2 => Some(seatbid_kkmh.bids[0].click_url.clone()),
                                _ => None,
                            }
                        },
                        url: seatbid_kkmh.bids[0].click_url.clone(),
                        urlfb: None,
                    };

                    for bid_kkmh in &seatbid_kkmh.bids {
                        bid.push(Bid {
                            id: Some(request_id.to_string()),
                            item: request.item[0].id.clone(),
                            price: { // update later
                                bid_kkmh.price as i32
                            },
                            burl: None,
                            lurl: None,
                            media: Ad {
                                id: bid_kkmh.ad_id.clone(),
                                display: Display {
                                    w: None,
                                    h: None,
                                    banner: {
                                        if assets.get_banner_size() > 0 {
                                            match &bid_kkmh.adms[0].imgs {
                                                Some(imgs) => {
                                                    if imgs.len() > 0 {
                                                        Some(Banner {
                                                            img: {
                                                                imgs[0].clone()
                                                            },
                                                            link: Some(link_asset.clone()),
                                                        })
                                                    } else {
                                                        None
                                                    }
                                                },
                                                None => None,
                                            }
                                        } else {
                                            None
                                        }
                                    },
                                    native: {
                                        if assets.get_asset_total_size() > 0 {
                                            let mut asset_vec = vec![];

                                            match &bid_kkmh.adms[0].title {
                                                Some(title) => {
                                                    asset_vec.push(Asset {
                                                        id: assets.consume_asset("title"),
                                                        req: 1,
                                                        title: Some(TitleAsset {
                                                            text: title.clone(),
                                                            subtitle: None,
                                                            desc: {
                                                                match &bid_kkmh.adms[0].desc {
                                                                    Some(desc) => Some(desc.clone()),
                                                                    None => None,
                                                                }
                                                            },
                                                            len: Some(title.clone().len() as i32),
                                                        }),
                                                        img: None,
                                                        video: None,
                                                        data: None,
                                                        html: None,
                                                        app: None,
                                                    });
                                                },
                                                None => (),
                                            }
                                            match &bid_kkmh.adms[0].icon {
                                                Some(icon) => {
                                                    if icon.len() > 0 {
                                                        asset_vec.push(Asset {
                                                            id: assets.consume_asset("icon"),
                                                            req: 1,
                                                            title: None,
                                                            img: Some(ImageAsset {
                                                                url: icon.clone(),
                                                                mime: None,
                                                                w: None,
                                                                h: None,
                                                                imagetype: Some(1),
                                                            }),
                                                            video: None,
                                                            data: None,
                                                            html: None,
                                                            app: None,
                                                        });
                                                    }
                                                },
                                                None => (),
                                            }
                                            match &bid_kkmh.adms[0].imgs {
                                                Some(imgs) => {
                                                    if bid_kkmh.adms[0].creative_type == 0 || bid_kkmh.adms[0].creative_type == 1 {
                                                        for img in imgs {
                                                            if img.len() > 0 {
                                                                if assets.get_asset_size("img") > 0 {
                                                                    asset_vec.push(Asset {
                                                                        id: assets.consume_asset("img"),
                                                                        req: 1,
                                                                        img: {
                                                                            Some(ImageAsset {
                                                                                url: img.clone(),
                                                                                mime: None,
                                                                                w: None,
                                                                                h: None,
                                                                                imagetype: Some(3),
                                                                            })
                                                                        },
                                                                        title: None,
                                                                        video: None,
                                                                        data: None,
                                                                        html: None,
                                                                        app: None,
                                                                    });
                                                                }
                                                                if assets.get_asset_size("thumb") > 0 {
                                                                    asset_vec.push(Asset {
                                                                        id: assets.consume_asset("thumb"),
                                                                        req: 1,
                                                                        img: {
                                                                            Some(ImageAsset {
                                                                                url: img.clone(),
                                                                                mime: None,
                                                                                w: None,
                                                                                h: None,
                                                                                imagetype: Some(501),
                                                                            })
                                                                        },
                                                                        title: None,
                                                                        video: None,
                                                                        data: None,
                                                                        html: None,
                                                                        app: None,
                                                                    });
                                                                }
                                                            }
                                                        }
                                                    }
                                                    if bid_kkmh.adms[0].creative_type == 2 || bid_kkmh.adms[0].creative_type == 3 {
                                                        for img in imgs {
                                                            if img.len() > 0 {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("video#cover"),
                                                                    req: 1,
                                                                    img: {
                                                                        Some(ImageAsset {
                                                                            url: img.clone(),
                                                                            mime: None,
                                                                            w: None,
                                                                            h: None,
                                                                            imagetype: Some(3),
                                                                        })
                                                                    },
                                                                    title: None,
                                                                    video: None,
                                                                    data: None,
                                                                    html: None,
                                                                    app: None,
                                                                });
                                                            }
                                                        }
                                                    }
                                                },
                                                None => (),
                                            }
                                            match &bid_kkmh.adms[0].video {
                                                Some(video) => {
                                                    asset_vec.push(Asset {
                                                        id: assets.consume_asset("video"),
                                                        req: 1,
                                                        title: None,
                                                        img: None,
                                                        video: Some(VideoAsset {
                                                            url: video.url.clone(),
                                                            mime: None,
                                                            w: None,
                                                            h: None,
                                                            dur: None,
                                                            skipoffset: None,
                                                            size: None,
                                                            delivery: None,
                                                            orientation: None,
                                                            autolanding: 0,
                                                            clickable: 0,
                                                        }),
                                                        data: None,
                                                        html: None,
                                                        app: None,
                                                    });

                                                    match &video.cover_url {
                                                        Some(cover_url) => {
                                                            asset_vec.push(Asset {
                                                                id: assets.consume_asset("video#cover"),
                                                                req: 1,
                                                                img: {
                                                                    Some(ImageAsset {
                                                                        url: cover_url.clone(),
                                                                        mime: None,
                                                                        w: None,
                                                                        h: None,
                                                                        imagetype: Some(3),
                                                                    })
                                                                },
                                                                title: None,
                                                                video: None,
                                                                data: None,
                                                                html: None,
                                                                app: None,
                                                            });
                                                        },
                                                        None => (),
                                                    }
                                                },
                                                None => (),
                                            }

                                            match &bid_kkmh.appinfo {
                                                Some(appinfo) => {
                                                    asset_vec.push(Asset {
                                                        id: assets.consume_asset("app"),
                                                        req: 0,
                                                        app: Some(AppAsset {
                                                            name: appinfo.name.clone(),
                                                            desc: None,
                                                            descurl: None,
                                                            domain: None,
                                                            bundle: Some(appinfo.bundle.clone()),
                                                            ver: Some(appinfo.version.clone()),
                                                            developer: None,
                                                            icon: Some(appinfo.icon.clone()),
                                                            storeid: None,
                                                            storeurl: None,
                                                            paid: 0,
                                                            size: None,
                                                            md5: None,
                                                            registration: None,
                                                            privacy: None,
                                                            privacyurl: None,
                                                            permission: None,
                                                            permissionurl: None,
                                                        }),
                                                        title: None,
                                                        img: None,
                                                        video: None,
                                                        data: None,
                                                        html: None,
                                                    });
                                                },
                                                None => (),
                                            }

                                            Some(Native {
                                                asset: asset_vec,
                                                link: Some(link_asset.clone()),
                                            })
                                        } else {
                                            None
                                        }
                                    },
                                    event: {
                                        let mut event_vec = vec![];

                                        for url in &bid_kkmh.monitor.exposal_urls {
                                            event_vec.push(Event {
                                                eventtype: 501,
                                                method: 1,
                                                url: replace_macro(url),
                                                header: None,
                                                content: None,
                                            });
                                        }
                                        for url in &bid_kkmh.monitor.clk_urls {
                                            event_vec.push(Event {
                                                eventtype: 502,
                                                method: 1,
                                                url: replace_macro(url),
                                                header: None,
                                                content: None,
                                            });
                                        }
                                        match &bid_kkmh.monitor.dn_begin_urls {
                                            Some(dn_begin_urls) => {
                                                for url in dn_begin_urls {
                                                    event_vec.push(Event {
                                                        eventtype: 601,
                                                        method: 1,
                                                        url: replace_macro(url),
                                                        header: None,
                                                        content: None,
                                                    });
                                                }
                                            },
                                            None => (),
                                        }
                                        match &bid_kkmh.monitor.dn_completed_urls {
                                            Some(dn_completed_urls) => {
                                                for url in dn_completed_urls {
                                                    event_vec.push(Event {
                                                        eventtype: 602,
                                                        method: 1,
                                                        url: replace_macro(url),
                                                        header: None,
                                                        content: None,
                                                    });
                                                }
                                            },
                                            None => (),
                                        }
                                        match &bid_kkmh.monitor.in_begin_urls {
                                            Some(in_begin_urls) => {
                                                for url in in_begin_urls {
                                                    event_vec.push(Event {
                                                        eventtype: 603,
                                                        method: 1,
                                                        url: replace_macro(url),
                                                        header: None,
                                                        content: None,
                                                    });
                                                }
                                            },
                                            None => (),
                                        }
                                        match &bid_kkmh.monitor.in_completed_urls {
                                            Some(in_completed_urls) => {
                                                for url in in_completed_urls {
                                                    event_vec.push(Event {
                                                        eventtype: 604,
                                                        method: 1,
                                                        url: replace_macro(url),
                                                        header: None,
                                                        content: None,
                                                    });
                                                }
                                            },
                                            None => (),
                                        }
                                        match &bid_kkmh.monitor.app_launch_urls {
                                            Some(app_launch_urls) => {
                                                for url in app_launch_urls {
                                                    event_vec.push(Event {
                                                        eventtype: 605,
                                                        method: 1,
                                                        url: replace_macro(url),
                                                        header: None,
                                                        content: None,
                                                    });
                                                }
                                            },
                                            None => (),
                                        }

                                        event_vec
                                    }
                                },
                                advertiser: None,
                                advertisericon: None,
                            },
                        });
                    }

                    seatbids.push(Seatbid {
                        bid,
                    });
                }

                Some(seatbids)
            },
        };

        Ok(response)
    }

    async fn bidding_notify_win(_url: String, _win_price: i32, _next_price: i32, _iv: &String, _connection: &Connection, _pool: &HttpPool) {

    }

    async fn bidding_notify_lose(_url: String, _lose_price: i32, _lose_reason: i32, _lose_adn_name: &String, _iv: &String, _connection: &Connection, _pool: &HttpPool) {

    }

    fn encrypt_price(price: i32, _iv: &String, _connection: &Connection) -> String {
        price.to_string()
    }

}

fn replace_macro(orig: &String) -> String {
    let mut replaced = orig.clone();

    replaced = replaced.replace("__TS_SECOND__", "__TS_S__");

    replaced
}
