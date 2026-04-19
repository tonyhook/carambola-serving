use std::time::Duration;

use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyInit};
use base64::prelude::*;
use chrono::TimeZone;
use chrono_tz::Tz;
use urlencoding::encode;

use crate::{protocol::*, Assets, Cache, Client, Connection, HttpPool, Identifiers, Price, ResultMessage};

type Aes128EcbEnc = ecb::Encryptor<aes::Aes128>;

pub mod request;
pub mod response;

pub use request::MygolbsRequest;
pub use response::MygolbsResponse;

pub struct Mygolbs {

}

impl Client for Mygolbs {

    async fn request(request: &Request, connection: &Connection, pool: &HttpPool, cache: &Cache) -> Result<Response, ResultMessage> {
        let pid = connection.client_tag_id.split("|").nth(0).unwrap();
        let app_id = connection.client_tag_id.split("|").nth(1).unwrap();

        let request_id = cache.get_sequence();

        let mut assets = Assets::new(request);
        let identifiers = Identifiers::new(request);

        let request_mygolbs = MygolbsRequest {
            traceid: request_id.to_string(),
            app_id: app_id.to_string(),
            pid: pid.to_string(),
            appname: {
                match &connection.client_media_appname {
                    Some(client_media_appname) => Some(client_media_appname.clone()),
                    None => {
                        match &request.context.app {
                            Some(app) => Some(app.name.clone()),
                            None => None,
                        }
                    },
                }
            },
            bundle_id: {
                match &connection.client_media_apppackage {
                    Some(client_media_apppackage) => Some(client_media_apppackage.clone()),
                    None => {
                        match &request.context.app {
                            Some(app) => app.bundle.clone(),
                            None => None,
                        }
                    },
                }
            },
            appversion: {
                match &request.context.app {
                    Some(app) => app.ver.clone(),
                    None => None,
                }
            },
            appversion_code: None,
            appstoreversion: None,
            bid_floor: Some(Price::to_client(connection, request.item[0].flr)),
            elapsetime: {
                match &request.context.device.boottime {
                    Some(boottime) => {
                        let mut boottime = boottime.clone();
                        if boottime.len() > 10 && !boottime.contains(".") {
                            boottime = boottime.split_at(10).0.to_string();
                        }
                        match boottime.split(".").nth(0).unwrap().parse::<i32>() {
                            Ok(boottime) => Some(boottime),
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
            ver_code_of_ag: request.context.device.storev.clone(),
            ver_code_of_hms: request.context.device.hmsv.clone(),
            ppi: {
                match request.context.device.ppi {
                    Some(ppi) => ppi,
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.device.ppi is required for upstream".to_string(),
                    }),
                }
            },
            screendensity: {
                match request.context.device.pxratio {
                    Some(pxratio) => pxratio,
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.device.pxratio is required for upstream".to_string(),
                    }),
                }
            },
            nw: {
                match &request.context.device.contype {
                    Some(2) => 20,
                    Some(4) => 2,
                    Some(5) => 3,
                    Some(6) => 4,
                    Some(7) => 5,
                    _ => 1,
                }
            },
            androidid: {
                match identifiers.get_id(509, 0) {
                    Some(uid) => Some(uid.id.clone()),
                    None => None,
                }
            },
            androidid_md5: {
                match identifiers.get_id(510, 0) {
                    Some(uid) => Some(uid.id.clone()),
                    None => None,
                }
            },
            imei: {
                match identifiers.get_id(501, 0) {
                    Some(uid) => Some(uid.id.clone()),
                    None => Some("".to_string()),
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
                    None => Some("".to_string()),
                }
            },
            oaid_md5: {
                match identifiers.get_id(506, 0) {
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
            caid_old: {
                match identifiers.get_id(513, 1) {
                    Some(uid) => Some(uid.id.clone()),
                    None => None,
                }
            },
            caid_old_version: {
                match identifiers.get_id(513, 1) {
                    Some(uid) => uid.ver.clone(),
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
            ua: request.context.device.ua.clone(),
            mac: {
                match identifiers.get_id(511, 0) {
                    Some(uid) => Some(uid.id.clone()),
                    None => None,
                }
            },
            vendor: {
                match &request.context.device.brand {
                    Some(brand) => {
                        brand.clone()
                    },
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.device.brand is required for upstream".to_string(),
                    }),
                }
            },
            devicetype: {
                match &request.context.device.model {
                    Some(model) => {
                        model.clone()
                    },
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.device.model is required for upstream".to_string(),
                    }),
                }
            },
            sv: {
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
            s: {
                match &request.context.device.os {
                    Some(2) => {
                        "android".to_string()
                    },
                    Some(13) => {
                        "ios".to_string()
                    },
                    _ => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.device.os is required for upstream".to_string(),
                    }),
                }
            },
            w: {
                match request.context.device.w {
                    Some(w) => {
                        w
                    },
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.device.w is required for upstream".to_string(),
                    }),
                }
            },
            h: {
                match request.context.device.h {
                    Some(h) => {
                        h
                    },
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.device.h is required for upstream".to_string(),
                    }),
                }
            },
            adw: {
                match assets.get_banner() {
                    Some(banner) => banner.w.clone(),
                    None => None,
                }
            },
            adh: {
                match assets.get_banner() {
                    Some(banner) => banner.h.clone(),
                    None => None,
                }
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
            lng: {
                match &request.context.device.geo {
                    Some(geo) => {
                        match &geo.lon {
                            Some(lon) => Some(lon.to_string()),
                            None => None,
                        }
                    },
                    None => None,
                }
            },
            lat: {
                match &request.context.device.geo {
                    Some(geo) => {
                        match &geo.lat {
                            Some(lat) => Some(lat.to_string()),
                            None => None,
                        }
                    },
                    None => None,
                }
            },
            carrier: {
                match &request.context.device.carrier {
                    Some(carrier) => {
                        match carrier.as_str() {
                            "cmcc" => "46000".to_string(),
                            "unicom" => "46001".to_string(),
                            "telecom" => "46003".to_string(),
                            "cbn" => "46015".to_string(),
                            _ => "-1".to_string(),
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
                    Some(uid) => Some(uid.id.clone()),
                    None => None,
                }
            },
            isdeeplink: Some(true),
            isul: Some(true),
            boot_mark: request.context.device.bootmark.clone(),
            update_mark: request.context.device.updatemark.clone(),
            manufacturer: request.context.device.make.clone(),
            update_time: request.context.device.updatetime.clone(),
            api_version: 10,
            rom_version: request.context.device.romv.clone(),
            orientation: {
                match request.context.device.orientation {
                    Some(501) => 0,
                    Some(502) => 1,
                    _ => 0,
                }
            },
            memory_size: {
                match request.context.device.sysmemory {
                    Some(sysmemory) => {
                        Some(sysmemory / 1024)
                    },
                    None => None,
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
                                Some((t.timestamp() - utc.timestamp()).to_string())
                            },
                            Err(_) => return Err(ResultMessage {
                                code: 998,
                                message: "request.context.device.timezone is malformat for upstream, should be like Asia/Shanghai".to_string(),
                            }),
                        }
                    },
                    None => None,
                }
            },
            model_code: request.context.device.hwmodel.clone(),
            disk_size: {
                match request.context.device.sysdisksize {
                    Some(sysdisksize) => {
                        Some(sysdisksize / 1024)
                    },
                    None => None,
                }
            },
            lmt: request.context.device.lmt.clone(),
            phone_name: {
                match identifiers.get_id(527, 0) {
                    Some(uid) => Some(uid.id.clone()),
                    None => None,
                }
            },
            os_update_time_second: request.context.device.romtime.clone(),
            battery_status: request.context.device.sysbatterystatus.clone(),
            battery_power: request.context.device.sysbatterypower.clone(),
            cpu_number: request.context.device.syscpu.clone(),
            cpu_frequency: request.context.device.syscpufreq.clone(),
            al: None,
            support_video: Some(1),
        };

        let query = serde_url_params::to_string(&request_mygolbs).unwrap();

        let response_mygolbs: MygolbsResponse;

        let client = {
            let pool_mygolbs_lock = pool.pool_mygolbs.clone();
            let pool_mygolbs = pool_mygolbs_lock.read().unwrap();
            pool_mygolbs.clone()
        };
        let response_mygolbs_raw = client.get(format!("{}{}", "http://api.mygolbs.cn/ssp/ad/get?", query).as_str())
            .header("Accept-Encoding", "gzip")
            .header("Connection", "keep-alive")
            .timeout(Duration::from_millis(connection.timeout))
            .send().await;

        match response_mygolbs_raw {
            Ok(response_mygolbs_raw) => {
                let status = response_mygolbs_raw.status();
                if status == 204 {
                    return Err(ResultMessage {
                        code: 993,
                        message: "".to_string(),
                    });
                } else if status != 200 {
                    return Err(ResultMessage {
                        code: 992,
                        message: {
                            match response_mygolbs_raw.text().await {
                                Ok(text) => format!("upstream error {}: {}", status, text),
                                Err(_) => format!("upstream error {}", status),
                            }
                        },
                    });
                } else {
                    match response_mygolbs_raw.text().await {
                        Ok(text) => {
                            match serde_json::from_str::<MygolbsResponse>(&text) {
                                Ok(json) => {
                                    response_mygolbs = json;
                                },
                                Err(_) => {
                                    return Err(ResultMessage {
                                        code: 997,
                                        message: "".to_string(),
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

                        let link_asset = LinkAsset {
                            linktype: {
                                match response_mygolbs.media_style {
                                    Some(1) => 1,
                                    Some(2) => 2,
                                    Some(3) => 1,
                                    Some(4) => 3,
                                    _ => 1,
                                }
                            },
                            universallink: {
                                match &response_mygolbs.universal_link {
                                    Some(universal_link) => Some(universal_link.clone()),
                                    None => None,
                                }
                            },
                            storeid: None,
                            deeplink: {
                                match &response_mygolbs.deep_link {
                                    Some(deep_link) => Some(deep_link.clone()),
                                    None => None,
                                }
                            },
                            quickapplink: None,
                            wechatmppath: {
                                match &response_mygolbs.wx_mini_pro_path {
                                    Some(deep_link) => Some(deep_link.clone()),
                                    None => None,
                                }
                            },
                            wechatmpid: {
                                match &response_mygolbs.wx_mini_pro_id {
                                    Some(deep_link) => Some(deep_link.clone()),
                                    None => None,
                                }
                            },
                            marketurl: None,
                            downloadurl: {
                                match response_mygolbs.media_style {
                                    Some(2) => {
                                        match &response_mygolbs.link {
                                            Some(link) => Some(link.clone()),
                                            None => None,
                                        }
                                    },
                                    Some(4) => {
                                        match &response_mygolbs.link {
                                            Some(link) => Some(link.clone()),
                                            None => None,
                                        }
                                    },
                                    _ => None,
                                }
                            },
                            url: {
                                match response_mygolbs.link {
                                    Some(link) => {
                                        link.clone()
                                    },
                                    None => "".to_string(),
                                }
                            },
                            urlfb: None,
                        };

                        let bid = Bid {
                            id: Some(request_id.to_string()),
                            item: request.item[0].id.clone(),
                            price: { // update later
                                match response_mygolbs.price {
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
                                match &response_mygolbs.win_notice_url {
                                    Some(win_notice_url) => {
                                        let mut burl = Vec::<String>::new();
                                        for win_notice_url1 in win_notice_url {
                                            let mut nurl = win_notice_url1.clone();
                                            nurl = nurl.replace("__PRICE__", "__WIN_PRICE__");
                                            burl.push(replace_macro(&nurl));
                                        }
                                        Some(burl)
                                    },
                                    None => None,
                                }
                            },
                            lurl: {
                                match &response_mygolbs.lurl {
                                    Some(lose_notice_url) => {
                                        let mut lurl = Vec::<String>::new();
                                        for lurl1 in lose_notice_url {
                                            let mut nurl = lurl1.clone();
                                            nurl = nurl.replace("__WIN_PRICE__", "__LOSE_PRICE__");
                                            nurl = nurl.replace("__PRICE__", "__LOSE_PRICE__");
                                            nurl = nurl.replace("__REASON__", "__LOSE_REASON__");
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
                                            match &response_mygolbs.pic {
                                                Some(pic) => {
                                                    Some(Banner {
                                                        img: pic.clone(),
                                                        link: Some(link_asset.clone()),
                                                    })
                                                },
                                                None => {
                                                    match &response_mygolbs.pics {
                                                        Some(pics) => {
                                                            if pics.len() > 0 {
                                                                Some(Banner {
                                                                    img: pics[0].clone(),
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

                                            match &response_mygolbs.video_url {
                                                Some(video_url) => {
                                                    asset_vec.push(Asset {
                                                        id: assets.consume_asset("video"),
                                                        req: 1,
                                                        video: Some(VideoAsset {
                                                            url: video_url.clone(),
                                                            mime: {
                                                                match response_mygolbs.mime_type {
                                                                    Some(1) => Some("video/mp4".to_string()),
                                                                    Some(2) => Some("video/3gp".to_string()),
                                                                    Some(3) => Some("video/x-msvideo".to_string()),
                                                                    Some(4) => Some("video/x-flv".to_string()),
                                                                    Some(5) => Some("video/xms-wmv".to_string()),
                                                                    Some(6) => Some("video/quicktime".to_string()),
                                                                    _ => None,
                                                                }
                                                            },
                                                            w: response_mygolbs.width,
                                                            h: response_mygolbs.height,
                                                            dur: response_mygolbs.duration,
                                                            skipoffset: None,
                                                            size: None,
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

                                                    match &response_mygolbs.cover_url {
                                                        Some(cover_url) => {
                                                            asset_vec.push(Asset {
                                                                id: assets.consume_asset("video#cover"),
                                                                req: 1,
                                                                img: Some(ImageAsset {
                                                                    url: cover_url.clone(),
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
                                                },
                                                None => (),
                                            }

                                            match &response_mygolbs.pic {
                                                Some(pic) => {
                                                    if assets.get_asset_size("img") > 0 {
                                                        asset_vec.push(Asset {
                                                            id: assets.consume_asset("img"),
                                                            req: 1,
                                                            img: Some(ImageAsset {
                                                                url: pic.clone(),
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
                                                                url: pic.clone(),
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

                                            match &response_mygolbs.pics {
                                                Some(pics) => {
                                                    if assets.get_asset_size("img") > 0 {
                                                        for pic in pics {
                                                            asset_vec.push(Asset {
                                                                id: assets.consume_asset("img"),
                                                                req: 1,
                                                                img: Some(ImageAsset {
                                                                    url: pic.clone(),
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
                                                        for pic in pics {
                                                            asset_vec.push(Asset {
                                                                id: assets.consume_asset("thumb"),
                                                                req: 1,
                                                                img: Some(ImageAsset {
                                                                    url: pic.clone(),
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

                                            match &response_mygolbs.app_name {
                                                Some(app_name) => {
                                                    asset_vec.push(Asset {
                                                        id: assets.consume_asset("app"),
                                                        req: 0,
                                                        app: Some(AppAsset {
                                                            name: app_name.clone(),
                                                            desc: None,
                                                            descurl: response_mygolbs.description_url.clone(),
                                                            domain: None,
                                                            bundle: response_mygolbs.package.clone(),
                                                            ver: None,
                                                            developer: response_mygolbs.comp_name.clone(),
                                                            icon: response_mygolbs.app_icon.clone(),
                                                            storeid: None,
                                                            storeurl: None,
                                                            paid: 0,
                                                            size: None,
                                                            md5: None,
                                                            registration: None,
                                                            privacy: None,
                                                            privacyurl: response_mygolbs.secret_url.clone(),
                                                            permission: None,
                                                            permissionurl: response_mygolbs.permission_url.clone(),
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

                                        match &response_mygolbs.unfold_monitor_link {
                                            Some(unfold_monitor_link) => {
                                                for event in unfold_monitor_link {
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
                                        match &response_mygolbs.click_monitor_link {
                                            Some(click_monitor_link) => {
                                                for event in click_monitor_link {
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
                                        match &response_mygolbs.dws_urls {
                                            Some(dws_urls) => {
                                                for event in dws_urls {
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
                                        match &response_mygolbs.dwe_urls {
                                            Some(dwe_urls) => {
                                                for event in dwe_urls {
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
                                        match &response_mygolbs.inst_b_urls {
                                            Some(inst_b_urls) => {
                                                for event in inst_b_urls {
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
                                        match &response_mygolbs.inst_urls {
                                            Some(inst_urls) => {
                                                for event in inst_urls {
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
                                        match &response_mygolbs.dn_urls {
                                            Some(dn_urls) => {
                                                for event in dn_urls {
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
                                        match &response_mygolbs.dplink_urls {
                                            Some(dplink_urls) => {
                                                for event in dplink_urls {
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
                                        match &response_mygolbs.dplink_try_urls {
                                            Some(dplink_try_urls) => {
                                                for event in dplink_try_urls {
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
                                        match &response_mygolbs.dplink_err_urls {
                                            Some(dplink_err_urls) => {
                                                for event in dplink_err_urls {
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
                                        match &response_mygolbs.video_start_urls {
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
                                        match &response_mygolbs.video_first_quartile_urls {
                                            Some(video_first_quartile_urls) => {
                                                for event in video_first_quartile_urls {
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
                                        match &response_mygolbs.video_midpoint_urls {
                                            Some(video_midpoint_urls) => {
                                                for event in video_midpoint_urls {
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
                                        match &response_mygolbs.video_third_quartile_urls {
                                            Some(video_third_quartile_urls) => {
                                                for event in video_third_quartile_urls {
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
                                        match &response_mygolbs.video_end_urls {
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
                                        match &response_mygolbs.video_fail_urls {
                                            Some(video_fail_urls) => {
                                                for event in video_fail_urls {
                                                    event_vec.push(Event {
                                                        eventtype: 724,
                                                        method: 1,
                                                        url: replace_macro(event),
                                                        header: None,
                                                        content: None,
                                                    });
                                                }
                                            },
                                            None => (),
                                        }
                                        match &response_mygolbs.clk {
                                            Some(clk) => {
                                                for event in clk {
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
                                        match &response_mygolbs.close {
                                            Some(close) => {
                                                for event in close {
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
                                        match &response_mygolbs.skip {
                                            Some(skip) => {
                                                for event in skip {
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
                                        match &response_mygolbs.mute {
                                            Some(mute) => {
                                                for event in mute {
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
                                        match &response_mygolbs.unmute {
                                            Some(unmute) => {
                                                for event in unmute {
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
                                        match &response_mygolbs.suspend {
                                            Some(suspend) => {
                                                for event in suspend {
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
                                        match &response_mygolbs.unsuspend {
                                            Some(unsuspend) => {
                                                for event in unsuspend {
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
        };

        Ok(response)
    }

    async fn bidding_notify_win(url: String, win_price: i32, _next_price: i32, iv: &String, connection: &Connection, pool: &HttpPool) {
        let encrypt_price = Self::encrypt_price(win_price, iv, connection);
        let replaced_url = url
            .replace("__WIN_PRICE__", &encode(encrypt_price.as_str()))
            .replace("__TYPE__", "100");

        let client = {
            let pool_mygolbs_lock = pool.pool_mygolbs.clone();
            let pool_mygolbs = pool_mygolbs_lock.read().unwrap();
            pool_mygolbs.clone()
        };
        let _ = client.get(replaced_url).send().await;
    }

    async fn bidding_notify_lose(url: String, lose_price: i32, lose_reason: i32, _lose_adn_name: &String, iv: &String, connection: &Connection, pool: &HttpPool) {
        let encrypt_price = Self::encrypt_price(lose_price, iv, connection);
        let replaced_url = url
            .replace("__LOSE_PRICE__", &encode(encrypt_price.as_str()))
            .replace("__TYPE__", "101")
            .replace("__REASON__", match lose_reason {
                1 => "媒体侧底价过滤",
                2 => "广告竞价失败",
                _ => "",
            });

        let client = {
            let pool_mygolbs_lock = pool.pool_mygolbs.clone();
            let pool_mygolbs = pool_mygolbs_lock.read().unwrap();
            pool_mygolbs.clone()
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

        let mut key = [0u8; 16];
        if connection.client_ekey.len() == 16 {
            key[0..16].copy_from_slice(connection.client_ekey.as_bytes());
        } else {
            key[0..16].copy_from_slice(&hex::decode(&connection.client_ekey).unwrap());
        };

        let cipher = Aes128EcbEnc::new(key[0..16].into())
            .encrypt_padded_mut::<Pkcs7>(&mut buffer, pos)
            .unwrap();

        let message_base64 = BASE64_STANDARD.encode(cipher);

        message_base64.replace("+", "-").replace("/", "_").replace("=", "")
    }

}

fn replace_macro(orig: &String) -> String {
    let mut replaced = orig.clone();

    replaced = replaced.replace("__START_TS__", "__EVENT_TIME_START__");
    replaced = replaced.replace("__START_TS_ST__", "__EVENT_TIME_START_S__");
    replaced = replaced.replace("__TS__", "__EVENT_TIME_END__");
    replaced = replaced.replace("__TS_ST__", "__EVENT_TIME_END_S__");
    replaced = replaced.replace("__PHEIGHT__", "__HEIGHT__");
    replaced = replaced.replace("__PWIDTH__", "__WIDTH__");
    replaced = replaced.replace("__DOWN_X__", "__R_DOWN_X__");
    replaced = replaced.replace("__DOWN_Y__", "__R_DOWN_Y__");
    replaced = replaced.replace("__UP_X__", "__R_UP_X__");
    replaced = replaced.replace("__UP_Y__", "__R_UP_Y__");
    replaced = replaced.replace("__DOWN_MX__", "__DOWN_X__");
    replaced = replaced.replace("__DOWN_MY__", "__DOWN_Y__");
    replaced = replaced.replace("__UP_MX__", "__UP_X__");
    replaced = replaced.replace("__UP_MY__", "__UP_Y__");
    replaced = replaced.replace("__DOWN_SX__", "__ABS_DOWN_X__");
    replaced = replaced.replace("__DOWN_SY__", "__ABS_DOWN_Y__");
    replaced = replaced.replace("__UP_SX__", "__ABS_UP_X__");
    replaced = replaced.replace("__UP_SY__", "__ABS_UP_Y__");
    replaced = replaced.replace("__X__", "__R_DOWN_X__");
    replaced = replaced.replace("__Y__", "__R_DOWN_Y__");
    replaced = replaced.replace("__DPWIDTH__", "__DP_WIDTH__");
    replaced = replaced.replace("__DPHEIGHT__", "__DP_HEIGHT__");
    replaced = replaced.replace("__DP_DOWN_MX__", "__DOWN_X__");
    replaced = replaced.replace("__DP_DOWN_MY__", "__DOWN_Y__");
    replaced = replaced.replace("__DP_UP_MX__", "__UP_X__");
    replaced = replaced.replace("__DP_UP_MY__", "__UP_Y__");
    replaced = replaced.replace("__DISPLAY_LU_SX__", "__LT_X__");
    replaced = replaced.replace("__DISPLAY_LU_SY__", "__LT_Y__");
    replaced = replaced.replace("__DISPLAY_RD_SX__", "__RB_X__");
    replaced = replaced.replace("__DISPLAY_RD_SY__", "__RB_Y__");
    replaced = replaced.replace("__VIDEO_STARTTIME__", "__VIDEO_BEGIN_TIME__");
    replaced = replaced.replace("__VIDEO_ENDTIME__", "__VIDEO_END_TIME__");
    replaced = replaced.replace("__VIDEO_EVENTTIME__", "__TS__");
    replaced = replaced.replace("__VIDEO_PROCESS__", "__VIDEO_PLAY_PROGRESS_S__");
    replaced = replaced.replace("__VIDEO_MS_PROCESS__", "__VIDEO_PLAY_PROGRESS__");
    replaced = replaced.replace("__VIDEO_RATE__", "__VIDEO_PLAY_RATIO__");
    replaced = replaced.replace("__VIDEO_DURATION__", "__VIDEO_TIME__");
    replaced = replaced.replace("__BEHAVIOR__", "__VIDEO_PLAY_TRIGGER_0__");
    replaced = replaced.replace("__PLAY_FIRST_FRAME__", "__VIDEO_FIRST_FRAME__");
    replaced = replaced.replace("__PLAY_LAST_FRAME__", "__VIDEO_LAST_FRAME__");
    replaced = replaced.replace("__SCENE__", "__VIDEO_PLAY_SCENE__");
    replaced = replaced.replace("__PLAY_TYPE__", "__VIDEO_PLAY_TYPE__");
    replaced = replaced.replace("__VIDEO_STATUS__", "__VIDEO_PLAY_STATUS__");
    replaced = replaced.replace("__INTERACTIVE_MODE__", "__SLD__");
    replaced = replaced.replace("__X_MAX_ACC_HUNDRED__", "__X_MAX_ACC__");
    replaced = replaced.replace("__Y_MAX_ACC_HUNDRED__", "__Y_MAX_ACC__");
    replaced = replaced.replace("__Z_MAX_ACC_HUNDRED__", "__Z_MAX_ACC__");

    replaced
}
