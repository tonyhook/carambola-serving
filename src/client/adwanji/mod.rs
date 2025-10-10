use std::time::Duration;

use chrono::{Datelike, Local, TimeZone};
use chrono_tz::Tz;

use crate::{protocol::*, Assets, Cache, Client, Connection, HttpPool, Identifiers, Price, ResultMessage};

pub mod app_asset;
pub mod app;
pub mod banner_asset;
pub mod banner_format;
pub mod bid;
pub mod device;
pub mod events;
pub mod feed_asset;
pub mod feed_format;
pub mod geo;
pub mod imp;
pub mod request;
pub mod response;
pub mod user;
pub mod video_asset;
pub mod video_format;

pub use app_asset::AdwanjiAppAsset;
pub use app::AdwanjiApp;
pub use banner_asset::AdwanjiBannerAsset;
pub use banner_format::AdwanjiBannerFormat;
pub use bid::AdwanjiBid;
pub use device::AdwanjiDevice;
pub use events::AdwanjiEvents;
pub use feed_asset::AdwanjiFeedAsset;
pub use feed_format::AdwanjiFeedFormat;
pub use geo::AdwanjiGeo;
pub use imp::AdwanjiImp;
pub use request::AdwanjiRequest;
pub use response::AdwanjiResponse;
pub use user::AdwanjiUser;
pub use video_asset::AdwanjiVideoAsset;
pub use video_format::AdwanjiVideoFormat;

pub struct Adwanji {

}

impl Client for Adwanji {

    async fn request(request: &Request, connection: &Connection, pool: &HttpPool, cache: &Cache) -> Result<Response, ResultMessage> {
        let request_id = cache.get_sequence();

        let mut assets = Assets::new(request);
        let identifiers = Identifiers::new(request);

        let request_adwanji = AdwanjiRequest {
            id: {
                request_id.to_string()
            },
            imp: AdwanjiImp {
                slotid: {
                    connection.client_tag_id.clone()
                },
                banner: {
                    match assets.get_banner() {
                        Some(displayfmt) =>
                                Some(AdwanjiBannerFormat {
                                    w: {
                                        match displayfmt.w {
                                            Some(w) => w,
                                            None => return Err(ResultMessage {
                                                code: 998,
                                                message: "request.item[0].spec.display.displayfmt.w is required for upstream".to_string(),
                                            }),
                                        }
                                    },
                                    h: {
                                        match displayfmt.h {
                                            Some(h) => h,
                                            None => return Err(ResultMessage {
                                                code: 998,
                                                message: "request.item[0].spec.display.displayfmt.h is required for upstream".to_string(),
                                            }),
                                        }
                                    },
                                    pos: {
                                        match request.item[0].spec.display.pos {
                                            Some(pos) => {
                                                match pos {
                                                    0 => return Err(ResultMessage {
                                                        code: 998,
                                                        message: "request.item[0].spec.display.pos should be 4/5/7/501 for upstream".to_string(),
                                                    }),
                                                    1 => return Err(ResultMessage {
                                                        code: 998,
                                                        message: "request.item[0].spec.display.pos should be 4/5/7/501 for upstream".to_string(),
                                                    }),
                                                    2 => return Err(ResultMessage {
                                                        code: 998,
                                                        message: "request.item[0].spec.display.pos should be 4/5/7/501 for upstream".to_string(),
                                                    }),
                                                    3 => return Err(ResultMessage {
                                                        code: 998,
                                                        message: "request.item[0].spec.display.pos should be 4/5/7/501 for upstream".to_string(),
                                                    }),
                                                    4 => 1,
                                                    5 => 2,
                                                    6 => return Err(ResultMessage {
                                                        code: 998,
                                                        message: "request.item[0].spec.display.pos should be 4/5/7/501 for upstream".to_string(),
                                                    }),
                                                    7 => {
                                                        match request.item[0].spec.display.instl {
                                                            0 => 4,
                                                            1 => 5,
                                                            _ => return Err(ResultMessage {
                                                                code: 998,
                                                                message: "request.item[0].spec.display.instl should be 0/1 for upstream".to_string(),
                                                            }),
                                                        }
                                                    },
                                                    501 => 4,
                                                    _ => return Err(ResultMessage {
                                                        code: 998,
                                                        message: "request.item[0].spec.display.pos should be 4/5/7 for upstream".to_string(),
                                                    }),
                                                }
                                            },
                                            None => return Err(ResultMessage {
                                                code: 998,
                                                message: "request.item[0].spec.display.pos is required for upstream".to_string(),
                                            }),
                                        }
                                    },
                                }),
                        None => None,
                    }
                },
                feed: {
                    if (assets.get_asset_size("img") == 1 || assets.get_asset_size("img") == 3) && assets.get_asset_size("img") == assets.get_asset_total_size() {
                        let img = assets.get_current_asset("img").unwrap().img.clone().unwrap();
                        Some(AdwanjiFeedFormat {
                            w: match img.w {
                                Some(w) => Some(w),
                                None => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.item[0].spec.display.nativefmt.asset.img.w is required for upstream".to_string(),
                                }),
                            },
                            h: match img.h {
                                Some(h) => Some(h),
                                None => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.item[0].spec.display.nativefmt.asset.img.h is required for upstream".to_string(),
                                }),
                            },
                            feedtype: if assets.get_asset_size("img") == 1 {
                                [1].to_vec()
                            } else if assets.get_asset_size("img") == 3 {
                                [2].to_vec()
                            } else {
                                return Err(ResultMessage {
                                    code: 998,
                                    message: "number of request.item[0].spec.display.nativefmt.asset.img should be 1 or 3 for upstream".to_string(),
                                })
                            },
                        })
                    } else if assets.get_asset_size("img") == 0 {
                        None
                    } else {
                        return Err(ResultMessage {
                            code: 998,
                            message: "number of request.item[0].spec.display.nativefmt.asset.img should be 0, 1 or 3 for upstream".to_string(),
                        });
                    }
                },
                video: {
                    if assets.get_asset_size("video") == 1 && assets.get_asset_size("video") == assets.get_asset_total_size() {
                        let video = assets.get_current_asset("video").unwrap().video.clone().unwrap();
                        Some(AdwanjiVideoFormat {
                            userid: {
                                match &request.context.user.id {
                                    Some(id) => Some(id.clone()),
                                    None => None,
                                }
                            },
                            w: video.w,
                            h: video.h,
                            minduration: video.mindur,
                            maxduration: video.maxdur,
                        })
                    } else {
                        if assets.get_asset_size("video") > 1 && assets.get_asset_size("video") == assets.get_asset_total_size() {
                            return Err(ResultMessage {
                                code: 998,
                                message: "number of request.item[0].spec.display.nativefmt.asset.video should be 1 for upstream".to_string(),
                            });
                        } else {
                            None
                        }
                    }
                },
                support_deeplink: {
                    1
                },
                support_universal: {
                    1
                },
                bid_price: {
                    Some(Price::to_client(connection, request.item[0].flr.map(f64::from)) as i32)
                },
            },
            app: {
                match &request.context.app {
                    Some(app) => AdwanjiApp {
                        name: {
                            match &connection.client_media_appname {
                                Some(client_media_appname) => client_media_appname.clone(),
                                None => app.name.clone(),
                            }
                        },
                        bundle: {
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
                        ver: {
                            match &app.ver {
                                Some(ver) => ver.clone(),
                                None => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.context.app.ver is required for upstream".to_string(),
                                }),
                            }
                        },
                        paid: {
                            app.paid
                        },
                        keywords: {
                            None
                        },
                        storeurl: {
                            match &app.storeurl {
                                Some(storeurl) => Some(storeurl.clone()),
                                None => None,
                            }
                        },
                        itunesid: {
                            match &app.storeid {
                                Some(storeid) => Some(storeid.clone()),
                                None => None,
                            }
                        },
                    },
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.app is required for upstream".to_string(),
                    }),
                }
            },
            device: AdwanjiDevice {
                ua: {
                    request.context.device.ua.clone()
                },
                geo: {
                    match &request.context.device.geo {
                        Some(geo) => AdwanjiGeo {
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
                            coordinate: {
                                match geo.coordinate {
                                    Some(1) => Some(1),
                                    Some(2) => Some(2),
                                    Some(3) => Some(3),
                                    _ => None,
                                }
                            },
                            timestamp: None,
                            accu: geo.accur,
                            city_code: None,
                            city: None,
                        },
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.geo is required for upstream".to_string(),
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
                ipv6: {
                    match &request.context.device.ipv6 {
                        Some(ipv6) => Some(ipv6.clone()),
                        None => None,
                    }
                },
                devicetype: {
                    match request.context.device.devicetype {
                        Some(devicetype) => {
                            match devicetype {
                                1 => 4,
                                2 => 2,
                                3 => 3,
                                4 => 4,
                                5 => 5,
                                6 => 0,
                                7 => 7,
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
                make: {
                    match &request.context.device.make {
                        Some(make) => make.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.make is required for upstream".to_string(),
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
                os: {
                    match request.context.device.os {
                        Some(os) => {
                            match os {
                                2 => 0,
                                13 => 1,
                                _ => 9,
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
                oslevel: {
                    match request.context.device.oslevel {
                        Some(oslevel) => oslevel,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.oslevel is required for upstream".to_string(),
                        }),
                    }
                },
                resolution: {
                    let width;
                    let height;
                    match request.context.device.w {
                        Some(w) => width = w,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.w is required for upstream".to_string(),
                        }),
                    }
                    match request.context.device.h {
                        Some(h) => height = h,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.h is required for upstream".to_string(),
                        }),
                    }
                    format!("{}{}{}", width.to_string(), "*".to_string(), height.to_string())
                },
                sh: {
                    match request.context.device.h {
                        Some(h) => h,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.h is required for upstream".to_string(),
                        }),
                    }
                },
                sw: {
                    match request.context.device.w {
                        Some(w) => w,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.w is required for upstream".to_string(),
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
                dpi: {
                    request.context.device.ppi.clone()
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
                orientation: {
                    match request.context.device.orientation {
                        Some(orientation) => {
                            match orientation {
                                501 => 0,
                                502 => 1,
                                _ => 9,
                            }
                        },
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.orientation is required for upstream".to_string(),
                        }),
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
                idfv: {
                    match identifiers.get_id(515, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                udid: {
                    None
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
                dpid: {
                    match identifiers.get_id(514, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                aid: {
                    match identifiers.get_id(509, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                aidmd5: {
                    match identifiers.get_id(510, 0) {
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
                oaidmd5: {
                    match identifiers.get_id(506, 0) {
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
                caidver: {
                    match identifiers.get_id(513, 0) {
                        Some(uid) => uid.ver.clone(),
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
                meid: {
                    match identifiers.get_id(520, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                carrier: {
                    match &request.context.device.carrier {
                        Some(carrier) => carrier.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.carrier is required for upstream".to_string(),
                        }),
                    }
                },
                conn: {
                    match &request.context.device.contype {
                        Some(contype) => {
                            match contype {
                                1 => 9,
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
                imsi: {
                    match identifiers.get_id(503, 0) {
                        Some(uid) => uid.id.clone(),
                        None => "".to_string(),
                    }
                },
                pkgs: {
                    match &request.context.device.app {
                        Some(app) => Some(app.clone()),
                        None => None,
                    }
                },
                appstorever: {
                    match &request.context.device.storev {
                        Some(storev) => Some(storev.clone()),
                        None => None,
                    }
                },
                appstorevername: {
                    match &request.context.device.storename {
                        Some(storename) => Some(storename.clone()),
                        None => None,
                    }
                },
                wifissid: {
                    match identifiers.get_id(524, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                wifimac: {
                    match identifiers.get_id(522, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                serialno: {
                    request.context.device.serial.clone()
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
                countrycode: {
                    match &request.context.device.country {
                        Some(country) => country.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.country is required for upstream".to_string(),
                        }),
                    }
                },
                uiver: {
                    request.context.device.uiv.clone()
                },
                romver: {
                    match &request.context.device.romv {
                        Some(romv) => Some(romv.clone()),
                        None => None,
                    }
                },
                hmsver: {
                    match &request.context.device.hmsv {
                        Some(hmsv) => Some(hmsv.clone()),
                        None => None,
                    }
                },
                hwagver: {
                    match &request.context.device.storev {
                        Some(storev) => Some(storev.clone()),
                        None => None,
                    }
                },
                compilingtime: {
                    match &request.context.device.romtime {
                        Some(romtime) => Some(romtime.clone()),
                        None => None,
                    }
                },
                starttime: {
                    match &request.context.device.boottime {
                        Some(boottime) => boottime.split(".").nth(0).unwrap().to_string(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.boottime is required for upstream".to_string(),
                        }),
                    }
                },
                startnanotime: {
                    match &request.context.device.boottime {
                        Some(boottime) => boottime.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.boottime is required for upstream".to_string(),
                        }),
                    }
                },
                startmilltime: {
                    match &request.context.device.boottime {
                        Some(boottime) => {
                            let mut slices = boottime.split(".");
                            if slices.clone().count().eq(&2) {
                                format!("{}.{}", &slices.nth(0).unwrap(), &slices.nth(1).unwrap()[..3])
                            } else {
                                format!("{}.{}", &slices.nth(0).unwrap(), "000")
                            }
                        }
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.boottime is required for upstream".to_string(),
                        }),
                    }
                },
                birthtime: {
                    match &request.context.device.inittime {
                        Some(inittime) => inittime.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.inittime is required for upstream".to_string(),
                        }),
                    }
                },
                osupdatetime: {
                    match &request.context.device.updatetime {
                        Some(updatetime) => updatetime.split(".").nth(0).unwrap().to_string(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.updatetime is required for upstream".to_string(),
                        }),
                    }
                },
                osupdatenanotime: {
                    match &request.context.device.updatetime {
                        Some(boottime) => boottime.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.updatetime is required for upstream".to_string(),
                        }),
                    }
                },
                hwname: {
                    match &request.context.device.hwname {
                        Some(hwname) => hwname.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.hwname is required for upstream".to_string(),
                        }),
                    }
                },
                hwmodel: {
                    match &request.context.device.hwmodel {
                        Some(hwmodel) => hwmodel.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.hwmodel is required for upstream".to_string(),
                        }),
                    }
                },
                hwmachine: {
                    match &request.context.device.hwmachine {
                        Some(hwmachine) => hwmachine.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.hwmachine is required for upstream".to_string(),
                        }),
                    }
                },
                sysmemory: {
                    match request.context.device.sysmemory {
                        Some(sysmemory) => sysmemory.to_string(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.sysmemory is required for upstream".to_string(),
                        }),
                    }
                },
                sysdisksize: {
                    match &request.context.device.sysdisksize {
                        Some(sysdisksize) => sysdisksize.to_string(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.sysdisksize is required for upstream".to_string(),
                        }),
                    }
                },
                cpunum: {
                    match &request.context.device.syscpu {
                        Some(syscpu) => Some(syscpu.to_string()),
                        None => None,
                    }
                },
                cpufreq: {
                    match &request.context.device.syscpufreq {
                        Some(syscpufreq) => Some(syscpufreq.to_string()),
                        None => None,
                    }
                },
                timezone: {
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
                updatemark: {
                    match &request.context.device.updatemark {
                        Some(updatemark) => updatemark.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.updatemark is required for upstream".to_string(),
                        }),
                    }
                },
                bootmark: {
                    match &request.context.device.bootmark {
                        Some(bootmark) => bootmark.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.bootmark is required for upstream".to_string(),
                        }),
                    }
                },
                battery_status: {
                    match request.context.device.sysbatterystatus {
                        Some(sysbatterystatus) => Some(sysbatterystatus),
                        None => None,
                    }
                },
                battery_power: {
                    match request.context.device.sysbatterypower {
                        Some(sysbatterypower) => Some(sysbatterypower),
                        None => None,
                    }
                },
                idfa_policy: {
                    match request.context.device.lmt {
                        Some(lmt) => Some(lmt),
                        None => None,
                    }
                },
                pre_caid: {
                    match identifiers.get_id(513, 1) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                pre_caid_version: {
                    match identifiers.get_id(513, 1) {
                        Some(uid) => uid.ver.clone(),
                        None => None,
                    }
                },
                screen_size: {
                    match request.context.device.size {
                        Some(size) => Some(size.to_string()),
                        None => None,
                    }
                },
                reffer: {
                    None
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
                paid: {
                    match identifiers.get_id(519, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                lastcaid: {
                    None
                },
                lastcaidver: {
                    None
                },
            },
            user: AdwanjiUser {
                id: {
                    match &request.context.user.id {
                        Some(id) => id.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.user.id is required for upstream".to_string(),
                        }),
                    }
                },
                gender: {
                    match &request.context.user.gender {
                        Some(gender) => {
                            match gender.as_str() {
                                "M" => Some(1),
                                "F" => Some(2),
                                "O" => Some(0),
                                _ => None,
                            }
                        }
                        None => None,
                    }
                },
                age: {
                    match request.context.user.yob {
                        Some(yob) => {
                            let year = Local::now().year();
                            Some(year - yob)
                        },
                        None => None,
                    }
                },
                keywords: {
                    match &request.context.user.keywords {
                        Some(keywords) => keywords.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.user.keywords is required for upstream".to_string(),
                        }),
                    }
                },
            },
        };

        let response_adwanji: AdwanjiResponse;

        let client = {
            let pool_adwanji_lock = pool.pool_adwanji.clone();
            let pool_adwanji = pool_adwanji_lock.read().unwrap();
            pool_adwanji.clone()
        };
        let response_adwanji_raw = client.post("https://api.adwanji.com/ad/v5/")
            .json(&request_adwanji)
            .header("Accept-Encoding", "gzip")
            .timeout(Duration::from_millis(connection.timeout))
            .send().await;

        match response_adwanji_raw {
            Ok(response_adwanji_raw) => {
                let status = response_adwanji_raw.status();
                if status != 200 {
                    return Err(ResultMessage {
                        code: 992,
                        message: {
                            match response_adwanji_raw.text().await {
                                Ok(text) => format!("upstream error {}: {}", status, text),
                                Err(_) => format!("upstream error {}", status),
                            }
                        },
                    });
                } else {
                    match response_adwanji_raw.text().await {
                        Ok(text) => {
                            match serde_json::from_str::<AdwanjiResponse>(&text) {
                                Ok(json) => {
                                    response_adwanji = json;

                                    match response_adwanji.code {
                                        101 => {
                                            return Err(ResultMessage {
                                                code: 993,
                                                message: "".to_string(),
                                            });
                                        },
                                        103 => {
                                            return Err(ResultMessage {
                                                code: 992,
                                                message: "upstream error: parameter error".to_string(),
                                            });
                                        },
                                        104 => {
                                            return Err(ResultMessage {
                                                code: 992,
                                                message: "upstream error: unknown error".to_string(),
                                            });
                                        },
                                        201 => {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: "upstream error: beyond qps".to_string(),
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
                match &response_adwanji.bid {
                    Some(bid) => {
                        Some([Seatbid {
                            bid: {
                                let mut bids = vec![];

                                let link_asset = LinkAsset {
                                    linktype: {
                                        match bid.download_type {
                                            Some(download_type) => {
                                                match download_type {
                                                    2 => 2,
                                                    3 => 1,
                                                    4 => 3,
                                                    _ => 1,
                                                }
                                            },
                                            None => 1,
                                        }
                                    },
                                    universallink: {
                                        match &bid.universal_url {
                                            Some(universal_url) => Some(universal_url.clone()),
                                            None => None,
                                        }
                                    },
                                    storeid: {
                                        match &bid.ios_app_id {
                                            Some(ios_app_id) => Some(ios_app_id.clone()),
                                            None => None,
                                        }
                                    },
                                    deeplink: {
                                        match &bid.deeplink {
                                            Some(deeplink) => Some(deeplink.clone()),
                                            None => None,
                                        }
                                    },
                                    quickapplink: None,
                                    wechatmppath: None,
                                    wechatmpid: None,
                                    marketurl: {
                                        match &bid.market_url {
                                            Some(market_url) => Some(market_url.clone()),
                                            None => None,
                                        }
                                    },
                                    downloadurl: {
                                        match &bid.download_url {
                                            Some(download_url) => Some(download_url.clone()),
                                            None => None,
                                        }
                                    },
                                    url: {
                                        match &bid.landing_url {
                                            Some(landing_url) => replace_macro(landing_url),
                                            None => "".to_string(),
                                        }
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
                                    burl: None,
                                    lurl: None,
                                    media: Ad {
                                        id: response_adwanji.id.clone(),
                                        display: Display {
                                            w: {
                                                if assets.get_banner_size() > 0 {
                                                    match &bid.banner {
                                                        Some(banner) => {
                                                            banner.w
                                                        },
                                                        None => None,
                                                    }
                                                } else {
                                                    if assets.get_asset_total_size() > 0 {
                                                        match &bid.video {
                                                            Some(video) => {
                                                                video.w
                                                            },
                                                            None => None,
                                                        }
                                                    } else {
                                                        None
                                                    }
                                                }
                                            },
                                            h: {
                                                if assets.get_banner_size() > 0 {
                                                    match &bid.banner {
                                                        Some(banner) => {
                                                            banner.h
                                                        },
                                                        None => None,
                                                    }
                                                } else {
                                                    if assets.get_asset_total_size() > 0 {
                                                        match &bid.video {
                                                            Some(video) => {
                                                                video.h
                                                            },
                                                            None => None,
                                                        }
                                                    } else {
                                                        None
                                                    }
                                                }
                                            },
                                            banner: {
                                                if assets.get_banner_size() > 0 {
                                                    match &bid.banner {
                                                        Some(banner) => {
                                                            Some(Banner {
                                                                img: banner.iurl.clone(),
                                                                link: Some(link_asset.clone()),
                                                            })
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

                                                    match &bid.video {
                                                        Some(video) => {
                                                            if assets.get_asset_size("video") > 0 {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("video"),
                                                                    req: 1,
                                                                    video: Some(VideoAsset {
                                                                        url: video.iurl.clone(),
                                                                        mime: None,
                                                                        w: video.w,
                                                                        h: video.h,
                                                                        dur: video.duration,
                                                                        skipoffset: video.keep_duration,
                                                                        size: video.size,
                                                                        delivery: None,
                                                                        orientation: None,
                                                                        autolanding: video.is_auto_langding,
                                                                        clickable: video.clickable,
                                                                    }),
                                                                    title: None,
                                                                    img: None,
                                                                    data: None,
                                                                    html: None,
                                                                    app: None,
                                                                });
                                                            }
                                                            if assets.get_asset_size("video#cover") > 0 && video.cover_url.is_some() {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("video#cover"),
                                                                    req: 0,
                                                                    img: Some(ImageAsset {
                                                                        url: video.cover_url.clone().unwrap(),
                                                                        mime: None,
                                                                        w: video.cover_w,
                                                                        h: video.cover_h,
                                                                        imagetype: Some(3),
                                                                    }),
                                                                    title: None,
                                                                    video: None,
                                                                    data: None,
                                                                    html: None,
                                                                    app: None,
                                                                });
                                                            }
                                                            if assets.get_asset_size("video#icon") > 0 && video.ad_icon.is_some() {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("video#icon"),
                                                                    req: 0,
                                                                    img: Some(ImageAsset {
                                                                        url: video.ad_icon.clone().unwrap(),
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
                                                            }
                                                            if assets.get_asset_size("video#end#img") > 0 && video.end_url.is_some() {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("video#end#img"),
                                                                    req: 0,
                                                                    img: Some(ImageAsset {
                                                                        url: video.end_url.clone().unwrap(),
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
                                                            if assets.get_asset_size("video#end#title") > 0 && video.ad_text.is_some() {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("video#end#title"),
                                                                    req: 0,
                                                                    title: Some(TitleAsset {
                                                                        text: video.ad_text.clone().unwrap(),
                                                                        subtitle: None,
                                                                        desc: video.ad_description.clone(),
                                                                        len: Some(video.ad_text.clone().unwrap().clone().len() as i32),
                                                                    }),
                                                                    img: None,
                                                                    video: None,
                                                                    data: None,
                                                                    html: None,
                                                                    app: None,
                                                                });
                                                            }
                                                            if assets.get_asset_size("video#end#button#text") > 0 && video.button_text.is_some() {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("video#end#button#text"),
                                                                    req: 0,
                                                                    data: Some(DataAsset {
                                                                        value: video.button_text.clone().unwrap(),
                                                                        len: Some(video.button_text.clone().unwrap().len() as i32),
                                                                        datatype: Some(12),
                                                                    }),
                                                                    title: None,
                                                                    img: None,
                                                                    video: None,
                                                                    html: None,
                                                                    app: None,
                                                                });
                                                            }
                                                            if assets.get_asset_size("video#end#html") > 0 && video.end_html.is_some() {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("video#end#html"),
                                                                    req: 0,
                                                                    html: Some(HtmlAsset {
                                                                        html: video.end_html.clone(),
                                                                        link: None,
                                                                        len: Some(video.end_html.clone().unwrap().len() as i32),
                                                                    }),
                                                                    title: None,
                                                                    img: None,
                                                                    video: None,
                                                                    data: None,
                                                                    app: None,
                                                                });
                                                            }
                                                        },
                                                        None => (),
                                                    }
                                                    match &bid.feed {
                                                        Some(feed) => {
                                                            if assets.get_asset_size("title") > 0 {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("title"),
                                                                    req: 1,
                                                                    title: Some(TitleAsset {
                                                                        text: feed.title.clone(),
                                                                        subtitle: None,
                                                                        desc: Some(feed.desc.clone()),
                                                                        len: Some(feed.title.clone().len() as i32),
                                                                    }),
                                                                    img: None,
                                                                    video: None,
                                                                    data: None,
                                                                    html: None,
                                                                    app: None,
                                                                });
                                                            }
                                                            for img in feed.imgs.iter() {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("img"),
                                                                    req: 1,
                                                                    img: {
                                                                        Some(ImageAsset {
                                                                            url: img.iurl.clone(),
                                                                            mime: img.mimes.clone(),
                                                                            w: img.w,
                                                                            h: img.h,
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
                                                        },
                                                        None => (),
                                                    }

                                                    match &bid.app {
                                                        Some(app) => {
                                                            asset_vec.push(Asset {
                                                                id: assets.consume_asset("app"),
                                                                req: 0,
                                                                app: Some(AppAsset {
                                                                    name: app.name.clone(),
                                                                    desc: None,
                                                                    descurl: None,
                                                                    domain: None,
                                                                    bundle: Some(app.pack.clone()),
                                                                    ver: app.vers.clone(),
                                                                    developer: None,
                                                                    icon: app.icon.clone(),
                                                                    storeid: app.itunesid.clone(),
                                                                    storeurl: None,
                                                                    paid: 0,
                                                                    size: app.size,
                                                                    md5: app.md5.clone(),
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

                                                match &bid.events {
                                                    Some(events) => {
                                                        match &events.els {
                                                            Some(els) => {
                                                                for event in els {
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
                                                        match &events.cls {
                                                            Some(cls) => {
                                                                for event in cls {
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
                                                        match &events.sdls {
                                                            Some(sdls) => {
                                                                for event in sdls {
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
                                                        match &events.edls {
                                                            Some(edls) => {
                                                                for event in edls {
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
                                                        match &events.sils {
                                                            Some(sils) => {
                                                                for event in sils {
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
                                                        match &events.eils {
                                                            Some(eils) => {
                                                                for event in eils {
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
                                                        match &events.ials {
                                                            Some(ials) => {
                                                                for event in ials {
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
                                                        match &events.dclst {
                                                            Some(dclst) => {
                                                                for event in dclst {
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
                                                        match &events.dcls {
                                                            Some(dcls) => {
                                                                for event in dcls {
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
                                                        match &events.dclsf {
                                                            Some(dclsf) => {
                                                                for event in dclsf {
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
                                                        match &events.v_click {
                                                            Some(v_click) => {
                                                                for event in v_click {
                                                                    event_vec.push(Event {
                                                                        eventtype: 721,
                                                                        method: 1,
                                                                        url: replace_macro(event),
                                                                        header: None,
                                                                        content: None,
                                                                    });
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                        match &events.v_start {
                                                            Some(v_start) => {
                                                                for event in v_start {
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
                                                        match &events.v_first_quartile {
                                                            Some(v_first_quartile) => {
                                                                for event in v_first_quartile {
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
                                                        match &events.v_midpoint {
                                                            Some(v_midpoint) => {
                                                                for event in v_midpoint {
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
                                                        match &events.v_third_quartile {
                                                            Some(v_third_quartile) => {
                                                                for event in v_third_quartile {
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
                                                        match &events.v_complete {
                                                            Some(v_complete) => {
                                                                for event in v_complete {
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
                                                        match &events.v_mute {
                                                            Some(v_mute) => {
                                                                for event in v_mute {
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
                                                        match &events.v_unmute {
                                                            Some(v_unmute) => {
                                                                for event in v_unmute {
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
                                                        match &events.v_skip {
                                                            Some(v_skip) => {
                                                                for event in v_skip {
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
                                                        match &events.v_close {
                                                            Some(v_close) => {
                                                                for event in v_close {
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
                                                        match &events.v_suspend {
                                                            Some(v_suspend) => {
                                                                for event in v_suspend {
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
                                                        match &events.v_full {
                                                            Some(v_full) => {
                                                                for event in v_full {
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
                                                        match &events.v_cancelfull {
                                                            Some(v_cancelfull) => {
                                                                for event in v_cancelfull {
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
                                                        match &events.v_play3s {
                                                            Some(v_play3s) => {
                                                                for event in v_play3s {
                                                                    event_vec.push(Event {
                                                                        eventtype: 706,
                                                                        method: 1,
                                                                        url: replace_macro(event),
                                                                        header: None,
                                                                        content: None,
                                                                    });
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                        match &events.v_play5s {
                                                            Some(v_play5s) => {
                                                                for event in v_play5s {
                                                                    event_vec.push(Event {
                                                                        eventtype: 707,
                                                                        method: 1,
                                                                        url: replace_macro(event),
                                                                        header: None,
                                                                        content: None,
                                                                    });
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                        match &events.v_replay {
                                                            Some(v_replay) => {
                                                                for event in v_replay {
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
                                                        match &events.v_upscroll {
                                                            Some(v_upscroll) => {
                                                                for event in v_upscroll {
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
                                                        match &events.v_downscroll {
                                                            Some(v_downscroll) => {
                                                                for event in v_downscroll {
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
                                                        match &events.v_continue_play {
                                                            Some(v_continue_play) => {
                                                                for event in v_continue_play {
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
                                                        match &events.v_load_success {
                                                            Some(v_load_success) => {
                                                                for event in v_load_success {
                                                                    event_vec.push(Event {
                                                                        eventtype: 719,
                                                                        method: 1,
                                                                        url: replace_macro(event),
                                                                        header: None,
                                                                        content: None,
                                                                    });
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                        match &events.v_load_fail {
                                                            Some(v_load_fail) => {
                                                                for event in v_load_fail {
                                                                    event_vec.push(Event {
                                                                        eventtype: 720,
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
                                        advertiser: None,
                                        advertisericon: None,
                                    },
                                };

                                bids.push(bid);

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

    replaced = replaced.replace("__DP_DOWN_X__", "__DOWN_X__");
    replaced = replaced.replace("__DP_DOWN_Y__", "__DOWN_Y__");
    replaced = replaced.replace("__DP_UP_X__", "__UP_X__");
    replaced = replaced.replace("__DP_UP_Y__", "__UP_Y__");

    replaced = replaced.replace("__PNT_DOWN_X__", "__ABS_DOWN_X__");
    replaced = replaced.replace("__PNT_DOWN_Y__", "__ABS_DOWN_Y__");
    replaced = replaced.replace("__PNT_UP_X__", "__ABS_UP_X__");
    replaced = replaced.replace("__PNT_UP_Y__", "__ABS_UP_Y__");

    replaced = replaced.replace("__DISPLAY_LUX__", "__LT_X__");
    replaced = replaced.replace("__DISPLAY_LUY__", "__LT_Y__");
    replaced = replaced.replace("__DISPLAY_RDX__", "__RB_X__");
    replaced = replaced.replace("__DISPLAY_RDY__", "__RB_Y__");

    replaced = replaced.replace("__AZCX__", "__UP_X__");
    replaced = replaced.replace("__AZCY__", "__UP_Y__");
    replaced = replaced.replace("__AZMX__", "__DOWN_X__");
    replaced = replaced.replace("__AZMY__", "__DOWN_Y__");

    replaced = replaced.replace("__BTN_R_DOWN_X__", "__BUTTON_RB_X__");
    replaced = replaced.replace("__BTN_R_DOWN_Y__", "__BUTTON_RB_Y__");
    replaced = replaced.replace("__BTN_L_UP_X__", "__BUTTON_LT_X__");
    replaced = replaced.replace("__BTN_L_UP_Y__", "__BUTTON_LT_Y__");

    replaced = replaced.replace("__LONGITUDE__", "__LON__");
    replaced = replaced.replace("__LATITUDE__", "__LAT__");

    replaced = replaced.replace("__E_END_S__", "__EVENT_TIME_END_S__");
    replaced = replaced.replace("__E_END__", "__EVENT_TIME_END__");

    replaced = replaced.replace("__DPLINK__", "__DP_STATUS__");
    replaced = replaced.replace("__DLD_PHASE__", "__DOWNLOAD_PHASE__");

    replaced = replaced.replace("__PROGRESS__", "__VIDEO_PLAY_PROGRESS_S__");
    replaced = replaced.replace("__PROGRESS_MS__", "__VIDEO_PLAY_PROGRESS__");
    replaced = replaced.replace("__PLAY_FIRST_FRAME__", "__VIDEO_FIRST_FRAME__");
    replaced = replaced.replace("__PLAY_LAST_FRAME__", "__VIDEO_LAST_FRAME__");
    replaced = replaced.replace("__SCENE__", "__VIDEO_PLAY_SCENE__");
    replaced = replaced.replace("__TYPE__", "__VIDEO_PLAY_TYPE__");
    replaced = replaced.replace("__BEHAVIOR__", "__VIDEO_PLAY_TRIGGER__");
    replaced = replaced.replace("__STATUS__", "__VIDEO_PLAY_STATUS__");
    replaced = replaced.replace("__P_DURATION__", "__VIDEO_PLAY_DURATION__");
    replaced = replaced.replace("_RATE__", "__VIDEO_PLAY_RATIO__");

    replaced
}
