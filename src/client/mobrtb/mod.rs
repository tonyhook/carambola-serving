use std::{collections::HashMap, time::Duration};

use chrono::{Datelike, Local};
use reqwest::Url;
use urlencoding::encode;

use crate::{protocol::*, Assets, Cache, Client, Connection, HttpPool, Identifiers, Price, ResultMessage};

pub mod ad_format;
pub mod ad;
pub mod app;
pub mod device;
pub mod image;
pub mod request;
pub mod response;
pub mod user;
pub mod video;

pub use ad_format::MobrtbAdFormat;
pub use ad::MobrtbAd;
pub use app::MobrtbApp;
pub use device::MobrtbDevice;
pub use image::MobrtbImage;
pub use request::MobrtbRequest;
pub use response::MobrtbResponse;
pub use user::MobrtbUser;
pub use video::MobrtbVideo;

pub struct Mobrtb {

}

impl Client for Mobrtb {

    async fn request(request: &Request, connection: &Connection, pool: &HttpPool, cache: &Cache) -> Result<Response, ResultMessage> {
        let unit_token = connection.client_tag_id.split("|").nth(0).unwrap();
        let media_token = connection.client_tag_id.split("|").nth(1).unwrap();

        let request_id = cache.get_sequence();

        let assets = Assets::new(request);
        let mut title_index = 0;
        let mut img_index = 0;
        let mut icon_index = 0;
        let mut video_index = 0;
        let mut video_cover_index = 0;
        let mut html_index = 0;
        let identifiers = Identifiers::new(request);

        let request_mobrtb = MobrtbRequest {
            id: {
                Some(request_id.to_string())
            },
            version: {
                "2.0.0".to_string()
            },
            ads: [MobrtbAdFormat {
                ad_unit_token: unit_token.to_string(),
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
                floor_price: Some(Price::to_client(connection, request.item[0].flr) as f64),
                support_js: None,
            }].to_vec(),
            app: {
                match &request.context.app {
                    Some(app) => MobrtbApp {
                        name: app.name.clone(),
                        version: {
                            match &app.ver {
                                Some(ver) => ver.clone(),
                                None => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.context.app.ver is required for upstream".to_string(),
                                }),
                            }
                        },
                        bundle: {
                            match &app.bundle {
                                Some(bundle) => bundle.clone(),
                                None => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.context.app.bundle is required for upstream".to_string(),
                                }),
                            }
                        },
                        deeplink_mode: Some(1),
                    },
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.app is required for upstream".to_string(),
                    }),
                }
            },
            device: MobrtbDevice {
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
                user_agent: request.context.device.ua.clone(),
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
                                2 => "android".to_string(),
                                13 => "ios".to_string(),
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
                connection_type: {
                    match &request.context.device.contype {
                        Some(contype) => {
                            match contype {
                                2 => "wifi".to_string(),
                                4 => "2g".to_string(),
                                5 => "3g".to_string(),
                                6 => "4g".to_string(),
                                7 => "5g".to_string(),
                                _ => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.context.device.contype should be 2/4/5/6/7 for upstream".to_string(),
                                }),
                            }
                        },
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.contype is required for upstream".to_string(),
                        }),
                    }
                },
                orientation: {
                    match request.context.device.orientation {
                        Some(orientation) => {
                            match orientation {
                                501 => "portrait".to_string(),
                                502 => "landscape".to_string(),
                                _ => return Err(ResultMessage {
                                    code: 998,
                                    message: "request.context.device.contype should be 501/502 for upstream".to_string(),
                                }),
                            }
                        },
                        None => return Err(ResultMessage {
                            code: 998,
                            message: "request.context.device.orientation is required for upstream".to_string(),
                        }),
                    }
                },
                plmn: {
                    match &request.context.device.carrier {
                        Some(carrier) => {
                            match carrier.as_str() {
                                "cmcc" => Some("46000".to_string()),
                                "unicom" => Some("46001".to_string()),
                                "telecom" => Some("46003".to_string()),
                                _ => None,
                            }
                        },
                        None => None,
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
                language: request.context.device.lang.clone(),
                screen_width: request.context.device.w,
                screen_height: request.context.device.h,
                screen_dpi: request.context.device.ppi.clone(),
                screen_pxratio: request.context.device.pxratio.clone(),
                geo_longitude: {
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
                geo_latitude: {
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
                installed_apps: request.context.device.app.clone(),
                device_type: {
                    match request.context.device.devicetype {
                        Some(4) => Some(1),
                        Some(5) => Some(2),
                        Some(3) => Some(3),
                        Some(2) => Some(4),
                        Some(_) => Some(0),
                        None => Some(0),
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
                android_id: {
                    match identifiers.get_id(509, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                android_id_md5: {
                    match identifiers.get_id(510, 0) {
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
                android_advertising_id: None,
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
                rom_version: request.context.device.romv.clone(),
                sys_compiling_time: {
                    match &request.context.device.romtime {
                        Some(romtime) => {
                            let rom_timestamp = romtime.split(".").nth(0).unwrap();
                            match rom_timestamp.parse::<i64>() {
                                Ok(rom_timestamp) => {
                                    Some(rom_timestamp)
                                },
                                Err(_) => None,
                            }
                        },
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
                idfv: {
                    match identifiers.get_id(515, 0) {
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
                caid_md5: {
                    match identifiers.get_id(513, 0) {
                        Some(uid) => Some(format!("{:x}", md5::compute(uid.id.as_bytes()))),
                        None => None,
                    }
                },
                caid_version: {
                    match identifiers.get_id(513, 0) {
                        Some(uid) => uid.ver.clone(),
                        None => None,
                    }
                },
                caid2: {
                    match identifiers.get_id(513, 1) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                caid2_md5: {
                    match identifiers.get_id(513, 1) {
                        Some(uid) => Some(format!("{:x}", md5::compute(uid.id.as_bytes()))),
                        None => None,
                    }
                },
                caid2_version: {
                    match identifiers.get_id(513, 1) {
                        Some(uid) => uid.ver.clone(),
                        None => None,
                    }
                },
                openudid: None,
                boot_mark: request.context.device.bootmark.clone(),
                update_mark: request.context.device.updatemark.clone(),
                paid: {
                    match identifiers.get_id(519, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                device_startup_time: request.context.device.boottime.clone(),
                system_update_time: request.context.device.updatetime.clone(),
                system_init_time: {
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
            },
            user: Some(MobrtbUser {
                age: {
                    match request.context.user.yob {
                        Some(yob) => {
                            let year = Local::now().year();
                            Some(year - yob)
                        },
                        None => None,
                    }
                },
                gender: request.context.user.gender.clone(),
                keywords: {
                    match &request.context.user.keywords {
                        Some(keywords) => Some(keywords.split(",").map(|s| s.to_string()).collect()),
                        None => None,
                    }
                },
            }),
            need_https: None,
        };

        let response_mobrtb: MobrtbResponse;

        let client = {
            let pool_mobrtb_lock = pool.pool_mobrtb.clone();
            let pool_mobrtb = pool_mobrtb_lock.read().unwrap();
            pool_mobrtb.clone()
        };
        let response_mobrtb_raw = client.post(format!("{}{}", "https://api.mobrtb.com/ad/xy/", media_token).as_str())
            .json(&request_mobrtb)
            .header("Accept-Encoding", "gzip")
            .header("Connection", "keep-alive")
            .header("Content-Type", "application/json")
            .timeout(Duration::from_millis(connection.timeout))
            .send().await;

        match response_mobrtb_raw {
            Ok(response_mobrtb_raw) => {
                let status = response_mobrtb_raw.status();
                if status == 204 {
                    return Err(ResultMessage {
                        code: 993,
                        message: "".to_string(),
                    });
                } else if status != 200 {
                    return Err(ResultMessage {
                        code: 992,
                        message: {
                            match response_mobrtb_raw.text().await {
                                Ok(text) => format!("upstream error {}: {}", status, text),
                                Err(_) => format!("upstream error {}", status),
                            }
                        },
                    });
                } else {
                    match response_mobrtb_raw.text().await {
                        Ok(text) => {
                            match serde_json::from_str::<MobrtbResponse>(&text) {
                                Ok(json) => {
                                    response_mobrtb = json;
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

                for ad_mobrtb in &response_mobrtb.ads {
                    let link_asset = LinkAsset {
                        linktype: {
                            match &ad_mobrtb.action {
                                1 => 1,
                                2 => 1,
                                6 => 2,
                                7 => 1,
                                8 => 1,
                                _ => 1,
                            }
                        },
                        universallink: None,
                        storeid: None,
                        deeplink: {
                            match &ad_mobrtb.deeplink_url {
                                Some(deeplink_url) => Some(replace_macro(deeplink_url)),
                                None => None,
                            }
                        },
                        quickapplink: None,
                        wechatmppath: ad_mobrtb.mini_program_path.clone(),
                        wechatmpid: ad_mobrtb.mini_program_id.clone(),
                        marketurl: None,
                        downloadurl: None,
                        url: replace_macro(&ad_mobrtb.target_url),
                        urlfb: None,
                    };

                    let bid = Bid {
                        id: Some(request_id.to_string()),
                        item: request.item[0].id.clone(),
                        price: { // update later
                            match ad_mobrtb.price {
                                Some(price) => price as i32,
                                None => connection.default_price,
                            }
                        },
                        burl: {
                            let mut burl = Vec::<String>::new();
                            let mut nurl = ad_mobrtb.win_notice_tracker.clone();
                            nurl = nurl.replace("{XY_PRICE}", "__WIN_PRICE__");
                            burl.push(replace_macro(&nurl));
                            Some(burl)
                        },
                        lurl: None,
                        media: Ad {
                            id: ad_mobrtb.ad_id.to_string(),
                            display: {
                                let mut display = Display {
                                    w: None,
                                    h: None,
                                    banner: None,
                                    native: None,
                                    event: vec![],
                                };

                                let mut asset_vec = vec![];

                                if request.item[0].spec.display.displayfmt.is_some() {
                                    match &ad_mobrtb.images {
                                        Some(images) => {
                                            display.w = Some(ad_mobrtb.width);
                                            display.h = Some(ad_mobrtb.height);

                                            if images.len() > 0 {
                                                display.banner = Some(Banner {
                                                    img: images[0].url.clone(),
                                                    link: Some(link_asset.clone()),
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                }
                                if request.item[0].spec.display.nativefmt.is_some() {
                                    match &ad_mobrtb.title {
                                        Some(title) => {
                                            if assets.title_asset.len() - title_index > 0 {
                                                asset_vec.push(Asset {
                                                    id: assets.title_asset.get(title_index).unwrap().id,
                                                    req: 1,
                                                    title: Some(TitleAsset {
                                                        text: title.clone(),
                                                        subtitle: ad_mobrtb.subtitle.clone(),
                                                        desc: {
                                                            match &ad_mobrtb.description {
                                                                Some(description) => Some(description.clone()),
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

                                                title_index += 1;
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_mobrtb.images {
                                        Some(images) => {
                                            for image in images {
                                                if assets.img_asset.len() - img_index > 0 {
                                                    asset_vec.push(Asset {
                                                        id: assets.img_asset.get(img_index).unwrap().id,
                                                        req: 1,
                                                        title: None,
                                                        img: Some(ImageAsset {
                                                            url: image.url.clone(),
                                                            mime: None,
                                                            w: image.width,
                                                            h: image.height,
                                                            imagetype: Some(3),
                                                        }),
                                                        video: None,
                                                        data: None,
                                                        html: None,
                                                        app: None,
                                                    });

                                                    img_index += 1;
                                                }
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_mobrtb.icon {
                                        Some(icon) => {
                                            if assets.icon_asset.len() - icon_index > 0 {
                                                asset_vec.push(Asset {
                                                    id: assets.icon_asset.get(icon_index).unwrap().id,
                                                    req: 1,
                                                    title: None,
                                                    img: Some(ImageAsset {
                                                        url: icon.url.clone(),
                                                        mime: None,
                                                        w: icon.width,
                                                        h: icon.height,
                                                        imagetype: Some(1),
                                                    }),
                                                    video: None,
                                                    data: None,
                                                    html: None,
                                                    app: None,
                                                });

                                                icon_index += 1;
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_mobrtb.logo {
                                        Some(logo) => {
                                            if assets.icon_asset.len() - icon_index > 0 {
                                                asset_vec.push(Asset {
                                                    id: assets.icon_asset.get(icon_index).unwrap().id,
                                                    req: 1,
                                                    title: None,
                                                    img: Some(ImageAsset {
                                                        url: logo.url.clone(),
                                                        mime: None,
                                                        w: logo.width,
                                                        h: logo.height,
                                                        imagetype: Some(1),
                                                    }),
                                                    video: None,
                                                    data: None,
                                                    html: None,
                                                    app: None,
                                                });

                                                icon_index += 1;
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_mobrtb.video {
                                        Some(video) => {
                                            if assets.video_asset.len() - video_index > 0 {
                                                asset_vec.push(Asset {
                                                    id: assets.video_asset.get(video_index).unwrap().id,
                                                    req: 1,
                                                    title: None,
                                                    img: None,
                                                    video: Some(VideoAsset {
                                                        url: video.url.clone(),
                                                        mime: None,
                                                        w: None,
                                                        h: None,
                                                        dur: video.duration,
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
                                                });

                                                video_index += 1;
                                            }

                                            match &ad_mobrtb.video_cover {
                                                Some(video_cover) => {
                                                    if assets.video_cover_asset.len() - video_cover_index > 0 {
                                                        asset_vec.push(Asset {
                                                            id: assets.video_cover_asset.get(video_cover_index).unwrap().id,
                                                            req: 1,
                                                            title: None,
                                                            img: Some(ImageAsset {
                                                                url: video_cover.url.clone(),
                                                                mime: None,
                                                                w: video_cover.width,
                                                                h: video_cover.height,
                                                                imagetype: Some(3),
                                                            }),
                                                            video: None,
                                                            data: None,
                                                            html: None,
                                                            app: None,
                                                        });

                                                        video_cover_index += 1;
                                                    }
                                                },
                                                None => (),
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_mobrtb.ratings {
                                        Some(ratings) => {
                                            for asset in &assets.data_asset {
                                                if asset.data.clone().unwrap().datatype == 501 {
                                                    asset_vec.push(Asset {
                                                        id: asset.id,
                                                        req: 1,
                                                        title: None,
                                                        img: None,
                                                        video: None,
                                                        data: Some(DataAsset {
                                                            value: ratings.clone(),
                                                            len: None,
                                                            datatype: Some(501),
                                                        }),
                                                        html: None,
                                                        app: None,
                                                    });
                                                }
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_mobrtb.button_text {
                                        Some(button_text) => {
                                            for asset in &assets.data_asset {
                                                if asset.data.clone().unwrap().datatype == 12 {
                                                    asset_vec.push(Asset {
                                                        id: asset.id,
                                                        req: 1,
                                                        title: None,
                                                        img: None,
                                                        video: None,
                                                        data: Some(DataAsset {
                                                            value: button_text.clone(),
                                                            len: None,
                                                            datatype: Some(12),
                                                        }),
                                                        html: None,
                                                        app: None,
                                                    });
                                                }
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_mobrtb.likes {
                                        Some(likes) => {
                                            for asset in &assets.data_asset {
                                                if asset.data.clone().unwrap().datatype == 4 {
                                                    asset_vec.push(Asset {
                                                        id: asset.id,
                                                        req: 1,
                                                        title: None,
                                                        img: None,
                                                        video: None,
                                                        data: Some(DataAsset {
                                                            value: likes.clone(),
                                                            len: None,
                                                            datatype: Some(4),
                                                        }),
                                                        html: None,
                                                        app: None,
                                                    });
                                                }
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_mobrtb.downloads {
                                        Some(downloads) => {
                                            for asset in &assets.data_asset {
                                                if asset.data.clone().unwrap().datatype == 5 {
                                                    asset_vec.push(Asset {
                                                        id: asset.id,
                                                        req: 1,
                                                        title: None,
                                                        img: None,
                                                        video: None,
                                                        data: Some(DataAsset {
                                                            value: downloads.clone(),
                                                            len: None,
                                                            datatype: Some(5),
                                                        }),
                                                        html: None,
                                                        app: None,
                                                    });
                                                }
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad_mobrtb.html_snippet {
                                        Some(html_snippet) => {
                                            if assets.html_asset.len() - html_index > 0 {
                                                asset_vec.push(Asset {
                                                    id: assets.html_asset.get(html_index).unwrap().id,
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

                                                html_index += 1;
                                            }
                                        }
                                        None => (),
                                    }
                                    match &ad_mobrtb.html_url {
                                        Some(html_url) => {
                                            if assets.html_asset.len() - html_index > 0 {
                                                asset_vec.push(Asset {
                                                    id: assets.html_asset.get(html_index).unwrap().id,
                                                    req: 1,
                                                    title: None,
                                                    img: None,
                                                    video: None,
                                                    data: None,
                                                    html: Some(HtmlAsset {
                                                        html: None,
                                                        link: Some(html_url.clone()),
                                                        len: None,
                                                    }),
                                                    app: None,
                                                });

                                                html_index += 1;
                                            }
                                        }
                                        None => (),
                                    }
                                }

                                match &ad_mobrtb.download_app_name {
                                    Some(download_app_name) => {
                                        let asset = Asset {
                                            id:(assets.asset_size + 1) as i32,
                                            req: 0,
                                            title: None,
                                            img: None,
                                            video: None,
                                            data: None,
                                            html: None,
                                            app: Some(AppAsset {
                                                name: download_app_name.clone(),
                                                desc: None,
                                                descurl: None,
                                                domain: None,
                                                bundle: ad_mobrtb.download_app_bundle.clone(),
                                                ver: ad_mobrtb.download_app_version.clone(),
                                                developer: None,
                                                icon: None,
                                                storeid: None,
                                                storeurl: None,
                                                paid: 0,
                                                size: ad_mobrtb.download_app_size,
                                                md5: None,
                                                registration: None,
                                                privacy: None,
                                                privacyurl: ad_mobrtb.privacy_url.clone(),
                                                permission: None,
                                                permissionurl: ad_mobrtb.permission_url.clone(),
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

                                for event in &ad_mobrtb.impression_trackers {
                                    event_vec.push(Event {
                                        eventtype: 501,
                                        method: 1,
                                        url: {
                                            let url = replace_macro(event);
                                            let price = match ad_mobrtb.price {
                                                Some(price) => {
                                                    if price > 0.0 {
                                                        price as i32
                                                    } else {
                                                        connection.default_price
                                                    }
                                                },
                                                None => connection.default_price,
                                            };
                                            let encrypt_price = Self::encrypt_price(price, &"".to_string(), connection);
                                            url.replace("{XY_PRICE}", &encode(encrypt_price.as_str()))
                                        },
                                        header: None,
                                        content: None,
                                    });
                                }
                                for event in &ad_mobrtb.click_trackers {
                                    event_vec.push(Event {
                                        eventtype: 502,
                                        method: 1,
                                        url: replace_macro(event),
                                        header: None,
                                        content: None,
                                    });
                                }
                                match &ad_mobrtb.download_begin_trackers {
                                    Some(download_begin_trackers) => {
                                        for event in download_begin_trackers {
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
                                match &ad_mobrtb.download_ended_trackers {
                                    Some(download_ended_trackers) => {
                                        for event in download_ended_trackers {
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
                                match &ad_mobrtb.install_begin_trackers {
                                    Some(install_begin_trackers) => {
                                        for event in install_begin_trackers {
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
                                match &ad_mobrtb.install_ended_trackers {
                                    Some(install_ended_trackers) => {
                                        for event in install_ended_trackers {
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
                                match &ad_mobrtb.video_play_begin_trackers {
                                    Some(video_play_begin_trackers) => {
                                        for event in video_play_begin_trackers {
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
                                match &ad_mobrtb.video_play_break_trackers {
                                    Some(video_play_break_trackers) => {
                                        for event in video_play_break_trackers {
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
                                match &ad_mobrtb.video_play_ended_trackers {
                                    Some(video_play_ended_trackers) => {
                                        for event in video_play_ended_trackers {
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
                                match &ad_mobrtb.deeplink_app_not_installed_trackers {
                                    Some(deeplink_app_not_installed_trackers) => {
                                        for event in deeplink_app_not_installed_trackers {
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
                                match &ad_mobrtb.deeplink_app_installed_trackers {
                                    Some(deeplink_app_installed_trackers) => {
                                        for event in deeplink_app_installed_trackers {
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
                                match &ad_mobrtb.deeplink_app_invoke_failed_trackers {
                                    Some(deeplink_app_invoke_failed_trackers) => {
                                        for event in deeplink_app_invoke_failed_trackers {
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
                                match &ad_mobrtb.deeplink_app_invoke_success_trackers {
                                    Some(deeplink_app_invoke_success_trackers) => {
                                        for event in deeplink_app_invoke_success_trackers {
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
                                match &ad_mobrtb.click_area_report_url {
                                    Some(click_area_report_url) => {
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
                                    },
                                    None => (),
                                }

                                display.event = event_vec;

                                display
                            },
                            advertiser: ad_mobrtb.advertiser_name.clone(),
                            advertisericon: None,
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
            let pool_mobrtb_lock = pool.pool_mobrtb.clone();
            let pool_mobrtb = pool_mobrtb_lock.read().unwrap();
            pool_mobrtb.clone()
        };
        let _ = client.get(replaced_url).send().await;
    }

    async fn bidding_notify_lose(_url: String, _lose_price: i32, _lose_reason: i32, _lose_adn_name: &String, _iv: &String, _connection: &Connection, _pool: &HttpPool) {

    }

    fn encrypt_price(price: i32, _iv: &String, _connection: &Connection) -> String {
        price.to_string()
    }

}

fn replace_macro(orig: &String) -> String {
    let mut replaced = orig.clone();

    replaced = replaced.replace("{XY_CLICK_DOWN_X}", "__DOWN_X__");
    replaced = replaced.replace("{XY_CLICK_DOWN_Y}", "__DOWN_Y__");
    replaced = replaced.replace("{XY_CLICK_UP_X}", "__UP_X__");
    replaced = replaced.replace("{XY_CLICK_UP_Y}", "__UP_Y__");
    replaced = replaced.replace("{XY_CLICK_DOWN_LX}", "__DOWN_X__");
    replaced = replaced.replace("{XY_CLICK_DOWN_LY}", "__DOWN_Y__");
    replaced = replaced.replace("{XY_CLICK_UP_LX}", "__UP_X__");
    replaced = replaced.replace("{XY_CLICK_UP_LY}", "__UP_Y__");

    replaced
}
