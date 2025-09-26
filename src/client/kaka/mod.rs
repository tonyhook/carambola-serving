use std::time::Duration;

use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyInit};
use base64::prelude::*;
use chrono::{Datelike, Local};
use urlencoding::encode;

use crate::{protocol::*, Assets, Cache, Client, Connection, HttpPool, Identifiers, Price, ResultMessage};

type Aes128EcbEnc = ecb::Encryptor<aes::Aes128>;

pub mod ad;
pub mod app;
pub mod app_info;
pub mod accepted_size;
pub mod caid;
pub mod creative;
pub mod device;
pub mod geo;
pub mod image;
pub mod imp;
pub mod media;
pub mod network;
pub mod request;
pub mod response;
pub mod user;

pub use ad::KakaAd;
pub use app::KakaApp;
pub use app_info::KakaAppInfo;
pub use accepted_size::KakaAcceptedSize;
pub use caid::KakaCaid;
pub use creative::KakaCreative;
pub use device::KakaDevice;
pub use geo::KakaGeo;
pub use image::KakaImage;
pub use imp::KakaImp;
pub use media::KakaMedia;
pub use network::KakaNetwork;
pub use request::KakaRequest;
pub use response::KakaResponse;
pub use user::KakaUser;

pub struct Kaka {

}

impl Client for Kaka {

    async fn request(request: &Request, connection: &Connection, pool: &HttpPool, cache: &Cache) -> Result<Response, ResultMessage> {
        let id = connection.client_tag_id.clone();

        let request_id = cache.get_sequence();

        let mut assets = Assets::new(request);
        let identifiers = Identifiers::new(request);

        let request_kaka = KakaRequest {
            request_id: {
                request_id.to_string()
            },
            api_version: {
                "1.5".to_string()
            },
            device: KakaDevice {
                android_id: {
                    match identifiers.get_id(509, 0) {
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
                idfa: {
                    match identifiers.get_id(507, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                caids: {
                    let mut caids = vec![];

                    match identifiers.get_id(513, 0) {
                        Some(uid) => {
                            caids.push(KakaCaid {
                                caid: {
                                    uid.id.clone()
                                },
                                version: {
                                    match &uid.ver {
                                        Some(ver) => ver.clone(),
                                        None => "".to_string(),
                                    }
                                },
                            });
                        },
                        None => (),
                    };

                    match identifiers.get_id(513, 1) {
                        Some(uid) => {
                            caids.push(KakaCaid {
                                caid: {
                                    uid.id.clone()
                                },
                                version: {
                                    match &uid.ver {
                                        Some(ver) => ver.clone(),
                                        None => "".to_string(),
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
                aaid: {
                    match identifiers.get_id(514, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                paid: {
                    match identifiers.get_id(519, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                device_type: {
                    match request.context.device.devicetype {
                        Some(devicetype) => {
                            match devicetype {
                                4 => 1,
                                5 => 2,
                                3 => 3,
                                _ => 0,
                            }
                        },
                        None => 0,
                    }
                },
                platform: {
                    match request.context.device.os {
                        Some(os) => {
                            match os {
                                2 => 0,
                                13 => 1,
                                501 => 2,
                                _ => 0,
                            }
                        },
                        None => 0,
                    }
                },
                os_version: {
                    request.context.device.osv.clone()
                },
                make: {
                    request.context.device.make.clone()
                },
                model: {
                    request.context.device.model.clone()
                },
                ua: {
                    request.context.device.ua.clone()
                },
                ip: {
                    match &request.context.device.ip {
                        Some(ip) => ip.clone(),
                        None => {
                            match &request.context.device.ipv6 {
                                Some(ipv6) => ipv6.clone(),
                                None => "".to_string(),
                            }
                        },
                    }
                },
                mac: {
                    match identifiers.get_id(511, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                network: {
                    KakaNetwork {
                        connection_type: {
                            match &request.context.device.contype {
                                Some(4) => Some("g2".to_string()),
                                Some(5) => Some("g3".to_string()),
                                Some(6) => Some("g4".to_string()),
                                Some(7) => Some("g5".to_string()),
                                Some(2) => Some("wifi".to_string()),
                                _ => Some("unknown".to_string()),
                            }
                        },
                        operator_type: {
                            match &request.context.device.carrier {
                                Some(carrier) => {
                                    match carrier.as_str() {
                                        "cmcc" => Some("M".to_string()),
                                        "unicom" => Some("C".to_string()),
                                        "telecom" => Some("T".to_string()),
                                        _ => Some("unknown".to_string()),
                                    }
                                },
                                None => Some("unknown".to_string()),
                            }
                        },
                    }
                },
                dpi: {
                    request.context.device.ppi.clone()
                },
                screen_height: {
                    match request.context.device.h {
                        Some(h) => h,
                        None => 0,
                    }
                },
                screen_width: {
                    match request.context.device.w {
                        Some(w) => w,
                        None => 0,
                    }
                },
                installed_app: {
                    request.context.device.app.clone()
                },
                boot_mark: {
                    request.context.device.bootmark.clone()
                },
                update_mark: {
                    request.context.device.updatemark.clone()
                },
                device_init_time: {
                    request.context.device.inittime.clone()
                },
                system_update_time: {
                    request.context.device.updatetime.clone()
                },
                system_boot_time: {
                    request.context.device.boottime.clone()
                },
                appstore_version: {
                    request.context.device.storev.clone()
                },
                hmscore_version: {
                    request.context.device.hmsv.clone()
                },
                app_list: {
                    match &request.context.device.app {
                        Some(app) => Some(app.split(",").map(|s| s.to_string()).collect()),
                        None => None,
                    }
                },
            },
            app: {
                match &request.context.app {
                    Some(app) => KakaApp {
                        name: app.name.clone(),
                        version: app.ver.clone(),
                        pkg_name: {
                            match app.bundle {
                                Some(ref bundle) => bundle.clone(),
                                None => "".to_string(),
                            }
                        },
                    },
                    None => KakaApp {
                        name: "".to_string(),
                        version: None,
                        pkg_name: "".to_string(),
                    },
                }
            },
            geo: {
                match &request.context.device.geo {
                    Some(geo) => Some(KakaGeo {
                        latitude: geo.lat,
                        longitude: geo.lon,
                    }),
                    None => None,
                }
            },
            user: Some(KakaUser {
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
                    request.context.user.keywords.clone()
                },
            }),
            imps: [KakaImp {
                id,
                ad_type: {
                    let mut ad_type = 0;
                    let instl = request.item[0].spec.display.instl;

                    if assets.get_banner_size() > 0 {
                        if instl == 1 {
                            ad_type = 1;
                        } else {
                            if request.item[0].spec.display.w > request.item[0].spec.display.h {
                                ad_type = 0;
                            } else {
                                ad_type = 2;
                            }
                        }
                    }
                    if assets.get_asset_size("img") > 0 {
                        ad_type = 3;
                    }
                    if assets.get_asset_size("thumb") > 0 {
                        ad_type = 3;
                    }
                    if assets.get_asset_size("video") > 0 {
                        if instl == 1 {
                            ad_type = 1;
                        } else {
                            ad_type = 2
                        }
                    }

                    ad_type
                },
                pos: {
                    match request.item[0].spec.display.pos {
                        Some(2) => Some(0),
                        Some(3) => Some(1),
                        Some(501) => Some(2),
                        Some(7) => {
                            match request.item[0].spec.display.instl {
                                1 => Some(3),
                                _ => Some(4),
                            }
                        },
                        _ => None,
                    }
                },
                accepted_size: {
                    [KakaAcceptedSize {
                        width: request.item[0].spec.display.w.clone(),
                        height: request.item[0].spec.display.h.clone(),
                    }].to_vec()
                },
                accepted_creative_types: {
                    None
                },
                accepted_interaction_type: {
                    None
                },
                bid_floor: {
                    Some(Price::to_client(connection, request.item[0].flr.map(f64::from)) as i32)
                },
            }].to_vec(),
        };

        let response_kaka: KakaResponse;

        let client = {
            let pool_kaka_lock = pool.pool_kaka.clone();
            let pool_kaka = pool_kaka_lock.read().unwrap();
            pool_kaka.clone()
        };
        let response_kaka_raw = client.post(format!("{}", "https://ssp.kakamobi.cn/api/web/v3/ssp/get.htm").as_str())
            .json(&request_kaka)
            .header("Accept-Encoding", "gzip")
            .header("Connection", "keep-alive")
            .header("Content-Type", "application/json;charset=UTF-8")
            .timeout(Duration::from_millis(connection.timeout))
            .send().await;

        match response_kaka_raw {
            Ok(response_kaka_raw) => {
                let status = response_kaka_raw.status();
                if status == 204 {
                    return Err(ResultMessage {
                        code: 993,
                        message: "".to_string(),
                    });
                } else if status != 200 {
                    return Err(ResultMessage {
                        code: 992,
                        message: "".to_string(),
                    });
                } else {
                    match response_kaka_raw.text().await {
                        Ok(text) => {
                            match serde_json::from_str::<KakaResponse>(&text) {
                                Ok(json) => {
                                    response_kaka = json;
                                    match response_kaka.success {
                                        true => {
                                            match &response_kaka.ads {
                                                Some(ads) => {
                                                    if ads.len() == 0 {
                                                        return Err(ResultMessage {
                                                            code: 993,
                                                            message: "".to_string(),
                                                        });
                                                    }
                                                }
                                                None => {
                                                    return Err(ResultMessage {
                                                        code: 993,
                                                        message: {
                                                            match &response_kaka.message {
                                                                Some(message) => format!("upstream error {}: {}", response_kaka.error_code, message),
                                                                None => format!("upstream error {}", response_kaka.error_code),
                                                            }
                                                        },
                                                    });
                                                },
                                            }
                                        },
                                        false => {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: {
                                                    match &response_kaka.message {
                                                        Some(message) => format!("upstream error {}: {}", response_kaka.error_code, message),
                                                        None => format!("upstream error {}", response_kaka.error_code),
                                                    }
                                                },
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
                let mut bids = vec![];

                for ad in &response_kaka.ads.unwrap() {
                    let link_asset = LinkAsset {
                        linktype: {
                            match &ad.creative.interaction_type {
                                Some(3) => 2,
                                _ => 1,
                            }
                        },
                        universallink: {
                            match &ad.creative.deeplink {
                                Some(deeplink_url) => Some(replace_macro(deeplink_url)),
                                None => None,
                            }
                        },
                        storeid: None,
                        deeplink: {
                            match &ad.creative.deeplink {
                                Some(deeplink_url) => Some(replace_macro(deeplink_url)),
                                None => None,
                            }
                        },
                        quickapplink: None,
                        wechatmppath: {
                            match &ad.creative.wechat_applet_path {
                                Some(wechat_applet_path) => Some(replace_macro(wechat_applet_path)),
                                None => None,
                            }
                        },
                        wechatmpid: {
                            match &ad.creative.wechat_applet_id {
                                Some(wechat_applet_id) => Some(replace_macro(wechat_applet_id)),
                                None => None,
                            }
                        },
                        marketurl: None,
                        downloadurl: None,
                        url: {
                            match &ad.creative.target_url {
                                Some(target_url) => replace_macro(target_url),
                                None => "".to_string(),
                            }
                        },
                        urlfb: None,
                    };

                    let bid = Bid {
                        id: Some(request_id.to_string()),
                        item: request.item[0].id.clone(),
                        price: { // update later
                            match ad.bid_price {
                                Some(bid_price) => bid_price as i32,
                                None => connection.default_price,
                            }
                        },
                        burl: {
                            let mut burl = Vec::<String>::new();
                            for win_notice_url in ad.creative.win_notice_url.clone() {
                                let nurl = win_notice_url.clone();
                                let nurl = nurl.replace("{MC_SSP_PRICE}", "__WIN_PRICE__");
                                burl.push(replace_macro(&nurl));
                            }
                            Some(burl)
                        },
                        lurl: None,
                        media: Ad {
                            id: {
                                match &ad.id {
                                    Some(id) => id.clone(),
                                    None => "".to_string(),
                                }
                            },
                            display: {
                                let mut display = Display {
                                    w: None,
                                    h: None,
                                    banner: None,
                                    native: None,
                                    event: vec![],
                                };

                                let mut asset_vec = vec![];

                                if assets.get_banner_size() > 0 {
                                    match &ad.creative.images {
                                        Some(images) => {
                                            if images.len() > 0 && images[0].url.is_some() {
                                                display.w = {
                                                    match &images[0].width {
                                                        Some(width) => {
                                                            match width.parse::<i32>() {
                                                                Ok(parsed_width) => Some(parsed_width),
                                                                Err(_) => None,
                                                            }
                                                        },
                                                        None => None
                                                    }
                                                };
                                                display.h = {
                                                    match &images[0].height {
                                                        Some(height) => {
                                                            match height.parse::<i32>() {
                                                                Ok(parsed_height) => Some(parsed_height),
                                                                Err(_) => None,
                                                            }
                                                        },
                                                        None => None
                                                    }
                                                };
                                                display.banner = Some(Banner {
                                                    img: images[0].url.clone().unwrap(),
                                                    link: Some(link_asset.clone()),
                                                });
                                            }
                                        },
                                        None => (),
                                    }
                                }
                                if assets.get_asset_total_size() > 0 {
                                    match &ad.creative.title {
                                        Some(title) => {
                                            asset_vec.push(Asset {
                                                id: assets.consume_asset("title"),
                                                req: 1,
                                                title: Some(TitleAsset {
                                                    text: title.clone(),
                                                    subtitle: None,
                                                    desc: {
                                                        match &ad.creative.description {
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
                                        },
                                        None => (),
                                    }
                                    match &ad.creative.images {
                                        Some(images) => {
                                            if assets.get_asset_size("img") > 0 {
                                                for image in images {
                                                    match &image.url {
                                                        Some(url) => {
                                                            if url.len() > 0 {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("img"),
                                                                    req: 1,
                                                                    title: None,
                                                                    img: Some(ImageAsset {
                                                                        url: url.clone(),
                                                                        mime: None,
                                                                        w: {
                                                                            match &image.width {
                                                                                Some(width) => {
                                                                                    match width.parse::<i32>() {
                                                                                        Ok(parsed_width) => Some(parsed_width),
                                                                                        Err(_) => None,
                                                                                    }
                                                                                },
                                                                                None => None
                                                                            }
                                                                        },
                                                                        h: {
                                                                            match &image.height {
                                                                                Some(height) => {
                                                                                    match height.parse::<i32>() {
                                                                                        Ok(parsed_height) => Some(parsed_height),
                                                                                        Err(_) => None,
                                                                                    }
                                                                                },
                                                                                None => None
                                                                            }
                                                                        },
                                                                        imagetype: Some(3),
                                                                    }),
                                                                    video: None,
                                                                    data: None,
                                                                    html: None,
                                                                    app: None,
                                                                });
                                                            }
                                                        },
                                                        None => continue,
                                                    }
                                                }
                                            }
                                            if assets.get_asset_size("thumb") > 0 {
                                                for image in images {
                                                    match &image.url {
                                                        Some(url) => {
                                                            if url.len() > 0 {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("thumb"),
                                                                    req: 1,
                                                                    title: None,
                                                                    img: Some(ImageAsset {
                                                                        url: url.clone(),
                                                                        mime: None,
                                                                        w: {
                                                                            match &image.width {
                                                                                Some(width) => {
                                                                                    match width.parse::<i32>() {
                                                                                        Ok(parsed_width) => Some(parsed_width),
                                                                                        Err(_) => None,
                                                                                    }
                                                                                },
                                                                                None => None
                                                                            }
                                                                        },
                                                                        h: {
                                                                            match &image.height {
                                                                                Some(height) => {
                                                                                    match height.parse::<i32>() {
                                                                                        Ok(parsed_height) => Some(parsed_height),
                                                                                        Err(_) => None,
                                                                                    }
                                                                                },
                                                                                None => None
                                                                            }
                                                                        },
                                                                        imagetype: Some(501),
                                                                    }),
                                                                    video: None,
                                                                    data: None,
                                                                    html: None,
                                                                    app: None,
                                                                });
                                                            }
                                                        },
                                                        None => continue,
                                                    }
                                                }
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad.creative.icon {
                                        Some(icon) => {
                                            match &icon.url {
                                                Some(url) => {
                                                    if url.len() > 0 {
                                                        asset_vec.push(Asset {
                                                            id: assets.consume_asset("icon"),
                                                            req: 1,
                                                            title: None,
                                                            img: Some(ImageAsset {
                                                                url: url.clone(),
                                                                mime: None,
                                                                w: {
                                                                    match &icon.width {
                                                                        Some(width) => {
                                                                            match width.parse::<i32>() {
                                                                                Ok(parsed_width) => Some(parsed_width),
                                                                                Err(_) => None,
                                                                            }
                                                                        },
                                                                        None => None
                                                                    }
                                                                },
                                                                h: {
                                                                    match &icon.height {
                                                                        Some(height) => {
                                                                            match height.parse::<i32>() {
                                                                                Ok(parsed_height) => Some(parsed_height),
                                                                                Err(_) => None,
                                                                            }
                                                                        },
                                                                        None => None
                                                                    }
                                                                },
                                                                imagetype: Some(1),
                                                            }),
                                                            video: None,
                                                            data: None,
                                                            html: None,
                                                            app: None,
                                                        });
                                                    }
                                                },
                                                None => continue,
                                            }
                                        },
                                        None => (),
                                    }
                                    match &ad.creative.media {
                                        Some(media) => {
                                            asset_vec.push(Asset {
                                                id: assets.consume_asset("video"),
                                                req: 1,
                                                title: None,
                                                img: None,
                                                video: Some(VideoAsset {
                                                    url: media.url.clone(),
                                                    mime: None,
                                                    w: Some(media.width),
                                                    h: Some(media.height),
                                                    dur: Some(media.duration),
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

                                            asset_vec.push(Asset {
                                                id: assets.consume_asset("video#cover"),
                                                req: 1,
                                                title: None,
                                                img: Some(ImageAsset {
                                                    url: media.first_frame.clone(),
                                                    mime: None,
                                                    w: None,
                                                    h: None,
                                                    imagetype: Some(3),
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

                                match &ad.creative.app_info {
                                    Some(app_info) => {
                                        let asset = Asset {
                                            id: assets.consume_asset("app"),
                                            req: 0,
                                            title: None,
                                            img: None,
                                            video: None,
                                            data: None,
                                            html: None,
                                            app: Some(AppAsset {
                                                name: app_info.app_name.clone(),
                                                desc: None,
                                                descurl: Some(app_info.app_description_url.clone()),
                                                domain: None,
                                                bundle: app_info.package_name.clone(),
                                                ver: Some(app_info.version_name.clone()),
                                                developer: Some(app_info.company_name.clone()),
                                                icon: app_info.icon_url.clone(),
                                                storeid: None,
                                                storeurl: app_info.download_url.clone(),
                                                paid: 0,
                                                size: None,
                                                md5: None,
                                                registration: None,
                                                privacy: None,
                                                privacyurl: Some(app_info.privacy_policy_url.clone()),
                                                permission: None,
                                                permissionurl: Some(app_info.app_permissions_url.clone()),
                                            }),
                                        };

                                        asset_vec.push(asset);
                                    },
                                    None => (),
                                }

                                if assets.get_asset_total_size() > 0 {
                                    display.native = Some(Native {
                                        asset: asset_vec,
                                        link: Some(link_asset.clone()),
                                    });
                                }

                                let mut event_vec = vec![];

                                match &ad.creative.show_url {
                                    Some(show_url) => {
                                        for event in show_url {
                                            event_vec.push(Event {
                                                eventtype: 501,
                                                method: 1,
                                                url: {
                                                    let url = replace_macro(event);
                                                    let price = match ad.bid_price {
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
                                                    url.replace("{MC_SSP_PRICE}", &encode(encrypt_price.as_str()))
                                                },
                                                header: None,
                                                content: None,
                                            });
                                        }
                                    },
                                    None => (),
                                }
                                match &ad.creative.click_url {
                                    Some(click_url) => {
                                        for event in click_url {
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
                                match &ad.creative.dpl_success {
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
                                match &ad.creative.dpl_fail {
                                    Some(dpl_fail) => {
                                        for event in dpl_fail {
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
                                match &ad.creative.download_start {
                                    Some(download_start) => {
                                        for event in download_start {
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
                                match &ad.creative.download_finish {
                                    Some(download_finish) => {
                                        for event in download_finish {
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
                                match &ad.creative.install_start {
                                    Some(install_start) => {
                                        for event in install_start {
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
                                match &ad.creative.install_finish {
                                    Some(install_finish) => {
                                        for event in install_finish {
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

                                display.event = event_vec;

                                display
                            },
                            advertiser: None,
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
            let pool_kaka_lock = pool.pool_kaka.clone();
            let pool_kaka = pool_kaka_lock.read().unwrap();
            pool_kaka.clone()
        };
        let _ = client.get(replaced_url).send().await;
    }

    async fn bidding_notify_lose(_url: String, _lose_price: i32, _lose_reason: i32, _lose_adn_name: &String, _iv: &String, _connection: &Connection, _pool: &HttpPool) {

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

        let key = connection.client_ekey.as_bytes();

        let cipher = Aes128EcbEnc::new(key[0..16].into())
            .encrypt_padded_mut::<Pkcs7>(&mut buffer, pos)
            .unwrap();

        let message_base64 = BASE64_STANDARD.encode(cipher);

        message_base64.replace("+", "-").replace("/", "_").replace("=", "")
    }

}

fn replace_macro(orig: &String) -> String {
    let mut replaced = orig.clone();

    replaced = replaced.replace("__TIMESTAMP__", "__TS__");

    replaced
}
