use std::{collections::HashMap, io::Write, time::Duration};

use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyInit};
use base64::prelude::*;
use chrono::{Datelike, Local, TimeZone, Utc};
use chrono_tz::Tz;
use flate2::{Compression, write::GzEncoder};
use reqwest::Url;
use urlencoding::encode;

use crate::{protocol::*, Assets, Cache, Client, Connection, HttpPool, Identifiers, Price, ResultMessage};

type Aes128EcbEnc = ecb::Encryptor<aes::Aes128>;

pub mod bid;
pub mod request;
pub mod response;

pub use bid::UstarsBid;
pub use request::UstarsRequest;
pub use response::UstarsResponse;

pub struct Ustars {

}

impl Client for Ustars {

    async fn request(request: &Request, connection: &Connection, pool: &HttpPool, cache: &Cache) -> Result<Response, ResultMessage> {
        let position_id = connection.client_tag_id.split("|").nth(0).unwrap();
        let media_id = connection.client_tag_id.split("|").nth(1).unwrap();
        let app_key = &connection.client_ekey;
        let app_secret = &connection.client_ikey;

        let request_id = cache.get_sequence();

        let mut assets = Assets::new(request);
        let identifiers = Identifiers::new(request);

        let request_ustars = UstarsRequest {
            req_id: {
                request_id.to_string()
            },
            version: {
                "1.4.2".to_string()
            },
            paid: {
                match identifiers.get_id(519, 0) {
                    Some(uid) => Some(uid.id.clone()),
                    None => None,
                }
            },
            imei: {
                match identifiers.get_id(501, 0) {
                    Some(uid) => Some(uid.id.clone()),
                    None => {
                        match identifiers.get_id(507, 0) {
                            Some(uid) => Some(uid.id.clone()),
                            None => None,
                        }
                    },
                }
            },
            oaid: {
                match identifiers.get_id(505, 0) {
                    Some(uid) => Some(uid.id.clone()),
                    None => None,
                }
            },
            imei_md5: {
                match identifiers.get_id(502, 0) {
                    Some(uid) => Some(uid.id.clone()),
                    None => {
                        match identifiers.get_id(508, 0) {
                            Some(uid) => Some(uid.id.clone()),
                            None => None,
                        }
                    },
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
            idfv: {
                match identifiers.get_id(515, 0) {
                    Some(uid) => Some(uid.id.clone()),
                    None => None,
                }
            },
            idfv_md5: {
                match identifiers.get_id(516, 0) {
                    Some(uid) => Some(uid.id.clone()),
                    None => None,
                }
            },
            openudid: None,
            openudid_md5: None,
            app_name: {
                match &request.context.app {
                    Some(app) => app.name.clone(),
                    None => "".to_string(),
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
            ip: {
                match &request.context.device.ip {
                    Some(ip) => ip.clone(),
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.device.ip is required for upstream".to_string(),
                    }),
                }
            },
            user_agent: {
                request.context.device.ua.clone()
            },
            mac: {
                match identifiers.get_id(511, 0) {
                    Some(uid) => uid.id.clone(),
                    None => "00:00:00:00:00:00".to_string(),
                }
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
            brand: {
                match &request.context.device.brand {
                    Some(brand) => brand.clone(),
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.device.brand is required for upstream".to_string(),
                    }),
                }
            },
            os_type: {
                match request.context.device.os {
                    Some(2) => "android".to_string(),
                    Some(13) => "ios".to_string(),
                    _ => "unknown".to_string(),
                }
            },
            os_version: {
                match &request.context.device.osv {
                    Some(osv) => osv.clone(),
                    None => "1.0".to_string(),
                }
            },
            device_width: {
                match request.context.device.w {
                    Some(w) => w,
                    None => 0,
                }
            },
            device_height: {
                match request.context.device.h {
                    Some(h) => h,
                    None => 0,
                }
            },
            width: {
                request.item[0].spec.display.w.clone()
            },
            height: {
                request.item[0].spec.display.h.clone()
            },
            video_type: {
                match assets.get_current_asset("video") {
                    Some(video) => {
                        video.video.clone().unwrap().mime.clone()
                    },
                    None => None,
                }
            },
            min_duration: {
                match assets.get_current_asset("video") {
                    Some(video) => {
                        video.video.clone().unwrap().mindur.clone()
                    },
                    None => None,
                }
            },
            max_duration: {
                match assets.get_current_asset("video") {
                    Some(video) => {
                        video.video.clone().unwrap().maxdur.clone()
                    },
                    None => None,
                }
            },
            max_length: {
                match assets.get_current_asset("video") {
                    Some(video) => {
                        video.video.clone().unwrap().maxsize.clone()
                    },
                    None => None,
                }
            },
            dpi: {
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
                    Some(pxratio) => pxratio,
                    None => 2.25,
                }
            },
            imsi: {
                match identifiers.get_id(503, 0) {
                    Some(uid) => Some(uid.id.clone()),
                    None => None,
                }
            },
            network: {
                match &request.context.device.carrier {
                    Some(carrier) => {
                        match carrier.as_str() {
                            "cmcc" => "46000".to_string(),
                            "unicom" => "46001".to_string(),
                            "telecom" => "46003".to_string(),
                            "cbn" => "46015".to_string(),
                            _ => "".to_string(),
                        }
                    },
                    None => "".to_string(),
                }
            },
            connection_type: {
                match request.context.device.contype {
                    Some(1) => "lan".to_string(),
                    Some(2) => "wifi".to_string(),
                    Some(4) => "2g".to_string(),
                    Some(5) => "3g".to_string(),
                    Some(6) => "4g".to_string(),
                    Some(7) => "5g".to_string(),
                    _ => "unknown".to_string(),
                }
            },
            longitude: {
                match &request.context.device.geo {
                    Some(geo) => {
                        match geo.lon {
                            Some(lon) => lon,
                            None => 0.0,
                        }
                    },
                    None => 0.0,
                }
            },
            latitude: {
                match &request.context.device.geo {
                    Some(geo) => {
                        match geo.lat {
                            Some(lat) => lat,
                            None => 0.0,
                        }
                    },
                    None => 0.0,
                }
            },
            city: {
                match &request.context.device.geo {
                    Some(geo) => {
                        geo.city.clone()
                    },
                    None => None,
                }
            },
            region: {
                match &request.context.device.geo {
                    Some(geo) => {
                        geo.district.clone()
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
                            "O" => Some(3),
                            _ => Some(3),
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
            sys_name: {
                match identifiers.get_id(527, 0) {
                    Some(uid) => Some(uid.id.clone()),
                    None => None,
                }
            },
            sys_time_zone: {
                match &request.context.device.timezone {
                    Some(timezone) => {
                        let tz: Result<Tz, chrono_tz::ParseError> = timezone.parse();
                        match tz {
                            Ok(tz) => {
                                let t = tz.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
                                let utc = chrono_tz::UTC.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
                                Some((t.timestamp() - utc.timestamp()).to_string())
                            },
                            Err(_) => None,
                        }
                    },
                    None => None,
                }
            },
            sys_country: {
                request.context.device.country.clone()
            },
            sys_language: {
                request.context.device.lang.clone()
            },
            sys_hardware_version: {
                request.context.device.hwv.clone()
            },
            sys_hardware_model: {
                request.context.device.hwmodel.clone()
            },
            sys_init_time: {
                request.context.device.inittime.clone()
            },
            sys_boot_time: {
                request.context.device.boottime.clone()
            },
            sys_update_time: {
                request.context.device.updatetime.clone()
            },
            sys_memory_size: {
                match request.context.device.sysmemory {
                    Some(sysmemory) => Some((sysmemory / 1073741824) as i32),
                    None => None,
                }
            },
            sys_disk_size: {
                match request.context.device.sysdisksize {
                    Some(sysdisksize) => Some((sysdisksize / 1073741824) as i32),
                    None => None,
                }
            },
            sys_cpu_num: {
                request.context.device.syscpu.clone()
            },
            sys_cpu_freq: {
                request.context.device.syscpufreq.clone()
            },
            sys_idfa_policy: {
                request.context.device.lmt.clone()
            },
            sys_battery_status: {
                request.context.device.sysbatterystatus.clone()
            },
            sys_battery_power: {
                request.context.device.sysbatterypower.clone()
            },
            hw_hms: {
                request.context.device.hmsv.clone()
            },
            appstore_version: {
                request.context.device.storev.clone()
            },
            boot_mark: {
                match &request.context.device.bootmark {
                    Some(boot_mark) => {
                        boot_mark.clone()
                    },
                    None => "0".to_string(),
                }
            },
            update_mark: {
                match &request.context.device.updatemark {
                    Some(boot_mark) => {
                        boot_mark.clone()
                    },
                    None => "0".to_string(),
                }
            },
            app_list: {
                match &request.context.device.app {
                    Some(app) => {
                        Some(app.split(",").map(|s| s.to_string()).collect())
                    },
                    None => None,
                }
            },
            bid_floor: {
                Some(Price::to_client(connection, request.item[0].flr.map(f64::from)) as i32)
            },
        };

        let json_string = serde_json::to_string(&request_ustars).unwrap();
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&json_string.as_bytes()).unwrap();
        let compressed_bytes = encoder.finish().unwrap();

        let response_ustars: UstarsResponse;

        let method = "POST";
        let api = format!("/ads/{}/{}", media_id, position_id);
        let date = Utc::now().to_rfc2822();
        let length = json_string.chars().count();
        let secret_md5 = format!("{:x}", md5::compute(app_secret));

        let message = format!("{}&{}&{}&{}&{}", method, api, date, length, secret_md5);
        let message_md5 = format!("{:x}", md5::compute(&message));

        let client = {
            let pool_ustars_lock = pool.pool_ustars.clone();
            let pool_ustars = pool_ustars_lock.read().unwrap();
            pool_ustars.clone()
        };
        let response_ustars_raw = client.post(format!("{}/{}/{}", if connection.test { "https://api-test.ustars.net.cn/ads" } else { "https://api.ustars.net.cn/ads" }, media_id, position_id))
            .body(compressed_bytes)
            .header("Accept-Encoding", "gzip")
            .header("Authorization", format!("{}:{}", app_key, message_md5))
            .header("Connection", "keep-alive")
            .header("Content-Encoding", "gzip")
            .header("Content-Type", "application/octet-stream")
            .header("P-Date", date)
            .timeout(Duration::from_millis(connection.timeout))
            .send().await;

        match response_ustars_raw {
            Ok(response_ustars_raw) => {
                let status = response_ustars_raw.status();
                if status == 200 {
                    match response_ustars_raw.text().await {
                        Ok(text) => {
                            match serde_json::from_str::<UstarsResponse>(&text) {
                                Ok(json) => {
                                    if json.ret == 0 {
                                        return Err(ResultMessage {
                                            code: 993,
                                            message: "".to_string(),
                                        });
                                    } else {
                                        response_ustars = json;
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
                } else if status == 204 {
                    return Err(ResultMessage {
                        code: 993,
                        message: "".to_string(),
                    });
                } else {
                    return Err(ResultMessage {
                        code: 992,
                        message: {
                            match response_ustars_raw.text().await {
                                Ok(text) => format!("upstream error {}: {}", status, text),
                                Err(_) => format!("upstream error {}", status),
                            }
                        },
                    });
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
                Some([Seatbid {
                    bid: {
                        let mut bids = vec![];

                        for bid in response_ustars.bids.as_ref().unwrap() {
                            let link_asset = LinkAsset {
                                linktype: {
                                    match bid.ad_type {
                                        1 => 1,
                                        2 => 1,
                                        3 => 1,
                                        4 => 2,
                                        5 => 1,
                                        _ => 1,
                                    }
                                },
                                universallink: None,
                                storeid: None,
                                deeplink: {
                                    match &bid.deeplink_url {
                                        Some(deep_link) => Some(replace_macro(&deep_link)),
                                        None => None,
                                    }
                                },
                                quickapplink: None,
                                wechatmppath: {
                                    match &bid.applet_path {
                                        Some(applet_path) => Some(applet_path.clone()),
                                        None => None,
                                    }
                                },
                                wechatmpid: {
                                    match &bid.applet_id {
                                        Some(applet_id) => Some(applet_id.clone()),
                                        None => None,
                                    }
                                },
                                marketurl: None,
                                downloadurl: {
                                    match bid.ad_type {
                                        4 => Some(replace_macro(&bid.url)),
                                        _ => None,
                                    }
                                },
                                url: {
                                    replace_macro(&bid.url)
                                },
                                urlfb: None,
                            };

                            let bid = Bid {
                                id: Some(request_id.to_string()),
                                item: request.item[0].id.clone(),
                                price: { // update later
                                    match bid.price {
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
                                    match &bid.win_notice {
                                        Some(win_notice) => {
                                            let mut burl = Vec::<String>::new();
                                            burl.push(replace_macro(win_notice));
                                            Some(burl)
                                        },
                                        None => None,
                                    }
                                },
                                lurl: {
                                    None
                                },
                                media: Ad {
                                    id: request.id.clone(),
                                    display: Display {
                                        w: None,
                                        h: None,
                                        banner: {
                                            if assets.get_banner_size() > 0 {
                                                match &bid.image_url {
                                                    Some(image_url) => {
                                                        Some(Banner {
                                                            img: image_url.clone(),
                                                            link: Some(link_asset.clone()),
                                                        })
                                                    },
                                                    None => {
                                                        match &bid.image_urls {
                                                            Some(image_urls) => {
                                                                if image_urls.len() > 0 {
                                                                    Some(Banner {
                                                                        img: image_urls[0].clone(),
                                                                        link: Some(link_asset.clone()),
                                                                    })
                                                                } else {
                                                                    None
                                                                }
                                                            },
                                                            None => {
                                                                None
                                                            },
                                                        }
                                                    },
                                                }
                                            } else {
                                                None
                                            }
                                        },
                                        native: {
                                            if assets.get_asset_total_size() > 0 {
                                                let mut asset_vec = vec![];

                                                match &bid.video_url {
                                                    Some(video_url) => {
                                                        asset_vec.push(Asset {
                                                            id: assets.consume_asset("video"),
                                                            req: 1,
                                                            video: Some(VideoAsset {
                                                                url: video_url.clone(),
                                                                mime: bid.video_type.clone(),
                                                                w: bid.width,
                                                                h: bid.height,
                                                                dur: {
                                                                    match bid.video_duration {
                                                                        Some(video_duration) => Some(video_duration as i32),
                                                                        None => None,
                                                                    }
                                                                },
                                                                skipoffset: bid.video_skip_min_time.clone(),
                                                                size: {
                                                                    match bid.video_size {
                                                                        Some(video_size) => Some(video_size as i32),
                                                                        None => None,
                                                                    }
                                                                },
                                                                delivery: Some(1),
                                                                orientation: bid.video_orientation.clone(),
                                                                autolanding: 0,
                                                                clickable: 0,
                                                            }),
                                                            title: None,
                                                            img: None,
                                                            data: None,
                                                            html: None,
                                                            app: None,
                                                        });

                                                        match &bid.video_end_card_url {
                                                            Some(video_end_card_url) => {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("video#cover"),
                                                                    req: 1,
                                                                    img: Some(ImageAsset {
                                                                        url: video_end_card_url.clone(),
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
                                                        match &bid.video_end_card_html {
                                                            Some(video_end_card_html) => {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("video#end#html"),
                                                                    req: 1,
                                                                    html: Some(HtmlAsset {
                                                                        html: Some(video_end_card_html.clone()),
                                                                        link: None,
                                                                        len: None,
                                                                    }),
                                                                    img: None,
                                                                    title: None,
                                                                    video: None,
                                                                    data: None,
                                                                    app: None,
                                                                });
                                                            },
                                                            None => (),
                                                        }
                                                    },
                                                    None => (),
                                                }

                                                match &bid.image_url {
                                                    Some(image_url) => {
                                                        if assets.get_asset_size("img") > 0 {
                                                            asset_vec.push(Asset {
                                                                id: assets.consume_asset("img"),
                                                                req: 1,
                                                                img: Some(ImageAsset {
                                                                    url: image_url.clone(),
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
                                                        }
                                                        if assets.get_asset_size("thumb") > 0 {
                                                            asset_vec.push(Asset {
                                                                id: assets.consume_asset("thumb"),
                                                                req: 1,
                                                                img: Some(ImageAsset {
                                                                    url: image_url.clone(),
                                                                    mime: None,
                                                                    w: None,
                                                                    h: None,
                                                                    imagetype: Some(501),
                                                                }),
                                                                title: None,
                                                                video: None,
                                                                data: None,
                                                                html: None,
                                                                app: None,
                                                            });
                                                        }
                                                    },
                                                    None => (),
                                                }

                                                match &bid.image_urls {
                                                    Some(image_urls) => {
                                                        if assets.get_asset_size("img") > 0 {
                                                            for image_url in image_urls {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("img"),
                                                                    req: 1,
                                                                    img: Some(ImageAsset {
                                                                        url: image_url.clone(),
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
                                                            }
                                                        }
                                                        if assets.get_asset_size("thumb") > 0 {
                                                            for image_url in image_urls {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("thumb"),
                                                                    req: 1,
                                                                    img: Some(ImageAsset {
                                                                        url: image_url.clone(),
                                                                        mime: None,
                                                                        w: None,
                                                                        h: None,
                                                                        imagetype: Some(501),
                                                                    }),
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

                                                match &bid.icon {
                                                    Some(icon) => {
                                                        asset_vec.push(Asset {
                                                            id: assets.consume_asset("icon"),
                                                            req: 1,
                                                            img: Some(ImageAsset {
                                                                url: icon.clone(),
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

                                                match &bid.icon_text {
                                                    Some(icon_text) => {
                                                        asset_vec.push(Asset {
                                                            id: assets.consume_asset("data#ctatext"),
                                                            req: 1,
                                                            title: None,
                                                            img: None,
                                                            video: None,
                                                            data: Some(DataAsset {
                                                                value: icon_text.clone(),
                                                                len: None,
                                                                datatype: Some(12),
                                                            }),
                                                            html: None,
                                                            app: None,
                                                        });
                                                    },
                                                    None => (),
                                                }

                                                match &bid.title {
                                                    Some(title) => {
                                                        asset_vec.push(Asset {
                                                            id: assets.consume_asset("title"),
                                                            req: 1,
                                                            title: Some(TitleAsset {
                                                                text: title.clone(),
                                                                subtitle: None,
                                                                desc: bid.desc.clone(),
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

                                                match &bid.app_name {
                                                    Some(app_name) => {
                                                        asset_vec.push(Asset {
                                                            id: assets.consume_asset("app"),
                                                            req: 0,
                                                            app: Some(AppAsset {
                                                                name: app_name.clone(),
                                                                desc: None,
                                                                descurl: None,
                                                                domain: None,
                                                                bundle: bid.app_package.clone(),
                                                                ver: None,
                                                                developer: bid.app_developer.clone(),
                                                                icon: None,
                                                                storeid: None,
                                                                storeurl: None,
                                                                paid: 0,
                                                                size: bid.app_size.clone(),
                                                                md5: None,
                                                                registration: None,
                                                                privacy: None,
                                                                privacyurl: bid.app_privacy_policy.clone(),
                                                                permission: None,
                                                                permissionurl: bid.app_permission.clone(),
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

                                            match &bid.show_urls {
                                                Some(show_urls) => {
                                                    for event in show_urls {
                                                        event_vec.push(Event {
                                                            eventtype: 501,
                                                            method: 1,
                                                            url: {
                                                                let url = replace_macro(event);
                                                                let price = match bid.price {
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
                                            match &bid.click_urls {
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
                                            match &bid.deeplink_report {
                                                Some(deeplink_report) => {
                                                    for event in deeplink_report {
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
                                            match &bid.deeplink_fail_urls {
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
                                            match &bid.start_download_urls {
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
                                            match &bid.finish_download_urls {
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
                                            match &bid.start_install_urls {
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
                                            match &bid.finish_install_urls {
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
                                            match &bid.video_start_urls {
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
                                            match &bid.video_quarter_urls {
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
                                            match &bid.video_half_urls {
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
                                            match &bid.video_three_quarters_urls {
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
                                            match &bid.video_end_urls {
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
                                            match &bid.video_mute_urls {
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
                                            match &bid.video_unmute_urls {
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
                                            match &bid.video_skip_urls {
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
                                            match &bid.video_close_urls {
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
                                            match &bid.video_pause_urls {
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
                                            match &bid.video_resume_urls {
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
                                            match &bid.video_replay_urls {
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
                                            match &bid.video_fullscreen_urls {
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
                                            match &bid.video_exit_fullscreen_urls {
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
                                            match &bid.video_up_scroll_urls {
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
                                            }match &bid.video_down_scroll_urls {
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
                                            match &bid.click_area_report_url {
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
        };

        Ok(response)
    }

    async fn bidding_notify_win(url: String, win_price: i32, _next_price: i32, iv: &String, connection: &Connection, pool: &HttpPool) {
        let encrypt_price = Self::encrypt_price(win_price, iv, connection);
        let replaced_url = url
            .replace("__WIN_PRICE__", &encode(encrypt_price.as_str()));

        let client = {
            let pool_ustars_lock = pool.pool_ustars.clone();
            let pool_ustars = pool_ustars_lock.read().unwrap();
            pool_ustars.clone()
        };
        let _ = client.get(replaced_url).send().await;
    }

    async fn bidding_notify_lose(_url: String, _lose_price: i32, _lose_reason: i32, _lose_adn_name: &String, _iv: &String, _connection: &Connection, _pool: &HttpPool) {

    }

    fn encrypt_price(price: i32, _iv: &String, connection: &Connection) -> String {
        let message = format!("{}", price);
        let plaintext = message.as_bytes();
        let pos = if plaintext.len() > 16 {
            16
        } else {
            plaintext.len()
        };
        let mut buffer = [0u8; 16];
        buffer[..pos].copy_from_slice(plaintext);

        let key = connection.client_ikey.as_bytes();

        let cipher = Aes128EcbEnc::new(key[0..16].into())
            .encrypt_padded_mut::<Pkcs7>(&mut buffer, pos)
            .unwrap();

        let message_base64 = BASE64_STANDARD.encode(cipher);

        message_base64.replace("+", "-").replace("/", "_").replace("=", "")
    }

}

fn replace_macro(orig: &String) -> String {
    let mut replaced = orig.clone();

    replaced = replaced.replace("__DOWN_POS_X__", "__ABS_DOWN_X__");
    replaced = replaced.replace("__DOWN_POS_Y__", "__ABS_DOWN_Y__");
    replaced = replaced.replace("__UP_POS_X__", "__ABS_UP_X__");
    replaced = replaced.replace("__UP_POS_Y__", "__ABS_UP_Y__");
    replaced = replaced.replace("__DOWN_AD_X__", "__R_DOWN_X__");
    replaced = replaced.replace("__DOWN_AD_Y__", "__R_DOWN_Y__");
    replaced = replaced.replace("__UP_AD_X__", "__R_UP_X__");
    replaced = replaced.replace("__UP_AD_Y__", "__R_UP_Y__");
    replaced = replaced.replace("__DIP_DOWN_AD_X__", "__R_DOWN_DP_X__");
    replaced = replaced.replace("__DIP_DOWN_AD_Y__", "__R_DOWN_DP_Y__");
    replaced = replaced.replace("__DIP_UP_AD_X__", "__R_UP_DP_X__");
    replaced = replaced.replace("__DIP_UP_AD_Y__", "__R_UP_DIP_Y__");

    replaced = replaced.replace("__AIT__", "__SLD__");
    replaced = replaced.replace("__LATITUDE__", "__LAT__");
    replaced = replaced.replace("__LONGITUDE__", "__LNG__");

    replaced = replaced.replace("__TS_SECOND__", "__TS_S__");

    replaced = replaced.replace("__DURATION__", "__VIDEO_TIME__");
    replaced = replaced.replace("__PLAY_MS__", "__VIDEO_PLAY_PROGRESS__");
    replaced = replaced.replace("__PLAY_BEGIN_TIME__", "__VIDEO_BEGIN_TIME__");
    replaced = replaced.replace("__PLAY_END_TIME__", "__VIDEO_END_TIME__");
    replaced = replaced.replace("__PLAY_FIRST_FRAME__", "__VIDEO_FIRST_FRAME__");
    replaced = replaced.replace("__PLAY_LAST_FRAME__", "__VIDEO_LAST_FRAME__");
    replaced = replaced.replace("__PLAY_SCENE__", "__VIDEO_PLAY_SCENE__");
    replaced = replaced.replace("__PLAY_TYPE__", "__VIDEO_PLAY_TYPE__");
    replaced = replaced.replace("__PLAY_BEHAVIOR__", "__VIDEO_PLAY_TRIGGER__");
    replaced = replaced.replace("__PLAY_STATUS__", "__VIDEO_PLAY_STATUS__");

    replaced
}
