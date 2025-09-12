use std::{collections::HashMap, io::Write, time::Duration};

use chrono::{Datelike, Local};
use flate2::{Compression, write::GzEncoder};
use reqwest::Url;
use sha1::{Digest, Sha1};
use urlencoding::encode;

use crate::{protocol::*, Assets, Cache, Client, Connection, HttpPool, Identifiers, Price, ResultMessage};

pub mod ad_info;
pub mod app_asset;
pub mod app;
pub mod caid;
pub mod deal;
pub mod device_id;
pub mod device;
pub mod ext;
pub mod geo;
pub mod image;
pub mod installed_app;
pub mod mini_program;
pub mod network;
pub mod request;
pub mod response;
pub mod slot;
pub mod track;
pub mod user;
pub mod video;

pub use ad_info::JmediumAdInfo;
pub use app_asset::JmediumAppAsset;
pub use app::JmediumApp;
pub use caid::JmediumCaid;
pub use deal::JmediumDeal;
pub use device_id::JmediumDeviceId;
pub use device::JmediumDevice;
pub use ext::JmediumExt;
pub use geo::JmediumGeo;
pub use image::JmediumImage;
pub use installed_app::JmediumInstalledApp;
pub use mini_program::JmediumMiniProgram;
pub use network::JmediumNetwork;
pub use request::JmediumRequest;
pub use response::JmediumResponse;
pub use slot::JmediumSlot;
pub use track::JmediumTrack;
pub use user::JmediumUser;
pub use video::JmediumVideo;

pub struct Jmedium {

}

impl Client for Jmedium {

    async fn request(request: &Request, connection: &Connection, pool: &HttpPool, cache: &Cache) -> Result<Response, ResultMessage> {
        let request_id = cache.get_sequence();

        let mut assets = Assets::new(request);
        let identifiers = Identifiers::new(request);

        let request_jmedium = JmediumRequest {
            request_id: {
                request_id.to_string()
            },
            slot: JmediumSlot {
                ad_slot_id: {
                    match connection.client_tag_id.parse() {
                        Ok(client_tag_id) => client_tag_id,
                        Err(_) => 0,
                    }
                },
                slottype: {
                    let mut slot_type = 0;
                    let reward = request.item[0].spec.reward;
                    let instl = request.item[0].spec.display.instl;

                    if assets.get_banner_size() > 0 {
                        if instl == 1 {
                            slot_type = 2;
                        } else {
                            if request.item[0].spec.display.w > request.item[0].spec.display.h {
                                slot_type = 1;
                            } else {
                                slot_type = 3;
                            }
                        }
                    }
                    if assets.get_asset_size("img") > 0 {
                        slot_type = 4;
                    }
                    if assets.get_asset_size("video") > 0 {
                        if instl == 1 {
                            slot_type = 7;
                        } else {
                            if reward == 1 {
                                slot_type = 5;
                            } else {
                                slot_type = 6;
                            }
                        }
                    }

                    slot_type
                },
                width: {
                    match request.item[0].spec.display.w {
                        Some(w) => {
                            w
                        },
                        None => 0,
                    }
                },
                height: {
                    match request.item[0].spec.display.h {
                        Some(h) => {
                            h
                        },
                        None => 0,
                    }
                },
            },
            deal: Some(JmediumDeal {
                bidfloor: Some(Price::to_client(connection, request.item[0].flr.map(f64::from)) as i32)
            }),
            app: JmediumApp {
                name: {
                    match &request.context.app {
                        Some(app) => app.name.clone(),
                        None => "".to_string(),
                    }
                },
                ver_name: {
                    match &request.context.app {
                        Some(app) => {
                            match &app.ver {
                                Some(ver) => ver.clone(),
                                None => "".to_string(),
                            }
                        },
                        None => "".to_string(),
                    }
                },
                ver_code: 0,
                pkg_name: {
                    match &request.context.app {
                        Some(app) => {
                            match &app.bundle {
                                Some(bundle) => bundle.clone(),
                                None => "".to_string(),
                            }
                        },
                        None => "".to_string(),
                    }
                },
                app_store_version: {
                    match &request.context.device.storev {
                        Some(storev) => storev.clone(),
                        None => "".to_string(),
                    }
                },
            },
            user: Some(JmediumUser {
                gender: {
                    match &request.context.user.gender {
                        Some(gender) => {
                            match gender.as_str() {
                                "M" => Some(1),
                                "F" => Some(2),
                                _ => Some(0),
                            }
                        },
                        None => None,
                    }
                },
                age: {
                    match request.context.user.yob {
                        Some(yob) => {
                            let year = Local::now().year();
                            let age = year - yob;
                            match age {
                                18..=23 => Some(1),
                                24..=30 => Some(2),
                                31..=40 => Some(3),
                                41..=49 => Some(4),
                                50.. => Some(5),
                                _ => Some(0),
                            }
                        },
                        None => None,
                    }
                },
                interest: None,
                installed_apps: {
                    let mut installed_apps = vec![];

                    match &request.context.device.app {
                        Some(app) => {
                            for app1 in app.split(",") {
                                installed_apps.push(JmediumInstalledApp {
                                    pkg_name: None,
                                    app_name: Some(app1.trim().to_string().clone()),
                                    app_version: None,
                                    app_version_code: None,
                                    system_app: None,
                                    first_install_time: None,
                                    last_update_time: None,
                                });
                            }
                        },
                        None => (),
                    }

                    Some(installed_apps)
                },
            }),
            device: JmediumDevice {
                os_type: {
                    match request.context.device.os {
                        Some(2) => 2,
                        Some(13) => 1,
                        Some(28) => 5,
                        Some(501) => 7,
                        _ => 0,
                    }
                },
                devicetype: {
                    match request.context.device.devicetype {
                        Some(1) => 1,
                        Some(4) => 1,
                        Some(5) => 2,
                        Some(3) => 3,
                        Some(7) => 3,
                        Some(2) => 4,
                        _ => 0,
                    }
                },
                os_version: {
                    match &request.context.device.osv {
                        Some(osv) => osv.clone(),
                        None => "".to_string(),
                    }
                },
                os_ui_version: {
                    request.context.device.uiv.clone()
                },
                android_api_level: {
                    request.context.device.oslevel.clone()
                },
                sys_compiling_time: {
                    match &request.context.device.romtime {
                        Some(romtime) => romtime.clone(),
                        None => "".to_string(),
                    }
                },
                sys_update_time: {
                    match &request.context.device.updatetime {
                        Some(updatetime) => updatetime.clone(),
                        None => "".to_string(),
                    }
                },
                sys_startup_time: {
                    match &request.context.device.boottime {
                        Some(boottime) => boottime.clone(),
                        None => "".to_string(),
                    }
                },
                sys_init_time: {
                    match &request.context.device.inittime {
                        Some(inittime) => inittime.clone(),
                        None => "".to_string(),
                    }
                },
                sys_update_time_nano_sec: {
                    match &request.context.device.updatetime {
                        Some(updatetime) => updatetime.replace(".", "::"),
                        None => "".to_string(),
                    }
                },
                sys_startup_time_milli_sec: {
                    match &request.context.device.boottime {
                        Some(boottime) => {
                            match boottime.parse::<f64>() {
                                Ok(time) =>  ((time * 1000.0) as i64).to_string(),
                                Err(_) => "".to_string(),
                            }
                        },
                        None => "".to_string(),
                    }
                },
                birth_mark: {
                    request.context.device.initmark.clone()
                },
                boot_mark: {
                    request.context.device.bootmark.clone()
                },
                update_mark: {
                    request.context.device.updatemark.clone()
                },
                rom_version: {
                    request.context.device.romv.clone()
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
                cpu_num: {
                    match request.context.device.syscpu {
                        Some(syscpu) => syscpu,
                        None => 0,
                    }
                },
                sys_disk_size: {
                    match request.context.device.sysdisksize {
                        Some(sysdisksize) => sysdisksize,
                        None => 0,
                    }
                },
                sys_memory_size: {
                    match request.context.device.sysmemory {
                        Some(sysmemory) => sysmemory,
                        None => 0,
                    }
                },
                model: {
                    match &request.context.device.model {
                        Some(model) => model.clone(),
                        None => "".to_string(),
                    }
                },
                hardware_model: {
                    request.context.device.hwmodel.clone()
                },
                language: {
                    match &request.context.device.lang {
                        Some(lang) => lang.clone(),
                        None => "".to_string(),
                    }
                },
                time_zone: {
                    match &request.context.device.timezone {
                        Some(timezone) => timezone.clone(),
                        None => "".to_string(),
                    }
                },
                hms_version: {
                    request.context.device.hmsv.clone()
                },
                harmony_os_version: {
                    match request.context.device.os {
                        Some(501) => request.context.device.osv.clone(),
                        _ => None,
                    }
                },
                hag_version: {
                    request.context.device.storev.clone()
                },
                support_deeplink: {
                    1
                },
                support_universal: {
                    1
                },
                make: {
                    match &request.context.device.make {
                        Some(make) => make.clone(),
                        None => "".to_string(),
                    }
                },
                brand: {
                    match &request.context.device.brand {
                        Some(brand) => brand.clone(),
                        None => "".to_string(),
                    }
                },
                imsi: {
                    match identifiers.get_id(503, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                width: {
                    match request.context.device.w {
                        Some(w) => w,
                        None => 0,
                    }
                },
                height: {
                    match request.context.device.h {
                        Some(h) => h,
                        None => 0,
                    }
                },
                density: {
                    match request.context.device.pxratio {
                        Some(pxratio) => Some(pxratio),
                        None => None,
                    }
                },
                dpi: {
                    match request.context.device.ppi {
                        Some(ppi) => Some(ppi),
                        None => None,
                    }
                },
                ppi: {
                    request.context.device.ppi.clone()
                },
                orientation: {
                    match request.context.device.orientation {
                        Some(501) => 1,
                        Some(502) => 2,
                        _ => 0,
                    }
                },
                screen_size: {
                    match request.context.device.size {
                        Some(size) => size.to_string(),
                        None => "".to_string(),
                    }
                },
                serialno: {
                    None
                },
            },
            device_id: JmediumDeviceId {
                imei: {
                    match identifiers.get_id(501, 0) {
                        Some(uid) => uid.id.clone(),
                        None => "".to_string(),
                    }
                },
                imei_md5: {
                    match identifiers.get_id(502, 0) {
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
                oaid_md5: {
                    match identifiers.get_id(506, 0) {
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
                android_id_md5: {
                    match identifiers.get_id(510, 0) {
                        Some(uid) => uid.id.clone(),
                        None => "".to_string(),
                    }
                },
                android_id_sha1: {
                    match identifiers.get_id(509, 0) {
                        Some(uid) => {
                            let mut hasher = Sha1::new();
                            hasher.update(uid.id.clone());
                            format!("{:x}", hasher.finalize())
                        },
                        None => "".to_string(),
                    }
                },
                idfa: {
                    match identifiers.get_id(507, 0) {
                        Some(uid) => uid.id.clone(),
                        None => "".to_string(),
                    }
                },
                idfa_md5: {
                    match identifiers.get_id(508, 0) {
                        Some(uid) => uid.id.clone(),
                        None => "".to_string(),
                    }
                },
                idfv: {
                    match identifiers.get_id(515, 0) {
                        Some(uid) => uid.id.clone(),
                        None => "".to_string(),
                    }
                },
                open_udid: {
                    "".to_string()
                },
            },
            caids: {
                let mut caids = vec![];

                match identifiers.get_id(513, 0) {
                    Some(uid) => {
                        caids.push(JmediumCaid {
                            caid: {
                                uid.id.clone()
                            },
                            version: {
                                match &uid.ver {
                                    Some(ver) => ver.clone(),
                                    None => "".to_string(),
                                }
                            },
                            generate_time: 0,
                            vendor: {
                                match &uid.vendor {
                                    Some(vendor) => {
                                        match vendor.parse() {
                                            Ok(vendor) => vendor,
                                            Err(_) => 0,
                                        }
                                    },
                                    None => 0,
                                }
                            },
                        });
                    },
                    None => (),
                };

                match identifiers.get_id(513, 1) {
                    Some(uid) => {
                        caids.push(JmediumCaid {
                            caid: {
                                uid.id.clone()
                            },
                            version: {
                                match &uid.ver {
                                    Some(ver) => ver.clone(),
                                    None => "".to_string(),
                                }
                            },
                            generate_time: 0,
                            vendor: {
                                match &uid.vendor {
                                    Some(vendor) => {
                                        match vendor.parse() {
                                            Ok(vendor) => vendor,
                                            Err(_) => 0,
                                        }
                                    },
                                    None => 0,
                                }
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
            network: JmediumNetwork {
                user_agent: {
                    request.context.device.ua.clone()
                },
                ip: {
                    match &request.context.device.ip {
                        Some(ip) => Some(ip.clone()),
                        None => None,
                    }
                },
                ipv6: {
                    match &request.context.device.ipv6 {
                        Some(ipv6) => Some(ipv6.clone()),
                        None => None,
                    }
                },
                mac: {
                    match identifiers.get_id(511, 0) {
                        Some(uid) => uid.id.clone(),
                        None => "".to_string(),
                    }
                },
                mac_md5: {
                    match identifiers.get_id(512, 0) {
                        Some(uid) => uid.id.clone(),
                        None => "".to_string(),
                    }
                },
                mac_sha1: {
                    match identifiers.get_id(511, 0) {
                        Some(uid) => {
                            let mut hasher = Sha1::new();
                            hasher.update(uid.id.clone());
                            format!("{:x}", hasher.finalize())
                        },
                        None => "".to_string(),
                    }
                },
                ssid: {
                    match identifiers.get_id(524, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                bssid: {
                    match identifiers.get_id(529, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                carrier: {
                    match &request.context.device.carrier {
                        Some(carrier) => {
                            match carrier.as_str() {
                                "cmcc" => 1,
                                "unicom" => 2,
                                "telecom" => 3,
                                _ => 0,
                            }
                        },
                        None => 0,
                    }
                },
                network_type: {
                    match &request.context.device.contype {
                        Some(1) => 10,
                        Some(2) => 1,
                        Some(3) => 0,
                        Some(4) => 2,
                        Some(5) => 3,
                        Some(6) => 4,
                        Some(7) => 5,
                        _ => 0,
                    }
                },
                mcc: {
                    match &request.context.device.carrier {
                        Some(carrier) => {
                            match carrier.as_str() {
                                "cmcc" => Some("460".to_string()),
                                "unicom" => Some("460".to_string()),
                                "telecom" => Some("460".to_string()),
                                "cbn" => Some("460".to_string()),
                                _ => None,
                            }
                        },
                        None => None,
                    }
                },
                mnc: {
                    match &request.context.device.carrier {
                        Some(carrier) => {
                            match carrier.as_str() {
                                "cmcc" => Some("00".to_string()),
                                "unicom" => Some("01".to_string()),
                                "telecom" => Some("03".to_string()),
                                "cbn" => Some("15".to_string()),
                                _ => None,
                            }
                        },
                        None => None,
                    }
                },
                country: {
                    request.context.device.country.clone()
                },
            },
            geo: {
                match &request.context.device.geo {
                    Some(geo) => {
                        Some(JmediumGeo {
                            coordinate_type: geo.coordinate.clone(),
                            latitude: geo.lat.clone(),
                            longitude: geo.lon.clone(),
                            timestamp: geo.timestamp.clone(),
                        })
                    },
                    None => None,
                }
            },
            ext: Some(JmediumExt {
                support_wechat: {
                    true
                },
                paid: {
                    match identifiers.get_id(519, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
            }),
        };

        let json_string = serde_json::to_vec(&request_jmedium).unwrap();
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&json_string).unwrap();
        let compressed_bytes = encoder.finish().unwrap();

        let response_jmedium: JmediumResponse;

        let client = {
            let pool_jmedium_lock = pool.pool_jmedium.clone();
            let pool_jmedium = pool_jmedium_lock.read().unwrap();
            pool_jmedium.clone()
        };
        let response_jmedium_raw = client.post(if connection.test { "http://api-test.jmedium.cn/api/ssp/ads" } else { "http://api.jmedium.cn/api/ssp/ads" })
            .body(compressed_bytes)
            .header("Accept-Encoding", "gzip")
            .header("Connection", "keep-alive")
            .header("Content-Encoding", "gzip")
            .header("Content-Type", "application/json")
            .timeout(Duration::from_millis(connection.timeout))
            .send().await;

        match response_jmedium_raw {
            Ok(response_jmedium_raw) => {
                let status = response_jmedium_raw.status();
                if status != 200 {
                    return Err(ResultMessage {
                        code: 992,
                        message: {
                            match response_jmedium_raw.text().await {
                                Ok(text) => format!("upstream error {}: {}", status, text),
                                Err(_) => format!("upstream error {}", status),
                            }
                        },
                    });
                } else {
                    match response_jmedium_raw.text().await {
                        Ok(text) => {
                            match serde_json::from_str::<JmediumResponse>(&text) {
                                Ok(json) => {
                                    response_jmedium = json;
                                    match response_jmedium.code {
                                        200 => (),
                                        210924 => {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: "".to_string(),
                                            });
                                        },
                                        210926 => {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: "".to_string(),
                                            });
                                        },
                                        110001 => {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: format!("upstream error: {}", response_jmedium.msg),
                                            });
                                        }
                                        _ => {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: format!("upstream error code {}: {}", response_jmedium.code, response_jmedium.msg),
                                            });
                                        },
                                    }
                                    if response_jmedium.data.is_none() {
                                        return Err(ResultMessage {
                                            code: 993,
                                            message: "".to_string(),
                                        });
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

                for ad_info in &response_jmedium.data.unwrap() {
                    let mut bid = vec![];

                    let link_asset = LinkAsset {
                        linktype: {
                            match ad_info.interaction_type {
                                0 => 1,
                                1 => 1,
                                3 => 1,
                                4 => 2,
                                5 => 1,
                                6 => 3,
                                _ => 1,
                            }
                        },
                        universallink: {
                            ad_info.universal_link.clone()
                        },
                        storeid: {
                            ad_info.market_url.clone()
                        },
                        deeplink: {
                            ad_info.deeplink.clone()
                        },
                        quickapplink: None,
                        wechatmppath: {
                            match &ad_info.mini_program {
                                Some(mini_program) => {
                                    match &mini_program.mini_program_path {
                                        Some(mini_program_path) => Some(mini_program_path.clone()),
                                        None => None,
                                    }
                                },
                                None => None,
                            }
                        },
                        wechatmpid: {
                            match &ad_info.mini_program {
                                Some(mini_program) => {
                                    match &mini_program.mini_program_id {
                                        Some(mini_program_id) => Some(mini_program_id.clone()),
                                        None => None,
                                    }
                                },
                                None => None,
                            }
                        },
                        marketurl: None,
                        downloadurl: {
                            ad_info.download_url.clone()
                        },
                        url: {
                            match &ad_info.kwai_landing_page_url {
                                Some(kwai_landing_page_url) => kwai_landing_page_url.clone(),
                                None => {
                                    match &ad_info.landing_page_url {
                                        Some(landing_page_url) => replace_macro(&landing_page_url),
                                        None => "".to_string(),
                                    }
                                },
                            }
                        },
                        urlfb: None,
                    };

                    bid.push(Bid {
                        id: Some(request_id.to_string()),
                        item: request.item[0].id.clone(),
                        price: { // update later
                            match ad_info.bid_price {
                                Some(bid_price) => bid_price,
                                None => connection.default_price,
                            }
                        },
                        burl: {
                            match &ad_info.win_notice_urls {
                                Some(win_notice_urls) => {
                                    let mut burl = Vec::<String>::new();
                                    for win_notice_url in win_notice_urls {
                                        let mut nurl = win_notice_url.clone();
                                        nurl = nurl.replace("__MF_AD_REQID__", request_id.to_string().as_str());
                                        nurl = nurl.replace("__MF_WIN_ECPM__", "__WIN_PRICE__");
                                        nurl = nurl.replace("__MF_LOSS_PR__", "__2ND_PRICE__");
                                        burl.push(replace_macro(&nurl));
                                    }
                                    Some(burl)
                                },
                                None => None,
                            }
                        },
                        lurl: {
                            match &ad_info.loss_notice_urls {
                                Some(loss_notice_urls) => {
                                    let mut lurl = Vec::<String>::new();
                                    for loss_notice_url in loss_notice_urls {
                                        let mut nurl = loss_notice_url.clone();
                                        nurl = nurl.replace("__MF_AD_REQID__", request_id.to_string().as_str());
                                        nurl = nurl.replace("__MF_WIN_ECPM__", "__LOSE_PRICE__");
                                        nurl = nurl.replace("__MF_LOSS_PR__", "");
                                        lurl.push(replace_macro(&nurl));
                                    }
                                    Some(lurl)
                                },
                                None => None,
                            }
                        },
                        media: Ad {
                            id: ad_info.request_id.clone(),
                            display: Display {
                                w: None,
                                h: None,
                                banner: {
                                    if assets.get_banner_size() > 0 {
                                        match &ad_info.images {
                                            Some(images) => {
                                                if images.len() > 0 {
                                                    Some(Banner {
                                                        img: {
                                                            match &images[0].url {
                                                                Some(url) => url.clone(),
                                                                None => "".to_string(),
                                                            }
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

                                        match &ad_info.title {
                                            Some(title) => {
                                                asset_vec.push(Asset {
                                                    id: assets.consume_asset("title"),
                                                    req: 1,
                                                    title: Some(TitleAsset {
                                                        text: title.clone(),
                                                        subtitle: None,
                                                        desc: {
                                                            match &ad_info.desc {
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
                                        match &ad_info.ad_icons {
                                            Some(ad_icons) => {
                                                for ad_icon in ad_icons {
                                                    match ad_icon {
                                                        Some(ad_icon) => {
                                                            asset_vec.push(Asset {
                                                                id: assets.consume_asset("icon"),
                                                                req: 1,
                                                                title: None,
                                                                img: Some(ImageAsset {
                                                                    url: ad_icon.clone(),
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
                                                        },
                                                        None => (),
                                                    }
                                                }
                                            },
                                            None => (),
                                        }
                                        match &ad_info.images {
                                            Some(images) => {
                                                for image in images {
                                                    if assets.get_asset_size("img") > 0 {
                                                        asset_vec.push(Asset {
                                                            id: assets.consume_asset("img"),
                                                            req: 1,
                                                            img: {
                                                                Some(ImageAsset {
                                                                    url: {
                                                                        match &image.url {
                                                                            Some(url) => url.clone(),
                                                                            None => "".to_string(),
                                                                        }
                                                                    },
                                                                    mime: None,
                                                                    w: image.width,
                                                                    h: image.height,
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
                                                                    url: {
                                                                        match &image.url {
                                                                            Some(url) => url.clone(),
                                                                            None => "".to_string(),
                                                                        }
                                                                    },
                                                                    mime: None,
                                                                    w: image.width,
                                                                    h: image.height,
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
                                        match &ad_info.video {
                                            Some(video) => {
                                                asset_vec.push(Asset {
                                                    id: assets.consume_asset("video"),
                                                    req: 1,
                                                    title: None,
                                                    img: None,
                                                    video: Some(VideoAsset {
                                                        url: {
                                                            match &video.url {
                                                                Some(url) => url.clone(),
                                                                None => "".to_string(),
                                                            }
                                                        },
                                                        mime: None,
                                                        w: video.width,
                                                        h: video.height,
                                                        dur: video.duration,
                                                        skipoffset: video.force_duration,
                                                        size: video.size,
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
                                                match &video.end_url_type {
                                                    Some(1) => {
                                                        match &video.end_url {
                                                            Some(end_url) => {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("video#end#img"),
                                                                    req: 1,
                                                                    img: {
                                                                        Some(ImageAsset {
                                                                            url: end_url.clone(),
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
                                                    Some(2) => {
                                                        match &video.end_url {
                                                            Some(end_url) => {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("video#end#html"),
                                                                    req: 1,
                                                                    html: {
                                                                        Some(HtmlAsset {
                                                                            html: None,
                                                                            link: Some(end_url.clone()),
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
                                                    Some(3) => {
                                                        match &video.end_url {
                                                            Some(end_url) => {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("video#end#html"),
                                                                    req: 1,
                                                                    html: {
                                                                        Some(HtmlAsset {
                                                                            html: Some(end_url.clone()),
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
                                                    _ => (),
                                                }
                                            },
                                            None => (),
                                        }

                                        match &ad_info.app {
                                            Some(app) => {
                                                asset_vec.push(Asset {
                                                    id: assets.consume_asset("app"),
                                                    req: 0,
                                                    app: Some(AppAsset {
                                                        name: {
                                                            match &app.name {
                                                                Some(name) => name.clone(),
                                                                None => "".to_string(),
                                                            }
                                                        },
                                                        desc: app.introduction_info.clone(),
                                                        descurl: app.introduction_info_url.clone(),
                                                        domain: None,
                                                        bundle: app.pkg_name.clone(),
                                                        ver: app.version.clone(),
                                                        developer: app.corporate.clone(),
                                                        icon: app.icon_url.clone(),
                                                        storeid: None,
                                                        storeurl: None,
                                                        paid: 0,
                                                        size: {
                                                            match app.size {
                                                                Some(size) => Some(size as i32),
                                                                None => None,
                                                            }
                                                        },
                                                        md5: None,
                                                        registration: app.record_number.clone(),
                                                        privacy: None,
                                                        privacyurl: app.privacy_policy_url.clone(),
                                                        permission: app.permission_info.clone(),
                                                        permissionurl: app.permission_url.clone(),
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

                                    for url in &ad_info.track.show_urls {
                                        event_vec.push(Event {
                                            eventtype: 501,
                                            method: 1,
                                            url: replace_macro(url),
                                            header: None,
                                            content: None,
                                        });
                                    }
                                    for url in &ad_info.track.click_urls {
                                        event_vec.push(Event {
                                            eventtype: 502,
                                            method: 1,
                                            url: replace_macro(url),
                                            header: None,
                                            content: None,
                                        });
                                    }
                                    match &ad_info.track.ad_load_urls {
                                        Some(ad_load_urls) => {
                                            for url in ad_load_urls {
                                                event_vec.push(Event {
                                                    eventtype: 501,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.ad_skip_urls {
                                        Some(ad_skip_urls) => {
                                            for url in ad_skip_urls {
                                                event_vec.push(Event {
                                                    eventtype: 509,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.ad_close_urls {
                                        Some(ad_close_urls) => {
                                            for url in ad_close_urls {
                                                event_vec.push(Event {
                                                    eventtype: 509,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.wechat_open_urls {
                                        Some(wechat_open_urls) => {
                                            for url in wechat_open_urls {
                                                event_vec.push(Event {
                                                    eventtype: 502,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.start_download_urls {
                                        Some(start_download_urls) => {
                                            for url in start_download_urls {
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
                                    match &ad_info.track.finish_download_urls {
                                        Some(finish_download_urls) => {
                                            for url in finish_download_urls {
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
                                    match &ad_info.track.pause_download_urls {
                                        Some(pause_download_urls) => {
                                            for url in pause_download_urls {
                                                event_vec.push(Event {
                                                    eventtype: 607,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.continue_download_urls {
                                        Some(continue_download_urls) => {
                                            for url in continue_download_urls {
                                                event_vec.push(Event {
                                                    eventtype: 608,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.delete_download_urls {
                                        Some(delete_download_urls) => {
                                            for url in delete_download_urls {
                                                event_vec.push(Event {
                                                    eventtype: 609,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.start_install_urls {
                                        Some(start_install_urls) => {
                                            for url in start_install_urls {
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
                                    match &ad_info.track.finish_install_urls {
                                        Some(finish_install_urls) => {
                                            for url in finish_install_urls {
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
                                    match &ad_info.track.active_app_urls {
                                        Some(active_app_urls) => {
                                            for url in active_app_urls {
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
                                    match &ad_info.track.deeplink_try_urls {
                                        Some(deeplink_try_urls) => {
                                            for url in deeplink_try_urls {
                                                event_vec.push(Event {
                                                    eventtype: 503,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.deeplink_success_urls {
                                        Some(deeplink_success_urls) => {
                                            for url in deeplink_success_urls {
                                                event_vec.push(Event {
                                                    eventtype: 504,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.deeplink_failure_urls {
                                        Some(deeplink_failure_urls) => {
                                            for url in deeplink_failure_urls {
                                                event_vec.push(Event {
                                                    eventtype: 505,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.deeplink_click_urls {
                                        Some(deeplink_click_urls) => {
                                            for url in deeplink_click_urls {
                                                event_vec.push(Event {
                                                    eventtype: 502,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.deeplink_installed_urls {
                                        Some(deeplink_installed_urls) => {
                                            for url in deeplink_installed_urls {
                                                event_vec.push(Event {
                                                    eventtype: 506,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.deeplink_uninstall_urls {
                                        Some(deeplink_uninstall_urls) => {
                                            for url in deeplink_uninstall_urls {
                                                event_vec.push(Event {
                                                    eventtype: 507,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.video_start_urls {
                                        Some(video_start_urls) => {
                                            for url in video_start_urls {
                                                event_vec.push(Event {
                                                    eventtype: 701,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.video_click_urls {
                                        Some(video_click_urls) => {
                                            for url in video_click_urls {
                                                event_vec.push(Event {
                                                    eventtype: 502,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.video_complete_urls {
                                        Some(video_complete_urls) => {
                                            for url in video_complete_urls {
                                                event_vec.push(Event {
                                                    eventtype: 705,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.video_fail_urls {
                                        Some(video_fail_urls) => {
                                            for url in video_fail_urls {
                                                event_vec.push(Event {
                                                    eventtype: 724,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.video_close_urls {
                                        Some(video_close_urls) => {
                                            for url in video_close_urls {
                                                event_vec.push(Event {
                                                    eventtype: 711,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.video_skip_urls {
                                        Some(video_skip_urls) => {
                                            for url in video_skip_urls {
                                                event_vec.push(Event {
                                                    eventtype: 710,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.video_pause_urls {
                                        Some(video_pause_urls) => {
                                            for url in video_pause_urls {
                                                event_vec.push(Event {
                                                    eventtype: 708,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.video_resume_urls {
                                        Some(video_resume_urls) => {
                                            for url in video_resume_urls {
                                                event_vec.push(Event {
                                                    eventtype: 709,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.video_replay_urls {
                                        Some(video_replay_urls) => {
                                            for url in video_replay_urls {
                                                event_vec.push(Event {
                                                    eventtype: 712,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.video_mute_urls {
                                        Some(video_mute_urls) => {
                                            for url in video_mute_urls {
                                                event_vec.push(Event {
                                                    eventtype: 713,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.video_unmute_urls {
                                        Some(video_unmute_urls) => {
                                            for url in video_unmute_urls {
                                                event_vec.push(Event {
                                                    eventtype: 714,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.video_fullscreen_urls {
                                        Some(video_fullscreen_urls) => {
                                            for url in video_fullscreen_urls {
                                                event_vec.push(Event {
                                                    eventtype: 715,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.video_exit_fullscreen_urls {
                                        Some(video_exit_fullscreen_urls) => {
                                            for url in video_exit_fullscreen_urls {
                                                event_vec.push(Event {
                                                    eventtype: 716,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.video_upscroll_urls {
                                        Some(video_upscroll_urls) => {
                                            for url in video_upscroll_urls {
                                                event_vec.push(Event {
                                                    eventtype: 717,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.video_downscroll_urls {
                                        Some(video_downscroll_urls) => {
                                            for url in video_downscroll_urls {
                                                event_vec.push(Event {
                                                    eventtype: 718,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.video_quartile_urls {
                                        Some(video_quartile_urls) => {
                                            for url in video_quartile_urls {
                                                event_vec.push(Event {
                                                    eventtype: 702,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.video_half_urls {
                                        Some(video_half_urls) => {
                                            for url in video_half_urls {
                                                event_vec.push(Event {
                                                    eventtype: 703,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.video_three_quartile_urls {
                                        Some(video_three_quartile_urls) => {
                                            for url in video_three_quartile_urls {
                                                event_vec.push(Event {
                                                    eventtype: 704,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.video_play_3s_urls {
                                        Some(video_play_3s_urls) => {
                                            for url in video_play_3s_urls {
                                                event_vec.push(Event {
                                                    eventtype: 706,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.video_play_5s_urls {
                                        Some(video_play_5s_urls) => {
                                            for url in video_play_5s_urls {
                                                event_vec.push(Event {
                                                    eventtype: 707,
                                                    method: 1,
                                                    url: replace_macro(url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_info.track.click_area_report_urls {
                                        Some(click_area_report_urls) => {
                                            let parsed_url = Url::parse(click_area_report_urls.as_str());
                                            match parsed_url {
                                                Ok(parsed_url) => {
                                                    let hash_query: HashMap<_, _> = parsed_url.query_pairs().into_owned().collect();
                                                    let sid = hash_query.get("sid");
                                                    let creative_id = hash_query.get("creative_id");
                                                    if sid.is_some() && creative_id.is_some() {
                                                        event_vec.push(Event {
                                                            eventtype: 502,
                                                            method: 502,
                                                            url: replace_macro(click_area_report_urls),
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
                    });

                    seatbids.push(Seatbid {
                        bid,
                    });
                }

                Some(seatbids)
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
            let pool_jmedium_lock = pool.pool_jmedium.clone();
            let pool_jmedium = pool_jmedium_lock.read().unwrap();
            pool_jmedium.clone()
        };
        let _ = client.get(replaced_url).send().await;
    }

    async fn bidding_notify_lose(url: String, lose_price: i32, _lose_reason: i32, _lose_adn_name: &String, iv: &String, connection: &Connection, pool: &HttpPool) {
        let encrypt_price = Self::encrypt_price(lose_price, iv, connection);
        let replaced_url = url
            .replace("__LOSE_PRICE__", &encode(encrypt_price.as_str()));

        let client = {
            let pool_jmedium_lock = pool.pool_jmedium.clone();
            let pool_jmedium = pool_jmedium_lock.read().unwrap();
            pool_jmedium.clone()
        };
        let _ = client.get(replaced_url).send().await;
    }

    fn encrypt_price(price: i32, _iv: &String, _connection: &Connection) -> String {
        price.to_string()
    }

}

fn replace_macro(orig: &String) -> String {
    let mut replaced = orig.clone();

    replaced = replaced.replace("__MF_IFR__", "__LOSE_REASON__");
    replaced = replaced.replace("__MF_ADN_TYPE__", "");
    replaced = replaced.replace("__MF_ADN_NAME__", "__LOSE_ADN_NAME__");
    replaced = replaced.replace("__MF_AD_N__", "");
    replaced = replaced.replace("__MF_AD_TITLE__", "");
    replaced = replaced.replace("__MF_IS_S__", "");
    replaced = replaced.replace("__MF_IS_C__", "");
    replaced = replaced.replace("__MF_AD_MT__", "");
    replaced = replaced.replace("__MF_AD_URL__", "");

    replaced = replaced.replace("__MF_CLICK_ID__", "__CLICK_ID__");
    replaced = replaced.replace("__MF_VIDEO_TIME__", "__VIDEO_TIME__");
    replaced = replaced.replace("__MF_PLAY_BEGIN_TIME__", "__VIDEO_BEGIN_TIME__");
    replaced = replaced.replace("__MF_PLAY_END_TIME__", "__VIDEO_END_TIME__");
    replaced = replaced.replace("__MF_PLAY_FIRST_FRAME__", "__VIDEO_FIRST_FRAME__");
    replaced = replaced.replace("__MF_PLAY_LAST_FRAME__", "__VIDEO_LAST_FRAME__");
    replaced = replaced.replace("__MF_PLAY_PROGRESS_MS__", "__VIDEO_PLAY_PROGRESS__");
    replaced = replaced.replace("__MF_PLAY_PROGRESS__", "__VIDEO_PLAY_PROGRESS_S__");
    replaced = replaced.replace("__MF_PLAY_PROGRESS_RATE__", "__VIDEO_PLAY_RATIO__");
    replaced = replaced.replace("__MF_EVENT_TIME_MS__", "__EVENT_TIME_START__");
    replaced = replaced.replace("__MF_EVENT_TIME__", "__EVENT_TIME_START_S__");
    replaced = replaced.replace("__MF_EVENT_END_TIME_MS__", "__EVENT_TIME_END__");
    replaced = replaced.replace("__MF_EVENT_END_TIME__", "__EVENT_TIME_END_S__");
    replaced = replaced.replace("__MF_CLICK_ID__", "__CLICK_ID__");
    replaced = replaced.replace("__MF_CLICKAREA__", "__CLICKAREA__");
    replaced = replaced.replace("__MF_WIDTH__", "__WIDTH__");
    replaced = replaced.replace("__MF_HEIGHT__", "__HEIGHT__");
    replaced = replaced.replace("__MF_ABS_DOWN_X__", "__ABS_DOWN_X__");
    replaced = replaced.replace("__MF_ABS_DOWN_Y__", "__ABS_DOWN_Y__");
    replaced = replaced.replace("__MF_ABS_UP_X__", "__ABS_UP_X__");
    replaced = replaced.replace("__MF_ABS_UP_Y__", "__ABS_UP_Y__");
    replaced = replaced.replace("__MF_POS_DOWN_X__", "__R_DOWN_X__");
    replaced = replaced.replace("__MF_POS_DOWN_Y__", "__R_DOWN_Y__");
    replaced = replaced.replace("__MF_POS_UP_X__", "__R_UP_X__");
    replaced = replaced.replace("__MF_POS_UP_Y__", "__R_UP_Y__");
    replaced = replaced.replace("__MF_DP_WIDTH__", "__DP_WIDTH__");
    replaced = replaced.replace("__MF_DP_HEIGHT__", "__DP_HEIGHT__");
    replaced = replaced.replace("__MF_POS_DP_DOWN_X__", "-999");
    replaced = replaced.replace("__MF_POS_DP_DOWN_Y__", "-999");
    replaced = replaced.replace("__MF_POS_DP_UP_X__", "-999");
    replaced = replaced.replace("__MF_POS_DP_UP_Y__", "-999");
    replaced = replaced.replace("__MF_AIT__", "__SLD__");
    replaced = replaced.replace("__MF_X_MAX_ACC__", "__X_MAX_ACC__");
    replaced = replaced.replace("__MF_Y_MAX_ACC__", "__Y_MAX_ACC__");
    replaced = replaced.replace("__MF_Z_MAX_ACC__", "__Z_MAX_ACC__");
    replaced = replaced.replace("__MF_TURN_X__", "__TURN_X__");
    replaced = replaced.replace("__MF_TURN_Y__", "__TURN_Y__");
    replaced = replaced.replace("__MF_TURN_Z__", "__TURN_Z__");
    replaced = replaced.replace("__MF_TURN_TIME__", "__TURN_TIME__");

    replaced
}
