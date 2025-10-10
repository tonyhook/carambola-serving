use std::{io::Write, time::Duration};

use base64::{prelude::BASE64_STANDARD, Engine};
use chrono::{DateTime, Datelike, Local, Utc};
use hmac::{Hmac, Mac};
use flate2::{Compression, write::GzEncoder};
use sha1::{Digest, Sha1};
use urlencoding::encode;

use crate::{protocol::*, Assets, Cache, Client, Connection, HttpPool, Identifiers, Price, ResultMessage};

type HmacSha1 = Hmac<Sha1>;

pub mod ad_slot;
pub mod app_data;
pub mod app;
pub mod bid;
pub mod device;
pub mod event_track;
pub mod geo;
pub mod image;
pub mod material_meta;
pub mod mini_program;
pub mod progress_track;
pub mod request;
pub mod response;
pub mod seat_bid;
pub mod site;
pub mod user;
pub mod video;

pub use ad_slot::BillowlinkAdSlot;
pub use app_data::BillowlinkAppData;
pub use app::BillowlinkApp;
pub use bid::BillowlinkBid;
pub use device::BillowlinkDevice;
pub use event_track::BillowlinkEventTrack;
pub use geo::BillowlinkGeo;
pub use image::BillowlinkImage;
pub use material_meta::BillowlinkMaterialMeta;
pub use mini_program::BillowlinkMiniProgram;
pub use progress_track::BillowlinkProgressTrack;
pub use request::BillowlinkRequest;
pub use response::BillowlinkResponse;
pub use seat_bid::BillowlinkSeatBid;
pub use site::BillowlinkSite;
pub use user::BillowlinkUser;
pub use video::BillowlinkVideo;

pub struct Billowlink {

}

impl Client for Billowlink {

    async fn request(request: &Request, connection: &Connection, pool: &HttpPool, cache: &Cache) -> Result<Response, ResultMessage> {
        let slot_id = connection.client_tag_id.split("|").nth(0).unwrap();
        let app_id = connection.client_tag_id.split("|").nth(1).unwrap();
        let app_cat = connection.client_tag_id.split("|").nth(2).unwrap();

        let request_id = cache.get_sequence();

        let mut assets = Assets::new(request);
        let identifiers = Identifiers::new(request);

        let request_billowlink = BillowlinkRequest {
            request_id: {
                request_id.to_string()
            },
            api_ver: {
                "v3".to_string()
            },
            ad_slot: BillowlinkAdSlot {
                slot_id: {
                    slot_id.to_string()
                },
                ad_type: {
                    let mut ad_type = 0;
                    let reward = request.item[0].spec.reward;
                    let instl = request.item[0].spec.display.instl;

                    if assets.get_banner_size() > 0 {
                        if instl == 1 {
                            ad_type = 2;
                        } else {
                            if request.item[0].spec.display.w > request.item[0].spec.display.h {
                                ad_type = 1;
                            } else {
                                ad_type = 3;
                            }
                        }
                    }
                    if assets.get_asset_size("img") > 0 {
                        ad_type = 4;
                    }
                    if assets.get_asset_size("thumb") > 0 {
                        ad_type = 4;
                    }
                    if assets.get_asset_size("video") > 0 {
                        if instl == 1 {
                            ad_type = 7;
                        } else {
                            if reward == 1 {
                                ad_type = 6;
                            } else {
                                ad_type = 5;
                            }
                        }
                    }

                    ad_type
                },
                bid_floor: {
                    Some(Price::to_client(connection, request.item[0].flr.map(f64::from)) as i64)
                },
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
                creative_type: {
                    let mut creative_type = 0;
                    if assets.get_banner_size() > 0 {
                        creative_type = 1;
                    }
                    if assets.get_asset_size("img") == 1 {
                        creative_type = 1;
                    }
                    if assets.get_asset_size("img") > 1 {
                        creative_type = 2;
                    }
                    if assets.get_asset_size("video") > 0 {
                        creative_type = 3;
                    }

                    creative_type
                },
                min_duration: {
                    if assets.get_asset_size("video") > 0 {
                        let video = assets.get_current_asset("video").unwrap().video.clone().unwrap();
                        video.mindur
                    } else {
                        None
                    }
                },
                max_duration: {
                    if assets.get_asset_size("video") > 0 {
                        let video = assets.get_current_asset("video").unwrap().video.clone().unwrap();
                        video.maxdur
                    } else {
                        None
                    }
                },
                skip: {
                    if assets.get_asset_size("video") > 0 {
                        let video = assets.get_current_asset("video").unwrap().video.clone().unwrap();
                        video.skip
                    } else {
                        None
                    }
                },
                skip_after: {
                    if assets.get_asset_size("video") > 0 {
                        let video = assets.get_current_asset("video").unwrap().video.clone().unwrap();
                        Some(video.skipafter)
                    } else {
                        None
                    }
                },
                video_type: {
                    Some(request.item[0].spec.reward)
                },
                banner_type: {
                    1
                },
            },
            site: {
                match &request.context.site {
                    Some(site) => {
                        Some(BillowlinkSite {
                            site_id: app_id.to_string(),
                            name: Some(site.name.clone()),
                            domain: site.domain.clone(),
                            page: site.page.clone(),
                            referrer: site.referrer.clone(),
                            cat: Some(app_cat.to_string()),
                        })
                    },
                    None => None,
                }
            },
            app: {
                match &request.context.app {
                    Some(app) => {
                        BillowlinkApp {
                            app_id: app_id.to_string(),
                            name: app.name.clone(),
                            bundle: {
                                match &app.bundle {
                                    Some(bundle) => bundle.clone(),
                                    None => return Err(ResultMessage {
                                        code: 998,
                                        message: "request.context.app.bundle is required for upstream".to_string(),
                                    }),
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
                            store_url: app.storeurl.clone(),
                            cat: Some(app_cat.to_string()),
                            keywords: None,
                            paid: Some(app.paid),
                        }
                    },
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.app is required for upstream".to_string(),
                    }),
                }
            },
            device: BillowlinkDevice {
                device_type: {
                    match request.context.device.devicetype {
                        Some(devicetype) => {
                            match devicetype {
                                1 => 1,
                                4 => 1,
                                5 => 2,
                                _ => 3,
                            }
                        },
                        None => 0,
                    }
                },
                dnt: {
                    Some(0)
                },
                ua: {
                    request.context.device.ua.clone()
                },
                ip: {
                    match &request.context.device.ip {
                        Some(ip) => {
                            ip.clone()
                        },
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.ip is required for upstream".to_string(),
                        }),
                    }
                },
                ipv6: {
                    request.context.device.ipv6.clone()
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
                                2 => 2,
                                13 => 1,
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
                        Some(osv) => {
                            osv.clone()
                        },
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.osv is required for upstream".to_string(),
                        }),
                    }
                },
                w: {
                    match request.context.device.w {
                        Some(w) => w,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.display.w is required for upstream".to_string(),
                        }),
                    }
                },
                h: {
                    match request.context.device.h {
                        Some(h) => h,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.display.h is required for upstream".to_string(),
                        }),
                    }
                },
                carrier: {
                    match &request.context.device.carrier {
                        Some(carrier) => {
                            match carrier.as_str() {
                                "cmcc" => "46000".to_string(),
                                "unicom" => "46001".to_string(),
                                "telecom" => "46003".to_string(),
                                _ => "-1".to_string(),
                            }
                        },
                        None => "-1".to_string(),
                    }
                },
                connection: {
                    match &request.context.device.contype {
                        Some(contype) => {
                            match contype {
                                1 => 1,
                                2 => 6,
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
                imei_sha1: {
                    match identifiers.get_id(501, 0) {
                        Some(uid) => {
                            let mut hasher = Sha1::new();
                            hasher.update(uid.id.clone());
                            Some(format!("{:x}", hasher.finalize()))
                        },
                        None => None,
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
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                android_id_sha1: {
                    match identifiers.get_id(509, 0) {
                        Some(uid) => {
                            let mut hasher = Sha1::new();
                            hasher.update(uid.id.clone());
                            Some(format!("{:x}", hasher.finalize()))
                        },
                        None => None,
                    }
                },
                imsi: {
                    match identifiers.get_id(503, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
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
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                idfa_sha1: {
                    match identifiers.get_id(507, 0) {
                        Some(uid) => {
                            let mut hasher = Sha1::new();
                            hasher.update(uid.id.clone());
                            Some(format!("{:x}", hasher.finalize()))
                        },
                        None => None,
                    }
                },
                caids: {
                    match identifiers.get_ids(504) {
                        Some(uids) => {
                            let mut caids = vec![];
                            for uid in uids {
                                match &uid.ver {
                                    Some(ver) => caids.push(format!("{}:{}", ver, uid.id)),
                                    _ => (),
                                }
                            }
                            caids
                        },
                        None => [].to_vec(),
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
                bssid: {
                    match identifiers.get_id(529, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                paid: {
                    match identifiers.get_id(519, 0) {
                        Some(uid) => uid.id.clone(),
                        None => "".to_string(),
                    }
                },
                aaid: {
                    match identifiers.get_id(514, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                geo: {
                    match &request.context.device.geo {
                        Some(geo) => {
                            Some(BillowlinkGeo {
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
                                city: geo.city.clone(),
                                province: geo.province.clone(),
                                district: geo.district.clone(),
                            })
                        },
                        None => None,
                    }
                },
                dpi: {
                    request.context.device.ppi.clone()
                },
                density: {
                    match request.context.device.pxratio {
                        Some(pxratio) => Some(pxratio as f32),
                        None => None,
                    }
                },
                country: {
                    request.context.device.country.clone()
                },
                language: {
                    request.context.device.lang.clone()
                },
                sys_ver: {
                    request.context.device.romv.clone()
                },
                app_store_package: {
                    request.context.device.storename.clone()
                },
                hwag_ver: {
                    request.context.device.storev.clone()
                },
                hms_ver: {
                    request.context.device.hmsv.clone()
                },
                hw_model: {
                    match &request.context.device.hwmodel {
                        Some(hwmodel) => hwmodel.clone(),
                        None => "".to_string(),
                    }
                },
                hw_name: {
                    match &request.context.device.hwname {
                        Some(hwname) => hwname.clone(),
                        None => "".to_string(),
                    }
                },
                hw_name_md5: {
                    match &request.context.device.hwname {
                        Some(hwname) => format!("{:x}", md5::compute(format!("{}", hwname).as_bytes())),
                        None => "".to_string(),
                    }
                },
                hw_machine: {
                    match &request.context.device.hwmachine {
                        Some(hwmachine) => hwmachine.clone(),
                        None => "".to_string(),
                    }
                },
                sys_memory: {
                    match &request.context.device.sysmemory {
                        Some(sysmemory) => sysmemory.to_string(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.sysmemory is required for upstream".to_string(),
                        }),
                    }
                },
                sys_disksize: {
                    match &request.context.device.sysdisksize {
                        Some(sysdisksize) => sysdisksize.to_string(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.sysdisksize is required for upstream".to_string(),
                        }),
                    }
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
                boot_mark: {
                    request.context.device.bootmark.clone()
                },
                update_mark: {
                    request.context.device.updatemark.clone()
                },
                device_initialize_time: {
                    match &request.context.device.inittime {
                        Some(inittime) => inittime.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.inittime is required for upstream".to_string(),
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
                os_update_time_sec: {
                    match &request.context.device.updatetime {
                        Some(updatetime) => updatetime.clone(),
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.updatetime is required for upstream".to_string(),
                        }),
                    }
                },
            },
            user: {
                Some(BillowlinkUser {
                    user_id: {
                        request.context.user.id.clone()
                    },
                    tags: {
                        None
                    },
                    gender: {
                        match &request.context.user.gender {
                            Some(gender) => {
                                match gender.as_str() {
                                    "M" => Some("male".to_string()),
                                    "F" => Some("female".to_string()),
                                    _ => None,
                                }
                            },
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
                        request.context.user.keywords.clone()
                    },
                    app_list: {
                        match &request.context.device.app {
                            Some(app) => Some(app.split(",").map(|s| s.to_string()).collect()),
                            None => None,
                        }
                    },
                })
            },
            bcat: {
                None
            },
            badv: {
                None
            },
            https: {
                Some(0)
            },
            support302: {
                Some(1)
            },
            deeplink: {
                1
            },
            timeout: {
                Some(connection.timeout as i32)
            },
            media_time: {
                None
            },
            ssp_time: {
                let utc: DateTime<Utc> = Utc::now();
                Some(utc.timestamp_millis())
            },
        };

        let json_string = serde_json::to_vec(&request_billowlink).unwrap();
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&json_string).unwrap();
        let compressed_bytes = encoder.finish().unwrap();

        let response_billowlink: BillowlinkResponse;

        let client = {
            let pool_billowlink_lock = pool.pool_billowlink.clone();
            let pool_billowlink = pool_billowlink_lock.read().unwrap();
            pool_billowlink.clone()
        };
        let response_billowlink_raw = client.post("http://dsp.billowlink.com/api/v3/adsz/xingguo")
            .body(compressed_bytes)
            .header("Accept-Encoding", "gzip")
            .header("Connection", "keep-alive")
            .header("Content-Encoding", "gzip")
            .header("Content-Type", "application/json")
            .timeout(Duration::from_millis(connection.timeout))
            .send().await;

        match response_billowlink_raw {
            Ok(response_billowlink_raw) => {
                let status = response_billowlink_raw.status();
                if status != 200 {
                    return Err(ResultMessage {
                        code: 992,
                        message: {
                            match response_billowlink_raw.text().await {
                                Ok(text) => format!("upstream error {}: {}", status, text),
                                Err(_) => format!("upstream error {}", status),
                            }
                        },
                    });
                } else {
                    match response_billowlink_raw.text().await {
                        Ok(text) => {
                            match serde_json::from_str::<BillowlinkResponse>(&text) {
                                Ok(json) => {
                                    if json.code == 0 {
                                        response_billowlink = json;
                                    } else {
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

                for seat_bid in response_billowlink.seat_bids.unwrap() {
                    let seatbid = Seatbid {
                        bid: {
                            let mut bids = vec![];

                            for bid in seat_bid.bids {
                                let link_asset = LinkAsset {
                                    linktype: {
                                        match bid.click_action {
                                            Some(0) => 1,
                                            Some(1) => {
                                                match &bid.event_track {
                                                    Some(event_track) => {
                                                        match event_track.gdt_tracks {
                                                            Some(1) => 3,
                                                            _ => 2,
                                                        }
                                                    },
                                                    None => 2,
                                                }
                                            },
                                            Some(2) => 1,
                                            _ => 0,
                                        }
                                    },
                                    universallink: None,
                                    storeid: None,
                                    deeplink: {
                                        bid.deeplink.clone()
                                    },
                                    quickapplink: None,
                                    wechatmppath: {
                                        match bid.creative.mini_program {
                                            Some(mini_program) => mini_program.mp_path.clone(),
                                            None => None,
                                        }
                                    },
                                    wechatmpid: None,
                                    marketurl: None,
                                    downloadurl: {
                                        match bid.click_action {
                                            Some(0) => None,
                                            Some(1) => {
                                                match &bid.event_track {
                                                    Some(event_track) => {
                                                        match event_track.gdt_tracks {
                                                            Some(1) => {
                                                                Some(bid.landing.clone())
                                                            },
                                                            _ => None,
                                                        }
                                                    },
                                                    None => None,
                                                }
                                            },
                                            Some(2) => None,
                                            _ => None,
                                        }
                                    },
                                    url: {
                                        bid.landing.clone()
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
                                                    price as i32
                                                } else {
                                                    connection.default_price
                                                }
                                            },
                                            None => connection.default_price,
                                        }
                                    },
                                    burl: {
                                        match &bid.nurl {
                                            Some(bidnurl) => {
                                                let mut burl = Vec::<String>::new();
                                                let mut nurl = bidnurl.clone();
                                                nurl = nurl.replace("__bid_id__", request_id.to_string().as_str());
                                                nurl = nurl.replace("__bid_price__", "__WIN_PRICE__");
                                                nurl = nurl.replace("__bid_price_plain__", format!("{}", {
                                                    match bid.price {
                                                        Some(price) => {
                                                            if price > 0 {
                                                                price as i32
                                                            } else {
                                                                connection.default_price
                                                            }
                                                        },
                                                        None => connection.default_price,
                                                    }
                                                }).as_str());
                                                burl.push(replace_macro(&nurl));
                                                Some(burl)
                                            },
                                            None => None,
                                        }
                                    },
                                    lurl: {
                                        match &bid.lurl {
                                            Some(bidlurl) => {
                                                let mut lurl = Vec::<String>::new();
                                                let mut nurl = bidlurl.clone();
                                                nurl = nurl.replace("__bid_price__", "__WIN_PRICE__");
                                                nurl = nurl.replace("__bid_id__", format!("{}", request_id).as_str());
                                                lurl.push(replace_macro(&nurl));
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
                                                    match &bid.creative.image {
                                                        Some(image) => {
                                                            match &image.url {
                                                                Some(url) => {
                                                                    if !url.is_empty() {
                                                                        Some(Banner {
                                                                            img: url.clone(),
                                                                            link: Some(link_asset.clone()),
                                                                        })
                                                                    } else {
                                                                        None
                                                                    }
                                                                },
                                                                None => None,
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

                                                    match &bid.creative.title {
                                                        Some(title) => {
                                                            asset_vec.push(Asset {
                                                                id: assets.consume_asset("title"),
                                                                req: 1,
                                                                title: Some(TitleAsset {
                                                                    text: title.clone(),
                                                                    subtitle: None,
                                                                    desc: bid.creative.description.clone(),
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

                                                    match &bid.creative.icon {
                                                        Some(icon) => {
                                                            match &icon.url {
                                                                Some(url) => {
                                                                    if !url.is_empty() {
                                                                        asset_vec.push(Asset {
                                                                            id: assets.consume_asset("icon"),
                                                                            req: 1,
                                                                            title: None,
                                                                            img: Some(ImageAsset {
                                                                                url: url.clone(),
                                                                                mime: None,
                                                                                w: icon.w.clone(),
                                                                                h: icon.h.clone(),
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
                                                        },
                                                        None => (),
                                                    }

                                                    match &bid.creative.image {
                                                        Some(image) => {
                                                            match &image.url {
                                                                Some(url) => {
                                                                    if !url.is_empty() && assets.get_asset_size("img") > 0 {
                                                                        asset_vec.push(Asset {
                                                                            id: assets.consume_asset("img"),
                                                                            req: 1,
                                                                            img: Some(ImageAsset {
                                                                                url: url.clone(),
                                                                                mime: None,
                                                                                w: image.w.clone(),
                                                                                h: image.h.clone(),
                                                                                imagetype: Some(3),
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
                                                        },
                                                        None => (),
                                                    }

                                                    match &bid.creative.images {
                                                        Some(images) => {
                                                            if assets.get_asset_size("img") > 0 {
                                                                for image in images {
                                                                    match &image.url {
                                                                        Some(url) => {
                                                                            if !url.is_empty() {
                                                                                asset_vec.push(Asset {
                                                                                    id: assets.consume_asset("img"),
                                                                                    req: 1,
                                                                                    img: Some(ImageAsset {
                                                                                        url: url.clone(),
                                                                                        mime: None,
                                                                                        w: image.w.clone(),
                                                                                        h: image.h.clone(),
                                                                                        imagetype: Some(3),
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
                                                                }
                                                            }
                                                            if assets.get_asset_size("thumb") > 0 {
                                                                for image in images {
                                                                    match &image.url {
                                                                        Some(url) => {
                                                                            if !url.is_empty() {
                                                                                asset_vec.push(Asset {
                                                                                    id: assets.consume_asset("thumb"),
                                                                                    req: 1,
                                                                                    img: Some(ImageAsset {
                                                                                        url: url.clone(),
                                                                                        mime: None,
                                                                                        w: image.w.clone(),
                                                                                        h: image.h.clone(),
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
                                                                }
                                                            }
                                                        },
                                                        None => (),
                                                    }

                                                    match &bid.creative.html_snippet {
                                                        Some(html_snippet) => {
                                                            asset_vec.push(Asset {
                                                                id: assets.consume_asset("html"),
                                                                req: 1,
                                                                title: None,
                                                                img: None,
                                                                video: None,
                                                                data: None,
                                                                html: Some(HtmlAsset {
                                                                    html: Some(html_snippet.clone()),
                                                                    link: None,
                                                                    len: None,
                                                                }),
                                                                app: None,
                                                            });
                                                        },
                                                        None => (),
                                                    }

                                                    match &bid.creative.video {
                                                        Some(video) => {
                                                            if video.url.is_some() {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("video"),
                                                                    req: 1,
                                                                    video: Some(VideoAsset {
                                                                        url: video.url.clone().unwrap(),
                                                                        mime: None,
                                                                        w: {
                                                                            match &video.resolution {
                                                                                Some(resolution) => Some(resolution.split("*").nth(0).unwrap().parse::<i32>().unwrap()),
                                                                                None => None,
                                                                            }
                                                                        },
                                                                        h: {
                                                                            match &video.resolution {
                                                                                Some(resolution) => Some(resolution.split("*").nth(1).unwrap().parse::<i32>().unwrap()),
                                                                                None => None,
                                                                            }
                                                                        },
                                                                        dur: {
                                                                            match video.duration {
                                                                                Some(duration) => Some(duration as i32),
                                                                                None => None,
                                                                            }
                                                                        },
                                                                        skipoffset: None,
                                                                        size: {
                                                                            match video.size {
                                                                                Some(size) => Some(size as i32),
                                                                                None => None,
                                                                            }
                                                                        },
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

                                                                match &video.after_html {
                                                                    Some(after_html) => {
                                                                        asset_vec.push(Asset {
                                                                            id: assets.consume_asset("video#end#html"),
                                                                            req: 0,
                                                                            html: Some(HtmlAsset {
                                                                                html: Some(after_html.clone()),
                                                                                link: None,
                                                                                len: Some(after_html.len() as i32),
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
                                                                match &video.cover {
                                                                    Some(cover) => {
                                                                        asset_vec.push(Asset {
                                                                            id: assets.consume_asset("video#cover"),
                                                                                req: 0,
                                                                                img: Some(ImageAsset {
                                                                                    url: cover.clone(),
                                                                                    mime: None,
                                                                                    w: video.coverw.clone(),
                                                                                    h: video.coverh.clone(),
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
                                                                match &video.title {
                                                                    Some(title) => {
                                                                        asset_vec.push(Asset {
                                                                            id: assets.consume_asset("video#end#title"),
                                                                            req: 0,
                                                                            title: Some(TitleAsset {
                                                                                text: title.clone(),
                                                                                subtitle: video.desc.clone(),
                                                                                desc: None,
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
                                                            }
                                                        },
                                                        None => (),
                                                    }

                                                    match &bid.creative.cta {
                                                        Some(cta) => {
                                                            asset_vec.push(Asset {
                                                                id: assets.consume_asset("data#ctatext"),
                                                                req: 1,
                                                                title: None,
                                                                img: None,
                                                                video: None,
                                                                data: Some(DataAsset {
                                                                    value: cta.clone(),
                                                                    len: None,
                                                                    datatype: Some(12),
                                                                }),
                                                                html: None,
                                                                app: None,
                                                            });
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
                                                                        match &app.name {
                                                                            Some(name) => name.clone(),
                                                                            None => "".to_string(),
                                                                        }
                                                                    },
                                                                    desc: app.intro.clone(),
                                                                    descurl: None,
                                                                    domain: None,
                                                                    bundle: app.bundle.clone(),
                                                                    ver: app.ver.clone(),
                                                                    developer: app.developer.clone(),
                                                                    icon: app.icon.clone(),
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
                                                                    registration: None,
                                                                    privacy: None,
                                                                    privacyurl: app.privacy_url.clone(),
                                                                    permission: app.permission.clone(),
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

                                                match &bid.event_track {
                                                    Some(event_track) => {
                                                        for event in &event_track.imp_tracks {
                                                            event_vec.push(Event {
                                                                eventtype: 501,
                                                                method: 1,
                                                                url: {
                                                                    let mut url = replace_macro(event);
                                                                    url = url.replace("__bid_id__", format!("{}", request_id).as_str());
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
                                                                    url = url.replace("__bid_price__", &encode(encrypt_price.as_str()));
                                                                    url = url.replace("__bid_price_plain__", price.to_string().as_str());

                                                                    url
                                                                },
                                                                header: None,
                                                                content: None,
                                                            });
                                                        }
                                                        match &event_track.clk_tracks {
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
                                                        match &event_track.app_uninstalled {
                                                            Some(app_uninstalled) => {
                                                                for event in app_uninstalled {
                                                                    event_vec.push(Event {
                                                                        eventtype: 507,
                                                                        method: 1,
                                                                        url: replace_macro(event),
                                                                        header: None,
                                                                        content: None,
                                                                    });
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                        match &event_track.app_installed {
                                                            Some(app_installed) => {
                                                                for event in app_installed {
                                                                    event_vec.push(Event {
                                                                        eventtype: 506,
                                                                        method: 1,
                                                                        url: replace_macro(event),
                                                                        header: None,
                                                                        content: None,
                                                                    });
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                        match &event_track.dpl_try {
                                                            Some(dpl_try) => {
                                                                for event in dpl_try {
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
                                                        match &event_track.dpl_success {
                                                            Some(dpl_success) => {
                                                                for event in dpl_success {
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
                                                        match &event_track.dpl_failed {
                                                            Some(dpl_failed) => {
                                                                for event in dpl_failed {
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
                                                        match &event_track.fallback_tracks {
                                                            Some(fallback_tracks) => {
                                                                for event in fallback_tracks {
                                                                    event_vec.push(Event {
                                                                        eventtype: 508,
                                                                        method: 1,
                                                                        url: replace_macro(event),
                                                                        header: None,
                                                                        content: None,
                                                                    });
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                        match &event_track.dl_tracks {
                                                            Some(dl_tracks) => {
                                                                for event in dl_tracks {
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
                                                        match &event_track.install_tracks {
                                                            Some(install_tracks) => {
                                                                for event in install_tracks {
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
                                                        match &event_track.dl_start_tracks {
                                                            Some(dl_start_tracks) => {
                                                                for event in dl_start_tracks {
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
                                                        match &event_track.install_start_tracks {
                                                            Some(install_start_tracks) => {
                                                                for event in install_start_tracks {
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
                                                        match &event_track.progress_tracks {
                                                            Some(progress_tracks) => {
                                                                match &progress_tracks.start {
                                                                    Some(start) => {
                                                                        for event in start {
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
                                                                match &progress_tracks.first_quartile {
                                                                    Some(first_quartile) => {
                                                                        for event in first_quartile {
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
                                                                match &progress_tracks.mid_point {
                                                                    Some(mid_point) => {
                                                                        for event in mid_point {
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
                                                                match &progress_tracks.third_quartile {
                                                                    Some(third_quartile) => {
                                                                        for event in third_quartile {
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
                                                                match &progress_tracks.complete {
                                                                    Some(complete) => {
                                                                        for event in complete {
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
                                                        match &event_track.autoplay {
                                                            Some(autoplay) => {
                                                                for event in autoplay {
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
                                                        match &event_track.video_pause {
                                                            Some(video_pause) => {
                                                                for event in video_pause {
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
                                                        match &event_track.video_resume {
                                                            Some(video_resume) => {
                                                                for event in video_resume {
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
                                                        match &event_track.skip_tracks {
                                                            Some(skip_tracks) => {
                                                                for event in skip_tracks {
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
                                                        match &event_track.stop_tracks {
                                                            Some(stop_tracks) => {
                                                                for event in stop_tracks {
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
                                                    },
                                                    None => (),
                                                }

                                                event_vec
                                            }
                                        },
                                        advertiser: {
                                            bid.source.clone()
                                        },
                                        advertisericon: {
                                            bid.source_logo.clone()
                                        },
                                    },
                                };

                                bids.push(bid);
                            }

                            bids
                        }
                    };

                    seatbids.push(seatbid);
                }

                Some(seatbids)
            },
        };

        Ok(response)
    }

    async fn bidding_notify_win(url: String, win_price: i32, _next_price: i32, iv: &String, connection: &Connection, pool: &HttpPool) {
        let encrypt_price = Self::encrypt_price(win_price, iv, connection);
        let replaced_url = url
            .replace("__WIN_PRICE__", &encode(encrypt_price.as_str()));

        let client = {
            let pool_billowlink_lock = pool.pool_billowlink.clone();
            let pool_billowlink = pool_billowlink_lock.read().unwrap();
            pool_billowlink.clone()
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

    fn encrypt_price(price: i32, iv: &String, connection: &Connection) -> String {
        let ekey = connection.client_ekey.as_bytes();
        let ikey = connection.client_ikey.as_bytes();

        let price_bytes = u64::to_be_bytes(price as u64).to_vec();
        let iv_bytes = match iv.parse::<u128>() {
            Ok(number) => u128::to_be_bytes(number),
            Err(_) => u128::to_be_bytes(0)
        };

        let mut price_pad: Vec<u8> = [].to_vec();
        let mac_ekey = HmacSha1::new_from_slice(&ekey);
        match mac_ekey {
            Ok(mut mac) => {
                mac.update(&iv_bytes.to_vec());
                price_pad = mac.finalize().into_bytes().to_vec();
            },
            Err(_) => (),
        }

        let enc_price: Vec<u8> = price_bytes.iter()
            .zip(price_pad[0..8].iter())
            .map(|(&x1, &x2)| x1 ^ x2)
            .collect();

        let mut sig = [].to_vec();
        let mac_ikey = HmacSha1::new_from_slice(&ikey);
        match mac_ikey {
            Ok(mut mac) => {
                mac.update(&[price_bytes.to_vec(), iv_bytes.to_vec()].concat());
                sig = mac.finalize().into_bytes().to_vec();
            },
            Err(_) => (),
        }
        let signature = sig[0..4].to_vec();

        BASE64_STANDARD.encode(&[iv_bytes.to_vec(), enc_price.to_vec(), signature.to_vec()].concat())
    }

}

fn replace_macro(orig: &String) -> String {
    let mut replaced = orig.clone();

    replaced = replaced.replace("__down_x__", "__R_DOWN_X__");
    replaced = replaced.replace("__down_y__", "__R_DOWN_Y__");
    replaced = replaced.replace("__up_x__", "__R_UP_X__");
    replaced = replaced.replace("__up_y__", "__R_UP_Y__");
    replaced = replaced.replace("__clk_time__", "__CLICK_TIME__");
    replaced = replaced.replace("__gdt_click_id__", "__CLICK_ID__");
    replaced = replaced.replace("__ts__", "__EVENT_TIME_START_S__");
    replaced = replaced.replace("__ts_end__", "__EVENT_TIME_END_S__");
    replaced = replaced.replace("__tms__", "__EVENT_TIME_START__");
    replaced = replaced.replace("__tms_end__", "__EVENT_TIME_END__");
    replaced = replaced.replace("__video_duration__", "__VIDEO_TIME__");
    replaced = replaced.replace("__play_sec__", "__VIDEO_PLAY_PROGRESS_S__");
    replaced = replaced.replace("__play_msec__", "__VIDEO_PLAY_PROGRESS__MS__");
    replaced = replaced.replace("__down_x_sc__", "__ABS_DOWN_X__");
    replaced = replaced.replace("__down_y_sc__", "__ABS_DOWN_Y__");
    replaced = replaced.replace("__up_x_sc__", "__ABS_UP_X__");
    replaced = replaced.replace("__up_y_sc__", "__ABS_UP_Y__");
    replaced = replaced.replace("__dp_down_x__", "__R_DOWN_DP_X__");
    replaced = replaced.replace("__dp_down_y__", "__R_DOWN_DP_Y__");
    replaced = replaced.replace("__dp_up_x__", "__R_UP_DP_X__");
    replaced = replaced.replace("__dp_up_y__", "__R_UP_DP_Y__");
    replaced = replaced.replace("__ad_width__", "__WIDTH__");
    replaced = replaced.replace("__ad_height__", "__HEIGHT__");
    replaced = replaced.replace("__ad_width_dp__", "__DP_WIDTH__");
    replaced = replaced.replace("__dp_height_dp__", "__DP_HEIGHT__");
    replaced = replaced.replace("__bl_sld__", "__SLD__");
    replaced = replaced.replace("__bl_xmax_acc__", "__X_MAX_ACC__");
    replaced = replaced.replace("__bl_ymax_acc__", "__Y_MAX_ACC__");
    replaced = replaced.replace("__bl_zmax_acc__", "__Z_MAX_ACC__");
    replaced = replaced.replace("__bl_turn_x__", "__TURN_X__");
    replaced = replaced.replace("__bl_turn_y__", "__TURN_Y__");
    replaced = replaced.replace("__bl_turn_z__", "__TURN_Z__");
    replaced = replaced.replace("__bl_turn_time__", "__TURN_TIME__");
    replaced = replaced.replace("__bl_up_time__", "__UP_TS__");
    replaced = replaced.replace("__bl_down_time__", "__DOWN_TS__");

    replaced
}
