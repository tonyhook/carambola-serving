use std::{collections::HashMap, io::Write, time::Duration};

use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyInit};
use base64::prelude::*;
use chrono::{Datelike, Local, Utc};
use flate2::{Compression, write::GzEncoder};
use reqwest::Url;
use urlencoding::encode;

use crate::{protocol::*, Assets, Cache, Client, Connection, HttpPool, Identifiers, Price, ResultMessage};

type Aes128EcbEnc = ecb::Encryptor<aes::Aes128>;

pub mod ad;
pub mod request;
pub mod response;

pub use ad::YibaAd;
pub use request::YibaRequest;
pub use response::YibaResponse;

pub struct Yiba {

}

impl Client for Yiba {

    async fn request(request: &Request, connection: &Connection, pool: &HttpPool, cache: &Cache) -> Result<Response, ResultMessage> {
        let ad_id = connection.client_tag_id.split("|").nth(0).unwrap();
        let secret = connection.client_tag_id.split("|").nth(1).unwrap();

        let request_id = cache.get_sequence();

        let mut assets = Assets::new(request);
        let identifiers = Identifiers::new(request);

        let timestamp = Utc::now().timestamp_millis();
        let sign = format!("{:x}", md5::compute(format!("{}&{}", timestamp, secret).as_bytes()));

        let request_yiba = YibaRequest {
            req_id: {
                request_id.to_string()
            },
            timestamp: {
                timestamp.to_string()
            },
            version: {
                "1.0".to_string()
            },
            bid_floor: {
                Price::to_client(connection, request.item[0].flr.map(f64::from)) as i32
            },
            app_name: {
                match &connection.client_media_appname {
                    Some(client_media_appname) => client_media_appname.clone(),
                    None => {
                        match &request.context.app {
                            Some(app) => {
                                app.name.clone()
                            },
                            None => return Err(ResultMessage {
                                code: 998,
                                message: "request.context.app is required for upstream".to_string(),
                            }),
                        }
                    }
                }
            },
            app_package: {
                match &request.context.app {
                    Some(app) => {
                        match &app.bundle {
                            Some(bundle) => bundle.clone(),
                            None => return Err(ResultMessage {
                                code: 998,
                                message: "request.context.app.bundle is required for upstream".to_string(),
                            }),
                        }
                    },
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.app is required for upstream".to_string(),
                    }),
                }
            },
            app_version: {
                match &request.context.app {
                    Some(app) => {
                        match &app.ver {
                            Some(ver) => ver.clone(),
                            None => return Err(ResultMessage {
                                code: 998,
                                message: "request.context.app.ver is required for upstream".to_string(),
                            }),
                        }
                    },
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.app is required for upstream".to_string(),
                    }),
                }
            },
            os_type: {
                match request.context.device.os {
                    Some(os) => {
                        match os {
                            2 => "Android".to_string(),
                            13 => "iOS".to_string(),
                            _ => "Unknown".to_string(),
                        }
                    },
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.device.os is required for upstream".to_string(),
                    }),
                }
            },
            os_version: {
                match &request.context.device.osv {
                    Some(osv) => osv.clone(),
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.device.osv is required for upstream".to_string(),
                    }),
                }
            },
            imei: {
                match identifiers.get_id(501, 0) {
                    Some(uid) => Some(uid.id.clone()),
                    None => None,
                }
            },
            imei_md5: {
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
            oaid_md5: {
                match identifiers.get_id(506, 0) {
                    Some(uid) => Some(uid.id.clone()),
                    None => None,
                }
            },
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
            idfa: {
                match identifiers.get_id(507, 0) {
                    Some(uid) => Some(uid.id.clone()),
                    None => None,
                }
            },
            idfa_md5: {
                match identifiers.get_id(508, 0) {
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
            caid: {
                match identifiers.get_id(513, 0) {
                    Some(uid) => Some(uid.id.clone()),
                    None => None,
                }
            },
            caid_version: {
                match identifiers.get_id(513, 0) {
                    Some(uid) => uid.ver.clone(),
                    None => None,
                }
            },
            openudid: {
                None
            },
            openudid_md5: {
                None
            },
            ip: {
                request.context.device.ip.clone()
            },
            ipv6: {
                request.context.device.ipv6.clone()
            },
            user_agent: {
                request.context.device.ua.clone()
            },
            mac: {
                match identifiers.get_id(511, 0) {
                    Some(uid) => Some(uid.id.clone()),
                    None => None,
                }
            },
            brand: {
                match &request.context.device.brand {
                    Some(brand) => brand.clone(),
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.device.brand is required for upstream".to_string(),
                    }),
                }
            },
            vendor: {
                request.context.device.make.clone()
            },
            model: {
                match &request.context.device.model {
                    Some(model) => model.clone(),
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.device.model is required for upstream".to_string(),
                    }),
                }
            },
            device_type: {
                match request.context.device.devicetype {
                    Some(1) => Some(1),
                    Some(3) => Some(3),
                    Some(4) => Some(1),
                    Some(5) => Some(2),
                    Some(7) => Some(3),
                    Some(_) => Some(0),
                    None => None,
                }
            },
            orientation: {
                match request.context.device.orientation {
                    Some(501) => Some(1),
                    Some(502) => Some(2),
                    Some(_) => Some(0),
                    None => None,
                }
            },
            device_width: {
                match request.context.device.w {
                    Some(w) => w,
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.device.w is required for upstream".to_string(),
                    }),
                }
            },
            device_height: {
                match request.context.device.h {
                    Some(h) => h,
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.device.h is required for upstream".to_string(),
                    }),
                }
            },
            width: {
                match request.item[0].spec.display.w {
                    Some(w) => w,
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.item[0].spec.display.w is required for upstream".to_string(),
                    })
                }
            },
            height: {
                match request.item[0].spec.display.h {
                    Some(h) => h,
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.item[0].spec.display.h is required for upstream".to_string(),
                    }),
                }
            },
            paid: {
                match identifiers.get_id(519, 0) {
                    Some(uid) => Some(uid.id.clone()),
                    None => None,
                }
            },
            aaid: {
                match identifiers.get_id(514, 0) {
                    Some(uid) => Some(uid.id.clone()),
                    None => None,
                }
            },
            ppi: {
                match request.context.device.ppi {
                    Some(ppi) => ppi,
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.device.ppi is required for upstream".to_string(),
                    }),
                }
            },
            density: {
                match request.context.device.pxratio {
                    Some(pxratio) => Some(pxratio),
                    None => None,
                }
            },
            imsi: {
                match identifiers.get_id(503, 0) {
                    Some(uid) => Some(uid.id.clone()),
                    None => None,
                }
            },
            carrier_type: {
                match &request.context.device.carrier {
                    Some(carrier) => {
                        match carrier.as_str() {
                            "cmcc" => "mobile".to_string(),
                            "unicom" => "unicom".to_string(),
                            "telecom" => "telecom".to_string(),
                            _ => "unknown".to_string(),
                        }
                    },
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.device.carrier is required for upstream".to_string(),
                    }),
                }
            },
            connection_type: {
                match &request.context.device.contype {
                    Some(contype) => {
                        match contype {
                            1 => 0,
                            2 => 1,
                            3 => 0,
                            4 => 2,
                            5 => 3,
                            6 => 4,
                            7 => 5,
                            _ => return Err(ResultMessage {
                                code: 998,
                                message: "request.context.device.contype should be 1-7 for upstream".to_string(),
                            }),
                        }
                    },
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.device.contype is required for upstream".to_string(),
                    }),
                }
            },
            longitude: {
                match &request.context.device.geo {
                    Some(geo) => {
                        geo.lon.clone()
                    },
                    None => None,
                }
            },
            latitude: {
                match &request.context.device.geo {
                    Some(geo) => {
                        geo.lat.clone()
                    },
                    None => None,
                }
            },
            user_gender: {
                match &request.context.user.gender {
                    Some(gender) => {
                        match gender.as_str() {
                            "M" => Some(1),
                            "F" => Some(2),
                            "O" => Some(0),
                            _ => Some(0),
                        }
                    }
                    None => None,
                }
            },
            user_age: {
                match request.context.user.yob {
                    Some(yob) => {
                        let year = Local::now().year();
                        Some(year - yob)
                    },
                    None => None,
                }
            },
            user_keywords: {
                match &request.context.user.keywords {
                    Some(keywords) => Some(keywords.split(",").map(|s| s.to_string()).collect()),
                    None => None,
                }
            },
            video_type: {
                if assets.get_asset_size("video") == 1 && assets.get_asset_size("video") == assets.get_asset_total_size() {
                    let video = assets.get_current_asset("video").unwrap().video.clone().unwrap();
                    match video.mime {
                        Some(mime) => {
                            let mut video_type = vec![];
                            for mime1 in mime {
                                let type1 = mime1.split("/").nth(1);
                                match type1 {
                                    Some(type1) => {
                                        video_type.push(type1.to_string());
                                    },
                                    None => (),
                                }
                            }
                            Some(video_type)
                        },
                        None => None,
                    }
                } else {
                    None
                }
            },
            min_duration: {
                if assets.get_asset_size("video") == 1 && assets.get_asset_size("video") == assets.get_asset_total_size() {
                    let video = assets.get_current_asset("video").unwrap().video.clone().unwrap();
                    match video.mindur {
                        Some(mindur) => {
                            Some(mindur)
                        },
                        None => None,
                    }
                } else {
                    None
                }
            },
            max_duration: {
                if assets.get_asset_size("video") == 1 && assets.get_asset_size("video") == assets.get_asset_total_size() {
                    let video = assets.get_current_asset("video").unwrap().video.clone().unwrap();
                    match video.maxdur {
                        Some(maxdur) => {
                            Some(maxdur)
                        },
                        None => None,
                    }
                } else {
                    None
                }
            },
            max_length: {
                if assets.get_asset_size("video") == 1 && assets.get_asset_size("video") == assets.get_asset_total_size() {
                    let video = assets.get_current_asset("video").unwrap().video.clone().unwrap();
                    match video.maxsize {
                        Some(maxsize) => {
                            Some(maxsize)
                        },
                        None => None,
                    }
                } else {
                    None
                }
            },
            boot_mark: {
                request.context.device.bootmark.clone()
            },
            update_mark: {
                request.context.device.updatemark.clone()
            },
            app_list: {
                match &request.context.device.app {
                    Some(app) => Some(app.split(",").map(|s| s.to_string()).collect()),
                    None => None,
                }
            },
            appstore_version: {
                request.context.device.storev.clone()
            },
            hms_version: {
                request.context.device.hmsv.clone()
            },
            sys_name: {
                match identifiers.get_id(527, 0) {
                    Some(uid) => Some(uid.id.clone()),
                    None => None,
                }
            },
            sys_boot_time: {
                match &request.context.device.boottime {
                    Some(boottime) => Some(boottime.split(".").nth(0).unwrap().to_string()),
                    None => None,
                }
            },
            sys_update_time: {
                match &request.context.device.updatetime {
                    Some(updatetime) => Some(updatetime.split(".").nth(0).unwrap().to_string()),
                    None => None,
                }
            },
            sys_init_time: {
                match &request.context.device.inittime {
                    Some(inittime) => Some(inittime.split(".").nth(0).unwrap().to_string()),
                    None => None,
                }
            },
            sys_start_nano_sec: {
                request.context.device.boottime.clone()
            },
            sys_update_nano_sec: {
                request.context.device.updatetime.clone()
            },
            sys_init_nano_sec: {
                match &request.context.device.inittime {
                    Some(inittime) => Some(inittime.clone()),
                    None => None,
                }
            },
            sys_memory_size: {
                match &request.context.device.sysmemory {
                    Some(sysmemory) => Some(sysmemory.to_string()),
                    None => None,
                }
            },
            sys_disk_size: {
                match &request.context.device.sysdisksize {
                    Some(sysdisksize) => Some(sysdisksize.to_string()),
                    None => None,
                }
            },
            sys_time_zone: {
                request.context.device.timezone.clone()
            },
            hardware_machine: {
                request.context.device.hwmachine.clone()
            },
            rom_version: {
                request.context.device.romv.clone()
            },
            country: {
                request.context.device.country.clone()
            },
            language: {
                request.context.device.lang.clone()
            },
        };

        let json_string = serde_json::to_vec(&request_yiba).unwrap();
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&json_string).unwrap();
        let compressed_bytes = encoder.finish().unwrap();

        let response_yiba: YibaResponse;

        let client = {
            let pool_yiba_lock = pool.pool_yiba.clone();
            let pool_yiba = pool_yiba_lock.read().unwrap();
            pool_yiba.clone()
        };
        let response_yiba_raw = client.post(format!("{}{}", if connection.test { "http://api-test.yiba18.cn/ad/" } else { "http://api.yiba18.cn/ad/" }, ad_id))
            .body(compressed_bytes)
            .header("Accept-Encoding", "gzip")
            .header("Connection", "keep-alive")
            .header("Content-Encoding", "gzip")
            .header("Content-Type", "application/octet-stream")
            .header("Sign", sign)
            .timeout(Duration::from_millis(connection.timeout))
            .send().await;

        match response_yiba_raw {
            Ok(response_yiba_raw) => {
                let status = response_yiba_raw.status();
                if status == 204 {
                    return Err(ResultMessage {
                        code: 993,
                        message: "".to_string(),
                    });
                } else if status != 200 {
                    return Err(ResultMessage {
                        code: 992,
                        message: {
                            match response_yiba_raw.text().await {
                                Ok(text) => format!("upstream error {}: {}", status, text),
                                Err(_) => format!("upstream error {}", status),
                            }
                        },
                    });
                } else {
                    match response_yiba_raw.text().await {
                        Ok(text) => {
                            match serde_json::from_str::<YibaResponse>(&text) {
                                Ok(json) => {
                                    response_yiba = json;

                                    match response_yiba.code {
                                        2001 => {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: "upstream error: no budget".to_string(),
                                            });
                                        },
                                        _ => (),
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
                match &response_yiba.ads {
                    Some(ads) => {
                        Some([Seatbid {
                            bid: {
                                let mut bids = vec![];

                                for ad in ads {
                                    let link_asset = LinkAsset {
                                        linktype: {
                                            match ad.ad_type {
                                                1 => 1,
                                                2 => 1,
                                                3 => 2,
                                                4 => 1,
                                                5 => 1,
                                                _ => 1,
                                            }
                                        },
                                        universallink: {
                                            None
                                        },
                                        storeid: {
                                            None
                                        },
                                        deeplink: {
                                            match &ad.deeplink_url {
                                                Some(deeplink_url) => Some(deeplink_url.clone()),
                                                None => None,
                                            }
                                        },
                                        quickapplink: None,
                                        wechatmppath: {
                                            match &ad.applet_path {
                                                Some(applet_path) => Some(applet_path.clone()),
                                                None => None,
                                            }
                                        },
                                        wechatmpid: {
                                            match &ad.applet_id {
                                                Some(applet_id) => Some(applet_id.clone()),
                                                None => None,
                                            }
                                        },
                                        marketurl: {
                                            None
                                        },
                                        downloadurl: {
                                            match ad.ad_type {
                                                3 => Some(ad.url.clone()),
                                                _ => None,
                                            }
                                        },
                                        url: {
                                            ad.url.clone()
                                        },
                                        urlfb: None,
                                    };

                                    let bid = Bid {
                                        id: Some(request_id.to_string()),
                                        item: request.item[0].id.clone(),
                                        price: { // update later
                                            match ad.price {
                                                Some(price) => {
                                                    if price > 0 {
                                                        price
                                                    } else {
                                                        connection.default_price
                                                    }
                                                },
                                                None => connection.default_price,
                                            }
                                        },
                                        burl: {
                                            match &ad.win_notice {
                                                Some(win_notice) => {
                                                    let mut burl = Vec::<String>::new();
                                                    burl.push(replace_macro(win_notice));
                                                    Some(burl)
                                                },
                                                None => None,
                                            }
                                        },
                                        lurl: None,
                                        media: Ad {
                                            id: ad.bid_id.clone(),
                                            display: Display {
                                                w: {
                                                    None
                                                },
                                                h: {
                                                    None
                                                },
                                                banner: {
                                                    if assets.get_banner_size() > 0 {
                                                        Some(Banner {
                                                            img: {
                                                                let images = ad.images.clone().unwrap();
                                                                images.get(0).unwrap().to_string()
                                                            },
                                                            link: Some(link_asset.clone()),
                                                        })
                                                    } else {
                                                        None
                                                    }
                                                },
                                                native: {
                                                    if assets.get_asset_total_size() > 0 {
                                                        let mut asset_vec = vec![];

                                                        match &ad.video_url {
                                                            Some(video_url) => {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("video"),
                                                                    req: 1,
                                                                    video: Some(VideoAsset {
                                                                        url: video_url.clone(),
                                                                        mime: ad.video_type.clone(),
                                                                        w: None,
                                                                        h: None,
                                                                        dur: ad.video_duration,
                                                                        skipoffset: ad.video_skip_min_time,
                                                                        size: ad.video_size,
                                                                        delivery: {
                                                                            match ad.video_prefetch {
                                                                                Some(0) => Some(1),
                                                                                Some(1) => Some(2),
                                                                                _ => None,
                                                                            }
                                                                        },
                                                                        orientation: ad.video_orientation,
                                                                        autolanding: 0,
                                                                        clickable: 0,
                                                                    }),
                                                                    title: None,
                                                                    img: None,
                                                                    data: None,
                                                                    html: None,
                                                                    app: None,
                                                                });
                                                            },
                                                            None => (),
                                                        }
                                                        match &ad.video_end_url {
                                                            Some(video_end_url) => {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("video#end#img"),
                                                                    req: 0,
                                                                    img: Some(ImageAsset {
                                                                        url: video_end_url.clone(),
                                                                        mime: None,
                                                                        w: None,
                                                                        h: None,
                                                                        imagetype: Some(3),
                                                                    }),
                                                                    title: None,
                                                                    video: None,
                                                                    data: None,
                                                                    html: None,
                                                                    app: None,
                                                                });
                                                            },
                                                            None => (),
                                                        }
                                                        match &ad.icon_url {
                                                            Some(icon_url) => {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("video#end#button#img"),
                                                                    req: 0,
                                                                    img: Some(ImageAsset {
                                                                        url: icon_url.clone(),
                                                                        mime: None,
                                                                        w: None,
                                                                        h: None,
                                                                        imagetype: Some(1),
                                                                    }),
                                                                    title: None,
                                                                    video: None,
                                                                    data: None,
                                                                    html: None,
                                                                    app: None,
                                                                });
                                                            },
                                                            None => (),
                                                        }
                                                        match &ad.icon_text {
                                                            Some(icon_text) => {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("video#end#button#text"),
                                                                    req: 0,
                                                                    data: Some(DataAsset {
                                                                        value: icon_text.clone(),
                                                                        len: Some(icon_text.len() as i32),
                                                                        datatype: Some(12),
                                                                    }),
                                                                    title: None,
                                                                    img: None,
                                                                    video: None,
                                                                    html: None,
                                                                    app: None,
                                                                });
                                                            },
                                                            None => (),
                                                        }
                                                        match &ad.video_end_html {
                                                            Some(video_end_html) => {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("video#end#html"),
                                                                    req: 0,
                                                                    html: Some(HtmlAsset {
                                                                        html: Some(video_end_html.clone()),
                                                                        link: None,
                                                                        len: Some(video_end_html.len() as i32),
                                                                    }),
                                                                    title: None,
                                                                    img: None,
                                                                    video: None,
                                                                    data: None,
                                                                    app: None,
                                                                });
                                                            },
                                                            None => (),
                                                        }
                                                        match &ad.title {
                                                            Some(title) => {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("title"),
                                                                    req: 1,
                                                                    title: Some(TitleAsset {
                                                                        text: title.clone(),
                                                                        subtitle: None,
                                                                        desc: ad.desc.clone(),
                                                                        len: Some(title.len() as i32),
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
                                                        match &ad.images {
                                                            Some(images) => {
                                                                if assets.get_asset_size("img") > 0 {
                                                                    for image in images.iter() {
                                                                        asset_vec.push(Asset {
                                                                            id: assets.consume_asset("img"),
                                                                            req: 1,
                                                                            img: {
                                                                                Some(ImageAsset {
                                                                                    url: image.clone(),
                                                                                    mime: None,
                                                                                    w: ad.width,
                                                                                    h: ad.height,
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
                                                                if assets.get_asset_size("thumb") > 0 {
                                                                    for image in images.iter() {
                                                                        asset_vec.push(Asset {
                                                                            id: assets.consume_asset("thumb"),
                                                                            req: 1,
                                                                            img: {
                                                                                Some(ImageAsset {
                                                                                    url: image.clone(),
                                                                                    mime: None,
                                                                                    w: ad.width,
                                                                                    h: ad.height,
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
                                                            },
                                                            None => (),
                                                        }
                                                        match &ad.html {
                                                            Some(html) => {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("html"),
                                                                    req: 1,
                                                                    html: Some(HtmlAsset {
                                                                        html: Some(html.clone()),
                                                                        link: None,
                                                                        len: Some(html.len() as i32),
                                                                    }),
                                                                    title: None,
                                                                    img: None,
                                                                    video: None,
                                                                    data: None,
                                                                    app: None,
                                                                });
                                                            },
                                                            None => (),
                                                        }

                                                        match &ad.app_name {
                                                            Some(app_name) => {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("app"),
                                                                    req: 0,
                                                                    app: Some(AppAsset {
                                                                        name: app_name.clone(),
                                                                        desc: None,
                                                                        descurl: None,
                                                                        domain: None,
                                                                        bundle: ad.app_package.clone(),
                                                                        ver: None,
                                                                        developer: ad.app_developer.clone(),
                                                                        icon: None,
                                                                        storeid: None,
                                                                        storeurl: None,
                                                                        paid: 0,
                                                                        size: None,
                                                                        md5: None,
                                                                        registration: None,
                                                                        privacy: None,
                                                                        privacyurl: ad.app_privacy_policy.clone(),
                                                                        permission: None,
                                                                        permissionurl: ad.app_permission.clone(),
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

                                                    match &ad.show_urls {
                                                        Some(show_urls) => {
                                                            for event in show_urls {
                                                                event_vec.push(Event {
                                                                    eventtype: 501,
                                                                    method: 1,
                                                                    url: {
                                                                        let url = replace_macro(event);
                                                                        let price = match ad.price {
                                                                            Some(price) => {
                                                                                if price > 0 {
                                                                                    price
                                                                                } else {
                                                                                    connection.default_price
                                                                                }
                                                                            },
                                                                            None => connection.default_price,
                                                                        };
                                                                        let encrypt_price = Self::encrypt_price(price, &"".to_string(), connection);
                                                                        url.replace("__WIN_PRICE__", &encode(encrypt_price.as_str()))
                                                                    },
                                                                    header: None,
                                                                    content: None,
                                                                });
                                                            }
                                                        },
                                                        None => (),
                                                    }
                                                    match &ad.click_urls {
                                                        Some(click_urls) => {
                                                            for event in click_urls {
                                                                event_vec.push(Event {
                                                                    eventtype: 502,
                                                                    method: 1,
                                                                    url: replace_macro(event),
                                                                    header: None,
                                                                    content: None,
                                                                });
                                                            }
                                                        },
                                                        None => (),
                                                    }
                                                    match &ad.deeplink_success_urls {
                                                        Some(deeplink_success_urls) => {
                                                            for event in deeplink_success_urls {
                                                                event_vec.push(Event {
                                                                    eventtype: 504,
                                                                    method: 1,
                                                                    url: replace_macro(event),
                                                                    header: None,
                                                                    content: None,
                                                                });
                                                            }
                                                        },
                                                        None => (),
                                                    }
                                                    match &ad.deeplink_fail_urls {
                                                        Some(deeplink_fail_urls) => {
                                                            for event in deeplink_fail_urls {
                                                                event_vec.push(Event {
                                                                    eventtype: 505,
                                                                    method: 1,
                                                                    url: replace_macro(event),
                                                                    header: None,
                                                                    content: None,
                                                                });
                                                            }
                                                        },
                                                        None => (),
                                                    }
                                                    match &ad.start_download_urls {
                                                        Some(start_download_urls) => {
                                                            for event in start_download_urls {
                                                                event_vec.push(Event {
                                                                    eventtype: 601,
                                                                    method: 1,
                                                                    url: replace_macro(event),
                                                                    header: None,
                                                                    content: None,
                                                                });
                                                            }
                                                        },
                                                        None => (),
                                                    }
                                                    match &ad.finish_download_urls {
                                                        Some(finish_download_urls) => {
                                                            for event in finish_download_urls {
                                                                event_vec.push(Event {
                                                                    eventtype: 602,
                                                                    method: 1,
                                                                    url: replace_macro(event),
                                                                    header: None,
                                                                    content: None,
                                                                });
                                                            }
                                                        },
                                                        None => (),
                                                    }
                                                    match &ad.start_install_urls {
                                                        Some(start_install_urls) => {
                                                            for event in start_install_urls {
                                                                event_vec.push(Event {
                                                                    eventtype: 603,
                                                                    method: 1,
                                                                    url: replace_macro(event),
                                                                    header: None,
                                                                    content: None,
                                                                });
                                                            }
                                                        },
                                                        None => (),
                                                    }
                                                    match &ad.finish_install_urls {
                                                        Some(finish_install_urls) => {
                                                            for event in finish_install_urls {
                                                                event_vec.push(Event {
                                                                    eventtype: 604,
                                                                    method: 1,
                                                                    url: replace_macro(event),
                                                                    header: None,
                                                                    content: None,
                                                                });
                                                            }
                                                        },
                                                        None => (),
                                                    }
                                                    match &ad.video_start_urls {
                                                        Some(video_start_urls) => {
                                                            for event in video_start_urls {
                                                                event_vec.push(Event {
                                                                    eventtype: 701,
                                                                    method: 1,
                                                                    url: replace_macro(event),
                                                                    header: None,
                                                                    content: None,
                                                                });
                                                            }
                                                        },
                                                        None => (),
                                                    }
                                                    match &ad.video_quarter_urls {
                                                        Some(video_quarter_urls) => {
                                                            for event in video_quarter_urls {
                                                                event_vec.push(Event {
                                                                    eventtype: 702,
                                                                    method: 1,
                                                                    url: replace_macro(event),
                                                                    header: None,
                                                                    content: None,
                                                                });
                                                            }
                                                        },
                                                        None => (),
                                                    }
                                                    match &ad.video_half_urls {
                                                        Some(video_half_urls) => {
                                                            for event in video_half_urls {
                                                                event_vec.push(Event {
                                                                    eventtype: 703,
                                                                    method: 1,
                                                                    url: replace_macro(event),
                                                                    header: None,
                                                                    content: None,
                                                                });
                                                            }
                                                        },
                                                        None => (),
                                                    }
                                                    match &ad.video_three_quarters_urls {
                                                        Some(video_three_quarters_urls) => {
                                                            for event in video_three_quarters_urls {
                                                                event_vec.push(Event {
                                                                    eventtype: 704,
                                                                    method: 1,
                                                                    url: replace_macro(event),
                                                                    header: None,
                                                                    content: None,
                                                                });
                                                            }
                                                        },
                                                        None => (),
                                                    }
                                                    match &ad.video_end_urls {
                                                        Some(video_end_urls) => {
                                                            for event in video_end_urls {
                                                                event_vec.push(Event {
                                                                    eventtype: 705,
                                                                    method: 1,
                                                                    url: replace_macro(event),
                                                                    header: None,
                                                                    content: None,
                                                                });
                                                            }
                                                        },
                                                        None => (),
                                                    }
                                                    match &ad.video_mute_urls {
                                                        Some(video_mute_urls) => {
                                                            for event in video_mute_urls {
                                                                event_vec.push(Event {
                                                                    eventtype: 713,
                                                                    method: 1,
                                                                    url: replace_macro(event),
                                                                    header: None,
                                                                    content: None,
                                                                });
                                                            }
                                                        },
                                                        None => (),
                                                    }
                                                    match &ad.video_unmute_urls {
                                                        Some(video_unmute_urls) => {
                                                            for event in video_unmute_urls {
                                                                event_vec.push(Event {
                                                                    eventtype: 714,
                                                                    method: 1,
                                                                    url: replace_macro(event),
                                                                    header: None,
                                                                    content: None,
                                                                });
                                                            }
                                                        },
                                                        None => (),
                                                    }
                                                    match &ad.video_skip_urls {
                                                        Some(video_skip_urls) => {
                                                            for event in video_skip_urls {
                                                                event_vec.push(Event {
                                                                    eventtype: 710,
                                                                    method: 1,
                                                                    url: replace_macro(event),
                                                                    header: None,
                                                                    content: None,
                                                                });
                                                            }
                                                        },
                                                        None => (),
                                                    }
                                                    match &ad.video_close_urls {
                                                        Some(video_close_urls) => {
                                                            for event in video_close_urls {
                                                                event_vec.push(Event {
                                                                    eventtype: 711,
                                                                    method: 1,
                                                                    url: replace_macro(event),
                                                                    header: None,
                                                                    content: None,
                                                                });
                                                            }
                                                        },
                                                        None => (),
                                                    }
                                                    match &ad.video_pause_urls {
                                                        Some(video_pause_urls) => {
                                                            for event in video_pause_urls {
                                                                event_vec.push(Event {
                                                                    eventtype: 708,
                                                                    method: 1,
                                                                    url: replace_macro(event),
                                                                    header: None,
                                                                    content: None,
                                                                });
                                                            }
                                                        },
                                                        None => (),
                                                    }
                                                    match &ad.video_resume_urls {
                                                        Some(video_resume_urls) => {
                                                            for event in video_resume_urls {
                                                                event_vec.push(Event {
                                                                    eventtype: 709,
                                                                    method: 1,
                                                                    url: replace_macro(event),
                                                                    header: None,
                                                                    content: None,
                                                                });
                                                            }
                                                        },
                                                        None => (),
                                                    }
                                                    match &ad.video_replay_urls {
                                                        Some(video_replay_urls) => {
                                                            for event in video_replay_urls {
                                                                event_vec.push(Event {
                                                                    eventtype: 712,
                                                                    method: 1,
                                                                    url: replace_macro(event),
                                                                    header: None,
                                                                    content: None,
                                                                });
                                                            }
                                                        },
                                                        None => (),
                                                    }
                                                    match &ad.video_fullscreen_urls {
                                                        Some(video_fullscreen_urls) => {
                                                            for event in video_fullscreen_urls {
                                                                event_vec.push(Event {
                                                                    eventtype: 715,
                                                                    method: 1,
                                                                    url: replace_macro(event),
                                                                    header: None,
                                                                    content: None,
                                                                });
                                                            }
                                                        },
                                                        None => (),
                                                    }
                                                    match &ad.video_exit_fullscreen_urls {
                                                        Some(video_exit_fullscreen_urls) => {
                                                            for event in video_exit_fullscreen_urls {
                                                                event_vec.push(Event {
                                                                    eventtype: 716,
                                                                    method: 1,
                                                                    url: replace_macro(event),
                                                                    header: None,
                                                                    content: None,
                                                                });
                                                            }
                                                        },
                                                        None => (),
                                                    }
                                                    match &ad.video_up_scroll_urls {
                                                        Some(video_up_scroll_urls) => {
                                                            for event in video_up_scroll_urls {
                                                                event_vec.push(Event {
                                                                    eventtype: 717,
                                                                    method: 1,
                                                                    url: replace_macro(event),
                                                                    header: None,
                                                                    content: None,
                                                                });
                                                            }
                                                        },
                                                        None => (),
                                                    }
                                                    match &ad.video_down_scroll_urls {
                                                        Some(video_down_scroll_urls) => {
                                                            for event in video_down_scroll_urls {
                                                                event_vec.push(Event {
                                                                    eventtype: 718,
                                                                    method: 1,
                                                                    url: replace_macro(event),
                                                                    header: None,
                                                                    content: None,
                                                                });
                                                            }
                                                        },
                                                        None => (),
                                                    }
                                                    match &ad.click_area_report_url {
                                                        Some(click_area_report_url) => {
                                                            let parsed_url = Url::parse(click_area_report_url.as_str());
                                                            match parsed_url {
                                                                Ok(parsed_url) => {
                                                                    let hash_query: HashMap<_, _> = parsed_url.query_pairs().into_owned().collect();
                                                                    let sid = hash_query.get("sid");
                                                                    let creative_id = hash_query.get("creative_id");
                                                                    if sid.is_some() && creative_id.is_some() {
                                                                        event_vec.push(Event {
                                                                            eventtype: 502,
                                                                            method: 502,
                                                                            url: replace_macro(click_area_report_url),
                                                                            header: None,
                                                                            content: Some(format!("{{\
                                                                                \"sld\":\"__SLD__\",\
                                                                                \"width\":\"__WIDTH__\",\
                                                                                \"height\":\"__HEIGHT__\",\
                                                                                \"down_x\":\"__R_DOWN_X__\",\
                                                                                \"down_y\":\"__R_DOWN_Y__\",\
                                                                                \"up_x\":\"__R_UP_X__\",\
                                                                                \"up_y\":\"__R_UP_Y__\",\
                                                                                \"click_element\":\"__CLICKELEMENT__\",\
                                                                                \"sid\":\"{}\",\
                                                                                \"creative_id\":\"{}\"\
                                                                            }}", sid.unwrap(), creative_id.unwrap()).to_string()),
                                                                        });
                                                                    }
                                                                },
                                                                Err(_) => (),
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
                                    };

                                    bids.push(bid);
                                }
                                bids
                            }
                        }].to_vec())
                    },
                    None => None,
                }
            }
        };

        Ok(response)
    }

    async fn bidding_notify_win(url: String, win_price: i32, _next_price: i32, iv: &String, connection: &Connection, pool: &HttpPool) {
        let encrypt_price = Self::encrypt_price(win_price, iv, connection);
        let replaced_url = url
            .replace("__WIN_PRICE__", &encode(encrypt_price.as_str()));

        let client = {
            let pool_yiba_lock = pool.pool_yiba.clone();
            let pool_yiba = pool_yiba_lock.read().unwrap();
            pool_yiba.clone()
        };
        let _ = client.get(replaced_url).send().await;
    }

    async fn bidding_notify_lose(_url: String, _lose_price: i32, _lose_reason: i32, _lose_adn_name: &String, _iv: &String, _connection: &Connection, _pool: &HttpPool) {

    }

    fn encrypt_price(price: i32, _iv: &String, connection: &Connection) -> String {
        let secret = connection.client_tag_id.split("|").nth(1).unwrap();

        let message = format!("{}", price);
        let plaintext = message.as_bytes();
        let pos = if plaintext.len() > 16 {
            16
        } else {
            plaintext.len()
        };
        let mut buffer = [0u8; 16];
        buffer[..pos].copy_from_slice(plaintext);

        let key = &hex::decode(secret).unwrap();

        let cipher = Aes128EcbEnc::new(key[0..16].into())
            .encrypt_padded_mut::<Pkcs7>(&mut buffer, pos)
            .unwrap();

        let message_base64 = BASE64_STANDARD.encode(cipher);

        message_base64.replace("+", "-").replace("/", "_").replace("=", "")
    }

}

fn replace_macro(orig: &String) -> String {
    let mut replaced = orig.clone();

    replaced = replaced.replace("__DOWN_AD_X__", "__R_DOWN_X__");
    replaced = replaced.replace("__DOWN_AD_Y__", "__R_DOWN_Y__");
    replaced = replaced.replace("__UP_AD_X__", "__R_DOWN_X__");
    replaced = replaced.replace("__UP_AD_Y__", "__R_DOWN_Y__");

    replaced = replaced.replace("__DOWN_SCREEN_X__", "__ABS_DOWN_X__");
    replaced = replaced.replace("__DOWN_SCREEN_Y__", "__ABS_DOWN_Y__");
    replaced = replaced.replace("__UP_SCREEN_X__", "__ABS_DOWN_X__");
    replaced = replaced.replace("__UP_SCREEN_Y__", "__ABS_DOWN_Y__");

    replaced = replaced.replace("__LONGITUDE__", "__LON__");
    replaced = replaced.replace("__LATITUDE__", "__LAT__");

    replaced = replaced.replace("__DURATION__", "__VIDEO_TIME__");
    replaced = replaced.replace("__PLAY_MS__", "__VIDEO_PLAY_PROGRESS__");

    replaced
}
