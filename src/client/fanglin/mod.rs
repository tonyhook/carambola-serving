use std::time::Duration;

use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyInit};
use base64::prelude::*;
use chrono::TimeZone;
use chrono_tz::Tz;
use urlencoding::encode;

use crate::{protocol::*, Assets, Cache, Client, Connection, Identifiers, Price, ResultMessage};

type Aes128EcbEnc = ecb::Encryptor<aes::Aes128>;

pub mod ad;
pub mod app;
pub mod device;
pub mod geo;
pub mod network;
pub mod pos;
pub mod request;
pub mod response;
pub mod user;
pub mod video;

pub use ad::FanglinAd;
pub use app::FanglinApp;
pub use device::FanglinDevice;
pub use network::FanglinNetwork;
pub use geo::FanglinGeo;
pub use pos::FanglinPos;
pub use request::FanglinRequest;
pub use response::FanglinResponse;
pub use user::FanglinUser;
pub use video::FanglinVideo;

pub struct Fanglin {

}

impl Client for Fanglin {

    async fn request(request: &Request, connection: &Connection, cache: &Cache) -> Result<Response, ResultMessage> {
        let request_id = cache.get_sequence();

        let assets = Assets::new(request);
        let identifiers = Identifiers::new(request);

        let request_fanglin = FanglinRequest {
            req_id: {
                request_id.to_string()
            },
            version: {
                "v3.1.3".to_string()
            },
            pos: FanglinPos {
                pid: connection.client_tag_id.clone(),
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
                bid_floor: {
                    Some(Price::to_client(connection, request.item[0].flr))
                },
                num: Some(1),
            },
            app: {
                match &request.context.app {
                    Some(app) => FanglinApp {
                        bundle: {
                            match &app.bundle {
                                Some(bundle) => bundle.clone(),
                                None => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.context.app.bundle is required for upstream".to_string(),
                                }),
                            }
                        },
                        app_ver: {
                            match &app.ver {
                                Some(ver) => ver.clone(),
                                None => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.context.app.ver is required for upstream".to_string(),
                                }),
                            }
                        },
                        app_name: {
                            app.name.clone()
                        },
                    },
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.app is required for upstream".to_string(),
                    }),
                }
            },
            device: FanglinDevice {
                device_type: {
                    match request.context.device.devicetype {
                        Some(devicetype) => {
                            match devicetype {
                                1 => 1,
                                3 => 3,
                                4 => 1,
                                5 => 2,
                                7 => 3,
                                _ => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.context.device.type should be 1/3/4/5/7 for upstream".to_string(),
                                }),
                            }
                        },
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.type is required for upstream".to_string(),
                        }),
                    }
                },
                os: {
                    match request.context.device.os {
                        Some(os) => {
                            match os {
                                2 => 1,
                                13 => 2,
                                501 => 3,
                                _ => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.context.device.os should be 2/13/501 for upstream".to_string(),
                                }),
                            }
                        },
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.os is required for upstream".to_string(),
                        }),
                    }
                },
                os_ver: {
                    match &request.context.device.osv {
                        Some(osv) => osv.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.osv is required for upstream".to_string(),
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
                vendor: {
                    match &request.context.device.make {
                        Some(make) => make.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.make is required for upstream".to_string(),
                        }),
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
                screen_width: {
                    match request.context.device.w {
                        Some(w) => w,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.w is required for upstream".to_string(),
                        }),
                    }
                },
                screen_height: {
                    match request.context.device.h {
                        Some(h) => h,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.h is required for upstream".to_string(),
                        }),
                    }
                },
                ua: {
                    request.context.device.ua.clone()
                },
                density: {
                    match request.context.device.pxratio {
                        Some(pxratio) => pxratio,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.pxratio is required for upstream".to_string(),
                        }),
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
                ppi: {
                    match request.context.device.ppi {
                        Some(ppi) => ppi,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.ppi is required for upstream".to_string(),
                        }),
                    }
                },
                imei: {
                    match identifiers.get_id(501, 0) {
                        Some(uid) => uid.id.clone(),
                        None => "".to_string(),
                    }
                },
                oaid: {
                    match identifiers.get_id(505, 0) {
                        Some(uid) => uid.id.clone(),
                        None => "".to_string(),
                    }
                },
                android_id: {
                    match identifiers.get_id(509, 0) {
                        Some(uid) => uid.id.clone(),
                        None => "".to_string(),
                    }
                },
                idfa: {
                    match identifiers.get_id(507, 0) {
                        Some(uid) => uid.id.clone(),
                        None => "".to_string(),
                    }
                },
                idfv: {
                    match identifiers.get_id(515, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                imsi: {
                    match identifiers.get_id(503, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                mac: {
                    match identifiers.get_id(511, 0) {
                        Some(uid) => uid.id.clone(),
                        None => "".to_string(),
                    }
                },
                orientation: {
                    match request.context.device.orientation {
                        Some(orientation) => {
                            match orientation {
                                501 => 1,
                                502 => 2,
                                _ => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.context.device.orientation should be 501/502 for upstream".to_string(),
                                }),
                            }
                        },
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.orientation is required for upstream".to_string(),
                        }),
                    }
                },
                open_id: {
                    None
                },
                caid: {
                    match identifiers.get_id(513, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                caid_ver: {
                    match identifiers.get_id(513, 0) {
                        Some(uid) => uid.ver.clone(),
                        None => None,
                    }
                },
                skan_vers: {
                    match &request.context.device.skan {
                        Some(skan) => Some(skan.to_vec()),
                        None => None,
                    }
                },
                aaid: {
                    match identifiers.get_id(514, 0) {
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
                boot_mark: {
                    match &request.context.device.bootmark {
                        Some(bootmark) => bootmark.clone(),
                        None => "".to_string(),
                    }
                },
                update_mark: {
                    match &request.context.device.updatemark {
                        Some(updatemark) => updatemark.clone(),
                        None => "".to_string(),
                    }
                },
                device_name_md5: {
                    match identifiers.get_id(528, 0) {
                        Some(uid) => uid.id.clone(),
                        None => "".to_string(),
                    }
                },
                hardware_machine: {
                    match &request.context.device.hwmachine {
                        Some(hwmachine) => hwmachine.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.hwmachine is required for upstream".to_string(),
                        }),
                    }
                },
                hardware_model: {
                    match &request.context.device.hwmodel {
                        Some(hwmodel) => hwmodel.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.hwmodel is required for upstream".to_string(),
                        }),
                    }
                },
                time_zone: {
                    match &request.context.device.timezone {
                        Some(timezone) => {
                            let tz: Result<Tz, chrono_tz::ParseError> = timezone.parse();
                            match tz {
                                Ok(tz) => {
                                    let t = tz.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
                                    let utc = chrono_tz::UTC.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
                                    (t.timestamp() - utc.timestamp()).to_string()
                                },
                                Err(_) => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.context.device.timezone is malformat for upstream, should be like Asia/Shanghai".to_string(),
                                }),
                            }
                        },
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.timezone is required for upstream".to_string(),
                        }),
                    }
                },
                disk_size: {
                    match &request.context.device.sysdisksize {
                        Some(sysdisksize) => sysdisksize.to_string(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.sysdisksize is required for upstream".to_string(),
                        }),
                    }
                },
                memory_size: {
                    match &request.context.device.sysmemory {
                        Some(sysmemory) => sysmemory.to_string(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.sysmemory is required for upstream".to_string(),
                        }),
                    }
                },
                country: {
                    request.context.device.country.clone()
                },
                language: {
                    request.context.device.lang.clone()
                },
                os_update_time: {
                    match &request.context.device.updatetime {
                        Some(updatetime) => updatetime.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.updatetime is required for upstream".to_string(),
                        }),
                    }
                },
                boot_time_sec: {
                    match &request.context.device.boottime {
                        Some(boottime) => boottime.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.boottime is required for upstream".to_string(),
                        }),
                    }
                },
                birth_time: {
                    match &request.context.device.inittime {
                        Some(inittime) => inittime.clone(),
                        None => {
                            match &request.context.device.birthtime {
                                Some(birthtime) => birthtime.clone(),
                                None => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.context.device.inittime is required for upstream".to_string(),
                                }),
                            }
                        },
                    }
                },
                os_com_time: {
                    request.context.device.romtime.clone()
                },
                elapse_time: {
                    None
                },
                battery_power: {
                    request.context.device.sysbatterypower.clone()
                },
                battery_status: {
                    request.context.device.sysbatterystatus.clone()
                },
                cpu_num: {
                    request.context.device.syscpu.clone()
                },
                cpu_frequency: {
                    request.context.device.syscpufreq.clone()
                },
                lmt: {
                    request.context.device.lmt.clone()
                },
                appstore_ver: {
                    request.context.device.storev.clone()
                },
                kernel_ver: {
                    request.context.device.romv.clone()
                },
                hms_ver: {
                    request.context.device.hmsv.clone()
                },
            },
            network: FanglinNetwork {
                ip: {
                    match &request.context.device.ip {
                        Some(ip) => ip.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.ip is required for upstream".to_string(),
                        }),
                    }
                },
                conn_type: {
                    match &request.context.device.contype {
                        Some(contype) => {
                            match contype {
                                1 => 100,
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
                operator_type: {
                    match &request.context.device.carrier {
                        Some(carrier) => {
                            match carrier.as_str() {
                                "cmcc" => 1,
                                "unicom" => 2,
                                "telecom" => 3,
                                _ => 100,
                            }
                        },
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.carrier is required for upstream".to_string(),
                        }),
                    }
                },
            },
            geo: FanglinGeo {
                lat: {
                    match &request.context.device.geo {
                        Some(geo) => {
                            match geo.lat {
                                Some(lat) => Some(lat),
                                None => None,
                            }
                        },
                        None => None,
                    }
                },
                lng: {
                    match &request.context.device.geo {
                        Some(geo) => {
                            match geo.lon {
                                Some(lon) => Some(lon),
                                None => None,
                            }
                        },
                        None => None,
                    }
                },
            },
            user: Some(FanglinUser {
                uid: {
                    None
                },
                apps: {
                    match &request.context.device.app {
                        Some(app) => Some(app.split(",").map(|s| s.to_string()).collect()),
                        None => None,
                    }
                }
            }),
            sup_dp: {
                1
            },
            protocol_type: {
                1
            },
        };

        let response_fanglin: FanglinResponse;

        let client = reqwest::ClientBuilder::new()
            .gzip(true)
            .no_brotli()
            .no_deflate()
            .build().unwrap();
        let response_fanglin_raw = client.post(if connection.test { "http://test.fanglinad.com:8888/get" } else { "http://test.fanglinad.com:8888/get" })
            .json(&request_fanglin)
            .header("Accept-Encoding", "gzip")
            .header("Connection", "keep-alive")
            .header("Content-Type", "application/json")
            .timeout(Duration::from_millis(connection.timeout))
            .send().await;

        match response_fanglin_raw {
            Ok(response_fanglin_raw) => {
                let status = response_fanglin_raw.status();
                if status != 200 {
                    return Err(ResultMessage {
                        code: 992,
                        message: {
                            match response_fanglin_raw.text().await {
                                Ok(text) => format!("upstream error {}: {}", status, text),
                                Err(_) => format!("upstream error {}", status),
                            }
                        },
                    });
                } else {
                    match response_fanglin_raw.text().await {
                        Ok(text) => {
                            match serde_json::from_str::<FanglinResponse>(&text) {
                                Ok(json) => {
                                    response_fanglin = json;

                                    match response_fanglin.code {
                                        0 => {
                                            if response_fanglin.data.is_none() {
                                                return Err(ResultMessage {
                                                    code: 993,
                                                    message: "".to_string(),
                                                });
                                            }
                                        },
                                        code => {
                                            return Err(ResultMessage {
                                                code: 992,
                                                message: format!("upstream error code {}", code),
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
                        message: "upstream request timeout".to_string(),
                    });
                } else {
                    return Err(ResultMessage {
                        code: 992,
                        message: format!("upstream request failed: {}", error.to_string()),
                    });
                }
            }
        }

        let response = Response {
            id: request.id.clone(),
            nbr: None,
            seatbid: {
                let data = &response_fanglin.data.unwrap();
                Some([Seatbid {
                    bid: {
                        let mut bids = vec![];

                        for ad in data {
                            let link_asset = LinkAsset {
                                linktype: {
                                    match &ad.action_type {
                                        Some(1) => 1,
                                        Some(2) => 1,
                                        Some(3) => 2,
                                        Some(4) => 3,
                                        Some(5) => 1,
                                        Some(6) => 1,
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
                                    match &ad.mini_program_path {
                                        Some(mini_program_path) => Some(mini_program_path.clone()),
                                        None => None,
                                    }
                                },
                                wechatmpid: {
                                    match &ad.mini_program_id {
                                        Some(mini_program_id) => Some(mini_program_id.clone()),
                                        None => None,
                                    }
                                },
                                marketurl: {
                                    None
                                },
                                downloadurl: {
                                    match &ad.action_type {
                                        Some(2) => Some(ad.landing_url.clone().unwrap()),
                                        Some(3) => Some(ad.landing_url.clone().unwrap()),
                                        Some(4) => Some(ad.landing_url.clone().unwrap()),
                                        _ => None,
                                    }
                                },
                                url: {
                                    match &ad.action_type {
                                        Some(1) => ad.landing_url.clone().unwrap(),
                                        Some(5) => ad.landing_url.clone().unwrap(),
                                        Some(6) => ad.landing_url.clone().unwrap(),
                                        _ => "".to_string(),
                                    }
                                },
                                urlfb: {
                                    ad.fallback_url.clone()
                                },
                            };

                            let bid = Bid {
                                id: Some(request_id.to_string()),
                                item: request.item[0].id.clone(),
                                price: {
                                    match ad.ecpm {
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
                                    match &ad.win_notice_url {
                                        Some(win_notice_url) => {
                                            let mut burl = Vec::<String>::new();
                                            let mut nurl = win_notice_url.clone();
                                            nurl = nurl.replace("__FLPRICE__", "__WIN_PRICE__");
                                            burl.push(replace_macro(&nurl));
                                            Some(burl)
                                        },
                                        None => None,
                                    }
                                },
                                lurl: None,
                                media: Ad {
                                    id: ad.pid.clone(),
                                    display: Display {
                                        w: {
                                            ad.width
                                        },
                                        h: {
                                            ad.height
                                        },
                                        banner: {
                                            if request.item[0].spec.display.displayfmt.is_some() {
                                                Some(Banner {
                                                    img: {
                                                        let imgs = ad.imgs.clone().unwrap();
                                                        imgs.get(0).unwrap().to_string()
                                                    },
                                                    link: Some(link_asset.clone()),
                                                })
                                            } else {
                                                None
                                            }
                                        },
                                        native: {
                                            if request.item[0].spec.display.nativefmt.is_some() {
                                                let mut asset_vec = vec![];

                                                match &ad.video {
                                                    Some(video) => {
                                                        if assets.video_asset.len() > 0 {
                                                            asset_vec.push(Asset {
                                                                id: assets.video_asset.get(0).unwrap().id,
                                                                req: 1,
                                                                video: Some(VideoAsset {
                                                                    url: video.video_url.clone(),
                                                                    mime: None,
                                                                    w: None,
                                                                    h: None,
                                                                    dur: Some(video.video_duration),
                                                                    skipoffset: None,
                                                                    size: Some(video.video_size),
                                                                    delivery: None,
                                                                    orientation: None,
                                                                    autolanding: 0,
                                                                    clickable: 0,
                                                                }),
                                                                title: None,
                                                                img: None,
                                                                data: None,
                                                                html: None,
                                                                app: None,
                                                            });
                                                        }
                                                    },
                                                    None => (),
                                                }

                                                        if assets.title_asset.len() > 0 {
                                                            asset_vec.push(Asset {
                                                                id: assets.title_asset.get(0).unwrap().id,
                                                                req: 1,
                                                                title: Some(TitleAsset {
                                                                    text: ad.title.clone().unwrap(),
                                                                    subtitle: None,
                                                                    desc: ad.desc.clone(),
                                                                    len: Some(ad.title.clone().unwrap().len() as i32),
                                                                }),
                                                                img: None,
                                                                video: None,
                                                                data: None,
                                                                html: None,
                                                                app: None,
                                                            });
                                                        }
                                                        for (i, asset) in assets.img_asset.iter().enumerate() {
                                                            if i < ad.imgs.clone().unwrap().len() {
                                                                asset_vec.push(Asset {
                                                                    id: asset.id,
                                                                    req: 1,
                                                                    img: {
                                                                        Some(ImageAsset {
                                                                            url: ad.imgs.clone().unwrap()[i].clone(),
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
                                                        if assets.html_asset.len() > 0 && ad.html.is_some() {
                                                            asset_vec.push(Asset {
                                                                id: assets.html_asset.get(0).unwrap().id,
                                                                req: 0,
                                                                html: Some(HtmlAsset {
                                                                    html: ad.html.clone(),
                                                                    link: None,
                                                                    len: Some(ad.html.clone().unwrap().len() as i32),
                                                                }),
                                                                title: None,
                                                                img: None,
                                                                video: None,
                                                                data: None,
                                                                app: None,
                                                            });
                                                        }

                                                if ad.app_name.is_some() {
                                                    asset_vec.push(Asset {
                                                        id: 0,
                                                        req: 0,
                                                        app: Some(AppAsset {
                                                            name: ad.app_name.clone().unwrap(),
                                                            domain: None,
                                                            bundle: ad.app_bundle.clone(),
                                                            ver: ad.app_version.clone(),
                                                            developer: None,
                                                            icon: None,
                                                            storeid: None,
                                                            storeurl: None,
                                                            paid: 0,
                                                            size: ad.app_size,
                                                            md5: None,
                                                            privacy: None,
                                                            permission: None,
                                                        }),
                                                        title: None,
                                                        img: None,
                                                        video: None,
                                                        data: None,
                                                        html: None,
                                                    });
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

                                            match &ad.imp_tracks {
                                                Some(imp_tracks) => {
                                                    for event in imp_tracks {
                                                        event_vec.push(Event {
                                                            eventtype: 501,
                                                            method: 1,
                                                            url: replace_macro(event),
                                                            header: None,
                                                            content: None,
                                                        });
                                                    }
                                                },
                                                None => (),
                                            }
                                            match &ad.clk_tracks {
                                                Some(clk_tracks) => {
                                                    for event in clk_tracks {
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
                                            match &ad.dn_start_tracks {
                                                Some(dn_start_tracks) => {
                                                    for event in dn_start_tracks {
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
                                            match &ad.dn_succ_tracks {
                                                Some(dn_succ_tracks) => {
                                                    for event in dn_succ_tracks {
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
                                            match &ad.in_start_tracks {
                                                Some(in_start_tracks) => {
                                                    for event in in_start_tracks {
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
                                            match &ad.in_succ_tracks {
                                                Some(in_succ_tracks) => {
                                                    for event in in_succ_tracks {
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
                                            match &ad.ap_start_tracks {
                                                Some(ap_start_tracks) => {
                                                    for event in ap_start_tracks {
                                                        event_vec.push(Event {
                                                            eventtype: 605,
                                                            method: 1,
                                                            url: replace_macro(event),
                                                            header: None,
                                                            content: None,
                                                        });
                                                    }
                                                },
                                                None => (),
                                            }
                                            match &ad.dp_try_tracks {
                                                Some(dp_try_tracks) => {
                                                    for event in dp_try_tracks {
                                                        event_vec.push(Event {
                                                            eventtype: 503,
                                                            method: 1,
                                                            url: replace_macro(event),
                                                            header: None,
                                                            content: None,
                                                        });
                                                    }
                                                },
                                                None => (),
                                            }
                                            match &ad.dp_succ_tracks {
                                                Some(dp_succ_tracks) => {
                                                    for event in dp_succ_tracks {
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
                                            match &ad.dp_err_tracks {
                                                Some(dp_err_tracks) => {
                                                    for event in dp_err_tracks {
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

                                            match &ad.video {
                                                Some(video) => {
                                                    match &video.vd_start_tracks {
                                                        Some(vd_start_tracks) => {
                                                            for event in vd_start_tracks {
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
                                                    match &video.vd_quar_tracks {
                                                        Some(vd_quar_tracks) => {
                                                            for event in vd_quar_tracks {
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
                                                    match &video.vd_mid_tracks {
                                                        Some(vd_mid_tracks) => {
                                                            for event in vd_mid_tracks {
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
                                                    match &video.vd_thd_tracks {
                                                        Some(vd_thd_tracks) => {
                                                            for event in vd_thd_tracks {
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
                                                    match &video.vd_end_tracks {
                                                        Some(vd_end_tracks) => {
                                                            for event in vd_end_tracks {
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
                                                },
                                                None => (),
                                            }

                                            event_vec
                                        }
                                    },
                                },
                            };

                            bids.push(bid);
                        }

                        bids
                    },
                }].to_vec())
            },
        };

        Ok(response)
    }

    async fn bidding_notify_win(url: String, win_price: i32, _next_price: i32, iv: &String, connection: &Connection) {
        let encrypt_price = Self::encrypt_price(win_price, iv, connection);
        let replaced_url = url
            .replace("__WIN_PRICE__", &encode(encrypt_price.as_str()));

        let client = reqwest::ClientBuilder::new()
            .build().unwrap();
        let _ = client.get(replaced_url).send().await;
    }

    async fn bidding_notify_lose(_url: String, _lose_price: i32, _lose_reason: i32, _lose_adn_name: &String, _iv: &String, _connection: &Connection) {

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

        let key = connection.client_ekey.as_bytes();

        let cipher = Aes128EcbEnc::new(key[0..16].into())
            .encrypt_padded_mut::<Pkcs7>(&mut buffer, pos)
            .unwrap();

        BASE64_STANDARD.encode(cipher)
    }

}

fn replace_macro(orig: &String) -> String {
    let mut replaced = orig.clone();

    replaced = replaced.replace("__ADOWN_X__", "__ABS_DOWN_X__");
    replaced = replaced.replace("__ADOWN_Y__", "__ABS_DOWN_Y__");
    replaced = replaced.replace("__AUP_X__", "__ABS_UP_X__");
    replaced = replaced.replace("__AUP_Y__", "__ABS_UP_Y__");

    replaced = replaced.replace("__DP_DOWN_X__", "__R_DOWN_X__");
    replaced = replaced.replace("__DP_DOWN_Y__", "__R_DOWN_Y__");
    replaced = replaced.replace("__DP_UP_X__", "__R_UP_X__");
    replaced = replaced.replace("__DP_UP_Y__", "__R_UP_Y__");

    replaced = replaced.replace("__CLICKID__", "__CLICK_ID__");
    replaced = replaced.replace("__SEC__", "__TS_S__");

    replaced
}
