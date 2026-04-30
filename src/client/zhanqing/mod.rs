use std::{collections::HashMap, time::{Duration, SystemTime, UNIX_EPOCH}};

use reqwest::Url;
use urlencoding::encode;

use crate::{protocol::*, Assets, Cache, Client, Connection, HttpPool, Identifiers, Price, ResultMessage};

pub mod ad;
pub mod app;
pub mod caid;
pub mod device;
pub mod extend_tracking;
pub mod playpercentage;
pub mod playtrackers;
pub mod request;
pub mod response;
pub mod video;

pub use ad::ZhanqingAd;
pub use app::ZhanqingApp;
pub use caid::ZhanqingCaid;
pub use device::ZhanqingDevice;
pub use extend_tracking::ZhanqingExtendTracking;
pub use playpercentage::ZhanqingPlaypercentage;
pub use playtrackers::ZhanqingPlaytrackers;
pub use request::ZhanqingRequest;
pub use response::ZhanqingResponse;
pub use video::ZhanqingVideo;

pub struct Zhanqing {

}

impl Client for Zhanqing {

    async fn request(request: &Request, connection: &Connection, pool: &HttpPool, cache: &Cache) -> Result<Response, ResultMessage> {
        let pid = connection.client_tag_id.split("|").nth(0).unwrap();
        let key = connection.client_tag_id.split("|").nth(1).unwrap();
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros();

        let request_id = cache.get_sequence();

        let mut assets = Assets::new(request);
        let identifiers = Identifiers::new(request);

        let request_zhanqing = ZhanqingRequest {
            rid: {
                request_id.to_string()
            },
            pid: {
                pid.to_string()
            },
            bidfloor: {
                Price::to_client(connection, request.item[0].flr.map(f64::from)) as i32
            },
            time: {
                time
            },
            token: {
                format!("{:x}", md5::compute(format!("{}{}{}", pid, key, time))).to_uppercase()
            },
            appinfo: ZhanqingApp {
                appver: {
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
                appname: {
                    match &connection.client_media_appname {
                        Some(client_media_appname) => client_media_appname.clone(),
                        None => {
                            match &request.context.app {
                                Some(app) => app.name.clone(),
                                None => "".to_string(),
                            }
                        },
                    }
                },
                pkgname: {
                    match &connection.client_media_apppackage {
                        Some(client_media_apppackage) => client_media_apppackage.clone(),
                        None => {
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
                    }
                },
                w: {
                    match request.item[0].spec.display.w {
                        Some(w) => w,
                        None => 0,
                    }
                },
                h: {
                    match request.item[0].spec.display.h {
                        Some(h) => h,
                        None => 0,
                    }
                },
            },
            deviceinfo: ZhanqingDevice {
                ip: {
                    match &request.context.device.ip {
                        Some(ip) => ip.clone(),
                        None => "".to_string(),
                    }
                },
                net: {
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
                                "cmcc" => 46000,
                                "unicom" => 46001,
                                "telecom" => 46003,
                                "cbn" => 46015,
                                _ => -1,
                            }
                        },
                        None => -1,
                    }
                },
                ua: {
                    request.context.device.ua.clone()
                },
                devicetype: {
                    match &request.context.device.devicetype {
                        Some(1) => 1,
                        Some(4) => 1,
                        Some(5) => 2,
                        Some(2) => 3,
                        Some(6) => 4,
                        Some(3) => 5,
                        _ => 0,
                    }
                },
                os: {
                    match &request.context.device.os {
                        Some(2) => 1,
                        Some(13) => 2,
                        _ => 0,
                    }
                },
                osv: {
                    match &request.context.device.osv {
                        Some(osv) => osv.clone(),
                        None => "".to_string(),
                    }
                },
                imsi: {
                    match identifiers.get_id(503, 0) {
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
                imeimd5: {
                    match identifiers.get_id(502, 0) {
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
                aid_md5: {
                    match identifiers.get_id(510, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                mac: {
                    match identifiers.get_id(511, 0) {
                        Some(uid) => uid.id.clone(),
                        None => "00:00:00:00:00:00".to_string(),
                    }
                },
                mac_md5: {
                    match identifiers.get_id(512, 0) {
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
                oid: None,
                brand: {
                    match &request.context.device.brand {
                        Some(brand) => brand.clone(),
                        None => "".to_string(),
                    }
                },
                model: {
                    match &request.context.device.model {
                        Some(model) => model.clone(),
                        None => "".to_string(),
                    }
                },
                density: {
                    match request.context.device.pxratio {
                        Some(pxratio) => Some(pxratio),
                        None => None,
                    }
                },
                sw: {
                    match request.context.device.w {
                        Some(w) => w,
                        None => 0,
                    }
                },
                sh: {
                    match request.context.device.h {
                        Some(h) => h,
                        None => 0,
                    }
                },
                so: {
                    match request.context.device.orientation {
                        Some(orientation) => {
                            match orientation {
                                501 => Some(1),
                                502 => Some(2),
                                _ => Some(0),
                            }
                        },
                        None => Some(0),
                    }
                },
                lon: {
                    match &request.context.device.geo {
                        Some(geo) => {
                            geo.lon.clone()
                        },
                        None => Some(0.0),
                    }
                },
                lat: {
                    match &request.context.device.geo {
                        Some(geo) => {
                            geo.lat.clone()
                        },
                        None => Some(0.0),
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
                boot_mark: {
                    request.context.device.bootmark.clone()
                },
                update_mark: {
                    request.context.device.updatemark.clone()
                },
                hms_ver: {
                    request.context.device.hmsv.clone()
                },
                appstore_version: {
                    request.context.device.storev.clone()
                },
                android_version: {
                    request.context.device.osv.clone()
                },
                device_start_time: {
                    match &request.context.device.boottime {
                        Some(boottime) => {
                            match boottime.split(".").nth(0).unwrap().parse::<i32>() {
                                Ok(boottime) => Some(boottime),
                                Err(_) => None,
                            }
                        },
                        None => None,
                    }
                },
                device_name_md5: {
                    match identifiers.get_id(528, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                device_name: {
                    match identifiers.get_id(527, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                sys_update_time: {
                    match &request.context.device.updatetime {
                        Some(updatetime) => {
                            match updatetime.split(".").nth(0).unwrap().parse::<i32>() {
                                Ok(updatetime) => Some(updatetime),
                                Err(_) => None,
                            }
                        },
                        None => None,
                    }
                },
                device_hard_disk: {
                    request.context.device.sysdisksize.clone()
                },
                device_memory: {
                    request.context.device.sysmemory.clone()
                },
                language: {
                    request.context.device.lang.clone()
                },
                time_zone: {
                    request.context.device.timezone.clone()
                },
                wifi_ssid: {
                    match identifiers.get_id(524, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                wifi_bssid: {
                    match identifiers.get_id(529, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                ppi: {
                    request.context.device.ppi.clone()
                },
                caid: None,
                boot_time: {
                    request.context.device.boottime.clone()
                },
                device_machine: {
                    request.context.device.hwmachine.clone()
                },
                device_model: {
                    request.context.device.hwmodel.clone()
                },
                inittime: {
                    request.context.device.inittime.clone()
                },
                caidver: None,
                paid: {
                    match identifiers.get_id(519, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                cpu_num: {
                    match request.context.device.syscpu {
                        Some(cpu) => Some(cpu.to_string()),
                        None => None,
                    }
                },
                dpi: {
                    request.context.device.ppi.clone()
                },
                duid: None,
                gps_type: {
                    match &request.context.device.geo {
                        Some(geo) => {
                            match &geo.coordinate {
                                Some(1) => Some(4),
                                Some(2) => Some(0),
                                Some(3) => Some(1),
                                _ => None,
                            }
                        },
                        None => None,
                    }
                },
                keyword: None,
                boot_time_nano: {
                    request.context.device.boottime.clone()
                },
                update_time_nano: {
                    request.context.device.updatetime.clone()
                },
                packages: {
                    match &request.context.device.app {
                        Some(app) => Some(app.split(",").map(|s| s.to_string()).collect()),
                        None => None,

                    }
                },
                caidvd: None,
                caid_list: {
                    let mut caid_list = vec![];

                    match identifiers.get_id(513, 0) {
                        Some(uid) => {
                            caid_list.push(ZhanqingCaid {
                                id: uid.id.clone(),
                                ver: {
                                    match &uid.ver {
                                        Some(ver) => ver.clone(),
                                        None => "".to_string(),
                                    }
                                },
                            });
                        },
                        None => (),
                    }
                    match identifiers.get_id(513, 1) {
                        Some(uid) => {
                            caid_list.push(ZhanqingCaid {
                                id: uid.id.clone(),
                                ver: {
                                    match &uid.ver {
                                        Some(ver) => ver.clone(),
                                        None => "".to_string(),
                                    }
                                },
                            });
                        },
                        None => (),
                    }

                    Some(caid_list)
                },
            },
        };

        let response_zhanqing: ZhanqingResponse;

        let client = {
            let pool_zhanqing_lock = pool.pool_zhanqing.clone();
            let pool_zhanqing = pool_zhanqing_lock.read().unwrap();
            pool_zhanqing.clone()
        };
        let response_zhanqing_raw = client.post("http://api.zhanq.net/api/r")
            .json(&request_zhanqing)
            .header("Accept-Encoding", "gzip")
            .header("Connection", "keep-alive")
            .header("Content-Type", "application/json")
            .timeout(Duration::from_millis(connection.timeout))
            .send().await;

        match response_zhanqing_raw {
            Ok(response_zhanqing_raw) => {
                let status = response_zhanqing_raw.status();
                if status != 200 {
                    return Err(ResultMessage {
                        code: 992,
                        message: {
                            match response_zhanqing_raw.text().await {
                                Ok(text) => format!("upstream error {}: {}", status, text),
                                Err(_) => format!("upstream error {}", status),
                            }
                        },
                    });
                } else {
                    match response_zhanqing_raw.text().await {
                        Ok(text) => {
                            match serde_json::from_str::<ZhanqingResponse>(&text) {
                                Ok(json) => {
                                    if json.status {
                                        response_zhanqing = json;
                                    } else {
                                        if json.message.as_str() == "No_Fill" {
                                            return Err(ResultMessage {
                                                code: 993,
                                                message: "".to_string(),
                                            });
                                        } else {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: format!("upstream error: {}", json.message),
                                            });
                                        }
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

                        for ad in response_zhanqing.ads.unwrap() {
                            let link_asset = LinkAsset {
                                linktype: {
                                    match ad.action {
                                        1 => 1,
                                        2 => 2,
                                        3 => 3,
                                        _ => 1,
                                    }
                                },
                                universallink: None,
                                storeid: None,
                                deeplink: {
                                    ad.durl.clone()
                                },
                                quickapplink: None,
                                wechatmppath: None,
                                wechatmpid: None,
                                marketurl: None,
                                downloadurl: None,
                                url: {
                                    match &ad.videos {
                                        Some(videos) => {
                                            match &videos.end_url {
                                                Some(end_url) => replace_macro(end_url),
                                                None => {
                                                    replace_macro(&ad.url)
                                                },
                                            }
                                        },
                                        None => {
                                            replace_macro(&ad.url)
                                        },
                                    }
                                },
                                urlfb: None,
                            };

                            let bid = Bid {
                                id: Some(request_id.to_string()),
                                item: request.item[0].id.clone(),
                                price: { // update later
                                    match ad.price {
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
                                    match &ad.nurl {
                                        Some(nurl) => {
                                            let mut burl = Vec::<String>::new();
                                            let mut nurl = nurl.clone();
                                            nurl = nurl.replace("$$WIN_PRICE$$", "__WIN_PRICE__");
                                            nurl = nurl.replace("$$win_price$$", "__WIN_PRICE__");
                                            burl.push(replace_macro(&nurl));
                                            Some(burl)
                                        },
                                        None => None,
                                    }
                                },
                                lurl: {
                                    match &ad.lose_notice_urls {
                                        Some(lose_notice_urls) => {
                                            let mut lurl = Vec::<String>::new();
                                            for lose_notice_url in lose_notice_urls {
                                                let mut nurl = lose_notice_url.clone();
                                                nurl = nurl.replace("$$WIN_PRICE$$", "__LOSE_PRICE__");
                                                nurl = nurl.replace("$$win_price$$", "__LOSE_PRICE__");
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
                                                match &ad.imgs {
                                                    Some(imgs) => {
                                                        Some(Banner {
                                                            img: imgs[0].clone(),
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

                                                match &ad.title {
                                                    Some(title) => {
                                                        asset_vec.push(Asset {
                                                            id: assets.consume_asset("title"),
                                                            req: 1,
                                                            title: Some(TitleAsset {
                                                                text: title.clone(),
                                                                subtitle: None,
                                                                desc: ad.desc.clone(),
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

                                                match &ad.icon {
                                                    Some(icon) => {
                                                        asset_vec.push(Asset {
                                                            id: assets.consume_asset("icon"),
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
                                                        });
                                                    },
                                                    None => (),
                                                }

                                                match &ad.imgs {
                                                    Some(imgs) => {
                                                        if assets.get_asset_size("img") > 0 {
                                                            for img in imgs {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("img"),
                                                                    req: 1,
                                                                    img: Some(ImageAsset {
                                                                        url: img.clone(),
                                                                        mime: None,
                                                                        w: ad.w.clone(),
                                                                        h: ad.h.clone(),
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
                                                            for img in imgs {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("thumb"),
                                                                    req: 1,
                                                                    img: Some(ImageAsset {
                                                                        url: img.clone(),
                                                                        mime: None,
                                                                        w: ad.w.clone(),
                                                                        h: ad.h.clone(),
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

                                                match &ad.html {
                                                    Some(html) => {
                                                        asset_vec.push(Asset {
                                                            id: assets.consume_asset("html"),
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
                                                        });
                                                    },
                                                    None => (),
                                                }

                                                match &ad.videos {
                                                    Some(videos) => {
                                                        match &videos.video_url {
                                                            Some(video_url) => {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("video"),
                                                                    req: 1,
                                                                    video: Some(VideoAsset {
                                                                        url: video_url.clone(),
                                                                        mime: None,
                                                                        w: videos.width.clone(),
                                                                        h: videos.height.clone(),
                                                                        dur: videos.video_duration.clone(),
                                                                        skipoffset: None,
                                                                        size: videos.length.clone(),
                                                                        delivery: {
                                                                            match videos.prefetch {
                                                                                Some(true) => Some(2),
                                                                                Some(false) => Some(1),
                                                                                None => None,
                                                                            }
                                                                        },
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

                                                                match &videos.html {
                                                                    Some(html) => {
                                                                        asset_vec.push(Asset {
                                                                            id: assets.consume_asset("video#end#html"),
                                                                            req: 0,
                                                                            html: Some(HtmlAsset {
                                                                                html: Some(html.clone()),
                                                                                link: None,
                                                                                len: Some(html.len() as i32),
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
                                                                match &videos.cover_url {
                                                                    Some(cover_url) => {
                                                                        asset_vec.push(Asset {
                                                                            id: assets.consume_asset("video#cover"),
                                                                                req: 0,
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
                                                                match &videos.end_card_html {
                                                                    Some(end_card_html) => {
                                                                        asset_vec.push(Asset {
                                                                            id: assets.consume_asset("video#end#html"),
                                                                            req: 0,
                                                                            html: Some(HtmlAsset {
                                                                                html: Some(end_card_html.clone()),
                                                                                link: None,
                                                                                len: Some(end_card_html.len() as i32),
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
                                                                match &videos.end_card_url {
                                                                    Some(end_card_url) => {
                                                                        asset_vec.push(Asset {
                                                                            id: assets.consume_asset("video#end#html"),
                                                                            req: 0,
                                                                            html: Some(HtmlAsset {
                                                                                html: None,
                                                                                link: Some(end_card_url.clone()),
                                                                                len: None,
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
                                                                match &videos.end_button_text {
                                                                    Some(end_button_text) => {
                                                                        asset_vec.push(Asset {
                                                                            id: assets.consume_asset("video#end#button#text"),
                                                                            req: 0,
                                                                            data: Some(DataAsset {
                                                                                value: end_button_text.clone(),
                                                                                len: Some(end_button_text.len() as i32),
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
                                                                match &videos.end_icon_url {
                                                                    Some(end_icon_url) => {
                                                                        asset_vec.push(Asset {
                                                                            id: assets.consume_asset("video#end#button#img"),
                                                                            req: 0,
                                                                            img: Some(ImageAsset {
                                                                                url: end_icon_url.clone(),
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
                                                                match &videos.end_title {
                                                                    Some(end_title) => {
                                                                        asset_vec.push(Asset {
                                                                            id: assets.consume_asset("video#end#title"),
                                                                            req: 0,
                                                                            title: Some(TitleAsset {
                                                                                text: end_title.clone(),
                                                                                subtitle: videos.end_desc.clone(),
                                                                                desc: None,
                                                                                len: Some(end_title.len() as i32),
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
                                                            },
                                                            None => (),
                                                        }
                                                    },
                                                    None => (),
                                                }

                                                match &ad.app_name {
                                                    Some(app_name) => {
                                                        asset_vec.push(Asset {
                                                            id: assets.consume_asset("app"),
                                                            req: 0,
                                                            app: Some(AppAsset {
                                                                name: app_name.clone(),
                                                                desc: ad.app_function_introduction.clone(),
                                                                descurl: None,
                                                                domain: None,
                                                                bundle: ad.pkgname.clone(),
                                                                ver: ad.app_ver.clone(),
                                                                developer: ad.app_developer_name.clone(),
                                                                icon: None,
                                                                storeid: None,
                                                                storeurl: None,
                                                                paid: 0,
                                                                size: ad.appsize.clone(),
                                                                md5: None,
                                                                registration: None,
                                                                privacy: None,
                                                                privacyurl: ad.app_privacy_url.clone(),
                                                                permission: None,
                                                                permissionurl: ad.app_permission.clone(),
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

                                            for event in &ad.exlist {
                                                event_vec.push(Event {
                                                    eventtype: 501,
                                                    method: 1,
                                                    url: replace_macro(event),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                            for event in &ad.cklist {
                                                event_vec.push(Event {
                                                    eventtype: 502,
                                                    method: 1,
                                                    url: replace_macro(event),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                            match &ad.exttracking {
                                                Some(exttracking) => {
                                                    match &exttracking.dktracking {
                                                        Some(dktracking) => {
                                                            for event in dktracking {
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
                                                    match &exttracking.dlstart {
                                                        Some(dlstart) => {
                                                            for event in dlstart {
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
                                                    match &exttracking.dlcomplete {
                                                        Some(dlcomplete) => {
                                                            for event in dlcomplete {
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
                                                    match &exttracking.istart {
                                                        Some(istart) => {
                                                            for event in istart {
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
                                                    match &exttracking.icomplete {
                                                        Some(icomplete) => {
                                                            for event in icomplete {
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
                                                    match &exttracking.activation {
                                                        Some(activation) => {
                                                            for event in activation {
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
                                                    match &exttracking.dkstarttracking {
                                                        Some(dkstarttracking) => {
                                                            for event in dkstarttracking {
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
                                                },
                                                None => (),
                                            }

                                            match &ad.videos {
                                                Some(videos) => {
                                                    match &videos.playpercentages {
                                                        Some(playpercentage) => {
                                                            for event in playpercentage {
                                                                match event.checkpoint {
                                                                    Some(3) => {
                                                                        match &event.urls {
                                                                            Some(urls) => {
                                                                                for url in urls {
                                                                                    event_vec.push(Event {
                                                                                        eventtype: 706,
                                                                                        method: 1,
                                                                                        url: replace_macro(&url),
                                                                                        header: None,
                                                                                        content: None,
                                                                                    });
                                                                                }

                                                                            },
                                                                            None => (),
                                                                        }
                                                                    },
                                                                    Some(5) => {
                                                                        match &event.urls {
                                                                            Some(urls) => {
                                                                                for url in urls {
                                                                                    event_vec.push(Event {
                                                                                        eventtype: 707,
                                                                                        method: 1,
                                                                                        url: replace_macro(&url),
                                                                                        header: None,
                                                                                        content: None,
                                                                                    });
                                                                                }

                                                                            },
                                                                            None => (),
                                                                        }
                                                                    },
                                                                    _ => {

                                                                    },
                                                                }
                                                            }
                                                        },
                                                        None => (),
                                                    }
                                                    match &videos.play_trackers {
                                                        Some(playtrackers) => {
                                                            match &playtrackers.mute {
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
                                                            match &playtrackers.unmute {
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
                                                            match &playtrackers.play {
                                                                Some(play) => {
                                                                    for event in play {
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
                                                            match &playtrackers.pause {
                                                                Some(pause) => {
                                                                    for event in pause {
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
                                                            match &playtrackers.replay {
                                                                Some(replay) => {
                                                                    for event in replay {
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
                                                            match &playtrackers.fullscreen {
                                                                Some(fullscreen) => {
                                                                    for event in fullscreen {
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
                                                            match &playtrackers.unfullscreen {
                                                                Some(unfullscreen) => {
                                                                    for event in unfullscreen {
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
                                                            match &playtrackers.upscroll {
                                                                Some(upscroll) => {
                                                                    for event in upscroll {
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
                                                            match &playtrackers.downscroll {
                                                                Some(downscroll) => {
                                                                    for event in downscroll {
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
                                                        },
                                                        None => (),
                                                    }
                                                    match &videos.video_loaded_trackers {
                                                        Some(video_loaded_trackers) => {
                                                            for event in video_loaded_trackers {
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
                                                    match &videos.start_play_trackers {
                                                        Some(start_play_trackers) => {
                                                            for event in start_play_trackers {
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
                                                    match &videos.end_play_trackers {
                                                        Some(end_play_trackers) => {
                                                            for event in end_play_trackers {
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
                                                    match &videos.video_close {
                                                        Some(video_close) => {
                                                            for event in video_close {
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
                                                    match &videos.video_skip {
                                                        Some(video_skip) => {
                                                            for event in video_skip {
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
                                                    match &videos.error {
                                                        Some(error) => {
                                                            for event in error {
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
                                                    match &videos.play_1_trackers {
                                                        Some(play_1_trackers) => {
                                                            for event in play_1_trackers {
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
                                                    match &videos.play_2_trackers {
                                                        Some(play_2_trackers) => {
                                                            for event in play_2_trackers {
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
                                                    match &videos.play_3_trackers {
                                                        Some(play_3_trackers) => {
                                                            for event in play_3_trackers {
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
                                                },
                                                None => (),
                                            }
                                            match &ad.click_area_report_url {
                                                Some(click_area_report_url) => {
                                                    for click_area_report_url1 in click_area_report_url {
                                                        let parsed_url = Url::parse(click_area_report_url1.as_str());
                                                        match parsed_url {
                                                            Ok(parsed_url) => {
                                                                let hash_query: HashMap<_, _> = parsed_url.query_pairs().into_owned().collect();
                                                                let sid = hash_query.get("sid");
                                                                let creative_id = hash_query.get("creative_id");
                                                                if sid.is_some() && creative_id.is_some() {
                                                                    event_vec.push(Event {
                                                                        eventtype: 502,
                                                                        method: 502,
                                                                        url: replace_macro(click_area_report_url1),
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
                                    advertiser: {
                                        ad.app_name.clone()
                                    },
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
            let pool_zhanqing_lock = pool.pool_zhanqing.clone();
            let pool_zhanqing = pool_zhanqing_lock.read().unwrap();
            pool_zhanqing.clone()
        };
        let _ = client.get(replaced_url).send().await;
    }

    async fn bidding_notify_lose(url: String, lose_price: i32, _lose_reason: i32, _lose_adn_name: &String, iv: &String, connection: &Connection, pool: &HttpPool) {
        let encrypt_price = Self::encrypt_price(lose_price, iv, connection);
        let replaced_url = url
            .replace("__LOSE_PRICE__", &encode(encrypt_price.as_str()));

        let client = {
            let pool_zhanqing_lock = pool.pool_zhanqing.clone();
            let pool_zhanqing = pool_zhanqing_lock.read().unwrap();
            pool_zhanqing.clone()
        };
        let _ = client.get(replaced_url).send().await;

    }

    fn encrypt_price(price: i32, _iv: &String, _connection: &Connection) -> String {
        price.to_string()
    }

}

fn replace_macro(orig: &String) -> String {
    let mut replaced = orig.clone();

    replaced = replaced.replace("$$LAT$$", "__LAT__");
    replaced = replaced.replace("$$LON$$", "__LON__");
    replaced = replaced.replace("$$DOWN_X$$", "__ABS_DOWN_X__");
    replaced = replaced.replace("$$DOWN_Y$$", "__ABS_DOWN_Y__");
    replaced = replaced.replace("$$UP_X$$", "__ABS_UP_X__");
    replaced = replaced.replace("$$UP_Y$$", "__ABS_UP_Y__");
    replaced = replaced.replace("$$OFFSET_X$$", "__DOWN_X__");
    replaced = replaced.replace("$$OFFSET_Y$$", "__DOWN_Y__");
    replaced = replaced.replace("$$OFFSET_UP_X$$", "__UP_X__");
    replaced = replaced.replace("$$OFFSET_UP_Y$$", "__UP_Y__");
    replaced = replaced.replace("$$TIME_START$$", "__DOWN_TS__");
    replaced = replaced.replace("$$TIME_END$$", "__UP_TS__");
    replaced = replaced.replace("$$TIME_TEN_START$$", "__DOWN_TS_S__");
    replaced = replaced.replace("$$TIME_TEN_END$$", "__UP_TS_S__");
    replaced = replaced.replace("$$DP_WIDTH$$", "__DP_WIDTH__");
    replaced = replaced.replace("$$DP_HEIGHT$$", "__DP_HEIGHT__");
    replaced = replaced.replace("$$DP_OFFSET_X$$", "__R_DOWN_X__");
    replaced = replaced.replace("$$DP_OFFSET_Y$$", "__R_DOWN_Y__");
    replaced = replaced.replace("$$DP_OFFSET_UP_X$$", "__R_UP_X__");
    replaced = replaced.replace("$$DP_OFFSET_UP_Y$$", "__R_UP_Y__");
    replaced = replaced.replace("$$SLD$$", "__SLD__");
    replaced = replaced.replace("$$X_MAX_ACC$$", "__X_MAX_ACC__");
    replaced = replaced.replace("$$Y_MAX_ACC$$", "__Y_MAX_ACC__");
    replaced = replaced.replace("$$Z_MAX_ACC$$", "__Z_MAX_ACC__");
    replaced = replaced.replace("$$TURN_X$$", "__TURN_X__");
    replaced = replaced.replace("$$TURN_Y$$", "__TURN_Y__");
    replaced = replaced.replace("$$TURN_Z$$", "__TURN_Z__");
    replaced = replaced.replace("$$TURN_TIME$$", "__TURN_TIME__");
    replaced = replaced.replace("$$DURATION$$", "__VIDEO_TIME__");
    replaced = replaced.replace("$$PLAY_BEGIN_TIME$$", "__VIDEO_BEGIN_TIME__");
    replaced = replaced.replace("$$PLAY_END_TIME$$", "__VIDEO_END_TIME__");
    replaced = replaced.replace("$$PLAY_FIRST_FRAME$$", "__VIDEO_FIRST_FRAME__");
    replaced = replaced.replace("$$PLAY_LAST_FRAME$$", "__VIDEO_LAST_FRAME__");
    replaced = replaced.replace("$$PLAY_SCEN$$", "__VIDEO_PLAY_SCENE__");
    replaced = replaced.replace("$$PLAY_TYPE$$", "__VIDEO_PLAY_TYPE__");
    replaced = replaced.replace("$$PLAY_BEHAVIOR$$", "__VIDEO_PLAY_TRIGGER__");
    replaced = replaced.replace("$$TARGET_APP_INSTALL$$", "__DP_TARGET__");

    replaced
}
