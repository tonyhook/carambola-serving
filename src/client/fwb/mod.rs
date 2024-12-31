use std::time::Duration;

use base64::prelude::*;
use chrono::{Datelike, Local};
use hmac::{Hmac, Mac};
use sha1::Sha1;
use urlencoding::encode;

use crate::{protocol::*, Assets, Cache, Client, Connection, HttpPool, Identifiers, Price, ResultMessage};

type HmacSha1 = Hmac<Sha1>;

pub mod admobject;
pub mod app_asset;
pub mod app;
pub mod asset_format;
pub mod asset;
pub mod bid;
pub mod button;
pub mod card;
pub mod data_asset;
pub mod data_format;
pub mod deal;
pub mod device;
pub mod events;
pub mod geo;
pub mod img_asset;
pub mod img_format;
pub mod imp;
pub mod native_format;
pub mod native_video_asset;
pub mod native_video_format;
pub mod native_asset;
pub mod pmp;
pub mod request;
pub mod response;
pub mod seatbid;
pub mod title_asset;
pub mod title_format;
pub mod user;
pub mod video_asset;
pub mod video_format;

pub use admobject::FwbAdmobject;
pub use app_asset::FwbAppAsset;
pub use app::FwbApp;
pub use asset_format::FwbAssetFormat;
pub use asset::FwbAsset;
pub use bid::FwbBid;
pub use button::FwbButton;
pub use card::FwbCard;
pub use data_asset::FwbDataAsset;
pub use data_format::FwbDataFormat;
pub use deal::FwbDeal;
pub use device::FwbDevice;
pub use events::FwbEvents;
pub use geo::FwbGeo;
pub use img_asset::FwbImgAsset;
pub use img_format::FwbImgFormat;
pub use imp::FwbImp;
pub use native_asset::FwbNativeAsset;
pub use native_format::FwbNativeFormat;
pub use native_video_asset::FwbNativeVideoAsset;
pub use native_video_format::FwbNativeVideoFormat;
pub use pmp::FwbPmp;
pub use request::FwbRequest;
pub use response::FwbResponse;
pub use seatbid::FwbSeatbid;
pub use title_asset::FwbTitleAsset;
pub use title_format::FwbTitleFormat;
pub use user::FwbUser;
pub use video_asset::FwbVideoAsset;
pub use video_format::FwbVideoFormat;

pub struct Fwb {

    // TODO: step_play_urls is not available

}

impl Client for Fwb {

    async fn request(request: &Request, connection: &Connection, pool: &HttpPool, cache: &Cache) -> Result<Response, ResultMessage> {
        let request_id = cache.get_sequence();

        let assets = Assets::new(request);
        let identifiers = Identifiers::new(request);

        let request_fwb = FwbRequest {
            id: {
                request_id.to_string()
            },
            version: {
                "2.0".to_string()
            },
            imp: [FwbImp {
                id: {
                    request_id.to_string()
                },
                tagid: {
                    connection.client_tag_id.clone()
                },
                bidfloor: {
                    Some(Price::to_client(connection, request.item[0].flr) as f64 / 100.0)
                },
                native: {
                    let mut fullscreen_video = false;
                    match &request.item[0].spec.display.nativefmt {
                        Some(nativefmt) => {
                            for asset in &nativefmt.asset {
                                match &asset.video {
                                    Some(video) => {
                                        match video.videotype {
                                          Some(3) => {
                                            fullscreen_video = true;
                                            break;
                                          },
                                          _ => (),
                                        }
                                    },
                                    None => (),
                                }
                            }
                        },
                        None => (),
                    }
                    if !fullscreen_video {
                        let mut native = FwbNativeFormat {
                            assets: {
                                let mut assets_fwb = vec![];

                                match &request.item[0].spec.display.displayfmt {
                                    Some(displayfmt) => {
                                        let asset_fwb = FwbAssetFormat {
                                            id: 1,
                                            isrequired: 1,
                                            title: None,
                                            img: Some(FwbImgFormat {
                                                imagetype: 3,
                                                wmin: {
                                                    match displayfmt.w {
                                                        Some(w) => w,
                                                        None => 0,
                                                    }
                                                },
                                                hmin: {
                                                    match displayfmt.h {
                                                        Some(h) => h,
                                                        None => 0,
                                                    }
                                                },
                                                mimes: None,
                                            }),
                                            video: None,
                                            data: None,
                                        };

                                        assets_fwb.push(asset_fwb);
                                    },
                                    None => (),
                                }
                                match &request.item[0].spec.display.nativefmt {
                                    Some(nativefmt) => {
                                        for asset in &nativefmt.asset {
                                            let mut asset_fwb = FwbAssetFormat {
                                                id: asset.id,
                                                isrequired: asset.req,
                                                title: None,
                                                img: None,
                                                video: None,
                                                data: None,
                                            };

                                            match &asset.title {
                                                Some(title) => {
                                                    asset_fwb.title = Some(FwbTitleFormat {
                                                        len: title.len,
                                                    });
                                                },
                                                None => (),
                                            }

                                            match &asset.img {
                                                Some(img) => {
                                                    asset_fwb.img = Some(FwbImgFormat {
                                                        imagetype: {
                                                            match img.imagetype {
                                                                Some(1) => 1,
                                                                Some(3) => 3,
                                                                Some(501) => 2,
                                                                _ => 3,
                                                            }
                                                        },
                                                        wmin: {
                                                            match img.wmin {
                                                                Some(wmin) => wmin,
                                                                None => 0,
                                                            }
                                                        },
                                                        hmin: {
                                                            match img.hmin {
                                                                Some(hmin) => hmin,
                                                                None => 0,
                                                            }
                                                        },
                                                        mimes: {
                                                            img.mime.clone()
                                                        },
                                                    });
                                                },
                                                None => (),
                                            }

                                            match &asset.video {
                                                Some(video) => {
                                                    match video.videotype {
                                                        Some(1) => {
                                                            asset_fwb.video = Some(FwbNativeVideoFormat {
                                                                wmin: {
                                                                    match video.w {
                                                                        Some(w) => w,
                                                                        None => 0,
                                                                    }
                                                                },
                                                                hmin: {
                                                                    match video.h {
                                                                        Some(h) => h,
                                                                        None => 0,
                                                                    }
                                                                },
                                                                mimes: {
                                                                    video.mime.clone()
                                                                },
                                                            });
                                                        },
                                                        _ => (),
                                                    }
                                                },
                                                None => (),
                                            }

                                            match &asset.data {
                                                Some(data) => {
                                                    asset_fwb.data = Some(FwbDataFormat {
                                                        datatype: {
                                                            match data.datatype {
                                                                2 => 1,
                                                                1 => 11,
                                                                12 => 12,
                                                                _ => 1,
                                                            }
                                                        },
                                                        len: {
                                                            data.len
                                                        },
                                                    });
                                                },
                                                None => (),
                                            }

                                            assets_fwb.push(asset_fwb);
                                        }
                                    },
                                    None => (),
                                }

                                assets_fwb
                            },
                            layout: {
                                let mut layout = 3;

                                if request.item[0].spec.display.displayfmt.is_some() {
                                    if request.item[0].spec.display.instl == 0 {
                                        layout = 501;
                                    } else {
                                        layout = 502;
                                    }
                                }

                                layout
                            },
                        };

                        if native.layout == 3 {
                            let mut main = false;
                            for asset in &native.assets {
                                match &asset.img {
                                    Some(img) => {
                                        if img.imagetype == 3 {
                                            main = true;
                                        }
                                    },
                                    None => (),
                                }
                            }

                            if !main {
                                let mut max_size = -1;
                                let mut index = 0;
                                for (i, asset) in native.assets.iter().enumerate() {
                                    match &asset.img {
                                        Some(img) => {
                                            if img.wmin * img.hmin > max_size {
                                                max_size = img.wmin * img.hmin;
                                                index = i;
                                            }
                                        },
                                        None => (),
                                    }
                                }
                                if let Some(img) = native.assets.get_mut(index).and_then(|a| a.img.as_mut()) {
                                    img.imagetype = 3;
                                }
                            }
                        }

                        Some(native)
                    } else {
                        None
                    }
                },
                video: {
                    let mut fullscreen_video = None;
                    match &request.item[0].spec.display.nativefmt {
                        Some(nativefmt) => {
                            for asset in &nativefmt.asset {
                                match &asset.video {
                                    Some(video) => {
                                        match video.videotype {
                                            Some(3) => {
                                                fullscreen_video = asset.video.clone();
                                                break;
                                            },
                                            _ => (),
                                        }
                                    },
                                    None => ()
                                }
                            }
                        },
                        None => (),
                    }
                    match fullscreen_video {
                        Some(fullscreen_video) => {
                            Some(FwbVideoFormat {
                                w: {
                                    fullscreen_video.w.unwrap_or(0)
                                },
                                h: {
                                    fullscreen_video.h.unwrap_or(0)
                                },
                                videotype: {
                                    3
                                },
                                minduration: fullscreen_video.mindur,
                                maxduration: fullscreen_video.maxdur,
                                startdelay: {
                                    match fullscreen_video.delay {
                                        Some(0) => Some(0),
                                        Some(-1) => Some(1),
                                        Some(-2) => Some(2),
                                        _ => None,
                                    }
                                },
                                mime: {
                                    match fullscreen_video.mime {
                                        Some(mime) => mime,
                                        None => ["video/mp4".to_string()].to_vec(),
                                    }
                                },
                                orientation: {
                                    None
                                },
                                delivery: {
                                    fullscreen_video.delivery
                                },
                            })
                        },
                        None => None,
                    }
                },
                isdeeplink: {
                    Some(true)
                },
                isdownload: {
                    Some(true)
                },
                isul: {
                    None
                },
                secure: {
                    None
                },
                pmp: {
                    None
                },
            }].to_vec(),
            app: {
                match &request.context.app {
                    Some(app) => FwbApp {
                        bundle: {
                            match &app.bundle {
                                Some(bundle) => bundle.clone(),
                                None => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.context.app.bundle is required for upstream".to_string(),
                                }),
                            }
                        },
                        name: {
                            Some(app.name.clone())
                        },
                        version: {
                            app.ver.clone()
                        },
                        cat: {
                            None
                        },
                        keywords: {
                            None
                        },
                    },
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.app is required for upstream".to_string(),
                    }),
                }
            },
            device: FwbDevice {
                os: {
                    match &request.context.device.os {
                        Some(2) => "android".to_string(),
                        Some(13) => "ios".to_string(),
                        _ => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.os should be 2/13 for upstream".to_string(),
                        }),
                    }
                },
                osv: {
                    request.context.device.osv.clone()
                },
                did: {
                    match identifiers.get_id(501, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                didmd5: {
                    match identifiers.get_id(502, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                oid: {
                    match identifiers.get_id(505, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                oidmd5: {
                    match identifiers.get_id(506, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                androidid: {
                    match identifiers.get_id(509, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                androididmd5: {
                    match identifiers.get_id(510, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                ifa: {
                    match identifiers.get_id(507, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                ifamd5: {
                    match identifiers.get_id(508, 0) {
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
                aaid: {
                    match identifiers.get_id(514, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                openudid: None,
                idfv: {
                    match identifiers.get_id(515, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                mac: {
                    match identifiers.get_id(511, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                macidmd5: {
                    match identifiers.get_id(512, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
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
                ipv6: request.context.device.ipv6.clone(),
                ua: request.context.device.ua.clone(),
                connectiontype: {
                    match request.context.device.contype {
                        Some(contype) => contype,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.contype is required for upstream".to_string(),
                        }),
                    }
                },
                devicetype: {
                    match request.context.device.devicetype {
                        Some(devicetype) => devicetype,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.devicetype is required for upstream".to_string(),
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
                carrier: {
                    match &request.context.device.carrier {
                        Some(carrier) => {
                            match carrier.as_str() {
                                "cmcc" => Some("mobile".to_string()),
                                "unicom" => Some("unicom".to_string()),
                                "telecom" => Some("telecom".to_string()),
                                _ => Some("unknown".to_string()),
                            }
                        },
                        None => None,
                    }
                },
                flashver: {
                    None
                },
                screenheight: {
                    match request.context.device.h {
                        Some(h) => h,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.h is required for upstream".to_string(),
                        }),
                    }
                },
                screenwidth: {
                    match request.context.device.w {
                        Some(w) => w,
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.w is required for upstream".to_string(),
                        }),
                    }
                },
                orientation: {
                    match request.context.device.orientation {
                        Some(501) => Some(2),
                        Some(502) => Some(1),
                        _ => None,
                    }
                },
                dpi: {
                    request.context.device.ppi.clone()
                },
                density: {
                    request.context.device.pxratio.clone()
                },
                ppi: {
                    request.context.device.ppi.clone()
                },
                geo: {
                    match &request.context.device.geo {
                        Some(geo) => {
                            Some(FwbGeo {
                                lat: geo.lat,
                                lon: geo.lon,
                                coordinate: geo.coordinate,
                                timestamp: geo.timestamp,
                                accu: {
                                    match geo.accur {
                                        Some(accur) => Some(accur as f64),
                                        None => None,
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
                elapsetime: {
                    None
                },
                romversion: {
                    request.context.device.romv.clone()
                },
                syscompilingtime: {
                    request.context.device.romtime.clone()
                },
                boot_mark: {
                    request.context.device.bootmark.clone()
                },
                update_mark: {
                    request.context.device.updatemark.clone()
                },
                ag: {
                    request.context.device.storev.clone()
                },
                hms: {
                    request.context.device.hmsv.clone()
                },
                wifi_mac: {
                    match identifiers.get_id(522, 0) {
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
                ssid: {
                    match identifiers.get_id(524, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                udid: {
                    None
                },
                paid: {
                    match identifiers.get_id(519, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                birth_time: {
                    match &request.context.device.inittime {
                        Some(inittime) => Some(inittime.clone()),
                        None => {
                            match &request.context.device.birthtime {
                                Some(birthtime) => Some(birthtime.clone()),
                                None => None,
                            }
                        },
                    }
                },
                start_time_msec: {
                    request.context.device.boottime.clone()
                },
                update_time_nsec: {
                    request.context.device.updatetime.clone()
                },
                language: {
                    request.context.device.lang.clone()
                },
                hardware_model: {
                    request.context.device.hwmodel.clone()
                },
                country: {
                    request.context.device.country.clone()
                },
                local_tz_time: {
                    request.context.device.timezone.clone()
                },
                device_name_md5: {
                    match identifiers.get_id(528, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                cpu_num: {
                    request.context.device.syscpu
                },
                disk_total: {
                    request.context.device.sysdisksize
                },
                mem_total: {
                    request.context.device.sysmemory
                },
                auth_status: {
                    request.context.device.lmt
                },
                skadnetwork_versions: {
                    request.context.device.skan.clone()
                },
                miuiversion: {
                    request.context.device.uiv.clone()
                },
            },
            user: FwbUser {
                id: request.context.user.id.clone(),
                gender: request.context.user.gender.clone(),
                yob: {
                    match request.context.user.yob {
                        Some(yob) => {
                            let year = Local::now().year();
                            Some(year - yob)
                        },
                        None => Some(0),
                    }
                },
                keywords: request.context.user.keywords.clone(),
            },
        };

        let response_fwb: FwbResponse;

        let client = {
            let pool_fwb_lock = pool.pool_fwb.clone();
            let pool_fwb = pool_fwb_lock.read().unwrap();
            pool_fwb.clone()
        };
        let response_fwb_raw = client.post("https://req.adx.xlqeai.com/awesome")
            .json(&request_fwb)
            .header("Accept-Encoding", "gzip")
            .header("Connection", "keep-alive")
            .header("Content-Type", "application/json")
            .timeout(Duration::from_millis(connection.timeout))
            .send().await;

        match response_fwb_raw {
            Ok(response_fwb_raw) => {
                let status = response_fwb_raw.status();
                if status == 204 {
                    return Err(ResultMessage {
                        code: 993,
                        message: "".to_string(),
                    });
                } else if status != 200 {
                    return Err(ResultMessage {
                        code: 992,
                        message: {
                            match response_fwb_raw.text().await {
                                Ok(text) => format!("upstream error {}: {}", status, text),
                                Err(_) => format!("upstream error {}", status),
                            }
                        },
                    });
                } else {
                    match response_fwb_raw.text().await {
                        Ok(text) => {
                            match serde_json::from_str::<FwbResponse>(&text) {
                                Ok(json) => {
                                    response_fwb = json;
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
                let mut bids = vec![];

                for bid_fwb in &response_fwb.seatbid.bid {
                    let mut link_asset = LinkAsset {
                        linktype: {
                            match bid_fwb.action.clone() {
                                Some(action) => {
                                    match action.as_str() {
                                        "1" => 1,
                                        "2" => 2,
                                        _ => 1,
                                    }
                                },
                                None => 1,
                            }
                        },
                        universallink: {
                            match &bid_fwb.universal_link {
                                Some(universal_link) => Some(universal_link.clone()),
                                None => None,
                            }
                        },
                        storeid: {
                            None
                        },
                        deeplink: {
                            match &bid_fwb.deeplink {
                                Some(deeplink) => Some(deeplink.clone()),
                                None => None,
                            }
                        },
                        quickapplink: {
                            match &bid_fwb.universal_link {
                                Some(universal_link) => Some(universal_link.clone()),
                                None => None,
                            }
                        },
                        wechatmppath: {
                            match &bid_fwb.wxapppath {
                                Some(wxapppath) => Some(wxapppath.clone()),
                                None => None,
                            }
                        },
                        wechatmpid: {
                            match &bid_fwb.wxappid {
                                Some(wxappid) => Some(wxappid.clone()),
                                None => None,
                            }
                        },
                        marketurl: {
                            match &bid_fwb.market_url {
                                Some(market_url) => Some(market_url.clone()),
                                None => None,
                            }
                        },
                        downloadurl: {
                            match &bid_fwb.download {
                                Some(download) => Some(download.clone()),
                                None => None,
                            }
                        },
                        url: {
                            match &bid_fwb.target {
                                Some(target) => replace_macro(target),
                                None => "".to_string(),
                            }
                        },
                        urlfb: None,
                    };

                    let bid = Bid {
                        id: Some(request_id.to_string()),
                        item: request.item[0].id.clone(),
                        price: { // update later
                            match bid_fwb.price {
                                Some(price) => {
                                    if price > 0.0 {
                                        price as i32
                                    } else {
                                        connection.default_price
                                    }
                                },
                                None => connection.default_price,
                            }
                        },
                        burl: {
                            match &bid_fwb.nurl {
                                Some(nurl) => {
                                    let mut burl = Vec::<String>::new();
                                    let mut nurl = nurl.clone();
                                    nurl = nurl.replace("__WINPRICE__", "__WIN_PRICE__");
                                    burl.push(replace_macro(&nurl));
                                    Some(burl)
                                },
                                None => None,
                            }
                        },
                        lurl: None,
                        media: Ad {
                            id: response_fwb.id.clone(),
                            display: {
                                let mut display = Display {
                                    w: None,
                                    h: None,
                                    banner: None,
                                    native: None,
                                    event: vec![],
                                };

                                let mut asset_vec = vec![];

                                match &bid_fwb.admobject {
                                    Some(admobject) => {
                                        match &admobject.native {
                                            Some(native) => {
                                                match &native.assets {
                                                    Some(assets_fwb) => {
                                                        for asset_fwb in assets_fwb {
                                                            if request.item[0].spec.display.displayfmt.is_some() {
                                                                match &asset_fwb.img {
                                                                    Some(img_fwb) => {
                                                                        display.banner = Some(Banner {
                                                                            img: img_fwb.url.clone(),
                                                                            link: Some(link_asset.clone()),
                                                                        });
                                                                        break;
                                                                    },
                                                                    None => (),
                                                                }
                                                            }
                                                            if request.item[0].spec.display.nativefmt.is_some() {
                                                                match &asset_fwb.title {
                                                                    Some(title_fwb) => {
                                                                        let asset = Asset {
                                                                            id: asset_fwb.id,
                                                                            req: asset_fwb.isrequired,
                                                                            title: Some(TitleAsset {
                                                                                text: title_fwb.text.clone(),
                                                                                subtitle: None,
                                                                                desc: None,
                                                                                len: None,
                                                                            }),
                                                                            img: None,
                                                                            video: None,
                                                                            data: None,
                                                                            html: None,
                                                                            app: None,
                                                                        };

                                                                        asset_vec.push(asset);
                                                                    },
                                                                    None => (),
                                                                }
                                                                match &asset_fwb.img {
                                                                    Some(img_fwb) => {
                                                                        let asset = Asset {
                                                                            id: asset_fwb.id,
                                                                            req: asset_fwb.isrequired,
                                                                            title: None,
                                                                            img: Some(ImageAsset {
                                                                                url: img_fwb.url.clone(),
                                                                                mime: None,
                                                                                w: img_fwb.w,
                                                                                h: img_fwb.h,
                                                                                imagetype: {
                                                                                    match img_fwb.imagetype {
                                                                                        Some(1) => Some(1),
                                                                                        Some(2) => Some(501),
                                                                                        Some(3) => Some(2),
                                                                                        _ => Some(3),
                                                                                    }
                                                                                },
                                                                            }),
                                                                            video: None,
                                                                            data: None,
                                                                            html: None,
                                                                            app: None,
                                                                        };

                                                                        asset_vec.push(asset);
                                                                    },
                                                                    None => (),
                                                                }
                                                                match &asset_fwb.video {
                                                                    Some(video_fwb) => {
                                                                        let asset = Asset {
                                                                            id: asset_fwb.id,
                                                                            req: asset_fwb.isrequired,
                                                                            title: None,
                                                                            img: None,
                                                                            video: Some(VideoAsset {
                                                                                url: video_fwb.url.clone(),
                                                                                mime: None,
                                                                                w: None,
                                                                                h: None,
                                                                                dur: video_fwb.duration,
                                                                                size: None,
                                                                                skipoffset: None,
                                                                                delivery: None,
                                                                                orientation: None,
                                                                                autolanding: 0,
                                                                                clickable: 0,
                                                                            }),
                                                                            data: None,
                                                                            html: None,
                                                                            app: None,
                                                                        };

                                                                        asset_vec.push(asset);

                                                                        match &video_fwb.cover {
                                                                            Some(cover) => {
                                                                                let asset = Asset {
                                                                                    id: asset_fwb.id,
                                                                                    req: asset_fwb.isrequired,
                                                                                    title: None,
                                                                                    img: Some(ImageAsset {
                                                                                        url: cover.clone(),
                                                                                        mime: None,
                                                                                        w: None,
                                                                                        h: None,
                                                                                        imagetype: Some(3),
                                                                                    }),
                                                                                    video: None,
                                                                                    data: None,
                                                                                    html: None,
                                                                                    app: None,
                                                                                };

                                                                                asset_vec.push(asset);
                                                                            },
                                                                            None => (),
                                                                        }
                                                                    },
                                                                    None => (),
                                                                }
                                                                match &asset_fwb.data {
                                                                    Some(data_fwb) => {
                                                                        let asset = Asset {
                                                                            id: asset_fwb.id,
                                                                            req: asset_fwb.isrequired,
                                                                            title: None,
                                                                            img: None,
                                                                            video: None,
                                                                            data: Some(DataAsset {
                                                                                value: data_fwb.value.clone(),
                                                                                len: None,
                                                                                datatype: None,
                                                                            }),
                                                                            html: None,
                                                                            app: None,
                                                                        };

                                                                        asset_vec.push(asset);
                                                                    },
                                                                    None => (),
                                                                }
                                                            }
                                                        }
                                                    },
                                                    None => (),
                                                }
                                            },
                                            None => (),
                                        }
                                        match &admobject.video {
                                            Some(video) => {
                                                let asset = Asset {
                                                    id: 1,
                                                    req: 1,
                                                    title: None,
                                                    img: None,
                                                    video: Some(VideoAsset {
                                                        url: video.url.clone(),
                                                        mime: {
                                                            match &video.mimes {
                                                                Some(mimes) => {
                                                                    if mimes.len() > 0 {
                                                                        Some(mimes.get(0).unwrap().clone())
                                                                    } else {
                                                                        None
                                                                    }
                                                                },
                                                                None => None,
                                                            }
                                                        },
                                                        w: video.w,
                                                        h: video.h,
                                                        dur: video.duration,
                                                        size: video.size,
                                                        skipoffset: video.skip_min_time,
                                                        delivery: None,
                                                        orientation: None,
                                                        autolanding: 0,
                                                        clickable: 0,
                                                    }),
                                                    data: None,
                                                    html: None,
                                                    app: None,
                                                };

                                                asset_vec.push(asset);

                                                let mut index = 1;

                                                match &video.card {
                                                    Some(card) => {
                                                        match card.cardtype {
                                                            Some(1) => {
                                                                match &card.url {
                                                                    Some(url) => {
                                                                        let asset = Asset {
                                                                            id: { index += 1; index },
                                                                            req: 1,
                                                                            title: None,
                                                                            img: Some(ImageAsset {
                                                                                url: url.clone(),
                                                                                mime: None,
                                                                                w: None,
                                                                                h: None,
                                                                                imagetype: Some(3),
                                                                            }),
                                                                            video: None,
                                                                            data: None,
                                                                            html: None,
                                                                            app: None,
                                                                        };

                                                                        asset_vec.push(asset);
                                                                    },
                                                                    None => (),
                                                                }
                                                            },
                                                            Some(2) => {
                                                                match &card.url {
                                                                    Some(url) => {
                                                                        let asset = Asset {
                                                                            id: { index += 1; index },
                                                                            req: 1,
                                                                            title: None,
                                                                            img: None,
                                                                            video: None,
                                                                            data: Some(DataAsset {
                                                                                value: url.clone(),
                                                                                len: None,
                                                                                datatype: Some(11),
                                                                            }),
                                                                            html: None,
                                                                            app: None,
                                                                        };

                                                                        asset_vec.push(asset);
                                                                    },
                                                                    None => (),
                                                                }
                                                            },
                                                            Some(4) => {
                                                                match &card.html {
                                                                    Some(html) => {
                                                                        let asset = Asset {
                                                                            id: { index += 1; index },
                                                                            req: 1,
                                                                            title: None,
                                                                            img: None,
                                                                            video: None,
                                                                            data: None,
                                                                            html: Some(HtmlAsset {
                                                                                html: Some(html.clone()),
                                                                                link: None,
                                                                                len: None,
                                                                            }),
                                                                            app: None,
                                                                        };

                                                                        asset_vec.push(asset);
                                                                    },
                                                                    None => (),
                                                                }
                                                            },
                                                            _ => (),
                                                        }

                                                        match &card.icon {
                                                            Some(icon) => {
                                                                let asset = Asset {
                                                                    id: { index += 1; index },
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
                                                                };

                                                                asset_vec.push(asset);
                                                            },
                                                            None => (),
                                                        }
                                                        match &card.title {
                                                            Some(title) => {
                                                                let asset = Asset {
                                                                    id: { index += 1; index },
                                                                    req: 1,
                                                                    title: Some(TitleAsset {
                                                                        text: title.clone(),
                                                                        subtitle: None,
                                                                        desc: None,
                                                                        len: None,
                                                                    }),
                                                                    img: None,
                                                                    video: None,
                                                                    data: None,
                                                                    html: None,
                                                                    app: None,
                                                                };

                                                                asset_vec.push(asset);
                                                            },
                                                            None => (),
                                                        }
                                                        match &card.content {
                                                            Some(content) => {
                                                                let asset = Asset {
                                                                    id: { index += 1; index },
                                                                    req: 1,
                                                                    title: None,
                                                                    img: None,
                                                                    video: None,
                                                                    data: Some(DataAsset {
                                                                        value: content.clone(),
                                                                        len: None,
                                                                        datatype: Some(2),
                                                                    }),
                                                                    html: None,
                                                                    app: None,
                                                                };

                                                                asset_vec.push(asset);
                                                            },
                                                            None => (),
                                                        }
                                                        match &card.rating {
                                                            Some(rating) => {
                                                                let asset = Asset {
                                                                    id: { index += 1; index },
                                                                    req: 1,
                                                                    title: None,
                                                                    img: None,
                                                                    video: None,
                                                                    data: Some(DataAsset {
                                                                        value: rating.to_string(),
                                                                        len: None,
                                                                        datatype: Some(3),
                                                                    }),
                                                                    html: None,
                                                                    app: None,
                                                                };

                                                                asset_vec.push(asset);
                                                            },
                                                            None => (),
                                                        }
                                                        match &card.button {
                                                            Some(button) => {
                                                                match &button.url {
                                                                    Some(url) => {
                                                                        link_asset.url = url.clone();
                                                                    },
                                                                    None => (),
                                                                }

                                                                let asset = Asset {
                                                                    id: { index += 1; index },
                                                                    req: 1,
                                                                    title: None,
                                                                    img: None,
                                                                    video: None,
                                                                    data: Some(DataAsset {
                                                                        value: {
                                                                            match &button.text {
                                                                                Some(text) => {
                                                                                    text.clone()
                                                                                },
                                                                                None => {
                                                                                    match &bid_fwb.action {
                                                                                        Some(action) => {
                                                                                            match action.as_str() {
                                                                                                "1" => "查看详情".to_string(),
                                                                                                "2" => "免费下载".to_string(),
                                                                                                _ => "查看详情".to_string(),
                                                                                            }
                                                                                        },
                                                                                        None => "查看详情".to_string(),
                                                                                    }
                                                                                }
                                                                            }
                                                                        },
                                                                        len: None,
                                                                        datatype: Some(12),
                                                                    }),
                                                                    html: None,
                                                                    app: None,
                                                                };

                                                                asset_vec.push(asset);
                                                            },
                                                            None => (),
                                                        }
                                                        match card.comments {
                                                            Some(comments) => {
                                                                let asset = Asset {
                                                                    id: { index += 1; index },
                                                                    req: 1,
                                                                    title: None,
                                                                    img: None,
                                                                    video: None,
                                                                    data: Some(DataAsset {
                                                                        value: comments.to_string(),
                                                                        len: None,
                                                                        datatype: Some(501),
                                                                    }),
                                                                    html: None,
                                                                    app: None,
                                                                };

                                                                asset_vec.push(asset);
                                                            },
                                                            None => (),
                                                        }
                                                    },
                                                    None => (),
                                                }
                                            },
                                            None => (),
                                        }
                                    },
                                    None => (),
                                }

                                match &bid_fwb.app {
                                    Some(app) => {
                                        let asset = Asset {
                                            id:(assets.asset_size + 1) as i32,
                                            req: 0,
                                            title: None,
                                            img: None,
                                            video: None,
                                            data: None,
                                            html: None,
                                            app: Some(AppAsset {
                                                name: app.name.clone().unwrap_or("".to_string()),
                                                domain: None,
                                                bundle: app.package.clone(),
                                                ver: app.version.clone(),
                                                developer: app.publisher.clone(),
                                                icon: app.icon.clone(),
                                                storeid: None,
                                                storeurl: None,
                                                paid: 0,
                                                size: app.size,
                                                md5: app.md5.clone(),
                                                privacy: app.privacy_link.clone(),
                                                permission: app.permission_link.clone(),
                                            }),
                                        };

                                        asset_vec.push(asset);
                                    },
                                    None => (),
                                }

                                if request.item[0].spec.display.nativefmt.is_some() {
                                    display.native = Some(Native {
                                        asset: asset_vec,
                                        link: Some(link_asset.clone()),
                                    });
                                }

                                let mut event_vec = vec![];

                                for event in &bid_fwb.events.imp_urls {
                                    event_vec.push(Event {
                                        eventtype: 501,
                                        method: 1,
                                        url: replace_macro(event),
                                        header: None,
                                        content: None,
                                    });
                                }
                                for event in &bid_fwb.events.click_urls {
                                    event_vec.push(Event {
                                        eventtype: 502,
                                        method: 1,
                                        url: replace_macro(event),
                                        header: None,
                                        content: None,
                                    });
                                }
                                match &bid_fwb.events.start_dod_urls {
                                    Some(start_dod_urls) => {
                                        for event in start_dod_urls {
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
                                match &bid_fwb.events.finish_dod_urls {
                                    Some(finish_dod_urls) => {
                                        for event in finish_dod_urls {
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
                                match &bid_fwb.events.start_install_urls {
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
                                match &bid_fwb.events.finish_install_urls {
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
                                match &bid_fwb.events.active_urls {
                                    Some(active_urls) => {
                                        for event in active_urls {
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
                                match &bid_fwb.events.start_play_urls {
                                    Some(start_play_urls) => {
                                        for event in start_play_urls {
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
                                match &bid_fwb.events.start_25play_urls {
                                    Some(start_25play_urls) => {
                                        for event in start_25play_urls {
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
                                match &bid_fwb.events.start_50play_urls {
                                    Some(start_50play_urls) => {
                                        for event in start_50play_urls {
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
                                match &bid_fwb.events.start_75play_urls {
                                    Some(start_75play_urls) => {
                                        for event in start_75play_urls {
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
                                match &bid_fwb.events.pause_play_urls {
                                    Some(pause_play_urls) => {
                                        for event in pause_play_urls {
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
                                match &bid_fwb.events.replay_urls {
                                    Some(replay_urls) => {
                                        for event in replay_urls {
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
                                match &bid_fwb.events.finish_play_urls {
                                    Some(finish_play_urls) => {
                                        for event in finish_play_urls {
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
                                match &bid_fwb.events.deeplink_pre_urls {
                                    Some(deeplink_pre_urls) => {
                                        for event in deeplink_pre_urls {
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
                                match &bid_fwb.events.deeplink_urls {
                                    Some(deeplink_urls) => {
                                        for event in deeplink_urls {
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
                                match &bid_fwb.events.deeplink_furls {
                                    Some(deeplink_furls) => {
                                        for event in deeplink_furls {
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
                                match &bid_fwb.events.mute_play_urls {
                                    Some(mute_play_urls) => {
                                        for event in mute_play_urls {
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
                                match &bid_fwb.events.skip_play_urls {
                                    Some(skip_play_urls) => {
                                        for event in skip_play_urls {
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
                                match &bid_fwb.events.close_play_urls {
                                    Some(close_play_urls) => {
                                        for event in close_play_urls {
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

                                display.event = event_vec;

                                display
                            },
                        },
                    };

                    bids.push(bid);

                }

                Some([Seatbid {
                    bid: bids,
                }].to_vec())
            }
        };

        Ok(response)
    }

    async fn bidding_notify_win(url: String, win_price: i32, _next_price: i32, iv: &String, connection: &Connection, pool: &HttpPool) {
        let encrypt_price = Self::encrypt_price(win_price, iv, connection);
        let replaced_url = url
            .replace("__WIN_PRICE__", &encode(encrypt_price.as_str()));

        let client = {
            let pool_fwb_lock = pool.pool_fwb.clone();
            let pool_fwb = pool_fwb_lock.read().unwrap();
            pool_fwb.clone()
        };
        let _ = client.get(replaced_url).send().await;
    }

    async fn bidding_notify_lose(_url: String, _lose_price: i32, _lose_reason: i32, _lose_adn_name: &String, _iv: &String, _connection: &Connection, _pool: &HttpPool) {

    }

    fn encrypt_price(price: i32, iv: &String, connection: &Connection) -> String {
        let mut ekey_base64 = connection.client_ekey.replace("-", "+").replace("_", "/");
        while ekey_base64.len() % 4 != 0 {
            ekey_base64.push_str("=");
        }
        let ekey = BASE64_STANDARD.decode(ekey_base64);
        let mut ikey_base64 = connection.client_ikey.replace("-", "+").replace("_", "/");
        while ikey_base64.len() % 4 != 0 {
            ikey_base64.push_str("=");
        }
        let ikey = BASE64_STANDARD.decode(ikey_base64);
        if ekey.is_err() || ikey.is_err() {
            return "".to_string();
        }
        let ekey = ekey.unwrap();
        let ikey = ikey.unwrap();

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

        let message_base64 = BASE64_STANDARD.encode(&[iv_bytes.to_vec(), enc_price.to_vec(), signature.to_vec()].concat());

        message_base64.replace("+", "-").replace("/", "_").replace("=", "")
    }

}

fn replace_macro(orig: &String) -> String {
    let mut replaced = orig.clone();

    replaced = replaced.replace("__TTS__", "__TS_S__");

    replaced = replaced.replace("__PHEIGHT__", "__DP_HEIGHT__");
    replaced = replaced.replace("__PWIDTH__", "__DP_WIDTH__");

    replaced = replaced.replace("__AD_LT_X__", "__LT_X__");
    replaced = replaced.replace("__AD_LT_Y__", "__LT_Y__");
    replaced = replaced.replace("__AD_RB_X__", "__RB_X__");
    replaced = replaced.replace("__AD_RB_X__", "__RB_X__");

    replaced = replaced.replace("__D_X__", "__ABS_DOWN_X__");
    replaced = replaced.replace("__D_Y__", "__ABS_DOWN_Y__");
    replaced = replaced.replace("__U_X__", "__ABS_UP_X__");
    replaced = replaced.replace("__U_Y__", "__ABS_UP_Y__");

    replaced = replaced.replace("__R_D_X__", "__DOWN_X__");
    replaced = replaced.replace("__R_D_Y__", "__DOWN_Y__");
    replaced = replaced.replace("__R_U_X__", "__UP_X__");
    replaced = replaced.replace("__R_U_Y__", "__UP_Y__");

    replaced = replaced.replace("__BEGIN_TIME__", "__VIDEO_BEGIN_TIME__");
    replaced = replaced.replace("__END_TIME__", "__VIDEO_END_TIME__");
    replaced = replaced.replace("__PLAY_FIRST_FRAME__", "__VIDEO_FIRST_FRAME__");
    replaced = replaced.replace("__PLAY_LAST_FRAME__", "__VIDEO_LAST_FRAME__");
    replaced = replaced.replace("__SCENE__", "__VIDEO_PLAY_SCENE__");
    replaced = replaced.replace("__TYPE__", "__VIDEO_PLAY_TYPE__");
    replaced = replaced.replace("__BEHAVIOR__", "__VIDEO_PLAY_TRIGGER__");
    replaced = replaced.replace("__STATUS__", "__VIDEO_PLAY_STATUS__");

    replaced = replaced.replace("__INER_IP__", "__IP__");

    replaced
}
