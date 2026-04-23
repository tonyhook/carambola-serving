use std::{collections::HashMap, time::Duration};

use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyInit};
use base64::prelude::*;
use chrono::{Datelike, Local, TimeZone};
use chrono_tz::Tz;
use reqwest::Url;
use urlencoding::encode;

use crate::{protocol::*, Assets, Cache, Client, Connection, HttpPool, Identifiers, Price, ResultMessage};

pub mod app_asset;
pub mod app;
pub mod banner_asset;
pub mod banner_format;
pub mod bid;
pub mod caid;
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
pub use caid::AdwanjiCaid;
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

type Aes256EcbEnc = ecb::Encryptor<aes::Aes256>;

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
                    connection.client_tag_id.split("|").nth(0).unwrap().to_string()
                },
                banner: {
                    match assets.get_banner() {
                        Some(displayfmt) =>
                                Some(AdwanjiBannerFormat {
                                    w: {
                                        match displayfmt.w {
                                            Some(w) => w,
                                            None => 0,
                                        }
                                    },
                                    h: {
                                        match displayfmt.h {
                                            Some(h) => h,
                                            None => 0,
                                        }
                                    },
                                    pos: {
                                        match request.item[0].spec.display.pos {
                                            Some(4) => 1,
                                            Some(5) => 2,
                                            Some(7) => {
                                                match request.item[0].spec.display.instl {
                                                    0 => 4,
                                                    1 => 5,
                                                    _ => 4,
                                                }
                                            },
                                            Some(501) => 3,
                                            _ => 3,
                                        }
                                    },
                                }),
                        None => None,
                    }
                },
                feed: {
                    if assets.get_asset_size("img") > 0 && assets.get_asset_size("video") == 0 {
                        let img = assets.get_current_asset("img").unwrap().img.clone().unwrap();
                        Some(AdwanjiFeedFormat {
                            w: match img.w {
                                Some(w) => Some(w),
                                None => None,
                            },
                            h: match img.h {
                                Some(h) => Some(h),
                                None => None,
                            },
                            feedtype: if assets.get_asset_size("img") == 1 {
                                [1].to_vec()
                            } else {
                                [2].to_vec()
                            },
                        })
                    } else {
                        None
                    }
                },
                video: {
                    if assets.get_asset_size("video") >= 1 {
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
                        None
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
                                        None => "".to_string(),
                                    }
                                },
                            }
                        },
                        ver: {
                            match &app.ver {
                                Some(ver) => ver.clone(),
                                None => "".to_string(),
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
                    None => AdwanjiApp {
                        name: "".to_string(),
                        bundle: "".to_string(),
                        ver: "".to_string(),
                        paid: 0,
                        keywords: None,
                        storeurl: None,
                        itunesid: None,
                    },
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
                                geo.lat.clone()
                            },
                            lon: {
                                geo.lon.clone()
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
                            accu: {
                                match geo.accur {
                                    Some(accur) => Some(accur as i32),
                                    None => None,
                                }
                            },
                            city_code: None,
                            city: None,
                        },
                        None => AdwanjiGeo {
                            lat: None,
                            lon: None,
                            coordinate: None,
                            timestamp: None,
                            accu: None,
                            city_code: None,
                            city: None,
                        },
                    }
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
                                _ => 0,
                            }
                        },
                        None => 0,
                    }
                },
                make: {
                    request.context.device.make.clone()
                },
                brand: {
                    request.context.device.brand.clone()
                },
                model: {
                    request.context.device.model.clone()
                },
                os: {
                    match request.context.device.os {
                        Some(2) => 0,
                        Some(13) => 1,
                        _ => 9,
                    }
                },
                osv: {
                    request.context.device.osv.clone()
                },
                oslevel: {
                    request.context.device.oslevel.clone()
                },
                resolution: {
                    let width;
                    let height;
                    match request.context.device.w {
                        Some(w) => width = w,
                        None => width = 0,
                    }
                    match request.context.device.h {
                        Some(h) => height = h,
                        None => height = 0,
                    }
                    format!("{}{}{}", width.to_string(), "*".to_string(), height.to_string())
                },
                sh: {
                    request.context.device.h.clone()
                },
                sw: {
                    request.context.device.w.clone()
                },
                ppi: {
                    request.context.device.ppi.clone()
                },
                dpi: {
                    request.context.device.ppi.clone()
                },
                density: {
                    request.context.device.pxratio.clone()
                },
                orientation: {
                    match request.context.device.orientation {
                        Some(501) => 0,
                        Some(502) => 1,
                        _ => 9,
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
                caids: {
                    let mut caids = vec![];
                    for index in 0..=1 {
                        if let Some(uid) = identifiers.get_id(513, index) {
                            caids.push(AdwanjiCaid {
                                id: uid.id.clone(),
                                version: uid.ver.clone(),
                            });
                        }
                    }
                    if caids.is_empty() { None } else { Some(caids) }
                },
                aaid: None,
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
                    request.context.device.carrier.clone()
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
                                _ => 0,
                            }
                        },
                        None => 0,
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
                    request.context.device.lang.clone()
                },
                countrycode: {
                    request.context.device.country.clone()
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
                        Some(boottime) => Some(boottime.split(".").nth(0).unwrap().to_string()),
                        None => None,
                    }
                },
                startnanotime: {
                    request.context.device.boottime.clone()
                },
                startmilltime: {
                    match &request.context.device.boottime {
                        Some(boottime) => {
                            let mut parts = boottime.splitn(2, ".");
                            let secs = parts.next().unwrap_or("0");
                            let millis = parts.next()
                                .map(|frac| if frac.len() >= 3 { &frac[..3] } else { "000" })
                                .unwrap_or("000");
                            Some(format!("{}.{}", secs, millis))
                        }
                        None => None,
                    }
                },
                mnt_id: {
                    None
                },
                client_time: {
                    Some(Local::now().timestamp_millis().to_string())
                },
                birthtime: {
                    request.context.device.inittime.clone()
                },
                osupdatetime: {
                    match &request.context.device.updatetime {
                        Some(updatetime) => Some(updatetime.split(".").nth(0).unwrap().to_string()),
                        None => None,
                    }
                },
                osupdatenanotime: {
                    request.context.device.updatetime.clone()
                },
                hwname: {
                    request.context.device.hwname.clone()
                },
                hwmodel: {
                    request.context.device.hwmodel.clone()
                },
                hwmachine: {
                    request.context.device.hwmachine.clone()
                },
                sysmemory: {
                    request.context.device.sysmemory.map(|sysmemory| sysmemory.to_string())
                },
                sysdisksize: {
                    request.context.device.sysdisksize.map(|sysdisksize| sysdisksize.to_string())
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
                                   Some((t.timestamp() - utc.timestamp()).to_string())
                                },
                                Err(_) => None,
                            }
                        },
                        None => None,
                    }
                },
                updatemark: {
                    request.context.device.updatemark.clone()
                },
                bootmark: {
                    request.context.device.bootmark.clone()
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
                                        Err(_) => Some(0),
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
                        None => "".to_string(),
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
                        Some(keywords) => Some(keywords.clone()),
                        None => None,
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
        let response_adwanji_raw = client.post("http://api.adx.admtvs.com/api/1/ad")
            .json(&request_adwanji)
            .header("Accept-Encoding", "gzip")
            .header("Connection", "keep-alive")
            .header("Content-Type", "application/json")
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
                                        100 => (),
                                        101 => {
                                            return Err(ResultMessage {
                                                code: 993,
                                                message: "".to_string(),
                                            });
                                        },
                                        103 => {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: "upstream error: parameter error".to_string(),
                                            });
                                        },
                                        104 => {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: "upstream error: unknown error".to_string(),
                                            });
                                        },
                                        201 => {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: "upstream error: beyond qps".to_string(),
                                            });
                                        },
                                        _ => {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: "upstream error: unknown error".to_string(),
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
                                    wechatmppath: bid.wxapppath.clone(),
                                    wechatmpid: bid.wxappid.clone(),
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
                                    lurl: {
                                        match &bid.events {
                                            Some(events) => {
                                                match &events.fail_notice {
                                                    Some(fail_notice) => {
                                                        let mut lurl = Vec::<String>::new();
                                                        for url in fail_notice {
                                                            lurl.push(replace_macro(url));
                                                        }
                                                        Some(lurl)
                                                    },
                                                    None => None,
                                                }
                                            },
                                            None => None,
                                        }
                                    },
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
                                                let mut asset_vec = vec![];

                                                match &bid.video {
                                                    Some(video) => {
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
                                                        if video.cover_url.is_some() {
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
                                                        if video.ad_icon.is_some() {
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
                                                        if video.end_url.is_some() {
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
                                                        if video.ad_text.is_some() {
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
                                                        if video.button_text.is_some() {
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
                                                        if video.end_html.is_some() {
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
                                                        asset_vec.push(Asset {
                                                            id: assets.consume_asset("title"),
                                                            req: 1,
                                                            title: Some(TitleAsset {
                                                                text: {
                                                                    match &feed.title {
                                                                        Some(title) => title.clone(),
                                                                        None => "".to_string(),
                                                                    }
                                                                },
                                                                subtitle: None,
                                                                desc: feed.desc.clone(),
                                                                len: {
                                                                    match &feed.title {
                                                                        Some(title) => Some(title.len() as i32),
                                                                        None => None,
                                                                    }
                                                                },
                                                            }),
                                                            img: None,
                                                            video: None,
                                                            data: None,
                                                            html: None,
                                                            app: None,
                                                        });
                                                        match &feed.imgs {
                                                            Some(imgs) => {
                                                                for img in imgs.iter() {
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
                                                    },
                                                    None => (),
                                                }

                                                match &bid.app {
                                                    Some(app) => {
                                                        if app.name.is_some() || app.pack.is_some() {
                                                            asset_vec.push(Asset {
                                                                id: assets.consume_asset("app"),
                                                                req: 0,
                                                                app: Some(AppAsset {
                                                                    name: {
                                                                        match &app.name {
                                                                            Some(name) => name.clone(),
                                                                            None => {
                                                                                match &app.pack {
                                                                                    Some(pack) => pack.clone(),
                                                                                    None => "".to_string(),
                                                                                }
                                                                            }
                                                                        }
                                                                    },
                                                                    desc: None,
                                                                    descurl: app.description_url.clone(),
                                                                    domain: None,
                                                                    bundle: app.pack.clone(),
                                                                    ver: app.vers.clone(),
                                                                    developer: app.author.clone(),
                                                                    icon: app.icon.clone(),
                                                                    storeid: app.itunesid.clone(),
                                                                    storeurl: None,
                                                                    paid: 0,
                                                                    size: app.size,
                                                                    md5: app.md5.clone(),
                                                                    registration: None,
                                                                    privacy: None,
                                                                    privacyurl: app.privacy_agreement.clone(),
                                                                    permission: None,
                                                                    permissionurl: app.permissions_url.clone(),
                                                                }),
                                                                title: None,
                                                                img: None,
                                                                video: None,
                                                                data: None,
                                                                html: None,
                                                            });
                                                        }
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

                                                match &bid.events {
                                                    Some(events) => {
                                                        match &events.els {
                                                            Some(els) => {
                                                                for event in els {
                                                                    event_vec.push(Event {
                                                                        eventtype: 501,
                                                                        method: 1,
                                                                        url: {
                                                                            let url = replace_macro(event);
                                                                            let price = match bid.price {
                                                                                Some(price) => {
                                                                                    if price > 0 {
                                                                                        price as i32
                                                                                    } else {
                                                                                        connection.default_price
                                                                                    }
                                                                                },
                                                                                None => connection.default_price,
                                                                            };
                                                                            let encrypt_price = Self::encrypt_price(price, &"".to_string(), connection);
                                                                            url.replace("__PRICE__", &encode(encrypt_price.as_str()))
                                                                        },
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
                                                        match &events.click_area_report_url {
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
                                                        match &events.clo {
                                                            Some(clo) => {
                                                                for event in clo {
                                                                    event_vec.push(Event {
                                                                        eventtype: 509,
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

    async fn bidding_notify_win(url: String, win_price: i32, next_price: i32, iv: &String, connection: &Connection, pool: &HttpPool) {
        let encrypt_win_price = Self::encrypt_price(win_price, iv, connection);
        let replaced_url = url
            .replace("__WIN_PRICE__", &encrypt_win_price)
            .replace("__PRICE__", &encrypt_win_price)
            .replace("__2ND_PRICE__", &next_price.to_string());

        let client = {
            let pool_adwanji_lock = pool.pool_adwanji.clone();
            let pool_adwanji = pool_adwanji_lock.read().unwrap();
            pool_adwanji.clone()
        };
        let _ = client.get(replaced_url).send().await;
    }

    async fn bidding_notify_lose(url: String, lose_price: i32, lose_reason: i32, lose_adn_name: &String, _iv: &String, _connection: &Connection, pool: &HttpPool) {
        let replaced_url = url
            .replace("__LOSE_PRICE__", &lose_price.to_string())
            .replace("__BID_ECPM__", &lose_price.to_string())
            .replace("__AD_ECPM__", &lose_price.to_string())
            .replace("__BID_FAIL_REASON__", &lose_reason.to_string())
            .replace("__LOSE_REASON__", &lose_reason.to_string())
            .replace("__LOSE_ADN_NAME__", lose_adn_name)
            .replace("__ADN_NAME__", lose_adn_name);

        let client = {
            let pool_adwanji_lock = pool.pool_adwanji.clone();
            let pool_adwanji = pool_adwanji_lock.read().unwrap();
            pool_adwanji.clone()
        };
        let _ = client.get(replaced_url).send().await;
    }

    fn encrypt_price(price: i32, _iv: &String, connection: &Connection) -> String {
        let message = format!("{}", price);
        let plaintext = message.as_bytes();
        let pos = plaintext.len();
        let mut buffer = [0u8; 32];
        buffer[..pos].copy_from_slice(plaintext);

        let key = connection.client_tag_id.split("|").nth(1).unwrap().as_bytes();
        let cipher = Aes256EcbEnc::new(key[0..32].into())
            .encrypt_padded_mut::<Pkcs7>(&mut buffer, pos)
            .unwrap();

        BASE64_STANDARD.encode(cipher).replace("+", "-").replace("/", "_").replace("=", "")
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
    replaced = replaced.replace("__P_DURATION__", "__VIDEO_PLAY_PROGRESS_L__");
    replaced = replaced.replace("_RATE__", "__VIDEO_PLAY_RATIO__");

    replaced
}
