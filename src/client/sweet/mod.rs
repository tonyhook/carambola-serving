use std::{collections::HashMap, io::Write, time::Duration};

use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyInit};
use base64::prelude::*;
use chrono::TimeZone;
use chrono_tz::Tz;
use flate2::{Compression, write::GzEncoder};
use reqwest::Url;
use urlencoding::encode;

use crate::{protocol::*, Assets, Cache, Client, Connection, HttpPool, Identifiers, Price, ResultMessage};

type Aes256EcbEnc = ecb::Encryptor<aes::Aes256>;

pub mod app_asset;
pub mod app;
pub mod bid;
pub mod caid_list;
pub mod device;
pub mod geo;
pub mod imp;
pub mod network;
pub mod request;
pub mod response;
pub mod tracker;
pub mod user;
pub mod video;

pub use app_asset::SweetAppAsset;
pub use app::SweetApp;
pub use bid::SweetBid;
pub use caid_list::SweetCaidList;
pub use device::SweetDevice;
pub use geo::SweetGeo;
pub use imp::SweetImp;
pub use network::SweetNetwork;
pub use request::SweetRequest;
pub use response::SweetResponse;
pub use tracker::SweetTracker;
pub use user::SweetUser;
pub use video::SweetVideo;

pub struct Sweet {

}

impl Client for Sweet {

    async fn request(request: &Request, connection: &Connection, pool: &HttpPool, cache: &Cache) -> Result<Response, ResultMessage> {
        let tag_id = connection.client_tag_id.clone();

        let request_id = cache.get_sequence();

        let mut assets = Assets::new(request);
        let identifiers = Identifiers::new(request);

        let request_sweet = SweetRequest {
            id: {
                request_id.to_string()
            },
            ver: {
                "1.0.8".to_string()
            },
            imp: SweetImp {
                tag_id: tag_id.clone(),
                w: {
                    match request.item[0].spec.display.w {
                        Some(w) => w,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.item[0].spec.display.w is required for upstream".to_string(),
                        }),
                    }
                },
                h: {
                    match request.item[0].spec.display.h {
                        Some(h) => h,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.item[0].spec.display.h is required for upstream".to_string(),
                        }),
                    }
                },
                imptype: {
                    if assets.get_banner_size() > 0 {
                        if request.item[0].spec.display.instl == 0 {
                            3 // splash
                        } else {
                            if request.item[0].spec.display.w.is_some() && request.item[0].spec.display.h.is_some() {
                                if request.item[0].spec.display.w.unwrap() > request.item[0].spec.display.h.unwrap() * 3 {
                                    1 // banner
                                } else {
                                    4 // interstitial
                                }
                            } else {
                                1 // banner
                            }
                        }
                    } else {
                        if assets.get_asset_size("img") == 1 {
                            if request.item[0].spec.display.instl == 0 {
                                3 // splash
                            } else {
                                if request.item[0].spec.display.w.is_some() && request.item[0].spec.display.h.is_some() {
                                    if request.item[0].spec.display.w.unwrap() > request.item[0].spec.display.h.unwrap() * 3 {
                                        1 // banner
                                    } else {
                                        4 // interstitial
                                    }
                                } else {
                                    1 // banner
                                }
                            }
                        } else if assets.get_asset_size("img") > 1 || assets.get_asset_size("thumb") > 0 {
                            2 // feeds
                        } else if assets.get_asset_size("video") > 0 {
                            if request.item[0].spec.reward == 0 {
                                5 // video
                            } else {
                                6 // rewarded video
                            }
                        } else {
                            return Err(ResultMessage {
                                code: 998,
                                message: "request.item[0].spec.display is not a proper type for upstream".to_string(),
                            });
                        }
                    }
                },
                pos: {
                    match request.item[0].spec.display.pos {
                        Some(1) => 1,
                        Some(2) => 4,
                        Some(3) => 2,
                        Some(4) => 1,
                        Some(5) => 2,
                        Some(6) => 4,
                        Some(7) => 5,
                        Some(501) => 3,
                        _ => 4,
                    }
                },
                c_type: {
                    if assets.get_banner_size() > 0 {
                        [2].to_vec()
                    } else {
                        if assets.get_asset_size("img") > 0 || assets.get_asset_size("thumb") > 0 {
                            [1, 2, 3, 4].to_vec()
                        } else if assets.get_asset_size("video") > 0 {
                            [5].to_vec()
                        } else {
                            [1, 2, 3, 4].to_vec()
                        }
                    }
                },
                ci_type: {
                    [1, 2, 3].to_vec()
                },
                dp: {
                    Some(1)
                },
                bid_floor: {
                    Some(Price::to_client(connection, request.item[0].flr))
                },
            },
            app: {
                match &request.context.app {
                    Some(app) => {
                        SweetApp {
                            name: {
                                app.name.clone()
                            },
                            bundle: {
                                match &app.bundle {
                                    Some(bundle) => {
                                        bundle.clone()
                                    },
                                    None => "".to_string(),
                                }
                            },
                            ver: {
                                match &app.ver {
                                    Some(ver) => {
                                        ver.clone()
                                    },
                                    None => "".to_string(),
                                }
                            },
                            store_url: {
                                match &app.storeurl {
                                    Some(storeurl) => {
                                        storeurl.clone()
                                    },
                                    None => "".to_string(),
                                }
                            },
                        }
                    },
                    None => {
                        return Err(ResultMessage {
                            code: 998,
                            message: "request.context.app is required for upstream".to_string(),
                        });
                    }
                }
            },
            user: Some(SweetUser {
                id: request.context.user.id.clone(),
                yob: {
                    match request.context.user.yob {
                        Some(yob) =>Some(yob.to_string()),
                        None => None,
                    }
                },
                gender: {
                    match &request.context.user.gender {
                        Some(gender) => {
                            match gender.as_str() {
                                "M" => Some(1),
                                "F" => Some(2),
                                _ => Some(0),
                            }
                        },
                        None => Some(0),
                    }
                },
                keywords: request.context.user.keywords.clone(),
            }),
            device: SweetDevice {
                ip: {
                    match &request.context.device.ip {
                        Some(ip) => ip.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.ip is required for upstream".to_string(),
                        }),
                    }
                },
                ipv6: {
                    request.context.device.ipv6.clone()
                },
                ua: {
                    request.context.device.ua.clone()
                },
                os: {
                    match request.context.device.os {
                        Some(os) => {
                            match os {
                                2 => "Android".to_string(),
                                13 => "iOS".to_string(),
                                _ => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.context.device.os should be 2/13 for upstream".to_string(),
                                }),
                            }
                        },
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.os is required for upstream".to_string(),
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
                device_type: {
                    match &request.context.device.devicetype {
                        Some(3) => 3,
                        Some(4) => 1,
                        Some(5) => 2,
                        Some(7) => 3,
                        _ => 0,
                    }
                },
                geo: {
                    match &request.context.device.geo {
                        Some(geo) => {
                            SweetGeo {
                                lat: {
                                    match geo.lat {
                                        Some(lat) => lat,
                                        None => 0.0,
                                    }
                                },
                                lon: {
                                    match geo.lon {
                                        Some(lon) => lon,
                                        None => 0.0,
                                    }
                                },
                                geotype: {
                                    match geo.geotype {
                                        Some(geotype) => geotype,
                                        None => 1,
                                    }
                                },
                                country: geo.country.clone(),
                                province: geo.province.clone(),
                                city: geo.city.clone(),
                            }
                        },
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.geo is required for upstream".to_string(),
                        }),
                    }
                },
                network: SweetNetwork {
                    con_type: {
                        match &request.context.device.contype {
                            Some(2) => 1,
                            Some(4) => 2,
                            Some(5) => 3,
                            Some(6) => 4,
                            Some(7) => 5,
                            _ => 0,
                        }
                    },
                    carrier: {
                        match &request.context.device.carrier {
                            Some(carrier) => {
                                match carrier.as_str() {
                                    "cmcc" => 1,
                                    "unicom" => 3,
                                    "telecom" => 2,
                                    _ => 0,
                                }
                            },
                            None => return Err(ResultMessage {
                                code: 998,
                                message: "request.context.device.carrier is required for upstream".to_string(),
                            }),
                        }
                    },
                    imsi: {
                        match identifiers.get_id(503, 0) {
                            Some(uid) => uid.id.clone(),
                            None => {
                                match &request.context.device.carrier {
                                    Some(carrier) => {
                                        match carrier.as_str() {
                                            "cmcc" => "46000".to_string(),
                                            "unicom" => "46001".to_string(),
                                            "telecom" => "46003".to_string(),
                                            _ => "46000".to_string(),
                                        }
                                    },
                                    None => "46000".to_string(),
                                }
                            },
                        }
                    },
                    mcc: {
                        "460".to_string()
                    },
                    mnc: {
                        match &request.context.device.carrier {
                            Some(carrier) => {
                                match carrier.as_str() {
                                    "cmcc" => "00".to_string(),
                                    "unicom" => "01".to_string(),
                                    "telecom" => "03".to_string(),
                                    _ => "00".to_string(),
                                }
                            },
                            None => "00".to_string(),
                        }
                    },
                    mac: {
                        match identifiers.get_id(511, 0) {
                            Some(uid) => Some(uid.id.clone()),
                            None => None,
                        }
                    },
                    mac_md5: {
                        match identifiers.get_id(512, 0) {
                            Some(uid) => Some(uid.id.clone()),
                            None => None,
                        }
                    },
                    ssid: {
                        match identifiers.get_id(524, 0) {
                            Some(uid) => Some(uid.id.clone()),
                            None => None,
                        }
                    },
                    wifi_mac: {
                        match identifiers.get_id(522, 0) {
                            Some(uid) => Some(uid.id.clone()),
                            None => None,
                        }
                    },
                },
                brand: {
                    match &request.context.device.make {
                        Some(make) => make.clone(),
                        None => "UNKNOWN".to_string(),
                    }
                },
                model: {
                    match &request.context.device.model {
                        Some(model) => model.clone(),
                        None => "UNKNOWN".to_string(),
                    }
                },
                model_code: {
                    match &request.context.device.hwmodel {
                        Some(hwmodel) => hwmodel.clone(),
                        None => "UNKNOWN".to_string(),
                    }
                },
                orientation: {
                    match request.context.device.orientation {
                        Some(501) => 1,
                        Some(502) => 2,
                        _ => 0,
                    }
                },
                dw: {
                    match request.context.device.w {
                        Some(w) => w,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.w is required for upstream".to_string(),
                        }),
                    }
                },
                dh: {
                    match request.context.device.h {
                        Some(h) => h,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.h is required for upstream".to_string(),
                        }),
                    }
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
                ppi: {
                    match request.context.device.ppi {
                        Some(ppi) => ppi,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.ppi is required for upstream".to_string(),
                        }),
                    }
                },
                screen_size: {
                    match request.context.device.size {
                        Some(size) => size,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.size is required for upstream".to_string(),
                        }),
                    }
                },
                serialno: {
                    Some("".to_string())
                },
                an_id: {
                    match identifiers.get_id(509, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                an_id_md5: {
                    match identifiers.get_id(510, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
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
                api_level: {
                    match request.context.device.oslevel {
                        Some(oslevel) => Some(oslevel.to_string()),
                        None => None,
                    }
                },
                paid: {
                    match identifiers.get_id(519, 0) {
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
                caid_list: {
                    let mut caids = vec![];

                    match identifiers.get_id(513, 0) {
                        Some(uid) => {
                            caids.push(SweetCaidList {
                                id: {
                                    uid.id.clone()
                                },
                                ver: {
                                    uid.ver.clone()
                                },
                            });
                        },
                        None => (),
                    };

                    match identifiers.get_id(513, 1) {
                        Some(uid) => {
                            caids.push(SweetCaidList {
                                id: {
                                    uid.id.clone()
                                },
                                ver: {
                                    uid.ver.clone()
                                },
                            });
                        },
                        None => (),
                    };

                    if caids.len() > 0 {
                        Some(caids)
                    } else {
                        None
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
                open_udid: {
                    None
                },
                device_name: {
                    match identifiers.get_id(527, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                device_name_md5: {
                    match identifiers.get_id(528, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                language: {
                    request.context.device.lang.clone()
                },
                country: {
                    request.context.device.country.clone()
                },
                rom_ver: {
                    request.context.device.romv.clone()
                },
                sys_compling_time: {
                    request.context.device.ipv6.clone()
                },
                boot_time: {
                    match &request.context.device.boottime {
                        Some(boottime) => {
                            let mut boottime = boottime.clone();
                            if boottime.len() > 10 && !boottime.contains(".") {
                                boottime = boottime.split_at(10).0.to_string();
                            }
                            match boottime.split(".").nth(0).unwrap().parse::<i32>() {
                                Ok(boottime) => Some(boottime),
                                Err(_) => None,
                            }
                        },
                        None => None,
                    }
                },
                update_time: {
                    match &request.context.device.updatetime {
                        Some(updatetime) => {
                            let mut updatetime = updatetime.clone();
                            if updatetime.len() > 10 && !updatetime.contains(".") {
                                updatetime = updatetime.split_at(10).0.to_string();
                            }
                            match updatetime.split(".").nth(0).unwrap().parse::<i32>() {
                                Ok(updatetime) => Some(updatetime),
                                Err(_) => None,
                            }
                        },
                        None => None,
                    }
                },
                init_time: {
                    request.context.device.inittime.clone()
                },
                disk_size: {
                    match &request.context.device.sysdisksize {
                        Some(sysdisksize) => Some((sysdisksize / 1024 / 1024 / 1024) as i32),
                        None => None,
                    }
                },
                memory_size: {
                    match &request.context.device.sysmemory {
                        Some(sysmemory) => Some((sysmemory / 1024 / 1024 / 1024) as i32),
                        None => None,
                    }
                },
                battery_status: {
                    request.context.device.sysbatterystatus.clone()
                },
                battery_power: {
                    request.context.device.sysbatterypower.clone()
                },
                cpu_num: {
                    request.context.device.syscpu.clone()
                },
                cpu_fre: {
                    request.context.device.syscpufreq.clone()
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
                                Err(_) => None,
                            }
                        },
                        None => None,
                    }
                },
                lmt: {
                    request.context.device.lmt.clone()
                },
                laccu: {
                    None
                },
                boot_mark: {
                    request.context.device.bootmark.clone()
                },
                update_mark: {
                    request.context.device.updatemark.clone()
                },
                app_store_ver: {
                    request.context.device.storev.clone()
                },
                hms_ver: {
                    request.context.device.hmsv.clone()
                },
                skadnetwork_ver: {
                    request.context.device.skan.clone()

                },
                installed_app: {
                    let mut installed_apps = vec![];

                    match &request.context.device.app {
                        Some(app) => {
                            for app1 in app.split(",") {
                                installed_apps.push(app1.to_string());
                            }
                        },
                        None => (),
                    }

                    Some(installed_apps)
                },
                t2: {
                    None
                },
                t8: {
                    None
                },
                kid: {
                    None
                },
                caid_vendor: {
                    match identifiers.get_id(513, 0) {
                        Some(uid) => {
                            match &uid.vendor {
                                Some(vendor) => {
                                    match vendor.parse() {
                                        Ok(vendor) => Some(vendor),
                                        Err(_) => None,
                                    }
                                },
                                None => None,
                            }
                        },
                        None => None,
                    }
                },
                boot_time_nano: {
                    request.context.device.boottime.clone()
                },
                update_time_nano: {
                    request.context.device.updatetime.clone()
                },
            },
        };

        let json_string = serde_json::to_vec(&request_sweet).unwrap();
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&json_string).unwrap();
        let compressed_bytes = encoder.finish().unwrap();

        let response_sweet: SweetResponse;

        let client = {
            let pool_sweet_lock = pool.pool_sweet.clone();
            let pool_sweet = pool_sweet_lock.read().unwrap();
            pool_sweet.clone()
        };
        let response_sweet_raw = client.post("http://madx.sweet-data.com/v2/adx")
            .body(compressed_bytes)
            .header("Accept-Encoding", "gzip")
            .header("Authorization", tag_id.clone())
            .header("Bid-Pattern", "bid")
            .header("Connection", "keep-alive")
            .header("Content-Encoding", "gzip")
            .header("Content-Type", "application/json")
            .timeout(Duration::from_millis(connection.timeout))
            .send().await;

        match response_sweet_raw {
            Ok(response_sweet_raw) => {
                let status = response_sweet_raw.status();
                if status != 200 {
                    return Err(ResultMessage {
                        code: 992,
                        message: {
                            match response_sweet_raw.text().await {
                                Ok(text) => format!("upstream error {}: {}", status, text),
                                Err(_) => format!("upstream error {}", status),
                            }
                        },
                    });
                } else {
                    match response_sweet_raw.text().await {
                        Ok(text) => {
                            match serde_json::from_str::<SweetResponse>(&text) {
                                Ok(json) => {
                                    match json.code {
                                        200 => {
                                            match json.bid {
                                                Some(_) => {
                                                    response_sweet = json;
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
                                        _ => {
                                            return Err(ResultMessage {
                                                code: 992,
                                                message: format!("upstream error {}: {}", json.code, &json.msg),
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

        let bid = response_sweet.bid.unwrap();

        let response = Response {
            id: request.id.clone(),
            nbr: None,
            seatbid: {
                Some([Seatbid {
                    bid: {
                        let mut bids = vec![];

                        let link_asset = LinkAsset {
                            linktype: {
                                match bid.ci_type {
                                    Some(1) => 1,
                                    Some(2) => 1,
                                    Some(3) => 2,
                                    Some(4) => 3,
                                    _ => 1,
                                }
                            },
                            universallink: {
                                match &bid.universal_link {
                                    Some(universal_link) => Some(universal_link.clone()),
                                    None => None,
                                }
                            },
                            storeid: None,
                            deeplink: {
                                match &bid.deeplink {
                                    Some(deeplink) => Some(deeplink.clone()),
                                    None => None,
                                }
                            },
                            quickapplink: None,
                            wechatmppath: None,
                            wechatmpid: None,
                            marketurl: None,
                            downloadurl: {
                                match bid.download_url {
                                    Some(download_url) => Some(download_url.clone()),
                                    None => None,
                                }
                            },
                            url: {
                                bid.landing_url.clone()
                            },
                            urlfb: None,
                        };

                        let bid = Bid {
                            id: Some(request_id.to_string()),
                            item: request.item[0].id.clone(),
                            price: { // update later
                                match bid.bid_floor {
                                    Some(bid_floor) => {
                                        if bid_floor > 0 {
                                            bid_floor
                                        } else {
                                            connection.default_price
                                        }
                                    },
                                    None => connection.default_price,
                                }
                            },
                            burl: {
                                match &bid.win_urls {
                                    Some(win_urls) => {
                                        let mut burl = Vec::<String>::new();
                                        for win_url in win_urls {
                                            let mut nurl = win_url.clone();
                                            nurl = nurl.replace("__PRICE__", "__WIN_PRICE__");
                                            nurl = nurl.replace("__LOSS_PR__", "__2ND_PRICE__");
                                            burl.push(replace_macro(&nurl));
                                        }
                                        Some(burl)
                                    },
                                    None => None,
                                }
                            },
                            lurl: {
                                match &bid.lose_urls {
                                    Some(lose_urls) => {
                                        let mut lurl = Vec::<String>::new();
                                        for lose_url in lose_urls {
                                            let mut nurl = lose_url.clone();
                                            nurl = nurl.replace("__AD_REQID__", request_id.to_string().as_str());
                                            nurl = nurl.replace("__AD_ECPM__", "__LOSE_PRICE__");
                                            lurl.push(replace_macro(&nurl));
                                        }
                                        Some(lurl)
                                    },
                                    None => None,
                                }
                            },
                            media: Ad {
                                id: request.id.clone(),
                                display: Display {
                                    w: None,
                                    h: None,
                                    banner: {
                                        if assets.get_banner_size() > 0 {
                                            match &bid.img_urls {
                                                Some(img_urls) => {
                                                    if img_urls.len() > 0 {
                                                        Some(Banner {
                                                            img: img_urls[0].clone(),
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
                                        } else {
                                            None
                                        }
                                    },
                                    native: {
                                        let mut asset_vec = vec![];

                                        match &bid.video {
                                            Some(video) => {
                                                asset_vec.push(Asset {
                                                    id: assets.consume_asset("video"),
                                                    req: 1,
                                                    video: Some(VideoAsset {
                                                        url: video.url.clone(),
                                                        mime: None,
                                                        w: video.vw.clone(),
                                                        h: video.vh.clone(),
                                                        dur: Some(video.duration),
                                                        skipoffset: video.skip_sec.clone(),
                                                        size: video.size.clone(),
                                                        delivery: {
                                                            match video.prefetch {
                                                                Some(true) => Some(2),
                                                                _ => None,
                                                            }
                                                        },
                                                        orientation: None,
                                                        autolanding: {
                                                            match video.com_landing {
                                                                Some(true) => 1,
                                                                _ => 0,
                                                            }
                                                        },
                                                        clickable: {
                                                            match video.pro_landing {
                                                                Some(true) => 1,
                                                                _ => 0,
                                                            }
                                                        },
                                                    }),
                                                    title: None,
                                                    img: None,
                                                    data: None,
                                                    html: None,
                                                    app: None,
                                                });

                                                for cover_img_url1 in &video.cover_img_url {
                                                    asset_vec.push(Asset {
                                                        id: assets.consume_asset("video#cover"),
                                                        req: 1,
                                                        img: Some(ImageAsset {
                                                            url: cover_img_url1.clone(),
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
                                                match &video.button_text {
                                                    Some(button_text) => {
                                                        asset_vec.push(Asset {
                                                            id: assets.consume_asset("video#end#button#text"),
                                                            req: 0,
                                                            data: Some(DataAsset {
                                                                value: button_text.clone(),
                                                                len: Some(button_text.len() as i32),
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
                                                match &video.end_img_url {
                                                    Some(end_img_url) => {
                                                        asset_vec.push(Asset {
                                                            id: assets.consume_asset("video#end#img"),
                                                            req: 0,
                                                            img: Some(ImageAsset {
                                                                url: end_img_url.clone(),
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
                                                match &video.end_html {
                                                    Some(end_img_url) => {
                                                        asset_vec.push(Asset {
                                                            id: assets.consume_asset("video#end#html"),
                                                            req: 0,
                                                            html: {
                                                                Some(HtmlAsset {
                                                                    html: Some(end_img_url.clone()),
                                                                    link: None,
                                                                    len: None,
                                                                })
                                                            },
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

                                        match &bid.img_urls {
                                            Some(img_urls) => {
                                                for img_url in img_urls {
                                                    if assets.get_asset_size("img") > 0 {
                                                        asset_vec.push(Asset {
                                                            id: assets.consume_asset("img"),
                                                            req: 1,
                                                            img: Some(ImageAsset {
                                                                url: img_url.clone(),
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
                                                                url: img_url.clone(),
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

                                        match &bid.app {
                                            Some(app) => {
                                                asset_vec.push(Asset {
                                                    id: assets.consume_asset("app"),
                                                    req: 0,
                                                    app: Some(AppAsset {
                                                        name: {
                                                            match app.name.clone() {
                                                                Some(name) => name,
                                                                None => "".to_string(),
                                                            }
                                                        },
                                                        desc: None,
                                                        descurl: None,
                                                        domain: None,
                                                        bundle: app.bundle.clone(),
                                                        ver: app.ver.clone(),
                                                        developer: app.developer.clone(),
                                                        icon: None,
                                                        storeid: None,
                                                        storeurl: None,
                                                        paid: 0,
                                                        size: app.size.clone(),
                                                        md5: None,
                                                        registration: None,
                                                        privacy: None,
                                                        privacyurl: app.privacy_url.clone(),
                                                        permission: app.perm_content.clone(),
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
                                    },
                                    event: {
                                        let mut event_vec = vec![];

                                        for tracker in  &bid.trackers {
                                            let eventtype = {
                                                match tracker.trackertype {
                                                    1 => 501,
                                                    2 => 502,
                                                    3 => 601,
                                                    4 => 602,
                                                    5 => 603,
                                                    6 => 604,
                                                    7 => 605,
                                                    8 => 509,
                                                    16 => 505,
                                                    17 => 504,
                                                    18 => 507,
                                                    19 => 506,
                                                    20 => 503,
                                                    31 => 701,
                                                    32 => 702,
                                                    33 => 703,
                                                    34 => 704,
                                                    35 => 705,
                                                    36 => 710,
                                                    37 => 715,
                                                    38 => 716,
                                                    39 => 719,
                                                    40 => 720,
                                                    41 => 713,
                                                    42 => 714,
                                                    43 => 708,
                                                    44 => 709,
                                                    45 => 724,
                                                    46 => 712,
                                                    47 => 717,
                                                    48 => 718,
                                                    49 => 711,
                                                    50 => 721,
                                                    _ => 0,
                                                }
                                            };
                                            for event in &tracker.urls {
                                                event_vec.push(Event {
                                                    eventtype,
                                                    method: 1,
                                                    url: {
                                                        if eventtype == 501 {
                                                            let url = replace_macro(event);
                                                            let price = match bid.bid_floor {
                                                                Some(bid_floor) => {
                                                                    if bid_floor > 0 {
                                                                        bid_floor
                                                                    } else {
                                                                        connection.default_price
                                                                    }
                                                                },
                                                                None => connection.default_price,
                                                            };
                                                            let encrypt_price = Self::encrypt_price(price, &"".to_string(), connection);
                                                            url.replace("__PRICE__", &encode(encrypt_price.as_str()))
                                                        } else {
                                                            replace_macro(event)
                                                        }
                                                    },
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        }
                                        match &bid.click_area_report_urls {
                                            Some(click_area_report_urls) => {
                                                for click_area_report_url in click_area_report_urls {
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
                                                }
                                            },
                                            None => (),
                                        }

                                        event_vec
                                    }
                                },
                                advertiser: None,
                                advertisericon: {
                                    bid.icon_url.clone()
                                },
                            },
                        };

                        bids.push(bid);

                        bids
                    }
                }].to_vec())
            },
        };

        Ok(response)
    }

    async fn bidding_notify_win(url: String, win_price: i32, next_price: i32, iv: &String, connection: &Connection, pool: &HttpPool) {
        let encrypt_price = Self::encrypt_price(win_price, iv, connection);
        let encrypt_next_price = Self::encrypt_price(next_price, iv, connection);
        let replaced_url = url
            .replace("__WIN_PRICE__", &encode(encrypt_price.as_str()))
            .replace("__2ND_PRICE__", &encode(encrypt_next_price.as_str()));

        let client = {
            let pool_sweet_lock = pool.pool_sweet.clone();
            let pool_sweet = pool_sweet_lock.read().unwrap();
            pool_sweet.clone()
        };
        let _ = client.get(replaced_url).send().await;
    }

    async fn bidding_notify_lose(url: String, lose_price: i32, _lose_reason: i32, _lose_adn_name: &String, iv: &String, connection: &Connection, pool: &HttpPool) {
        let encrypt_price = Self::encrypt_price(lose_price, iv, connection);
        let replaced_url = url
            .replace("__LOSE_PRICE__", &encode(encrypt_price.as_str()));

        let client = {
            let pool_sweet_lock = pool.pool_sweet.clone();
            let pool_sweet = pool_sweet_lock.read().unwrap();
            pool_sweet.clone()
        };
        let _ = client.get(replaced_url).send().await;
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

        let key = &connection.client_ekey.as_bytes();

        let cipher = Aes256EcbEnc::new(key[0..32].into())
            .encrypt_padded_mut::<Pkcs7>(&mut buffer, pos)
            .unwrap();

        let message_base64 = BASE64_STANDARD.encode(cipher);

        encode(message_base64.as_str()).to_string()
    }

}

fn replace_macro(orig: &String) -> String {
    let mut replaced = orig.clone();

    replaced = replaced.replace("__ADN_TYPE__", "");
    replaced = replaced.replace("__ADN_NAME__", "__LOSE_ADN_NAME__");
    replaced = replaced.replace("__AD_N__", "");
    replaced = replaced.replace("__AD_TI__", "");
    replaced = replaced.replace("__IS_S__", "");
    replaced = replaced.replace("__IS_C__", "");

    replaced = replaced.replace("__TIME_START__", "__EVENT_TIME_START__");
    replaced = replaced.replace("__TIME_START_SE__", "__EVENT_TIME_START_S__");
    replaced = replaced.replace("__TIME_END__", "__EVENT_TIME_END__");
    replaced = replaced.replace("__TIME_END_SE__", "__EVENT_TIME_END_S__");
    replaced = replaced.replace("__ADOWN_X__", "__ABS_DOWN_X__");
    replaced = replaced.replace("__ADOWN_Y__", "__ABS_DOWN_Y__");
    replaced = replaced.replace("__AUP_X__", "__ABS_UP_X__");
    replaced = replaced.replace("__AUP_Y__", "__ABS_UP_Y__");
    replaced = replaced.replace("__LONGITUDE__", "__LNG__");
    replaced = replaced.replace("__LATITUDE__", "__LAT__");
    replaced = replaced.replace("__DISPLAY_LUX__", "__LT_X__");
    replaced = replaced.replace("__DISPLAY_LUY__", "__LT_Y__");
    replaced = replaced.replace("__DISPLAY_RDX__", "__RB_X__");
    replaced = replaced.replace("__DISPLAY_RDY__", "__RB_Y__");
    replaced = replaced.replace("__BUTTON_LUX__", "__BUTTON_LT_X__");
    replaced = replaced.replace("__BUTTON_LUY__", "__BUTTON_LT_Y__");
    replaced = replaced.replace("__BUTTON_RDX__", "__BUTTON_RB_X__");
    replaced = replaced.replace("__BUTTON_RDY__", "__BUTTON_RB_Y__");
    replaced = replaced.replace("__DP_DOWN_X__", "__DOWN_DP_X__");
    replaced = replaced.replace("__DP_DOWN_Y__", "__DOWN_DP_Y__");
    replaced = replaced.replace("__DP_UP_X__", "__UP_DP_X__");
    replaced = replaced.replace("__DP_UP_Y__", "__UP_DP_Y__");
    replaced = replaced.replace("__TARGET_APP_INSTALL__", "__DP_TARGET__");
    replaced = replaced.replace("__VIDEO_BEHAVIOR__", "__VIDEO_PLAY_TRIGGER_0__");
    replaced = replaced.replace("__VIDEOPLAY_CUR__", "__VIDEO_PLAY_PROGRESS_S__");
    replaced = replaced.replace("__VIDEO_START_TIME__", "__VIDEO_BEGIN_TIME__");
    replaced = replaced.replace("__VIDEO_PLAY_FIRSR_FRAME__", "__VIDEO_FIRST_FRAME__");
    replaced = replaced.replace("__VIDEO_PLAY_LAST_FRAME__", "__VIDEO_LAST_FRAME__");
    replaced = replaced.replace("__VIDEO_SCENE__", "__VIDEO_PLAY_SCENE__");
    replaced = replaced.replace("__VIDEO_STATUS__", "__VIDEO_PLAY_STATUS__");

    replaced
}
