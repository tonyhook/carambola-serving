use std::time::Duration;

use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyInit};
use base64::prelude::*;
use chrono::{Datelike, Local, TimeZone, Utc};
use chrono_tz::Tz;
use urlencoding::encode;

use crate::{protocol::*, Assets, Cache, Client, Connection, HttpPool, Identifiers, Price, ResultMessage};

type Aes128EcbEnc = ecb::Encryptor<aes::Aes128>;

pub mod adslot;
pub mod adv;
pub mod app;
pub mod device;
pub mod monitor;
pub mod monitor_url_visit;
pub mod request;
pub mod response;
pub mod user;
pub mod video;
pub mod url_header;
pub mod url_visit;

pub use adslot::RichmobAdslot;
pub use adv::RichmobAdv;
pub use app::RichmobApp;
pub use device::RichmobDevice;
pub use monitor::RichmobMonitor;
pub use monitor_url_visit::RichmobMonitorUrlVisit;
pub use request::RichmobRequest;
pub use response::RichmobResponse;
pub use user::RichmobUser;
pub use video::RichmobVideo;
pub use url_header::RichmobUrlHeader;
pub use url_visit::RichmobUrlVisit;

pub struct Richmob {

}

impl Client for Richmob {

    async fn request(request: &Request, connection: &Connection, pool: &HttpPool, cache: &Cache) -> Result<Response, ResultMessage> {
        let request_id = cache.get_sequence();

        let mut assets = Assets::new(request);
        let identifiers = Identifiers::new(request);

        let request_richmob = RichmobRequest {
            request_id: {
                request_id.to_string()
            },
            doc_version: {
                "1.0.2".to_string()
            },
            source: {
                Some("app".to_string())
            },
            ua: {
                request.context.device.ua.clone()
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
            user: Some(RichmobUser {
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
                age: {
                    match request.context.user.yob {
                        Some(yob) => {
                            let year = Local::now().year();
                            Some(year - yob)
                        },
                        None => Some(0),
                    }
                },
                keywords: {
                    request.context.user.keywords.clone()
                },
            }),
            app: {
                match &request.context.app {
                    Some(app) => RichmobApp {
                        app_name: {
                            match &connection.client_media_appname {
                                Some(client_media_appname) => client_media_appname.clone(),
                                None => app.name.clone(),
                            }
                        },
                        package_name: {
                            match &connection.client_media_apppackage {
                                Some(client_media_apppackage) => client_media_apppackage.clone(),
                                None => {
                                    match &connection.client_media_apppackage {
                                        Some(client_media_apppackage) => client_media_apppackage.clone(),
                                        None => {
                                            match &app.bundle {
                                                Some(bundle) => bundle.clone(),
                                                None => return Err(ResultMessage {
                                                    code: 998,
                                                    message: "request.context.app.bundle is required for upstream".to_string(),
                                                }),
                                            }
                                        },
                                    }
                                },
                            }
                        },
                        app_category: {
                            None
                        },
                        version: {
                            match &app.ver {
                                Some(ver) => ver.clone(),
                                None => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.context.app.ver is required for upstream".to_string(),
                                }),
                            }
                        },
                        latitude: {
                            match &request.context.device.geo {
                                Some(geo) => {
                                    match geo.lat {
                                        Some(lat) => Some(format!("{}", lat).to_string()),
                                        None => None,
                                    }
                                },
                                None => None,
                            }
                        },
                        longitude: {
                            match &request.context.device.geo {
                                Some(geo) => {
                                    match geo.lon {
                                        Some(lon) => Some(format!("{}", lon).to_string()),
                                        None => None,
                                    }
                                },
                                None => None,
                            }
                        },
                        store_url: {
                            match &app.storeurl {
                                Some(storeurl) => {
                                    storeurl.clone()
                                },
                                None => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.context.app.storeurl is required for upstream".to_string(),
                                }),
                            }
                        },
                    },
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.app is required for upstream".to_string(),
                    }),
                }
            },
            device: RichmobDevice {
                device_id: {
                    match identifiers.get_id(507, 0) {
                        Some(uid) => uid.id.clone(),
                        None => {
                            match identifiers.get_id(509, 0) {
                                Some(uid) => uid.id.clone(),
                                None => "".to_string(),
                            }
                        },
                    }
                },
                device_id_md5: {
                    match identifiers.get_id(508, 0) {
                        Some(uid) => uid.id.clone(),
                        None => {
                            match identifiers.get_id(510, 0) {
                                Some(uid) => uid.id.clone(),
                                None => "".to_string(),
                            }
                        },
                    }
                },
                imei: {
                    match identifiers.get_id(501, 0) {
                        Some(uid) => uid.id.clone(),
                        None => "".to_string(),
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
                        Some(uid) => uid.id.clone(),
                        None => "".to_string(),
                    }
                },
                oaid_md5: {
                    match identifiers.get_id(506, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                open_udid: {
                    None
                },
                ssid: {
                    match identifiers.get_id(524, 0) {
                        Some(uid) => uid.id.clone(),
                        None => "".to_string(),
                    }
                },
                wifi_mac: {
                    match identifiers.get_id(522, 0) {
                        Some(uid) => uid.id.clone(),
                        None => "".to_string(),
                    }
                },
                phone_name: {
                    match identifiers.get_id(517, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                phone_name_md5: {
                    match identifiers.get_id(518, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                power_on_time: {
                    match &request.context.device.boottime {
                        Some(boottime) => {
                            let boot_timestamp = boottime.split(".").nth(0).unwrap();
                            match boot_timestamp.parse::<i64>() {
                                Ok(boot_timestamp) => {
                                    let timestamp = Utc::now().timestamp();
                                    Some((timestamp - boot_timestamp).to_string())
                                },
                                Err(_) => None,
                            }
                        },
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
                imsi: {
                    match identifiers.get_id(503, 0) {
                        Some(uid) => uid.id.clone(),
                        None => "".to_string(),
                    }
                },
                device_type: {
                    match request.context.device.devicetype {
                        Some(devicetype) => {
                            match devicetype {
                                1 => 1,
                                2 => 0,
                                3 => 0,
                                4 => 1,
                                5 => 2,
                                6 => 0,
                                7 => 3,
                                8 => 0,
                                _ => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.context.device.type should be 1-8 for upstream".to_string(),
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
                                2 => "Android".to_string(),
                                13 => "IOS".to_string(),
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
                os_version: {
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
                model: {
                    match &request.context.device.model {
                        Some(model) => model.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.model is required for upstream".to_string(),
                        }),
                    }
                },
                language: {
                    match &request.context.device.lang {
                        Some(lang) => lang.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.lang is required for upstream".to_string(),
                        }),
                    }
                },
                network: {
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
                operator_type: {
                    match &request.context.device.carrier {
                        Some(carrier) => {
                            match carrier.as_str() {
                                "cmcc" => 1,
                                "unicom" => 3,
                                "telecom" => 2,
                                _ => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.context.device.carrier should be cmcc/unicom/telecom for upstream".to_string(),
                                }),
                            }
                        },
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.carrier is required for upstream".to_string(),
                        }),
                    }
                },
                swidth: {
                    match request.context.device.w {
                        Some(w) => w,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.w is required for upstream".to_string(),
                        }),
                    }
                },
                sheight: {
                    match request.context.device.h {
                        Some(h) => h,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.h is required for upstream".to_string(),
                        }),
                    }
                },
                orientation: {
                    match request.context.device.orientation {
                        Some(orientation) => {
                            match orientation {
                                501 => Some(1),
                                502 => Some(2),
                                _ => None,
                            }
                        },
                        None => None,
                    }
                },
                dpi: {
                     match request.context.device.ppi {
                        Some(ppi) => Some(ppi as f64),
                        None => None,
                    }
                },
                rom_version: {
                    match &request.context.device.romv {
                        Some(romv) => romv.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.romv is required for upstream".to_string(),
                        }),
                    }
                },
                sys_compling_time: {
                    match &request.context.device.romtime {
                        Some(romtime) => romtime.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.romtime is required for upstream".to_string(),
                        }),
                    }
                },
                boot_time_sec: {
                    match &request.context.device.boottime {
                        Some(boottime) => {
                            match boottime.split(".").nth(0).unwrap().parse::<i32>() {
                                Ok(boottime) => boottime,
                                Err(_) => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.context.device.boottime is malformatted for upstream".to_string(),
                                }),
                            }
                        },
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.boottime is required for upstream".to_string(),
                        }),
                    }
                },
                boot_time_nano_sec: {
                    match &request.context.device.boottime {
                        Some(boottime) => boottime.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.boottime is required for upstream".to_string(),
                        }),
                    }
                },
                os_update_time_sec: {
                    match &request.context.device.updatetime {
                        Some(updatetime) => {
                            match updatetime.split(".").nth(0).unwrap().parse::<i32>() {
                                Ok(updatetime) => updatetime,
                                Err(_) => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.context.device.updatetime is malformatted for upstream".to_string(),
                                }),
                            }
                        },
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.updatetime is required for upstream".to_string(),
                        }),
                    }
                },
                os_update_time_nano_sec: {
                    match &request.context.device.updatetime {
                        Some(updatetime) => updatetime.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.updatetime is required for upstream".to_string(),
                        }),
                    }
                },
                disk_size: {
                    match &request.context.device.sysdisksize {
                        Some(sysdisksize) => (sysdisksize / 1024 / 1024 / 1024) as i32,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.sysdisksize is required for upstream".to_string(),
                        }),
                    }
                },
                battery_status: {
                    match request.context.device.sysbatterystatus {
                        Some(sysbatterystatus) => sysbatterystatus,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.sysbatterystatus is required for upstream".to_string(),
                        }),
                    }
                },
                battery_power: {
                    match request.context.device.sysbatterypower {
                        Some(sysbatterypower) => sysbatterypower,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.sysbatterypower is required for upstream".to_string(),
                        }),
                    }
                },
                memory_size: {
                    match &request.context.device.sysmemory {
                        Some(sysmemory) => (sysmemory / 1024 / 1024 / 1024) as i32,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.sysmemory is required for upstream".to_string(),
                        }),
                    }
                },
                cpu_num: {
                    match request.context.device.syscpu {
                        Some(syscpu) => syscpu,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.syscpu is required for upstream".to_string(),
                        }),
                    }
                },
                cpu_frequency: {
                    match request.context.device.syscpufreq {
                        Some(syscpufreq) => syscpufreq,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.syscpufreq is required for upstream".to_string(),
                        }),
                    }
                },
                model_code: {
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
                lmt: {
                    match request.context.device.lmt {
                        Some(lmt) => lmt,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.lmt is required for upstream".to_string(),
                        }),
                    }
                },
                laccu: {
                    match &request.context.device.geo {
                        Some(geo) => {
                            match geo.laccu {
                                Some(laccu) => laccu,
                                None => 1,
                            }
                        },
                        None => 1,
                    }
                },
                caid: {
                    match identifiers.get_id(513, 0) {
                        Some(uid) => uid.id.clone(),
                        None => "".to_string(),
                    }
                },
                caid_md5: {
                    None
                },
                caid_version: {
                    match identifiers.get_id(513, 0) {
                        Some(uid) => {
                            match &uid.ver {
                                Some(ver) => ver.clone(),
                                None => "".to_string(),
                            }
                        }
                        None => "".to_string(),
                    }
                },
                caid_vendor: {
                    match identifiers.get_id(513, 0) {
                        Some(uid) => {
                            match &uid.vendor {
                                Some(vendor) => {
                                    match vendor.parse::<i32>() {
                                        Ok(vendor) => Some(vendor),
                                        Err(_) => return Err(ResultMessage {
                                            code: 998,
                                            message: "caid_vendor should be 0/1/2 for upstream".to_string(),
                                        }),
                                    }
                                }
                                None => None,
                            }
                        },
                        None => None,
                    }
                },
                boot_mark: {
                    match &request.context.device.bootmark {
                        Some(bootmark) => Some(bootmark.clone()),
                        None => None,
                    }
                },
                update_mark: {
                    match &request.context.device.updatemark {
                        Some(updatemark) => Some(updatemark.clone()),
                        None => None,
                    }
                },
                app_store_version: {
                    match &request.context.device.storev {
                        Some(storev) => Some(storev.clone()),
                        None => None,
                    }
                },
                hms_version: {
                    match &request.context.device.hmsv {
                        Some(hmsv) => Some(hmsv.clone()),
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
                screen_size: {
                    match request.context.device.size {
                        Some(size) => size,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.size is required for upstream".to_string(),
                        }),
                    }
                },
                idfv: {
                    match identifiers.get_id(515, 0) {
                        Some(uid) => uid.id.clone(),
                        None => "".to_string(),
                    }
                },
                mcc: {
                    match &request.context.device.mccmnc {
                        Some(mccmnc) => {
                            if mccmnc.len() >= 3 {
                                mccmnc[0..3].to_string()
                            } else {
                                "460".to_string()
                            }
                        },
                        None => "460".to_string(),
                    }
                },
                mnc: {
                    match &request.context.device.mccmnc {
                        Some(mccmnc) => {
                            if mccmnc.len() >= 6 {
                                mccmnc[4..6].to_string()
                            } else {
                                match &request.context.device.carrier {
                                    Some(carrier) => {
                                        match carrier.as_str() {
                                            "cmcc" => "00".to_string(),
                                            "unicom" => "01".to_string(),
                                            "telecom" => "11".to_string(),
                                            _ => return Err(ResultMessage {
                                                code: 998,
                                                message: "request.context.device.carrier should be cmcc/unicom/telecom for upstream".to_string(),
                                            }),
                                        }
                                    },
                                    None => return Err(ResultMessage {
                                        code: 998,
                                        message: "request.context.device.carrier should be cmcc/unicom/telecom for upstream".to_string(),
                                    }),
                                }
                            }
                        },
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.carrier should be cmcc/unicom/telecom for upstream".to_string(),
                        }),
                    }
                },
                skadnetwork_versions: {
                    match &request.context.device.skan {
                        Some(skan) => skan.to_vec(),
                        None => vec![],
                    }
                },
                sys_init_time: {
                    match &request.context.device.inittime {
                        Some(inittime) => Some(inittime.clone()),
                        None => None,
                    }
                },
                api_level: {
                    None
                },
                country: {
                    match &request.context.device.country {
                        Some(country) => Some(country.clone()),
                        None => None,
                    }
                },
                paid: {
                    match identifiers.get_id(519, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                app_list: {
                    match &request.context.device.app {
                        Some(app) => Some(app.split(",").map(|s| s.to_string()).collect()),
                        None => None,
                    }
                },
                mnt_id: {
                    match &request.context.device.mntid {
                        Some(mntid) => Some(mntid.clone()),
                        None => None,
                    }
                },
            },
            adslot: RichmobAdslot {
                slot_id: connection.client_tag_id.split("|").nth(0).unwrap().to_string(),
                ad_type: {
                    let mut ad_type = 0;
                    if assets.get_banner_size() > 0 {
                        if request.item[0].spec.display.w.is_some() && request.item[0].spec.display.h.is_some() {
                            if request.item[0].spec.display.w.unwrap() / request.item[0].spec.display.h.unwrap() > 2 {
                                ad_type = 1
                            }
                        } else {
                            if request.item[0].spec.display.instl == 0 {
                                ad_type = 3
                            } else {
                                ad_type = 4
                            }
                        }
                    } else if assets.get_asset_size("video") == 0 {
                        ad_type = 2
                    } else if assets.get_asset_size("video") > 0 {
                        let video = assets.get_current_asset("video").unwrap().video.clone().unwrap();
                        if video.comp.is_some() {
                            ad_type = 5
                        } else {
                            ad_type = 6
                        }
                    } else {
                        return Err(ResultMessage {
                            code: 998,
                            message: "request.item[0].spec.display.displayfmt or request.item[0].spec.display.nativefmt (feed / video) is required for upstream".to_string(),
                        });
                    }
                    ad_type
                },
                position: {
                    match request.item[0].spec.display.pos {
                        Some(pos) => {
                            match pos {
                                0 => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.item[0].spec.display.pos should be 1-5/7/501 for upstream".to_string(),
                                }),
                                1 => 1,
                                2 => 4,
                                3 => 2,
                                4 => 1,
                                5 => 2,
                                6 => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.item[0].spec.display.pos should be 1-5/7/501 for upstream".to_string(),
                                }),
                                7 => 5,
                                501 => 3,
                                _ => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.item[0].spec.display.pos should be 1-5/7/501 for upstream".to_string(),
                                }),
                            }
                        },
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.item[0].spec.display.pos is required for upstream".to_string(),
                        }),
                    }
                },
                accepted_creative_types: {
                    None
                },
                accepted_interaction_type: {
                    None
                },
                width: {
                    match request.item[0].spec.display.w {
                        Some(w) => w,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.item[0].spec.display.w is required for upstream".to_string(),
                        }),
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
                price: {
                    Some(Price::to_client(connection, request.item[0].flr.map(f64::from)) as i32)
                },
            },
            deep_link: {
                true
            },
        };

        let response_richmob: RichmobResponse;

        let client = {
            let pool_richmob_lock = pool.pool_richmob.clone();
            let pool_richmob = pool_richmob_lock.read().unwrap();
            pool_richmob.clone()
        };
        let response_richmob_raw = client.post("http://ad.richmob.cn/api/ad")
            .json(&request_richmob)
            .header("Accept-Encoding", "gzip")
            .header("Connection", "keep-alive")
            .header("Content-Type", "application/json")
            .timeout(Duration::from_millis(connection.timeout))
            .send().await;

        match response_richmob_raw {
            Ok(response_richmob_raw) => {
                let status = response_richmob_raw.status();
                if status != 200 {
                    return Err(ResultMessage {
                        code: 992,
                        message: {
                            match response_richmob_raw.text().await {
                                Ok(text) => format!("upstream error {}: {}", status, text),
                                Err(_) => format!("upstream error {}", status),
                            }
                        },
                    });
                } else {
                    match response_richmob_raw.text().await {
                        Ok(text) => {
                            match serde_json::from_str::<RichmobResponse>(&text) {
                                Ok(json) => {
                                    response_richmob = json;

                                    match response_richmob.code {
                                        1001 => {
                                            return Err(ResultMessage {
                                                code: 993,
                                                message: "".to_string(),
                                            });
                                        },
                                        1002 => {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: "upstream error: processing failed".to_string(),
                                            });
                                        },
                                        1003 => {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: "upstream error: lost key parameter".to_string(),
                                            });
                                        },
                                        1004 => {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: "upstream error: parameter convert failed".to_string(),
                                            });
                                        },
                                        1005 => {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: "upstream error: invalid input".to_string(),
                                            });
                                        },
                                        1006 => {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: "upstream error: bad phone number format".to_string(),
                                            });
                                        },
                                        1007 => {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: "upstream error: invalid request".to_string(),
                                            });
                                        },
                                        1008 => {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: "upstream error: invalid ip".to_string(),
                                            });
                                        },
                                        1009 => {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: "upstream error: internal error".to_string(),
                                            });
                                        },
                                        4000 => {
                                            return Err(ResultMessage {
                                                code: 993,
                                                message: "".to_string(),
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
                Some([Seatbid {
                    bid: {
                        let mut bids = vec![];

                        match &response_richmob.adv {
                            Some(adv) => {
                                let link_asset = LinkAsset {
                                    linktype: {
                                        match adv.interaction_type {
                                            Some(1) => 1,
                                            Some(2) => 1,
                                            Some(3) => 2,
                                            Some(4) => 3,
                                            _ => 1,
                                        }
                                    },
                                    universallink: {
                                        match &adv.universal_link {
                                            Some(universal_link) => Some(universal_link.clone()),
                                            None => None,
                                        }
                                    },
                                    storeid: None,
                                    deeplink: {
                                        match &adv.deeplink {
                                            Some(deeplink_url) => Some(deeplink_url.clone()),
                                            None => None,
                                        }
                                    },
                                    quickapplink: None,
                                    wechatmppath: None,
                                    wechatmpid: None,
                                    marketurl: None,
                                    downloadurl: {
                                        match adv.interaction_type {
                                            Some(3) => {
                                                match &adv.download_url {
                                                    Some(download_url) => Some(download_url.clone()),
                                                    None => None,
                                                }
                                            },
                                            _ => None,
                                        }
                                    },
                                    url: adv.click_ad_url.clone(),
                                    urlfb: None,
                                };

                                let bid = Bid {
                                    id: Some(request_id.to_string()),
                                    item: request.item[0].id.clone(),
                                    price: { // update later
                                        match adv.price {
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
                                        match &adv.win_notify_urls {
                                            Some(win_notify_urls) => {
                                                let mut burl = Vec::<String>::new();
                                                for win_notify_url in win_notify_urls {
                                                    let mut nurl = win_notify_url.clone();
                                                    nurl = nurl.replace("__PRICE__", "__WIN_PRICE__");
                                                    burl.push(replace_macro(&nurl));
                                                }
                                                Some(burl)
                                            },
                                            None => None,
                                        }
                                    },
                                    lurl: {
                                        match &adv.lose_notify_urls {
                                            Some(lose_notify_urls) => {
                                                let mut lurl = Vec::<String>::new();
                                                for lose_notify_url in lose_notify_urls {
                                                    let mut nurl = lose_notify_url.clone();
                                                    nurl = nurl.replace("__LOSE_BIDFLOOR__", "__LOSE_PRICE__");
                                                    lurl.push(replace_macro(&nurl));
                                                }
                                                Some(lurl)
                                            },
                                            None => None,
                                        }
                                    },
                                    media: Ad {
                                        id: response_richmob.request_id.clone(),
                                        display: Display {
                                            w: {
                                                adv.width
                                            },
                                            h: {
                                                adv.height
                                            },
                                            banner: {
                                                if assets.get_banner_size() > 0 {
                                                    let images = adv.img_urls.clone().unwrap();
                                                    if images.len() > 0 {
                                                        Some(Banner {
                                                            img: {
                                                                images.get(0).unwrap().to_string()
                                                            },
                                                            link: Some(link_asset.clone()),
                                                        })
                                                    } else {
                                                        None
                                                    }
                                                } else {
                                                    None
                                                }
                                            },
                                            native: {
                                                if assets.get_asset_total_size() > 0 {
                                                    let mut asset_vec = vec![];

                                                    match &adv.title {
                                                        Some(title) => {
                                                            asset_vec.push(Asset {
                                                                id: assets.consume_asset("title"),
                                                                req: 1,
                                                                title: Some(TitleAsset {
                                                                    text: title.clone(),
                                                                    subtitle: None,
                                                                    desc: adv.descriptions.clone(),
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
                                                    match &adv.img_urls {
                                                        Some(img_urls) => {
                                                            for img_url in img_urls {
                                                                if assets.get_asset_size("img") > 0 {
                                                                    asset_vec.push(Asset {
                                                                        id: assets.consume_asset("img"),
                                                                        req: 1,
                                                                        img: {
                                                                            Some(ImageAsset {
                                                                                url: img_url.clone(),
                                                                                mime: None,
                                                                                w: adv.width,
                                                                                h: adv.height,
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
                                                                                url: img_url.clone(),
                                                                                mime: None,
                                                                                w: adv.width,
                                                                                h: adv.height,
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
                                                    match &adv.video {
                                                        Some(video) => {
                                                            asset_vec.push(Asset {
                                                                id: assets.consume_asset("video"),
                                                                req: 1,
                                                                video: Some(VideoAsset {
                                                                    url: video.src.clone(),
                                                                    mime: None,
                                                                    w: video.video_width,
                                                                    h: video.video_height,
                                                                    dur: Some(video.video_duration),
                                                                    skipoffset: video.skip_seconds,
                                                                    size: Some(video.size),
                                                                    delivery: {
                                                                        match video.prefetch {
                                                                            Some(false) => Some(1),
                                                                            Some(true) => Some(2),
                                                                            _ => None,
                                                                        }
                                                                    },
                                                                    orientation: None,
                                                                    autolanding: {
                                                                        match video.auto_landing {
                                                                            Some(false) => 0,
                                                                            Some(true) => 1,
                                                                            _ => 0,
                                                                        }
                                                                    },
                                                                    clickable: {
                                                                        match video.click_able {
                                                                            Some(false) => 0,
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
                                                            match &video.cover_img_url {
                                                                Some(cover_img_url) => {
                                                                    for cover_img_url1 in cover_img_url.iter() {
                                                                        asset_vec.push(Asset {
                                                                            id: assets.consume_asset("video#cover"),
                                                                            req: 1,
                                                                            img: {
                                                                                Some(ImageAsset {
                                                                                    url: cover_img_url1.clone(),
                                                                                    mime: None,
                                                                                    w: adv.width,
                                                                                    h: adv.height,
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
                                                                },
                                                                None => ()
                                                            }
                                                            match &video.end_img_url {
                                                                Some(end_img_url) => {
                                                                    for end_img_url1 in end_img_url.iter() {
                                                                        asset_vec.push(Asset {
                                                                            id: assets.consume_asset("video#end#img"),
                                                                            req: 1,
                                                                            img: {
                                                                                Some(ImageAsset {
                                                                                    url: end_img_url1.clone(),
                                                                                    mime: None,
                                                                                    w: adv.width,
                                                                                    h: adv.height,
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
                                                                },
                                                                None => ()
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
                                                            match &video.end_html {
                                                                Some(end_html) => {
                                                                    asset_vec.push(Asset {
                                                                        id: assets.consume_asset("video#end#html"),
                                                                        req: 0,
                                                                        html: Some(HtmlAsset {
                                                                            html: Some(end_html.clone()),
                                                                            link: None,
                                                                            len: Some(end_html.len() as i32),
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
                                                        },
                                                        None => (),
                                                    }

                                                    match &adv.app_name {
                                                        Some(app_name) => {
                                                            asset_vec.push(Asset {
                                                                id: assets.consume_asset("app"),
                                                                req: 0,
                                                                app: Some(AppAsset {
                                                                    name: app_name.clone(),
                                                                    desc: None,
                                                                    descurl: None,
                                                                    domain: None,
                                                                    bundle: adv.package_name.clone(),
                                                                    ver: adv.app_version.clone(),
                                                                    developer: adv.app_dev_com_name.clone(),
                                                                    icon: None,
                                                                    storeid: None,
                                                                    storeurl: None,
                                                                    paid: 0,
                                                                    size: None,
                                                                    md5: None,
                                                                    registration: None,
                                                                    privacy: None,
                                                                    privacyurl: {
                                                                        match &adv.app_privacy_urls {
                                                                            Some(app_privacy_urls) => {
                                                                                if app_privacy_urls.len() > 0 {
                                                                                    Some(app_privacy_urls.get(0).unwrap().clone())
                                                                                } else {
                                                                                    None
                                                                                }
                                                                            },
                                                                            None => None,
                                                                        }
                                                                    },
                                                                    permission: None,
                                                                    permissionurl: {
                                                                        match &adv.app_permission_urls {
                                                                            Some(app_permission_urls) => {
                                                                                if app_permission_urls.len() > 0 {
                                                                                    Some(app_permission_urls.get(0).unwrap().clone())
                                                                                } else {
                                                                                    None
                                                                                }
                                                                            },
                                                                            None => None,
                                                                        }
                                                                    },
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

                                                match &adv.monitors {
                                                    Some(monitors) => {
                                                        for monitor in monitors {
                                                            for url in &monitor.urls {
                                                                event_vec.push(Event {
                                                                    eventtype: {
                                                                        match &monitor.monitortype {
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
                                                                            _ => 501,
                                                                        }
                                                                    },
                                                                    method: {
                                                                        match &monitor.method {
                                                                            Some(method) => {
                                                                                match method.as_str() {
                                                                                    "GET" => 1,
                                                                                    "POST" => 502,
                                                                                    _ => 1,
                                                                                }
                                                                            },
                                                                            None => 1,
                                                                        }
                                                                    },
                                                                    url: {
                                                                        if monitor.monitortype == 501 {
                                                                            let url = replace_macro(url);
                                                                            let price = match adv.price {
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
                                                                            url.replace("__PRICE__", &encode(encrypt_price.as_str()))
                                                                        } else {
                                                                            replace_macro(url)
                                                                        }
                                                                    },
                                                                    header: {
                                                                        match &monitor.method {
                                                                            Some(method) => {
                                                                                match method.as_str() {
                                                                                    "POST" => {
                                                                                        match &monitor.content_type {
                                                                                            Some(content_type) => Some([Header {
                                                                                                key: "Content-Type".to_string(),
                                                                                                value: Some(content_type.clone()),
                                                                                            }].to_vec()),
                                                                                            None => None,
                                                                                        }
                                                                                    },
                                                                                    _ => None,
                                                                                }
                                                                            },
                                                                            None => None,
                                                                        }
                                                                    },
                                                                    content: {
                                                                        match &monitor.method {
                                                                            Some(method) => {
                                                                                match method.as_str() {
                                                                                    "POST" => {
                                                                                        match &monitor.report_content {
                                                                                            Some(report_content) => Some(report_content.to_string()),
                                                                                            None => None,
                                                                                        }
                                                                                    },
                                                                                    _ => None,
                                                                                }
                                                                            },
                                                                            None => None,
                                                                        }

                                                                    },
                                                                });
                                                            }
                                                        }
                                                    },
                                                    None => (),
                                                }
                                                match &adv.monitor_url_visit {
                                                    Some(monitor_url_visit) => {
                                                        match &monitor_url_visit.landingpagetracklist {
                                                            Some(landingpagetracklist) => {
                                                                for event in landingpagetracklist {
                                                                    if event.url.is_some() {
                                                                        event_vec.push(Event {
                                                                            eventtype: 508,
                                                                            method: 1,
                                                                            url: replace_macro(&event.url.clone().unwrap()),
                                                                            header: {
                                                                                if event.headers.is_some() {
                                                                                    let mut header_vec = vec![];
                                                                                    match &event.headers {
                                                                                        Some(headers) => {
                                                                                            for header in headers {
                                                                                                if header.key.is_some() {
                                                                                                    header_vec.push(Header {
                                                                                                        key: header.key.clone().unwrap(),
                                                                                                        value: header.value.clone(),
                                                                                                    });
                                                                                                }
                                                                                            }
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    Some(header_vec)
                                                                                } else {
                                                                                    None
                                                                                }
                                                                            },
                                                                            content: None,
                                                                        });
                                                                    }
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                        match &monitor_url_visit.imptracklist {
                                                            Some(imptracklist) => {
                                                                for event in imptracklist {
                                                                    if event.url.is_some() {
                                                                        event_vec.push(Event {
                                                                            eventtype: 501,
                                                                            method: 1,
                                                                            url: replace_macro(&event.url.clone().unwrap()),
                                                                            header: {
                                                                                if event.headers.is_some() {
                                                                                    let mut header_vec = vec![];
                                                                                    match &event.headers {
                                                                                        Some(headers) => {
                                                                                            for header in headers {
                                                                                                if header.key.is_some() {
                                                                                                    header_vec.push(Header {
                                                                                                        key: header.key.clone().unwrap(),
                                                                                                        value: header.value.clone(),
                                                                                                    });
                                                                                                }
                                                                                            }
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    Some(header_vec)
                                                                                } else {
                                                                                    None
                                                                                }
                                                                            },
                                                                            content: None,
                                                                        });
                                                                    }
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                        match &monitor_url_visit.clicktracklist {
                                                            Some(clicktracklist) => {
                                                                for event in clicktracklist {
                                                                    if event.url.is_some() {
                                                                        event_vec.push(Event {
                                                                            eventtype: 502,
                                                                            method: 1,
                                                                            url: replace_macro(&event.url.clone().unwrap()),
                                                                            header: {
                                                                                if event.headers.is_some() {
                                                                                    let mut header_vec = vec![];
                                                                                    match &event.headers {
                                                                                        Some(headers) => {
                                                                                            for header in headers {
                                                                                                if header.key.is_some() {
                                                                                                    header_vec.push(Header {
                                                                                                        key: header.key.clone().unwrap(),
                                                                                                        value: header.value.clone(),
                                                                                                    });
                                                                                                }
                                                                                            }
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    Some(header_vec)
                                                                                } else {
                                                                                    None
                                                                                }
                                                                            },
                                                                            content: None,
                                                                        });
                                                                    }
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                        match &monitor_url_visit.closetracklist {
                                                            Some(closetracklist) => {
                                                                for event in closetracklist {
                                                                    if event.url.is_some() {
                                                                        event_vec.push(Event {
                                                                            eventtype: 509,
                                                                            method: 1,
                                                                            url: replace_macro(&event.url.clone().unwrap()),
                                                                            header: {
                                                                                if event.headers.is_some() {
                                                                                    let mut header_vec = vec![];
                                                                                    match &event.headers {
                                                                                        Some(headers) => {
                                                                                            for header in headers {
                                                                                                if header.key.is_some() {
                                                                                                    header_vec.push(Header {
                                                                                                        key: header.key.clone().unwrap(),
                                                                                                        value: header.value.clone(),
                                                                                                    });
                                                                                                }
                                                                                            }
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    Some(header_vec)
                                                                                } else {
                                                                                    None
                                                                                }
                                                                            },
                                                                            content: None,
                                                                        });
                                                                    }
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                        match &monitor_url_visit.startdowntracklist {
                                                            Some(startdowntracklist) => {
                                                                for event in startdowntracklist {
                                                                    if event.url.is_some() {
                                                                        event_vec.push(Event {
                                                                            eventtype: 601,
                                                                            method: 1,
                                                                            url: replace_macro(&event.url.clone().unwrap()),
                                                                            header: {
                                                                                if event.headers.is_some() {
                                                                                    let mut header_vec = vec![];
                                                                                    match &event.headers {
                                                                                        Some(headers) => {
                                                                                            for header in headers {
                                                                                                if header.key.is_some() {
                                                                                                    header_vec.push(Header {
                                                                                                        key: header.key.clone().unwrap(),
                                                                                                        value: header.value.clone(),
                                                                                                    });
                                                                                                }
                                                                                            }
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    Some(header_vec)
                                                                                } else {
                                                                                    None
                                                                                }
                                                                            },
                                                                            content: None,
                                                                        });
                                                                    }
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                        match &monitor_url_visit.finishdowntracklist {
                                                            Some(finishdowntracklist) => {
                                                                for event in finishdowntracklist {
                                                                    if event.url.is_some() {
                                                                        event_vec.push(Event {
                                                                            eventtype: 602,
                                                                            method: 1,
                                                                            url: replace_macro(&event.url.clone().unwrap()),
                                                                            header: {
                                                                                if event.headers.is_some() {
                                                                                    let mut header_vec = vec![];
                                                                                    match &event.headers {
                                                                                        Some(headers) => {
                                                                                            for header in headers {
                                                                                                if header.key.is_some() {
                                                                                                    header_vec.push(Header {
                                                                                                        key: header.key.clone().unwrap(),
                                                                                                        value: header.value.clone(),
                                                                                                    });
                                                                                                }
                                                                                            }
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    Some(header_vec)
                                                                                } else {
                                                                                    None
                                                                                }
                                                                            },
                                                                            content: None,
                                                                        });
                                                                    }
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                        match &monitor_url_visit.downpausetracklist {
                                                            Some(downpausetracklist) => {
                                                                for event in downpausetracklist {
                                                                    if event.url.is_some() {
                                                                        event_vec.push(Event {
                                                                            eventtype: 607,
                                                                            method: 1,
                                                                            url: replace_macro(&event.url.clone().unwrap()),
                                                                            header: {
                                                                                if event.headers.is_some() {
                                                                                    let mut header_vec = vec![];
                                                                                    match &event.headers {
                                                                                        Some(headers) => {
                                                                                            for header in headers {
                                                                                                if header.key.is_some() {
                                                                                                    header_vec.push(Header {
                                                                                                        key: header.key.clone().unwrap(),
                                                                                                        value: header.value.clone(),
                                                                                                    });
                                                                                                }
                                                                                            }
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    Some(header_vec)
                                                                                } else {
                                                                                    None
                                                                                }
                                                                            },
                                                                            content: None,
                                                                        });
                                                                    }
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                        match &monitor_url_visit.downgoontracklist {
                                                            Some(downgoontracklist) => {
                                                                for event in downgoontracklist {
                                                                    if event.url.is_some() {
                                                                        event_vec.push(Event {
                                                                            eventtype: 608,
                                                                            method: 1,
                                                                            url: replace_macro(&event.url.clone().unwrap()),
                                                                            header: {
                                                                                if event.headers.is_some() {
                                                                                    let mut header_vec = vec![];
                                                                                    match &event.headers {
                                                                                        Some(headers) => {
                                                                                            for header in headers {
                                                                                                if header.key.is_some() {
                                                                                                    header_vec.push(Header {
                                                                                                        key: header.key.clone().unwrap(),
                                                                                                        value: header.value.clone(),
                                                                                                    });
                                                                                                }
                                                                                            }
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    Some(header_vec)
                                                                                } else {
                                                                                    None
                                                                                }
                                                                            },
                                                                            content: None,
                                                                        });
                                                                    }
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                        match &monitor_url_visit.downdeletetracklist {
                                                            Some(downdeletetracklist) => {
                                                                for event in downdeletetracklist {
                                                                    if event.url.is_some() {
                                                                        event_vec.push(Event {
                                                                            eventtype: 609,
                                                                            method: 1,
                                                                            url: replace_macro(&event.url.clone().unwrap()),
                                                                            header: {
                                                                                if event.headers.is_some() {
                                                                                    let mut header_vec = vec![];
                                                                                    match &event.headers {
                                                                                        Some(headers) => {
                                                                                            for header in headers {
                                                                                                if header.key.is_some() {
                                                                                                    header_vec.push(Header {
                                                                                                        key: header.key.clone().unwrap(),
                                                                                                        value: header.value.clone(),
                                                                                                    });
                                                                                                }
                                                                                            }
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    Some(header_vec)
                                                                                } else {
                                                                                    None
                                                                                }
                                                                            },
                                                                            content: None,
                                                                        });
                                                                    }
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                        match &monitor_url_visit.startinstalltracklist {
                                                            Some(startinstalltracklist) => {
                                                                for event in startinstalltracklist {
                                                                    if event.url.is_some() {
                                                                        event_vec.push(Event {
                                                                            eventtype: 603,
                                                                            method: 1,
                                                                            url: replace_macro(&event.url.clone().unwrap()),
                                                                            header: {
                                                                                if event.headers.is_some() {
                                                                                    let mut header_vec = vec![];
                                                                                    match &event.headers {
                                                                                        Some(headers) => {
                                                                                            for header in headers {
                                                                                                if header.key.is_some() {
                                                                                                    header_vec.push(Header {
                                                                                                        key: header.key.clone().unwrap(),
                                                                                                        value: header.value.clone(),
                                                                                                    });
                                                                                                }
                                                                                            }
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    Some(header_vec)
                                                                                } else {
                                                                                    None
                                                                                }
                                                                            },
                                                                            content: None,
                                                                        });
                                                                    }
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                        match &monitor_url_visit.finishinstalltracklist {
                                                            Some(finishinstalltracklist) => {
                                                                for event in finishinstalltracklist {
                                                                    if event.url.is_some() {
                                                                        event_vec.push(Event {
                                                                            eventtype: 604,
                                                                            method: 1,
                                                                            url: replace_macro(&event.url.clone().unwrap()),
                                                                            header: {
                                                                                if event.headers.is_some() {
                                                                                    let mut header_vec = vec![];
                                                                                    match &event.headers {
                                                                                        Some(headers) => {
                                                                                            for header in headers {
                                                                                                if header.key.is_some() {
                                                                                                    header_vec.push(Header {
                                                                                                        key: header.key.clone().unwrap(),
                                                                                                        value: header.value.clone(),
                                                                                                    });
                                                                                                }
                                                                                            }
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    Some(header_vec)
                                                                                } else {
                                                                                    None
                                                                                }
                                                                            },
                                                                            content: None,
                                                                        });
                                                                    }
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                        match &monitor_url_visit.activedtracklist {
                                                            Some(activedtracklist) => {
                                                                for event in activedtracklist {
                                                                    if event.url.is_some() {
                                                                        event_vec.push(Event {
                                                                            eventtype: 605,
                                                                            method: 1,
                                                                            url: replace_macro(&event.url.clone().unwrap()),
                                                                            header: {
                                                                                if event.headers.is_some() {
                                                                                    let mut header_vec = vec![];
                                                                                    match &event.headers {
                                                                                        Some(headers) => {
                                                                                            for header in headers {
                                                                                                if header.key.is_some() {
                                                                                                    header_vec.push(Header {
                                                                                                        key: header.key.clone().unwrap(),
                                                                                                        value: header.value.clone(),
                                                                                                    });
                                                                                                }
                                                                                            }
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    Some(header_vec)
                                                                                } else {
                                                                                    None
                                                                                }
                                                                            },
                                                                            content: None,
                                                                        });
                                                                    }
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                        match &monitor_url_visit.deeplinktracklist {
                                                            Some(deeplinktracklist) => {
                                                                for event in deeplinktracklist {
                                                                    if event.url.is_some() {
                                                                        event_vec.push(Event {
                                                                            eventtype: 504,
                                                                            method: 1,
                                                                            url: replace_macro(&event.url.clone().unwrap()),
                                                                            header: {
                                                                                if event.headers.is_some() {
                                                                                    let mut header_vec = vec![];
                                                                                    match &event.headers {
                                                                                        Some(headers) => {
                                                                                            for header in headers {
                                                                                                if header.key.is_some() {
                                                                                                    header_vec.push(Header {
                                                                                                        key: header.key.clone().unwrap(),
                                                                                                        value: header.value.clone(),
                                                                                                    });
                                                                                                }
                                                                                            }
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    Some(header_vec)
                                                                                } else {
                                                                                    None
                                                                                }
                                                                            },
                                                                            content: None,
                                                                        });
                                                                    }
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                        match &monitor_url_visit.deeplinkfailedtracklist {
                                                            Some(deeplinkfailedtracklist) => {
                                                                for event in deeplinkfailedtracklist {
                                                                    if event.url.is_some() {
                                                                        event_vec.push(Event {
                                                                            eventtype: 505,
                                                                            method: 1,
                                                                            url: replace_macro(&event.url.clone().unwrap()),
                                                                            header: {
                                                                                if event.headers.is_some() {
                                                                                    let mut header_vec = vec![];
                                                                                    match &event.headers {
                                                                                        Some(headers) => {
                                                                                            for header in headers {
                                                                                                if header.key.is_some() {
                                                                                                    header_vec.push(Header {
                                                                                                        key: header.key.clone().unwrap(),
                                                                                                        value: header.value.clone(),
                                                                                                    });
                                                                                                }
                                                                                            }
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    Some(header_vec)
                                                                                } else {
                                                                                    None
                                                                                }
                                                                            },
                                                                            content: None,
                                                                        });
                                                                    }
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                        match &monitor_url_visit.videostarttracklist {
                                                            Some(videostarttracklist) => {
                                                                for event in videostarttracklist {
                                                                    if event.url.is_some() {
                                                                        event_vec.push(Event {
                                                                            eventtype: 701,
                                                                            method: 1,
                                                                            url: replace_macro(&event.url.clone().unwrap()),
                                                                            header: {
                                                                                if event.headers.is_some() {
                                                                                    let mut header_vec = vec![];
                                                                                    match &event.headers {
                                                                                        Some(headers) => {
                                                                                            for header in headers {
                                                                                                if header.key.is_some() {
                                                                                                    header_vec.push(Header {
                                                                                                        key: header.key.clone().unwrap(),
                                                                                                        value: header.value.clone(),
                                                                                                    });
                                                                                                }
                                                                                            }
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    Some(header_vec)
                                                                                } else {
                                                                                    None
                                                                                }
                                                                            },
                                                                            content: None,
                                                                        });
                                                                    }
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                        match &monitor_url_visit.firstquartiletracklist {
                                                            Some(firstquartiletracklist) => {
                                                                for event in firstquartiletracklist {
                                                                    if event.url.is_some() {
                                                                        event_vec.push(Event {
                                                                            eventtype: 702,
                                                                            method: 1,
                                                                            url: replace_macro(&event.url.clone().unwrap()),
                                                                            header: {
                                                                                if event.headers.is_some() {
                                                                                    let mut header_vec = vec![];
                                                                                    match &event.headers {
                                                                                        Some(headers) => {
                                                                                            for header in headers {
                                                                                                if header.key.is_some() {
                                                                                                    header_vec.push(Header {
                                                                                                        key: header.key.clone().unwrap(),
                                                                                                        value: header.value.clone(),
                                                                                                    });
                                                                                                }
                                                                                            }
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    Some(header_vec)
                                                                                } else {
                                                                                    None
                                                                                }
                                                                            },
                                                                            content: None,
                                                                        });
                                                                    }
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                        match &monitor_url_visit.midpointtracklist {
                                                            Some(midpointtracklist) => {
                                                                for event in midpointtracklist {
                                                                    if event.url.is_some() {
                                                                        event_vec.push(Event {
                                                                            eventtype: 703,
                                                                            method: 1,
                                                                            url: replace_macro(&event.url.clone().unwrap()),
                                                                            header: {
                                                                                if event.headers.is_some() {
                                                                                    let mut header_vec = vec![];
                                                                                    match &event.headers {
                                                                                        Some(headers) => {
                                                                                            for header in headers {
                                                                                                if header.key.is_some() {
                                                                                                    header_vec.push(Header {
                                                                                                        key: header.key.clone().unwrap(),
                                                                                                        value: header.value.clone(),
                                                                                                    });
                                                                                                }
                                                                                            }
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    Some(header_vec)
                                                                                } else {
                                                                                    None
                                                                                }
                                                                            },
                                                                            content: None,
                                                                        });
                                                                    }
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                        match &monitor_url_visit.thirdquartiletracklist {
                                                            Some(thirdquartiletracklist) => {
                                                                for event in thirdquartiletracklist {
                                                                    if event.url.is_some() {
                                                                        event_vec.push(Event {
                                                                            eventtype: 704,
                                                                            method: 1,
                                                                            url: replace_macro(&event.url.clone().unwrap()),
                                                                            header: {
                                                                                if event.headers.is_some() {
                                                                                    let mut header_vec = vec![];
                                                                                    match &event.headers {
                                                                                        Some(headers) => {
                                                                                            for header in headers {
                                                                                                if header.key.is_some() {
                                                                                                    header_vec.push(Header {
                                                                                                        key: header.key.clone().unwrap(),
                                                                                                        value: header.value.clone(),
                                                                                                    });
                                                                                                }
                                                                                            }
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    Some(header_vec)
                                                                                } else {
                                                                                    None
                                                                                }
                                                                            },
                                                                            content: None,
                                                                        });
                                                                    }
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                        match &monitor_url_visit.videoendtracklist {
                                                            Some(videoendtracklist) => {
                                                                for event in videoendtracklist {
                                                                    if event.url.is_some() {
                                                                        event_vec.push(Event {
                                                                            eventtype: 705,
                                                                            method: 1,
                                                                            url: replace_macro(&event.url.clone().unwrap()),
                                                                            header: {
                                                                                if event.headers.is_some() {
                                                                                    let mut header_vec = vec![];
                                                                                    match &event.headers {
                                                                                        Some(headers) => {
                                                                                            for header in headers {
                                                                                                if header.key.is_some() {
                                                                                                    header_vec.push(Header {
                                                                                                        key: header.key.clone().unwrap(),
                                                                                                        value: header.value.clone(),
                                                                                                    });
                                                                                                }
                                                                                            }
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    Some(header_vec)
                                                                                } else {
                                                                                    None
                                                                                }
                                                                            },
                                                                            content: None,
                                                                        });
                                                                    }
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                        match &monitor_url_visit.videomutetracklist {
                                                            Some(videomutetracklist) => {
                                                                for event in videomutetracklist {
                                                                    if event.url.is_some() {
                                                                        event_vec.push(Event {
                                                                            eventtype: 713,
                                                                            method: 1,
                                                                            url: replace_macro(&event.url.clone().unwrap()),
                                                                            header: {
                                                                                if event.headers.is_some() {
                                                                                    let mut header_vec = vec![];
                                                                                    match &event.headers {
                                                                                        Some(headers) => {
                                                                                            for header in headers {
                                                                                                if header.key.is_some() {
                                                                                                    header_vec.push(Header {
                                                                                                        key: header.key.clone().unwrap(),
                                                                                                        value: header.value.clone(),
                                                                                                    });
                                                                                                }
                                                                                            }
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    Some(header_vec)
                                                                                } else {
                                                                                    None
                                                                                }
                                                                            },
                                                                            content: None,
                                                                        });
                                                                    }
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                        match &monitor_url_visit.videoskiptracklist {
                                                            Some(videoskiptracklist) => {
                                                                for event in videoskiptracklist {
                                                                    if event.url.is_some() {
                                                                        event_vec.push(Event {
                                                                            eventtype: 710,
                                                                            method: 1,
                                                                            url: replace_macro(&event.url.clone().unwrap()),
                                                                            header: {
                                                                                if event.headers.is_some() {
                                                                                    let mut header_vec = vec![];
                                                                                    match &event.headers {
                                                                                        Some(headers) => {
                                                                                            for header in headers {
                                                                                                if header.key.is_some() {
                                                                                                    header_vec.push(Header {
                                                                                                        key: header.key.clone().unwrap(),
                                                                                                        value: header.value.clone(),
                                                                                                    });
                                                                                                }
                                                                                            }
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    Some(header_vec)
                                                                                } else {
                                                                                    None
                                                                                }
                                                                            },
                                                                            content: None,
                                                                        });
                                                                    }
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                        match &monitor_url_visit.videoclosetracklist {
                                                            Some(videoclosetracklist) => {
                                                                for event in videoclosetracklist {
                                                                    if event.url.is_some() {
                                                                        event_vec.push(Event {
                                                                            eventtype: 711,
                                                                            method: 1,
                                                                            url: replace_macro(&event.url.clone().unwrap()),
                                                                            header: {
                                                                                if event.headers.is_some() {
                                                                                    let mut header_vec = vec![];
                                                                                    match &event.headers {
                                                                                        Some(headers) => {
                                                                                            for header in headers {
                                                                                                if header.key.is_some() {
                                                                                                    header_vec.push(Header {
                                                                                                        key: header.key.clone().unwrap(),
                                                                                                        value: header.value.clone(),
                                                                                                    });
                                                                                                }
                                                                                            }
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    Some(header_vec)
                                                                                } else {
                                                                                    None
                                                                                }
                                                                            },
                                                                            content: None,
                                                                        });
                                                                    }
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
                                        advertiser: None,
                                        advertisericon: None,
                                    },
                                };
                                bids.push(bid);
                            },
                            None => (),
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
            let pool_richmob_lock = pool.pool_richmob.clone();
            let pool_richmob = pool_richmob_lock.read().unwrap();
            pool_richmob.clone()
        };
        let _ = client.get(replaced_url).send().await;
    }

    async fn bidding_notify_lose(url: String, lose_price: i32, _lose_reason: i32, _lose_adn_name: &String, iv: &String, connection: &Connection, pool: &HttpPool) {
        let encrypt_price = Self::encrypt_price(lose_price, iv, connection);
        let replaced_url = url
            .replace("__LOSE_PRICE__", &encode(encrypt_price.as_str()));

        let client = {
            let pool_richmob_lock = pool.pool_richmob.clone();
            let pool_richmob = pool_richmob_lock.read().unwrap();
            pool_richmob.clone()
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

        let key = connection.client_tag_id.split("|").nth(1).unwrap().as_bytes();

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

    replaced = replaced.replace("__TIME_START__", "__EVENT_TIME_START__");
    replaced = replaced.replace("__TIME_END__", "__EVENT_TIME_END__");
    replaced = replaced.replace("__TIME_START_SE__", "__EVENT_TIME_START_S__");
    replaced = replaced.replace("__TIME_END_SE__", "__EVENT_TIME_END_S__");

    replaced = replaced.replace("__LONGITUDE__", "__LON__");
    replaced = replaced.replace("__LATITUDE__", "__LAT__");
    replaced = replaced.replace("__USERAGENT__", "__UA__");

    replaced = replaced.replace("__DISPLAY_LUX__", "__LT_X__");
    replaced = replaced.replace("__DISPLAY_LUY__", "__LT_Y__");
    replaced = replaced.replace("__DISPLAY_RDX__", "__RB_X__");
    replaced = replaced.replace("__DISPLAY_RDY__", "__RB_Y__");
    replaced = replaced.replace("__BUTTON_LUX__", "__BUTTON_LT_X__");
    replaced = replaced.replace("__BUTTON_LUY__", "__BUTTON_LT_Y__");
    replaced = replaced.replace("__BUTTON_RDX__", "__BUTTON_RB_X__");
    replaced = replaced.replace("__BUTTON_RDY__", "__BUTTON_RB_Y__");

    replaced = replaced.replace("__DPLINK__", "__DP_STATUS__");

    replaced = replaced.replace("__VIDEO_BEHAVIOR__", "__VIDEO_PLAY_TRIGGER_0__");
    replaced = replaced.replace("__VIDEO_PLAY_CUR__", "__VIDEO_PLAY_PROGRESS_S__");
    replaced = replaced.replace("__VIDEO_START_TIME__", "__VIDEO_BEGIN_TIME__");
    replaced = replaced.replace("__VIDEO_PLAY_FIRSR_FRAME__", "__VIDEO_FIRST_FRAME__");
    replaced = replaced.replace("__VIDEO_PLAY_LAST_FRAME__", "__VIDEO_LAST_FRAME__");
    replaced = replaced.replace("__VIDEO_SCENE__", "__VIDEO_PLAY_SCENE__");
    replaced = replaced.replace("__VIDEO_STATUS__", "__VIDEO_PLAY_STATUS__");

    replaced
}
