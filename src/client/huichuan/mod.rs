use std::collections::HashMap;

use reqwest::Url;
use urlencoding::encode;

use crate::{protocol::*, Assets, Cache, Client, Connection, HttpPool, Identifiers, Price, ResultMessage};

pub mod ad_action;
pub mod ad_app_info;
pub mod ad_content;
pub mod ad_device_info;
pub mod ad_ext_info;
pub mod ad_gps_info;
pub mod ad_pos_ext_info;
pub mod ad_pos_info;
pub mod ad_user_info;
pub mod ad;
pub mod exp_tags;
pub mod ext_info;
pub mod huichuan_ext_info;
pub mod page_info;
pub mod request;
pub mod res_info;
pub mod response;
pub mod slot_ad;
pub mod video;

pub use ad_action::HuichuanAdAction;
pub use ad_app_info::HuichuanAdAppInfo;
pub use ad_content::HuichuanAdContent;
pub use ad_device_info::HuichuanAdDeviceInfo;
pub use ad_ext_info::HuichuanAdExtInfo;
pub use ad_gps_info::HuichuanAdGpsInfo;
pub use ad_pos_ext_info::HuichuanAdPosExtInfo;
pub use ad_pos_info::HuichuanAdPosInfo;
pub use ad_user_info::HuichuanAdUserInfo;
pub use ad::HuichuanAd;
pub use exp_tags::HuichuanExpTags;
pub use ext_info::HuichuanExtInfo;
pub use huichuan_ext_info::HuichuanHuichuanExtInfo;
pub use page_info::HuichuanPageInfo;
pub use request::HuichuanRequest;
pub use res_info::HuichuanResInfo;
pub use response::HuichuanResponse;
pub use slot_ad::HuichuanSlotAd;
pub use video::HuichuanVideo;

pub struct Huichuan {

}

impl Client for Huichuan {

    async fn request(request: &Request, connection: &Connection, pool: &HttpPool, cache: &Cache) -> Result<Response, ResultMessage> {
        let slot_id = connection.client_tag_id.split("|").nth(0).unwrap();

        let request_id = cache.get_sequence();

        let mut assets = Assets::new(request);
        let identifiers = Identifiers::new(request);

        let request_huichuan = HuichuanRequest {
            ad_device_info: HuichuanAdDeviceInfo {
                android_id: {
                    match identifiers.get_id(509, 0) {
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
                aaid: {
                    match identifiers.get_id(514, 0) {
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
                caid: {
                    match identifiers.get_id(513, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                udid: {
                    None
                },
                open_udid: {
                    None
                },
                dit: {
                    match &request.context.device.inittime {
                        Some(inittime) => Some(inittime.clone()),
                        None => None,
                    }
                },
                sut: {
                    match &request.context.device.updatetime {
                        Some(updatetime) => Some(updatetime.clone()),
                        None => None,
                    }
                },
                sst: {
                    match &request.context.device.boottime {
                        Some(boottime) => Some(boottime.clone()),
                        None => None,
                    }
                },
                client_ip: {
                    match &request.context.device.ip {
                        Some(ip) => Some(ip.clone()),
                        None => {
                            match &request.context.device.ipv6 {
                                Some(ipv6) => Some(ipv6.clone()),
                                None => None,
                            }
                        },
                    }
                },
                osv: {
                    match &request.context.device.osv {
                        Some(osv) => osv.clone(),
                        None => "".to_string(),
                    }
                },
                os: {
                    match &request.context.device.os {
                        Some(2) => "android".to_string(),
                        Some(13) => "ios".to_string(),
                        Some(28) => "wp".to_string(),
                        _ => "other".to_string(),
                    }
                },
                is_jb: {
                    None
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
                brand: {
                    match &request.context.device.brand {
                        Some(brand) => brand.clone(),
                        None => "".to_string(),
                    }
                },
                device: {
                    match &request.context.device.model {
                        Some(model) => model.clone(),
                        None => "".to_string(),
                    }
                },
                access: {
                    match &request.context.device.contype {
                        Some(2) => "Wi-Fi".to_string(),
                        Some(4) => "2G".to_string(),
                        Some(5) => "3G".to_string(),
                        Some(6) => "4G".to_string(),
                        Some(7) => "5G".to_string(),
                        _ => "Unknown".to_string(),
                    }
                },
                cp: {
                    None
                },
                carrier: {
                    match &request.context.device.carrier {
                        Some(carrier) => {
                            match carrier.as_str() {
                                "cmcc" => Some("ChinaMobile".to_string()),
                                "unicom" => Some("ChinaUnicom".to_string()),
                                "telecom" => Some("ChinaTelecom".to_string()),
                                _ => Some("Unknown".to_string()),
                            }
                        },
                        None => None,
                    }
                },
                cpu: {
                    None
                },
                mac: {
                    match identifiers.get_id(511, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                nx: {
                    None
                },
                aid: {
                    match identifiers.get_id(514, 0) {
                        Some(uid) => Some(uid.id.clone()),
                        None => None,
                    }
                },
                hms_core_version: {
                    match &request.context.device.hmsv {
                        Some(hmsv) => Some(hmsv.clone()),
                        None => None,
                    }
                },
                ag_version: {
                    request.context.device.storev.clone()
                },
            },
            ad_app_info: HuichuanAdAppInfo {
                fr: {
                    match &request.context.device.os {
                        Some(2) => "android".to_string(),
                        Some(13) => "ios".to_string(),
                        _ => "other".to_string(),
                    }
                },
                utdid: {
                    None
                },
                ua: {
                    request.context.device.ua.clone()
                },
                pkg_name: {
                    match &request.context.app {
                        Some(app) => {
                            match &connection.client_media_apppackage {
                                Some(client_media_apppackage) => client_media_apppackage.clone(),
                                None => match &app.bundle {
                                    Some(bundle) => bundle.clone(),
                                    None => "".to_string(),
                                }
                            }
                        },
                        None => match &connection.client_media_apppackage {
                            Some(client_media_apppackage) => client_media_apppackage.clone(),
                            None => "".to_string(),
                        }
                    }
                },
                pkg_ver: {
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
                app_name: {
                    match &request.context.app {
                        Some(app) => {
                            match &connection.client_media_appname {
                                Some(client_media_appname) => client_media_appname.clone(),
                                None => app.name.clone(),
                            }
                        },
                        None => match &connection.client_media_appname {
                            Some(client_media_appname) => client_media_appname.clone(),
                            None => "".to_string(),
                        }
                    }
                },
                installed_app_ids: {
                    match &request.context.device.app {
                        Some(app) => {
                            let mut installed_app_ids = Vec::<i32>::new();
                            for app in app.split(",") {
                                match get_app_id(app) {
                                    Some(app_id) => installed_app_ids.push(app_id),
                                    None => (),
                                }
                            }
                            Some(installed_app_ids)
                        },
                        None => None,
                    }
                },
                is_ssl: {
                    Some("1".to_string())
                },
                category: {
                    let mut category = Vec::<i32>::new();
                    category.push(105001);
                    category
                },
                dn: {
                    None
                },
                sn: {
                    None
                },
                app_country: {
                    None
                },
                lang: {
                    None
                },
                timezone: {
                    None
                },
            },
            ad_gps_info: Some(HuichuanAdGpsInfo {
                gps_time: {
                    match &request.context.device.geo {
                        Some(geo) => {
                            match geo.timestamp {
                                Some(timestamp) => Some(timestamp as i32),
                                None => None,
                            }
                        },
                        None => None,
                    }
                },
                lng: {
                    match &request.context.device.geo {
                        Some(geo) => {
                            geo.lon.clone()
                        },
                        None => None,
                    }
                },
                lat: {
                    match &request.context.device.geo {
                        Some(geo) => {
                            geo.lat.clone()
                        },
                        None => None,
                    }
                },
                amap_code: {
                    None
                },
            }),
            ad_pos_info: {
                let mut ad_pos_info = Vec::<HuichuanAdPosInfo>::new();
                ad_pos_info.push(HuichuanAdPosInfo {
                    req_cnt: 1,
                    query: None,
                    media_slot_id: {
                        request.item[0].spec.tagid.clone()
                    },
                    slot_id: {
                        slot_id.parse().unwrap()
                    },
                    slot_type: 0,
                    aw: {
                        request.item[0].spec.display.w.clone()
                    },
                    ah: {
                        request.item[0].spec.display.h.clone()
                    },
                    cpm_floor: {
                        Some(Price::to_client(connection, request.item[0].flr.map(f64::from)) as i32)
                    },
                    video_maxduration: {
                        if assets.get_asset_size("video") > 0 {
                            let video = assets.get_current_asset("video").unwrap().video.clone().unwrap();
                            video.maxdur.clone()
                        } else {
                            None
                        }
                    },
                    video_minduration: {
                        if assets.get_asset_size("video") > 0 {
                            let video = assets.get_current_asset("video").unwrap().video.clone().unwrap();
                            video.mindur.clone()
                        } else {
                            None
                        }
                    },
                    ad_pos_ext_info: None,
                    budget_pkg: None,
                });
                ad_pos_info
            },
            page_info: {
                None
            },
            res_info: {
                None
            },
            ext_info: {
                None
            },
            exp_tags: {
                None
            },
            huichuan_ext_info: {
                None
            },
            protocol_version: {
                None
            },
            request_id: {
                Some(request_id.to_string())
            },
            ad_user_info: {
                None
            },
        };

        let mut header = [0u8; 16];
        header[1] = 2;
        let json = serde_json::to_string(&request_huichuan).unwrap().as_bytes().to_vec();
        let body = [header.to_vec(), json].concat();

        let response_huichuan: HuichuanResponse;

        let client = {
            let pool_huichuan_lock = pool.pool_huichuan.clone();
            let pool_huichuan = pool_huichuan_lock.read().unwrap();
            pool_huichuan.clone()
        };
        let response_huichuan_raw = client.post(format!("{}", if connection.test { "https://test.huichuan.sm.cn/auto/nativead" } else { "https://hc-ssp.sm.cn/nativead" }))
            .body(body)
            .header("Accept-Encoding", "gzip")
            .header("Connection", "keep-alive")
            .send().await;

        match response_huichuan_raw {
            Ok(response_huichuan_raw) => {
                let status = response_huichuan_raw.status();
                if status != 200 {
                    return Err(ResultMessage {
                        code: 992,
                        message: {
                            match response_huichuan_raw.text().await {
                                Ok(text) => format!("upstream error {}: {}", status, text),
                                Err(_) => format!("upstream error {}", status),
                            }
                        },
                    });
                } else {
                    match response_huichuan_raw.bytes().await {
                        Ok(bytes) => {
                            let json = String::from_utf8(bytes[16..].to_vec()).unwrap();
                            match serde_json::from_str::<HuichuanResponse>(&json) {
                                Ok(json) => {
                                    response_huichuan = json;
                                    match response_huichuan.code.as_str() {
                                        "0" => (),
                                        code => {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: {
                                                    match &response_huichuan.reason {
                                                        Some(reason) => format!("upstream error {}: {}", code, reason),
                                                        None => format!("upstream error {}", code),
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
                match &response_huichuan.slot_ad[0].ad {
                    Some(ads) => {
                        Some([Seatbid {
                            bid: {
                                let mut bids = vec![];

                                for ad in ads {
                                    let link_asset = LinkAsset {
                                        linktype: {
                                            match ad.ad_action.action.as_str() {
                                                "tab" => 1,
                                                "download" => 2,
                                                _ => 1,
                                            }
                                        },
                                        universallink: {
                                            match &ad.ad_content.adm_fixed_ulk {
                                                Some(adm_fixed_ulk) => Some(adm_fixed_ulk.clone()),
                                                None => None,
                                            }
                                        },
                                        storeid: {
                                            None
                                        },
                                        deeplink: {
                                            match &ad.ad_content.scheme_url_ad {
                                                Some(scheme_url_ad) => Some(scheme_url_ad.clone()),
                                                None => {
                                                    match &ad.ad_content.scheme {
                                                        Some(scheme) => Some(scheme.clone()),
                                                        None => None,
                                                    }
                                                },
                                            }
                                        },
                                        quickapplink: None,
                                        wechatmppath: {
                                            match &ad.ad_content.mini_app_path {
                                                Some(mini_app_path) => Some(mini_app_path.clone()),
                                                None => None,
                                            }
                                        },
                                        wechatmpid: {
                                            match &ad.ad_content.mini_app_id {
                                                Some(mini_app_id) => Some(mini_app_id.clone()),
                                                None => None,
                                            }
                                        },
                                        marketurl: {
                                            match &ad.ad_content.market_direct_url {
                                                Some(market_direct_url) => Some(market_direct_url.clone()),
                                                None => None,
                                            }
                                        },
                                        downloadurl: {
                                            match &ad.ad_content.download_url {
                                                Some(download_url) => Some(download_url.clone()),
                                                None => {
                                                    match &ad.turl.len() {
                                                        2 => Some(ad.turl[1].clone()),
                                                        _ => None,
                                                    }
                                                },
                                            }
                                        },
                                        url: {
                                            ad.turl[0].clone()
                                        },
                                        urlfb: None,
                                    };

                                    let bid = Bid {
                                        id: Some(request_id.to_string()),
                                        item: request.item[0].id.clone(),
                                        price: { // update later
                                            ad.ad_content.dsp_bid_price.parse().unwrap()
                                        },
                                        burl: {
                                            match &ad.wnurl {
                                                Some (wnurl) => {
                                                    let mut burl = Vec::<String>::new();
                                                    let mut nurl = wnurl.clone();
                                                    nurl = nurl.replace("${AUCTION_ID}", request_id.to_string().as_str());
                                                    nurl = nurl.replace("${AUCTION_BID_ID}", response_huichuan.sid.as_str());
                                                    nurl = nurl.replace("${AUCTION_IMP_ID}", slot_id);
                                                    nurl = nurl.replace("${AUCTION_PRICE}", "__WIN_PRICE__");
                                                    burl.push(replace_macro(&nurl));
                                                    Some(burl)
                                                },
                                                None => None,
                                            }
                                        },
                                        lurl: {
                                            match &ad.lnurl {
                                                Some(lnurl) => {
                                                    let mut lurl = Vec::<String>::new();
                                                    let mut nurl = lnurl.clone();
                                                    nurl = nurl.replace("${AUCTION_ID}", request_id.to_string().as_str());
                                                    nurl = nurl.replace("${AUCTION_BID_ID}", response_huichuan.sid.as_str());
                                                    nurl = nurl.replace("${AUCTION_IMP_ID}", slot_id);
                                                    nurl = nurl.replace("${AUCTION_PRICE}", "__LOSE_PRICE__");
                                                    lurl.push(replace_macro(&nurl));
                                                    Some(lurl)
                                                },
                                                None => None,
                                            }
                                        },
                                        media: Ad {
                                            id: response_huichuan.sid.clone(),
                                            display: Display {
                                                w: {
                                                    None
                                                },
                                                h: {
                                                    None
                                                },
                                                banner: {
                                                    if assets.get_banner_size() > 0 {
                                                        Some(Banner {
                                                            img: {
                                                                ad.ad_content.img_1.clone()
                                                            },
                                                            link: Some(link_asset.clone()),
                                                        })
                                                    } else {
                                                        None
                                                    }
                                                },
                                                native: {
                                                    if assets.get_asset_total_size() > 0 {
                                                        let mut asset_vec = vec![];

                                                        match &ad.ad_content.video_aliyun {
                                                            Some(video_aliyun) => {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("video"),
                                                                    req: 1,
                                                                    video: Some(VideoAsset {
                                                                        url: {
                                                                            match &video_aliyun.fd {
                                                                                Some(fd) => fd.clone(),
                                                                                None => {
                                                                                    match &video_aliyun.ld {
                                                                                        Some(ld) => ld.clone(),
                                                                                        None => "".to_string(),
                                                                                    }
                                                                                },
                                                                            }
                                                                        },
                                                                        mime: None,
                                                                        w: None,
                                                                        h: None,
                                                                        dur: {
                                                                            match &ad.ad_content.video_duration {
                                                                                Some(video_duration) => Some(video_duration.parse().unwrap()),
                                                                                None => None,
                                                                            }
                                                                        },
                                                                        skipoffset: None,
                                                                        size: {
                                                                            match &ad.ad_content.video_size {
                                                                                Some(video_size) => Some(video_size.parse().unwrap()),
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

                                                                if assets.get_asset_size("video#cover") > 0 {
                                                                    asset_vec.push(Asset {
                                                                        id: assets.consume_asset("video#cover"),
                                                                        req: 0,
                                                                        img: Some(ImageAsset {
                                                                            url: ad.ad_content.img_1.clone(),
                                                                            mime: None,
                                                                            w: Some(ad.ad_content.img_1_w.parse().unwrap()),
                                                                            h: Some(ad.ad_content.img_1_h.parse().unwrap()),
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
                                                        if assets.get_asset_size("img") > 0 {
                                                            asset_vec.push(Asset {
                                                                id: assets.consume_asset("img"),
                                                                req: 1,
                                                                img: {
                                                                    Some(ImageAsset {
                                                                        url: ad.ad_content.img_1.clone(),
                                                                        mime: None,
                                                                        w: Some(ad.ad_content.img_1_w.parse().unwrap()),
                                                                        h: Some(ad.ad_content.img_1_h.parse().unwrap()),
                                                                        imagetype: Some(3),
                                                                    })
                                                                },
                                                                title: None,
                                                                video: None,
                                                                data: None,
                                                                html: None,
                                                                app: None,
                                                            });
                                                            match &ad.ad_content.img_2 {
                                                                Some(img_2) => {
                                                                    asset_vec.push(Asset {
                                                                        id: assets.consume_asset("img"),
                                                                        req: 1,
                                                                        img: {
                                                                            Some(ImageAsset {
                                                                                url: img_2.clone(),
                                                                                mime: None,
                                                                                w: {
                                                                                    match &ad.ad_content.img_2_w {
                                                                                        Some(img_2_w) => Some(img_2_w.parse().unwrap()),
                                                                                        None => None,
                                                                                    }
                                                                                },
                                                                                h: {
                                                                                    match &ad.ad_content.img_2_h {
                                                                                        Some(img_2_h) => Some(img_2_h.parse().unwrap()),
                                                                                        None => None,
                                                                                    }
                                                                                },
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
                                                            match &ad.ad_content.img_3 {
                                                                Some(img_3) => {
                                                                    asset_vec.push(Asset {
                                                                        id: assets.consume_asset("img"),
                                                                        req: 1,
                                                                        img: {
                                                                            Some(ImageAsset {
                                                                                url: img_3.clone(),
                                                                                mime: None,
                                                                                w: {
                                                                                    match &ad.ad_content.img_3_w {
                                                                                        Some(img_3_w) => Some(img_3_w.parse().unwrap()),
                                                                                        None => None,
                                                                                    }
                                                                                },
                                                                                h: {
                                                                                    match &ad.ad_content.img_3_h {
                                                                                        Some(img_3_h) => Some(img_3_h.parse().unwrap()),
                                                                                        None => None,
                                                                                    }
                                                                                },
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
                                                        }
                                                        if assets.get_asset_size("thumb") > 0 {
                                                            asset_vec.push(Asset {
                                                                id: assets.consume_asset("thumb"),
                                                                req: 1,
                                                                img: {
                                                                    Some(ImageAsset {
                                                                        url: ad.ad_content.img_1.clone(),
                                                                        mime: None,
                                                                        w: Some(ad.ad_content.img_1_w.parse().unwrap()),
                                                                        h: Some(ad.ad_content.img_1_h.parse().unwrap()),
                                                                        imagetype: Some(501),
                                                                    })
                                                                },
                                                                title: None,
                                                                video: None,
                                                                data: None,
                                                                html: None,
                                                                app: None,
                                                            });
                                                            match &ad.ad_content.img_2 {
                                                                Some(img_2) => {
                                                                    asset_vec.push(Asset {
                                                                        id: assets.consume_asset("thumb"),
                                                                        req: 1,
                                                                        img: {
                                                                            Some(ImageAsset {
                                                                                url: img_2.clone(),
                                                                                mime: None,
                                                                                w: {
                                                                                    match &ad.ad_content.img_2_w {
                                                                                        Some(img_2_w) => Some(img_2_w.parse().unwrap()),
                                                                                        None => None,
                                                                                    }
                                                                                },
                                                                                h: {
                                                                                    match &ad.ad_content.img_2_h {
                                                                                        Some(img_2_h) => Some(img_2_h.parse().unwrap()),
                                                                                        None => None,
                                                                                    }
                                                                                },
                                                                                imagetype: Some(501),
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
                                                            match &ad.ad_content.img_3 {
                                                                Some(img_3) => {
                                                                    asset_vec.push(Asset {
                                                                        id: assets.consume_asset("thumb"),
                                                                        req: 1,
                                                                        img: {
                                                                            Some(ImageAsset {
                                                                                url: img_3.clone(),
                                                                                mime: None,
                                                                                w: {
                                                                                    match &ad.ad_content.img_3_w {
                                                                                        Some(img_3_w) => Some(img_3_w.parse().unwrap()),
                                                                                        None => None,
                                                                                    }
                                                                                },
                                                                                h: {
                                                                                    match &ad.ad_content.img_3_h {
                                                                                        Some(img_3_h) => Some(img_3_h.parse().unwrap()),
                                                                                        None => None,
                                                                                    }
                                                                                },
                                                                                imagetype: Some(501),
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
                                                        }
                                                        match &ad.ad_content.button_content {
                                                            Some(button_content) => {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("data#ctatext"),
                                                                    req: 1,
                                                                    title: None,
                                                                    img: None,
                                                                    video: None,
                                                                    data: Some(DataAsset {
                                                                        value: button_content.clone(),
                                                                        len: None,
                                                                        datatype: Some(12),
                                                                    }),
                                                                    html: None,
                                                                    app: None,
                                                                });
                                                            },
                                                            None => (),
                                                        }

                                                        match &ad.ad_content.app_name {
                                                            Some(app_name) => {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("app"),
                                                                    req: 0,
                                                                    app: Some(AppAsset {
                                                                        name: app_name.clone(),
                                                                        desc: ad.ad_content.function_desc.clone(),
                                                                        descurl: None,
                                                                        domain: None,
                                                                        bundle: ad.ad_content.package_name.clone(),
                                                                        ver: ad.ad_content.version_name.clone(),
                                                                        developer: ad.ad_content.developer.clone(),
                                                                        icon: ad.ad_content.app_logo.clone(),
                                                                        storeid: ad.ad_content.app_key.clone(),
                                                                        storeurl: None,
                                                                        paid: 0,
                                                                        size: None,
                                                                        md5: None,
                                                                        registration: None,
                                                                        privacy: None,
                                                                        privacyurl: ad.ad_content.privacy.clone(),
                                                                        permission: None,
                                                                        permissionurl: ad.ad_content.permission.clone(),
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

                                                    match &ad.vurl {
                                                        Some(vurl) => {
                                                            for event in vurl {
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
                                                    match &ad.curl {
                                                        Some(curl) => {
                                                            if ad.turl.len() == 1 {
                                                                for event in curl {
                                                                    event_vec.push(Event {
                                                                        eventtype: 502,
                                                                        method: 1,
                                                                        url: replace_macro(event),
                                                                        header: None,
                                                                        content: None,
                                                                    });
                                                                }
                                                            }
                                                            if ad.turl.len() == 2 {
                                                                for event in curl {
                                                                    event_vec.push(Event {
                                                                        eventtype: 502,
                                                                        method: 1,
                                                                        url: replace_macro(&format!("{}{}", event, "&hc_subid=0")),
                                                                        header: None,
                                                                        content: None,
                                                                    });
                                                                }
                                                                for event in curl {
                                                                    event_vec.push(Event {
                                                                        eventtype: 601,
                                                                        method: 1,
                                                                        url: replace_macro(&format!("{}{}", event, "&hc_subid=1")),
                                                                        header: None,
                                                                        content: None,
                                                                    });
                                                                }
                                                            }
                                                        },
                                                        None => (),
                                                    }
                                                    match &ad.scheme_feedback_url {
                                                        Some(scheme_feedback_url) => {
                                                            event_vec.push(Event {
                                                                eventtype: 504,
                                                                method: 1,
                                                                url: replace_macro(&format!("{}{}", scheme_feedback_url, "&event=scheme&appcode=0&jump_type=1&clickstm=__TS_S__")),
                                                                header: None,
                                                                content: None,
                                                            });
                                                            event_vec.push(Event {
                                                                eventtype: 507,
                                                                method: 1,
                                                                url: replace_macro(&format!("{}{}", scheme_feedback_url, "&event=scheme&appcode=1&jump_type=1&clickstm=__TS_S__")),
                                                                header: None,
                                                                content: None,
                                                            });
                                                            event_vec.push(Event {
                                                                eventtype: 505,
                                                                method: 1,
                                                                url: replace_macro(&format!("{}{}", scheme_feedback_url, "&event=scheme&appcode=5&jump_type=1&clickstm=__TS_S__")),
                                                                header: None,
                                                                content: None,
                                                            });
                                                        },
                                                        None => (),
                                                    }
                                                    match &ad.video_play_url {
                                                        Some(video_play_url) => {
                                                            event_vec.push(Event {
                                                                eventtype: 701,
                                                                method: 1,
                                                                url: replace_macro(&format!("{}{}", video_play_url, "&eid=1002&eventData=__TS__")),
                                                                header: None,
                                                                content: None,
                                                            });
                                                            event_vec.push(Event {
                                                                eventtype: 708,
                                                                method: 1,
                                                                url: replace_macro(&format!("{}{}", video_play_url, "&eid=1005&eventData=__TS__,__VIDEO_PLAY_PROGRESS_S__")),
                                                                header: None,
                                                                content: None,
                                                            });
                                                            event_vec.push(Event {
                                                                eventtype: 705,
                                                                method: 1,
                                                                url: replace_macro(&format!("{}{}", video_play_url, "&eid=6&eventData=__TS__,__VIDEO_PLAY_PROGRESS_S__,__VIDEO_TIME__,null,null")),
                                                                header: None,
                                                                content: None,
                                                            });
                                                            event_vec.push(Event {
                                                                eventtype: 711,
                                                                method: 1,
                                                                url: replace_macro(&format!("{}{}", video_play_url, "&eid=1004&eventData=__TS__,__VIDEO_PLAY_PROGRESS_S__,__VIDEO_TIME__,null,null")),
                                                                header: None,
                                                                content: None,
                                                            });
                                                        },
                                                        None => (),
                                                    }
                                                    match &ad.click_area_report_url {
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

                                                    event_vec
                                                }
                                            },
                                            advertiser: {
                                                match &ad.ad_content.logo_url {
                                                    Some(logo_url) => Some(logo_url.clone()),
                                                    None => {
                                                        match &ad.ad_content.app_logo {
                                                            Some(app_logo) => Some(app_logo.clone()),
                                                            None => Some(ad.ad_content.dsp_logo.clone()),
                                                        }
                                                    },
                                                }
                                            },
                                            advertisericon: {
                                                match &ad.ad_content.source {
                                                    Some(source) => Some(source.clone()),
                                                    None => {
                                                        match &ad.ad_content.app_name {
                                                            Some(app_name) => Some(app_name.clone()),
                                                            None => Some(ad.ad_content.dsp_name.clone()),
                                                        }
                                                    },
                                                }
                                            },
                                        },
                                    };

                                    bids.push(bid);
                                }
                                bids
                            }
                        }].to_vec())
                    },
                    None => {
                        return Err(ResultMessage {
                            code: 993,
                            message: "".to_string(),
                        });
                    },
                }
            }
        };

        Ok(response)
    }

    async fn bidding_notify_win(url: String, win_price: i32, _next_price: i32, iv: &String, connection: &Connection, pool: &HttpPool) {
        let encrypt_price = Self::encrypt_price(win_price, iv, connection);
        let replaced_url = url
            .replace("__WIN_PRICE__", &encode(encrypt_price.as_str()));

        let client = {
            let pool_huichuan_lock = pool.pool_huichuan.clone();
            let pool_huichuan = pool_huichuan_lock.read().unwrap();
            pool_huichuan.clone()
        };
        let _ = client.get(replaced_url).send().await;
    }

    async fn bidding_notify_lose(url: String, lose_price: i32, _lose_reason: i32, _lose_adn_name: &String, iv: &String, connection: &Connection, pool: &HttpPool) {
        let encrypt_price = Self::encrypt_price(lose_price, iv, connection);
        let replaced_url = url
            .replace("__LOSE_PRICE__", &encode(encrypt_price.as_str()));

        let client = {
            let pool_huichuan_lock = pool.pool_huichuan.clone();
            let pool_huichuan = pool_huichuan_lock.read().unwrap();
            pool_huichuan.clone()
        };
        let _ = client.get(replaced_url).send().await;
    }

    fn encrypt_price(price: i32, _iv: &String, _connection: &Connection) -> String {
        price.to_string()
    }

}

fn replace_macro(orig: &String) -> String {
    let mut replaced = orig.clone();

    replaced = replaced.replace("{TS}", "__TS_S__");

    replaced
}

fn get_app_id(app: &str) -> Option<i32> {
    if app == "手淘" || app == "com.taobao.taobao" { return Some(1); }
    if app == "支付宝" || app == "com.eg.android.AlipayGphone" { return Some(2); }
    if app == "饿了么" || app == "me.ele" { return Some(3); }
    if app == "闲鱼" || app == "com.taobao.idlefish" { return Some(4); }
    if app == "口碑" || app == "om.taobao.mobile.dipei" { return Some(5); }
    if app == "钉钉" || app == "com.alibaba.android.rimet" { return Some(6); }
    if app == "飞猪" || app == "com.taobao.trip" { return Some(7); }
    if app == "零售通" || app == "com.alibaba.wireless.lstretailer" { return Some(8); }
    if app == "优酷" || app == "com.youku.phone" { return Some(9); }
    if app == "网易考拉" || app == "com.kaola?" { return Some(10); }
    if app == "天猫" || app == "com.tmall.wireless" { return Some(11); }
    if app == "淘特" || app == "com.taobao.litetao" { return Some(12); }
    if app == "点淘" || app == "com.taobao.live" { return Some(13); }
    if app == "1688" || app == "com.alibaba.wireless" { return Some(14); }
    if app == "夸克浏览器" || app == "com.quark.browser" { return Some(15); }
    if app == "盒马" || app == "com.wudaokou.hippo" { return Some(16); }
    if app == "菜鸟裹裹" || app == "com.cainiao.wireless" { return Some(17); }
    if app == "喜马拉雅" || app == "com.ximalaya.ting.android" { return Some(18); }
    if app == "快手" || app == "com.smile.gifmaker" { return Some(19); }
    if app == "豆包" || app == "com.larus.nova" { return Some(20); }
    if app == "腾讯新闻" || app == "com.tencent.news" { return Some(21); }
    if app == "哔哩哔哩" || app == "tv.danmaku.bili" { return Some(22); }
    if app == "网易新闻-头条版" || app == "com.netease.newsreader.activity" { return Some(23); }
    if app == "拼多多" || app == "com.xunmeng.pinduoduo" { return Some(101); }
    if app == "美团外卖" || app == "com.sankuai.meituan.takeoutnew" { return Some(102); }
    if app == "滴滴出行" || app == "com.sdu.didi.psnger?" { return Some(103); }
    if app == "京东" || app == "com.jingdong.app.mall" { return Some(104); }
    if app == "唯品会" || app == "com.achievo.vipshop" { return Some(105); }
    if app == "爱奇艺" || app == "com.qiyi.video" { return Some(106); }
    if app == "美团" || app == "com.sankuai.meituan" { return Some(107); }
    if app == "手机百度" || app == "com.baidu.searchbox" { return Some(109); }
    if app == "小红书" || app == "com.xingin.xhs" { return Some(110); }
    if app == "天眼查" || app == "com.tianyancha.skyeye" { return Some(111); }
    if app == "携程" || app == "ctrip.android.view" { return Some(112); }
    if app == "贝壳" || app == "com.lianjia.beike" { return Some(113); }
    if app == "地铁跑酷" || app == "com.kiloo.subwaysurf.cn" { return Some(1003001); }
    if app == "光·遇" || app == "com.netease.sky" { return Some(1003002); }
    if app == "元气骑士" || app == "com.ChillyRoom.DungeonShooter" { return Some(1003003); }
    if app == "我的世界" || app == "com.netease.mc" { return Some(1003004); }
    if app == "蛋仔派对" || app == "com.netease.party" { return Some(1003005); }
    if app == "斗罗大陆" || app == "com.khorgas.hsdj" { return Some(1003006); }
    if app == "弹壳特攻队" || app == "com.habby.danke" { return Some(1003007); }
    if app == "植物大战僵尸2" || app == "com.popcap.ios.chs.PVZ2" { return Some(1003008); }
    if app == "迷你世界" || app == "com.minitech.miniworld" { return Some(1003009); }
    if app == "部落冲突账号绑定工具" || app == "com.supercell.clashofclans.uc" { return Some(1003010); }
    if app == "小小蚁国" || app == "com.sy.xxyiguo" { return Some(1003011); }
    if app == "三国志·战略版" || app == "com.s3.sgzzlb.cn" { return Some(1003012); }
    if app == "和平精英" || app == "com.tencent.tmgp.pubgmhd" { return Some(1003013); }
    if app == "海岛奇兵" || app == "com.supercell.reef.china" { return Some(1003014); }
    if app == "逃跑吧！少年" || app == "com.bairimeng.dmmdzz.ios" { return Some(1003015); }
    if app == "长安幻想" || app == "com.mq.cahx" { return Some(1003016); }
    if app == "春秋封神" || app == "com.iqiyi.DreamPlus.cqf" { return Some(1003017); }
    if app == "PUBG?Mobile" || app == "com.mottods.PUBG-Mobile-Guide" { return Some(1003018); }
    if app == "超凡先锋" || app == "com.netease.cfxf" { return Some(1003019); }
    if app == "荒野乱斗账号绑定工具" || app == "com.youzu.bs.aligames" { return Some(1003020); }
    if app == "三国志幻想大陆" || app == "com.Aligames.sgzhxdl" { return Some(1003021); }
    if app == "航海王热血航线" || app == "com.hermes.h1gameop" { return Some(1003022); }
    if app == "三国志·战棋版" || app == "com.s6.sgzzqb.cn" { return Some(1003023); }
    if app == "巅峰极速" || app == "com.netease.rc" { return Some(1003024); }
    if app == "口袋觉醒" || app == "com.jzlx.applestore" { return Some(1003025); }
    if app == "奥特曼传奇英雄" || app == "com.joym.legendher" { return Some(1003026); }
    if app == "狂野飙车9：竞速传奇" || app == "com.Aligames.kybc9" { return Some(1003027); }
    if app == "部落冲突" || app == "com.supercell.magic.china" { return Some(1003028); }
    if app == "镇魂街：天生为王" || app == "com.cmge.zhjtsww.gw" { return Some(1003029); }
    if app == "原神" || app == "com.miHoYo.Yuanshen" { return Some(1003030); }
    if app == "重生细胞" || app == "com.bilibili.deadcells.ios" { return Some(1003031); }
    if app == "暗黑破坏神：不朽" || app == "com.netease.immortal" { return Some(1003032); }
    if app == "biubiu加速器" || app == "com.njh.biubiu.cn" { return Some(1003033); }
    if app == "暗区突围" || app == "com.tencent.mf.uam" { return Some(1003034); }
    if app == "新仙剑奇侠传之挥剑问情" || app == "com.xjhjwq.qc.ios" { return Some(1003035); }
    if app == "崩坏3" || app == "com.miHoYo.bh3" { return Some(1003036); }
    if app == "绝地求生：刺激战场体验服" || app == "com.tencent.igce" { return Some(1003037); }
    if app == "异世代：光与魔法" || app == "com.yami.gymf" { return Some(1003038); }
    if app == "皇室战争账号绑定工具" || app == "com.supercell.clashroyale.ewan.cc" { return Some(1003039); }
    if app == "黎明觉醒：生机" || app == "com.tencent.toaa" { return Some(1003040); }
    if app == "明日之后" || app == "com.netease.mrzh" { return Some(1003041); }
    if app == "第五人格" || app == "com.netease.id5" { return Some(1003042); }
    if app == "新不良人" || app == "com.xinbuliangren.guanbao" { return Some(1003043); }
    if app == "王者荣耀" || app == "com.tencent.smoba" { return Some(1003044); }
    if app == "斗罗大陆：武魂觉醒" || app == "com.khorgas.dlsy" { return Some(1003045); }
    if app == "王牌战争" || app == "com.lastdayrulessurvival.heroio" { return Some(1003046); }
    if app == "闪烁之光" || app == "cqfl.ql.sh" { return Some(1003047); }
    if app == "汤姆猫跑酷" || app == "com.outfit7.talkingtomgoldrun" { return Some(1003048); }
    if app == "球球大作战" || app == "com.juzi.balls" { return Some(1003049); }
    if app == "吞噬星空：黎明" || app == "com.cmge.tsxk.ios" { return Some(1003050); }
    if app == "最后的原始人" || app == "com.sy.zhdysr" { return Some(1003051); }
    if app == "地下城与勇士" || app == "ajixk.comsi" { return Some(1003052); }
    if app == "猫和老鼠" || app == "com.netease.TomJerry" { return Some(1003053); }
    if app == "神仙道3" || app == "com.hermes.nvwa" { return Some(1003054); }
    if app == "奥特曼系列OL" || app == "com.metek.ultramansol" { return Some(1003055); }
    if app == "漫战无双" || app == "com.manzhansg.ios" { return Some(1003056); }
    if app == "亮剑" || app == "com.ttlj.game.sanuc6c" { return Some(1003057); }
    if app == "忍者必须死3" || app == "com.pandadastudio.ninjamustdie" { return Some(1003058); }
    if app == "幻想名将录" || app == "com.rzzj.yzsl" { return Some(1003059); }
    if app == "坦克世界闪击战" || app == "com.netease.wot" { return Some(1003060); }
    if app == "创造与魔法" || app == "com.hero.sm.ios.hero" { return Some(1003061); }
    if app == "使命召唤手游" || app == "com.tencent.tmgp.cod" { return Some(1003062); }
    if app == "逆水寒（会呼吸的江湖）" || app == "com.netease.nshm" { return Some(1003063); }
    if app == "凡人修仙传：人界篇" || app == "com.sy.frxxz" { return Some(1003064); }
    if app == "极无双2" || app == "com.taiyouxi.dl2" { return Some(1003065); }
    if app == "幻塔" || app == "com.pwrd.huanta" { return Some(1003066); }
    if app == "我的勇者" || app == "com.rsg.MyheroApp" { return Some(1003067); }
    if app == "登山赛车" || app == "com.fingersoft.hillclimbracing2" { return Some(1003068); }
    if app == "航海王：燃烧意志" || app == "com.Aligames.hhw" { return Some(1003069); }
    if app == "超级驾驶" || app == "com.tianyi.cjjs.game.mxo" { return Some(1003070); }
    if app == "贪吃蛇大作战" || app == "com.wepie.snakegame" { return Some(1003071); }
    if app == "模拟城市：我是市长" || app == "com.gamecomb.simcity" { return Some(1003072); }
    if app == "极限竞速地平线4" || app == "com.hasan.wcracing" { return Some(1003073); }
    if app == "方舟：生存进化" || app == "com.tsshjh.shys" { return Some(1003074); }
    if app == "口袋重制" || app == "com.liskf.dbf" { return Some(1003075); }
    if app == "太空行动" || app == "com.je.spaceaction" { return Some(1003076); }
    if app == "王牌竞速" || app == "com.netease.racer" { return Some(1003077); }
    if app == "不良人3" || app == "com.ycgames.blr" { return Some(1003078); }
    if app == "阴阳师" || app == "com.netease.onmyoji" { return Some(1003079); }
    if app == "香肠派对" || app == "com.xd.Sausage" { return Some(1003080); }
    if app == "蓝空幻想" || app == "com.xuanlan.lkhx" { return Some(1003081); }
    if app == "我的御剑日记" || app == "com.lovengame.wdyjrj.gf" { return Some(1003082); }
    if app == "我的总裁女友" || app == "com.playfun.kela" { return Some(1003083); }
    if app == "奥特曼：集结" || app == "com.cmge.atmjjx" { return Some(1003084); }
    if app == "阿瑞斯病毒" || app == "com.zzonegame.aresvirus" { return Some(1003085); }
    if app == "荒野行动" || app == "com.netease.hyxd" { return Some(1003086); }
    if app == "三国杀移动版" || app == "com.gameabc.sgsiphone" { return Some(1003087); }
    if app == "战双帕弥什" || app == "com.kurogame.haru.hero" { return Some(1003088); }
    if app == "空之要塞：启航" || app == "com.ahyqnet.yaosai" { return Some(1003089); }
    if app == "超能力冲刺" || app == "com.xsluckrts.cnlcc" { return Some(1003090); }
    if app == "率土之滨" || app == "com.netease.stzb" { return Some(1003091); }
    if app == "星球：重启" || app == "com.hermes.j1game" { return Some(1003092); }
    if app == "口袋梦幻精灵" || app == "com.summoners.kingdomios2" { return Some(1003093); }
    if app == "时空猎人3" || app == "com.bilibili.hunter3" { return Some(1003094); }
    if app == "爱琳诗篇" || app == "com.sdiw.cdss" { return Some(1003095); }
    if app == "火柴人战争" || app == "com.cwelcome.cstickmanwar1b" { return Some(1003096); }
    if app == "七猫免费小说" || app == "com.kmxs.reader" { return Some(1003097); }
    if app == "TapTap" || app == "com.taptap" { return Some(1003098); }
    if app == "抖音" || app == "com.ss.android.ugc.aweme" { return Some(1003099); }
    if app == "番茄免费小说" || app == "com.dragon.read" { return Some(1003100); }
    if app == "九游" || app == "cn.ninegame.gamemanager" { return Some(1003101); }
    if app == "三国志·战略版" || app == "com.aligames.sgzzlb.uc" { return Some(1003102); }
    if app == "得物" || app == "com.shizhuang.duapp" { return Some(1003104); }
    if app == "自如" || app == "com.ziroom.ziroomcustomer" { return Some(1003105); }
    if app == "百度极速版" || app == "com.baidu.searchbox.lite" { return Some(1003106); }
    if app == "途虎养车" || app == "cn.TuHu.android" { return Some(1003107); }
    if app == "今日头条极速版" || app == "com.ss.android.article.lite" { return Some(1003108); }
    if app == "洪恩识字" || app == "com.hongen.app.word" { return Some(1003109); }
    if app == "番茄畅听" || app == "com.xs.fm" { return Some(1003110); }
    if app == "应用宝" || app == "com.tencent.android.qqdownloader" { return Some(1003111); }
    if app == "贝壳找房" || app == "com.lianjia.beike" { return Some(1003112); }
    if app == "小7手游" || app == "com.smwl.x7market" { return Some(1003113); }
    if app == "996传奇盒子" || app == "com.xqhy.legendbox" { return Some(1003114); }
    if app == "金盛贵金属" || app == "com.jinsheng.jinsheng" { return Some(1003115); }
    if app == "飞猪旅行" || app == "com.taobao.trip" { return Some(1003116); }
    if app == "无尽的拉格朗日" || app == "com.netease.lglr" { return Some(1003117); }
    if app == "优酷视频" || app == "com.youku.phone" { return Some(1003118); }
    if app == "抖音极速版" || app == "com.ss.android.ugc.aweme.lite" { return Some(1003119); }
    if app == "偷星猫" || app == "com.touxingmao.appstore" { return Some(1003120); }
    if app == "叮咚买菜" || app == "com.yaya.zone" { return Some(1003121); }
    if app == "同程旅行" || app == "com.tongcheng.android" { return Some(1003122); }
    if app == "小米游戏中心" || app == "com.xiaomi.gamecenter" { return Some(1003123); }
    if app == "易车" || app == "com.yiche.autoeasy" { return Some(1003124); }
    if app == "巨兽战场" || app == "com.jszc.kkkwan" { return Some(1003125); }
    if app == "知乎" || app == "com.zhihu.android" { return Some(1003126); }
    if app == "率土之滨手游" || app == "com.netease.stzb.netease" { return Some(1003127); }
    if app == "快手极速版" || app == "com.kuaishou.nebula" { return Some(1003128); }
    if app == "瓜子二手车" || app == "com.ganji.android.haoche_c" { return Some(1003129); }
    if app == "领峰贵金属" || app == "com.igold.app" { return Some(1003130); }
    if app == "鱼泡网" || app == "io.dcloud.H576E6CC7" { return Some(1003131); }
    if app == "大话西游" || app == "com.netease.dhxy" { return Some(1003132); }
    if app == "MOMO陌陌" || app == "com.immomo.momo" { return Some(1003133); }
    if app == "旭日之城" || app == "com.camelgames.aoz.zha" { return Some(1003134); }
    if app == "快看" || app == "com.kuaikan.comic" { return Some(1003135); }
    if app == "企查查企业信用查询" || app == "com.android.icredit" { return Some(1003136); }
    if app == "众牧" || app == "com.nirvana.ylmc" { return Some(1003137); }
    if app == "交易猫" || app == "com.jym.mall" { return Some(1003138); }
    if app == "TK助手" || app == "com.loadodo.tkhelper" { return Some(1003139); }
    if app == "Ateen" || app == "com.quyue.clubprogram" { return Some(1003140); }
    if app == "红手指云手机" || app == "com.redfinger.app" { return Some(1003141); }
    if app == "百度网盘" || app == "com.baidu.netdisk" { return Some(1003142); }
    if app == "百视TV" || app == "com.bestv.app" { return Some(1003143); }
    if app == "梦幻西游网页版" || app == "com.netease.xyh5" { return Some(1003144); }
    if app == "66手游" || app == "com.ll.llgame" { return Some(1003145); }
    if app == "艾米直播" || app == "com.mobimtech.natives.ivp" { return Some(1003146); }
    if app == "三国杀" || app == "com.yoka.newsgs" { return Some(1003147); }
    if app == "魔域互通版" || app == "com.nd.myht" { return Some(1003148); }
    if app == "1号玩家" || app == "com.cw.gamebox" { return Some(1003149); }
    if app == "战火勋章" || app == "com.lilithgame.wgame.android.cn" { return Some(1003150); }
    if app == "住小帮" || app == "com.ss.android.homed" { return Some(1003151); }
    if app == "第五人格" || app == "com.netease.dwrg" { return Some(1003152); }
    if app == "千寻电话" || app == "com.jmtec.chihirotelephone" { return Some(1003153); }
    if app == "探探" || app == "com.p1.mobile.putong" { return Some(1003154); }
    if app == "开心消消乐" || app == "com.happyelements.AndroidAnimal" { return Some(1003155); }
    if app == "一念逍遥" || app == "com.leiting.xian" { return Some(1003156); }
    if app == "西西语音" || app == "com.ppyuewan.peiwan" { return Some(1003157); }
    if app == "六间房秀场" || app == "cn.v6.xiuchang" { return Some(1003158); }
    if app == "白鲸体育" || app == "com.major.magicfootball" { return Some(1003159); }
    if app == "磁力宅" || app == "com.videoplayer.magnetotaku" { return Some(1003160); }
    if app == "省省(原省省回头车)" || app == "com.huitouche.android.app" { return Some(1003161); }
    if app == "正保会计网校" || app == "com.cdel.accmobile" { return Some(1003162); }
    if app == "捕鱼新纪元" || app == "com.yuwan.byxjy" { return Some(1003163); }
    if app == "悟空浏览器" || app == "com.cat.readall" { return Some(1003164); }
    if app == "早游戏" || app == "com.qijin189.huosuapp" { return Some(1003165); }
    if app == "洪恩拼音" || app == "com.ihuman.pinyin" { return Some(1003166); }
    if app == "知聊" || app == "com.yyk.knowchat" { return Some(1003167); }
    if app == "冒险者总动员" || app == "com.arcade.mmgame" { return Some(1003168); }
    if app == "万能水印打卡相机" || app == "com.cqaizhe.wannengcamera" { return Some(1003169); }
    if app == "九重试炼" || app == "com.chenz.vamsur.gn" { return Some(1003170); }
    if app == "Chat Eve" || app == "com.xmtayun.chateve" { return Some(1003171); }
    if app == "AR实景导航软件" || app == "com.tm.smartscankinge" { return Some(1003172); }
    if app == "神武4" || app == "com.duoyi.shenwu3" { return Some(1003173); }
    if app == "狼人杀" || app == "com.c2vl.kgamebox" { return Some(1003174); }
    if app == "刺猬猫阅读" || app == "com.kuangxiangciweimao.novel" { return Some(1003175); }
    if app == "市值风云" || app == "com.paiba.app000004" { return Some(1003176); }
    if app == "bt手游盒子" || app == "com.fujing.btsyhz" { return Some(1003177); }
    if app == "埋堆堆" || app == "com.tvbc.maiduidui" { return Some(1003178); }
    if app == "逍遥情缘手游" || app == "com.skysgame.xyqy.gf65000" { return Some(1003179); }
    if app == "翻咔" || app == "com.cupidapp.live" { return Some(1003180); }
    if app == "川川云手机" || app == "com.chuanchuanyun.android" { return Some(1003181); }
    if app == "OurPlay原谷歌空间" || app == "com.excean.gspace" { return Some(1003182); }
    if app == "双开助手微分身版" || app == "com.excelliance.dualaid" { return Some(1003183); }
    if app == "恋小帮" || app == "com.sq.lovehelper" { return Some(1003184); }
    if app == "皮皮虾" || app == "com.sup.android.superb" { return Some(1003185); }
    if app == "天天盈球" || app == "com.aicai.yingqiu" { return Some(1003186); }
    if app == "松果倾诉" || app == "com.app.pinealgland" { return Some(1003187); }
    if app == "映客直播" || app == "com.meelive.ingkee" { return Some(1003188); }
    if app == "修改水印相机" || app == "com.hcn.mm" { return Some(1003189); }
    if app == "竞暴捕鱼" || app == "com.ucgg.jbby" { return Some(1003190); }
    if app == "万国觉醒" || app == "com.lilithgames.rok.offical.cn" { return Some(1003191); }
    if app == "响指连点器" || app == "com.bojun.autoclick" { return Some(1003192); }
    if app == "天龙八部2：飞龙战天" || app == "com.ptq.z8h.xl0" { return Some(1003193); }
    if app == "贝乐虎儿歌" || app == "com.ubestkid.beilehu.android" { return Some(1003194); }
    if app == "YY" || app == "com.duowan.mobile" { return Some(1003195); }
    if app == "电讯云网咖" || app == "com.cloudcomputer.cloudnetworkcafe" { return Some(1003196); }
    if app == "万能点击器" || app == "com.yuanlue.auto_clicker" { return Some(1003197); }
    if app == "Logo设计软件" || app == "cn.hudun.androidlogodesign" { return Some(1003198); }
    if app == "欢乐掼蛋" || app == "com.k7game.app.guandan" { return Some(1003199); }
    if app == "给力心理咨询" || app == "com.geilixinli.android.full.user" { return Some(1003200); }
    if app == "密马游戏交易" || app == "com.mula.mall" { return Some(1003201); }
    if app == "折扣手游" || app == "com.qushihd.zksyhz" { return Some(1003202); }
    if app == "天涯明月刀" || app == "com.tencent.tmgp.wuxia" { return Some(1003203); }
    if app == "一键抠图" || app == "com.shuojie.easyphoto" { return Some(1003204); }
    if app == "会玩手游" || app == "com.xiantu.hw" { return Some(1003205); }
    if app == "布谷鸟配音" || app == "com.weidu.cuckoodub" { return Some(1003206); }
    if app == "兔小贝识字" || app == "com.qpx.app.chinese" { return Some(1003207); }
    if app == "车况查询" || app == "com.echronos.carconditiontreasure" { return Some(1003208); }
    if app == "搞定水印相机" || app == "com.gaoding.mm" { return Some(1003209); }
    if app == "白金岛三打哈" || app == "cn.limsam.sdh" { return Some(1003210); }
    if app == "欢乐麻将" || app == "com.qqgame.happymj" { return Some(1003211); }
    if app == "轻甜" || app == "com.xyz.qingtian" { return Some(1003212); }
    if app == "觅伊" || app == "com.datalink.miyi" { return Some(1003213); }
    if app == "分身大师" || app == "com.qihoo.magic" { return Some(1003214); }
    if app == "万能识图" || app == "com.jime.stu" { return Some(1003215); }
    if app == "宠胖胖" || app == "com.longyan.mmmutually" { return Some(1003216); }
    if app == "吉工家" || app == "com.jizhi.jlongg" { return Some(1003217); }
    if app == "趣漫相机" || app == "com.jmtec.cartoon" { return Some(1003218); }
    if app == "比地标讯快车" || app == "com.bxkc.android" { return Some(1003219); }
    if app == "蔚蓝档案" || app == "com.RoamingStar.BlueArchive" { return Some(1003220); }
    if app == "高途" || app == "com.gaotu100.superclass" { return Some(1003221); }
    if app == "懒人驾考" || app == "com.jx885.lrjk" { return Some(1003222); }
    if app == "爱奇艺小说" || app == "com.qiyi.video.reader" { return Some(1003223); }
    if app == "万能电影播放器-影音播放" || app == "com.lixiangdong.mediaplayer" { return Some(1003224); }
    if app == "1905电影网" || app == "com.m1905.mobilefree" { return Some(1003225); }
    if app == "电影频道" || app == "newmediacctv6.com.cctv6" { return Some(1003226); }
    if app == "中影电影" || app == "cn.mopon.film.zygj" { return Some(1003227); }
    if app == "CGV电影" || app == "com.cgv.cn.movie" { return Some(1003228); }
    if app == "在线电影租赁 Netflix" || app == "com.netflix.mediaclient" { return Some(1003229); }
    if app == "Google Play电影 Google Play Movies" || app == "com.google.android.videos" { return Some(1003230); }
    if app == "南瓜电影" || app == "cn.vcinema.cinema" { return Some(1003231); }
    if app == "趣小说-免费小说大全" || app == "com.fread.interestingnovel" { return Some(1003232); }
    if app == "笔趣阁免费小说大全" || app == "com.example.reader.main" { return Some(1003233); }
    if app == "西瓜免费小说" || app == "com.dzmf.zmfxsdq" { return Some(1003234); }
    if app == "免费小说阅读星" || app == "com.yueduxing.xsreader" { return Some(1003235); }
    if app == "TXT全本免费小说全集" || app == "cn.ttkmfxs.novel" { return Some(1003236); }
    if app == "TXT全本免费小说快搜" || app == "com.txtqbmfxsks.freenovel" { return Some(1003237); }
    if app == "红豆免费小说" || app == "me.hongdou.reader" { return Some(1003238); }
    if app == "搜狗免费小说极速版" || app == "com.sogou.reader.free" { return Some(1003239); }
    if app == "豆豆免费小说" || app == "com.kanshu.ksgb.fastread.doudou" { return Some(1003240); }
    if app == "飞读免费小说" || app == "com.yuewen.cooperate.reader.free" { return Some(1003241); }
    if app == "悦读免费小说" || app == "com.shuqi.contq4" { return Some(1003242); }
    if app == "免费小说专区" || app == "com.jrtd.mfxszq" { return Some(1003243); }
    if app == "得间免费小说" || app == "com.chaozh.iReader.dj" { return Some(1003244); }
    if app == "必看免费小说" || app == "com.lwby.breader.ad" { return Some(1003245); }
    if app == "得间免费小说极速版" || app == "com.chaozh.iReader.dj.speed" { return Some(1003246); }
    if app == "洋葱免费小说" || app == "com.bikann.mfxssk" { return Some(1003247); }
    if app == "必阅免费小说" || app == "com.bikann.dzsk" { return Some(1003248); }
    if app == "江湖免费小说" || app == "com.zhangdu.bumblebee" { return Some(1003249); }
    if app == "熊猫免费小说" || app == "com.xm.freader" { return Some(1003250); }
    if app == "速看免费小说" || app == "com.chaozh.xincao.only.sk" { return Some(1003251); }
    if app == "爱奇艺VR(Glass版)" || app == "com.iqiyi.ivrcinema.huaweivrglass" { return Some(1003252); }
    if app == "爱奇艺遍知" || app == "com.iqiyi.knowledge" { return Some(1003253); }
    if app == "爱奇艺奇巴布" || app == "com.qiyi.video.child" { return Some(1003254); }
    if app == "爱奇艺极速版" || app == "com.qiyi.video.lite" { return Some(1003255); }
    if app == "爱奇艺叭嗒" || app == "com.iqiyi.acg" { return Some(1003256); }
    if app == "爱奇艺Pad" || app == "com.qiyi.video.pad" { return Some(1003257); }
    if app == "爱奇艺随刻" || app == "tv.pps.mobile" { return Some(1003258); }
    if app == "爱奇艺体育" || app == "com.ssports.mobile.video" { return Some(1003259); }
    if app == "全本免费快读小说" || app == "cc.quanbennovel" { return Some(1003260); }
    if app == "独阅读小说" || app == "com.novel.du" { return Some(1003261); }
    if app == "AA小说下载阅读器" || app == "com.aareader" { return Some(1003262); }
    if app == "耽美小说大全" || app == "com.mengjun.Novel_Vest" { return Some(1003263); }
    if app == "点阅小说大全" || app == "com.dotreader.dnovel" { return Some(1003264); }
    if app == "西瓜小说" || app == "com.dianzhong.xgxs" { return Some(1003265); }
    if app == "小说全集" || app == "com.dz.mfxsqj" { return Some(1003266); }
    if app == "次元姬小说" || app == "com.xunyou.rb" { return Some(1003267); }
    if app == "小说总动员" || app == "com.foreader.xingyue" { return Some(1003268); }
    if app == "扎堆小说" || app == "com.cootek.literature" { return Some(1003269); }
    if app == "全民写小说" || app == "com.mengjun.write_novel_qm" { return Some(1003270); }
    if app == "手机写小说" || app == "com.wyfc.writenovel" { return Some(1003271); }
    if app == "米阅小说" || app == "com.duokan.freereader" { return Some(1003272); }
    if app == "书旗小说极速版" || app == "com.shuqi.controller.lite" { return Some(1003273); }
    if app == "奇迹免费小说" || app == "reader.com.xmly.xmlyreader" { return Some(1003274); }
    if app == "怡阅小说" || app == "com.yiyue.yuekan" { return Some(1003275); }
    if app == "淘小说" || app == "com.martian.ttbook" { return Some(1003276); }
    if app == "米读小说" || app == "com.lechuan.midunovel" { return Some(1003277); }
    if app == "追读小说" || app == "com.yykuaile.sh" { return Some(1003278); }
    if app == "话本小说" || app == "com.huabenapp" { return Some(1003279); }
    if app == "塔读小说免费版" || app == "com.tadu.read" { return Some(1003280); }
    if app == "菠萝包轻小说" || app == "com.sfacg" { return Some(1003281); }
    if app == "晋江小说阅读" || app == "com.jjwxc.reader" { return Some(1003282); }
    if app == "17K小说" || app == "com.chineseall.reader" { return Some(1003283); }
    if app == "UC大字版" || app == "com.ucmobile.elder" { return Some(1003284); }
    if app == "阅读星免费小说" || app == "com.iBookStar.activity" { return Some(1003285); }
    if app == "七猫精品小说" || app == "com.book2345.reader" { return Some(1003286); }
    if app == "全本畅读TXT小说" || app == "com.kdqbmfxs.reader" { return Some(1003287); }
    if app == "宜搜小说快读版" || app == "cn.wejuan.reader" { return Some(1003288); }
    if app == "追书免费全本小说" || app == "cc.mianfeinovel" { return Some(1003289); }
    if app == "快读全本小说" || app == "cn.kdqbxs.reader" { return Some(1003290); }
    if app == "小说追书大全" || app == "com.ytkj.zsdq" { return Some(1003291); }
    if app == "小小优酷" || app == "com.youkuchild.android" { return Some(1003292); }
    if app == "寒武纪年小说" || app == "com.hanwujinian.adq" { return Some(1003293); }
    if app == "掌阅爽读小说" || app == "com.syhzx.shuangduFree" { return Some(1003294); }
    if app == "同花顺房贷计算器" || app == "com.hexin.mortgagecalc" { return Some(1003295); }
    if app == "汽车之家车主版" || app == "com.autohome.mycar" { return Some(1003296); }
    if app == "汽车之家极速版" || app == "com.autohome.speed" { return Some(1003297); }
    if app == "腾讯会议" || app == "com.tencent.wemeet.app" { return Some(1003298); }
    if app == "小米随星借-小米官方借款" || app == "com.airstar.loan" { return Some(1003299); }
    if app == "众安贷" || app == "com.zaxd.loan" { return Some(1003300); }
    if app == "360借条" || app == "com.qihoo.loan" { return Some(1003301); }
    if app == "你我借款" || app == "com.geerong.niwoloan.android" { return Some(1003302); }
    if app == "维信卡卡贷" || app == "com.vcredit.kkcredit" { return Some(1003303); }
    if app == "万达贷" || app == "com.wandaloans.timesloan" { return Some(1003304); }
    if app == "民生易贷" || app == "com.msyd.client" { return Some(1003305); }
    if app == "招联好期贷" || app == "com.mucfc.hqdapp" { return Some(1003306); }
    if app == "大地时贷" || app == "com.ccicnet.customer" { return Some(1003307); }
    if app == "好车e贷" || app == "com.ugoodtech.goodcareasyloan" { return Some(1003308); }
    if app == "榕树贷款" || app == "com.shuqu.banyan" { return Some(1003309); }
    if app == "鑫梦享消费贷" || app == "cn.com.njxmxbank.mbank" { return Some(1003310); }
    if app == "易借速贷" || app == "com.linzi.easy" { return Some(1003311); }
    if app == "洋钱罐借款" || app == "com.lingyue.zebraloan" { return Some(1003312); }
    if app == "美借" || app == "com.gomejr.icash" { return Some(1003313); }
    if app == "乐享借" || app == "com.lightpalm.fenqia" { return Some(1003314); }
    if app == "小米贷款" || app == "com.xiaomi.loan" { return Some(1003315); }
    if app == "花薪借钱贷款平台" || app == "com.taojinjia.charlotte" { return Some(1003316); }
    if app == "车贷在线" || app == "com.xiaoyutimes.carloanonline" { return Some(1003317); }
    if app == "你我贷借款" || app == "com.niwodai.universityloan" { return Some(1003318); }
    if app == "小赢卡贷" || app == "com.xiaoying.cardloan" { return Some(1003319); }
    if app == "房贷" || app == "cn.sqcat.caculator" { return Some(1003320); }
    if app == "时光分期" || app == "com.rong.fastloan" { return Some(1003321); }
    if app == "融易分期" || app == "com.rong360.ybfq" { return Some(1003322); }
    if app == "好分期" || app == "com.renrendai.haohuan" { return Some(1003323); }
    if app == "易车极速版" || app == "com.yiche.autofast" { return Some(1003324); }
    if app == "易车汽车报价" || app == "com.yiche.price" { return Some(1003325); }
    if app == "雪球股票" || app == "com.xueqiu.android" { return Some(1003326); }
    if app == "广发证券易淘金" || app == "com.gf.client" { return Some(1003327); }
    if app == "同花顺炒股票" || app == "com.hexin.plat.android" { return Some(1003328); }
    if app == "七月影视大全" || app == "com.app.julymovies" { return Some(1003329); }
    if app == "月亮影视大全" || app == "com.app.moontv" { return Some(1003330); }
    if app == "掌上影视大全" || app == "com.video.zs" { return Some(1003331); }
    if app == "完美影视大全" || app == "com.video.wanmei" { return Some(1003332); }
    if app == "好看影视大全" || app == "com.video.lizhi" { return Some(1003333); }
    if app == "番茄影视大全" || app == "com.fanqie.lizhi" { return Some(1003334); }
    if app == "影视大全纯净版" || app == "com.all.video" { return Some(1003335); }
    if app == "追剧影视大全" || app == "com.follow.video" { return Some(1003336); }
    if app == "七七影视大全" || app == "com.sevenVideo.app.android" { return Some(1003337); }
    if app == "东方财富证券" || app == "com.eastmoney.android.newyork" { return Some(1003338); }
    if app == "UC浏览器 国际版" || app == "com.UCMobile.intl" { return Some(1003339); }
    if app == "今日影视" || app == "com.now.video" { return Some(1003340); }
    if app == "CIBN环球影视" || app == "com.cibn.tv" { return Some(1003341); }
    if app == "集影视频工具箱" || app == "com.mycp.videocompress" { return Some(1003342); }
    if app == "2345影视大全" || app == "com.yingshi2345" { return Some(1003343); }
    if app == "BOSS直聘" || app == "com.hpbr.bosszhipin" { return Some(1003344); }
    if app == "有驾" || app == "com.baidu.autocar" { return Some(1003345); }
    if app == "优信二手车" || app == "com.uxin.usedcar" { return Some(1003346); }
    if app == "移动手机贷" || app == "com.droid.credit" { return Some(1003347); }
    if app == "手机淘宝" || app == "pro.rgmophju.hrjvks.ptvh" { return Some(1003348); }
    if app == "影视大全WTV" || app == "cn.quicktv.androidpro" { return Some(1003349); }
    if app == "腾讯课堂" || app == "com.tencent.edu" { return Some(1003350); }
    if app == "万得股票" || app == "wind.android" { return Some(1003351); }
    if app == "爱股票" || app == "com.aigupiao.ui" { return Some(1003352); }
    if app == "红塔证券" || app == "com.hexin.plat.android.HongtaSecurity" { return Some(1003353); }
    if app == "兴业证券优理宝" || app == "com.eno.xyzq.page" { return Some(1003354); }
    if app == "开源证券肥猫" || app == "com.kyscgenuiphone" { return Some(1003355); }
    if app == "德邦证券高端版" || app == "com.tebonsc" { return Some(1003356); }
    if app == "微证券" || app == "com.weizq" { return Some(1003357); }
    if app == "新时代证券" || app == "com.hundsun.stockwinner.xsdzq1" { return Some(1003358); }
    if app == "宏信证券智慧版" || app == "com.hexingzq.dzh" { return Some(1003359); }
    if app == "平安证券" || app == "com.hundsun.winner.pazq" { return Some(1003360); }
    if app == "选股宝" || app == "com.wallstreetcn.meepo" { return Some(1003361); }
    if app == "南京证券大智慧" || app == "com.nanjingzq.dzh" { return Some(1003362); }
    if app == "股吧" || app == "com.eastmoney.android.gubaproj" { return Some(1003363); }
    if app == "中原证券掌中网专业版" || app == "com.hexin.plat.android.ZhongyuanSecurity" { return Some(1003364); }
    if app == "经传股事汇" || app == "cn.jingzhuan.stock" { return Some(1003365); }
    if app == "金股在线" || app == "net.dxzq.jgzx" { return Some(1003366); }
    if app == "南京证券金罗盘" || app == "com.cssweb.android.main" { return Some(1003367); }
    if app == "海豚股票" || app == "com.ss.android.caijing.stock" { return Some(1003368); }
    if app == "方正证券小方" || app == "com.foundersc.app.xf" { return Some(1003369); }
    if app == "中泰齐富通（原齐鲁证券）" || app == "com.qlscupgrade" { return Some(1003370); }
    if app == "淘股吧" || app == "com.taoguba.app" { return Some(1003371); }
    if app == "芒果电单车" || app == "com.whxxcy.mango" { return Some(1003372); }
    if app == "芒果云" || app == "com.mango.hnxwlb" { return Some(1003373); }
    if app == "财经杂志" || app == "com.caijing" { return Some(1003374); }
    if app == "富途牛牛" || app == "cn.futu.trader" { return Some(1003375); }
    if app == "萝卜投研" || app == "com.datayes.irr" { return Some(1003376); }
    if app == "中邮证券" || app == "com.hundsun.stockwinner.zyouzq" { return Some(1003377); }
    if app == "21财经" || app == "com.twentyfirstcbh.epaper" { return Some(1003378); }
    if app == "Tiger Trade 老虎股票" || app == "com.tigerbrokers.stock" { return Some(1003379); }
    if app == "好买基金" || app == "howbuy.android.palmfund" { return Some(1003380); }
    if app == "同花顺投资记账本" || app == "com.hexin.zhanghu" { return Some(1003381); }
    if app == "金太阳" || app == "com.guosen.android" { return Some(1003382); }
    if app == "会选股" || app == "com.baidao.silver" { return Some(1003383); }
    if app == "东方证券期货" || app == "com.dzqh.online11" { return Some(1003384); }
    if app == "同花顺期货通" || app == "com.hexin.android.futures" { return Some(1003385); }
    if app == "蛋卷基金" || app == "com.xueqiu.fund" { return Some(1003386); }
    if app == "英为财情Investing.com 财经投资" || app == "com.fusionmedia.investingCN" { return Some(1003387); }
    if app == "哈富证券" || app == "com.eastmoney.android.lead" { return Some(1003388); }
    if app == "青石证券港股美股" || app == "com.bs.trade" { return Some(1003389); }
    if app == "南方基金" || app == "com.nanfangjijin.app" { return Some(1003390); }
    if app == "广发基金" || app == "com.hctforgf.gff" { return Some(1003391); }
    if app == "金色财经" || app == "com.jinse.app" { return Some(1003392); }
    if app == "天弘基金" || app == "com.thfund.client" { return Some(1003393); }
    if app == "慧博投资分析" || app == "cn.com.hibor" { return Some(1003394); }
    if app == "股拍" || app == "com.spero.vision.vsnapp" { return Some(1003395); }
    if app == "万联e万通" || app == "com.hexin.plat.android.WanlianSecurity" { return Some(1003396); }
    if app == "和讯财经" || app == "com.hexun.news" { return Some(1003397); }
    if app == "恒大智慧社区" || app == "com.hd.smartVillage" { return Some(1003398); }
    if app == "芝士财富" || app == "com.cheese.stock" { return Some(1003399); }
    if app == "汇正财经" || app == "com.gzhzcj" { return Some(1003400); }
    if app == "国泰君安君弘" || app == "com.guotai.dazhihui" { return Some(1003401); }
    if app == "陆基金" || app == "com.lufax.lufunds" { return Some(1003402); }
    if app == "兴全基金" || app == "com.xyqqfund.android" { return Some(1003403); }
    if app == "国海金探号" || app == "com.eno.android" { return Some(1003404); }
    if app == "证券时报" || app == "com.stcn.newmedia.activity" { return Some(1003405); }
    if app == "呱呱财经" || app == "com.guagua.finance" { return Some(1003406); }
    if app == "股掌柜炒股票" || app == "com.sscf.investment" { return Some(1003407); }
    if app == "信达同花顺" || app == "com.hexin.plat.android.XindaSecurity" { return Some(1003408); }
    if app == "选股冠军" || app == "com.lanyife.picker" { return Some(1003409); }
    if app == "通达信指标炒股票开户首选" || app == "com.tdx.AndroidNew" { return Some(1003410); }
    if app == "同花顺爱基金" || app == "com.hexin.android.bank" { return Some(1003411); }
    if app == "益盟操盘手" || app == "cn.emoney.emstock" { return Some(1003412); }
    if app == "同花顺iFinD" || app == "com.hexin.ifind.android" { return Some(1003413); }
    if app == "新浪财经" || app == "cn.com.sina.finance" { return Some(1003414); }
    if app == "蜻蜓点金" || app == "zhongxinjiantou.szkingdom.android.newphone" { return Some(1003415); }
    if app == "随身行" || app == "com.wenhua.bamboo" { return Some(1003416); }
    if app == "信达天下" || app == "com.thinkive.investxdtx" { return Some(1003417); }
    if app == "华安徽赢" || app == "com.thinkive.android.invest_ha" { return Some(1003418); }
    if app == "国元点金" || app == "guoyuan.szkingdom.android.phone" { return Some(1003419); }
    if app == "换手率" || app == "com.hsl.stock" { return Some(1003420); }
    if app == "国金佣金宝" || app == "cn.com.gjzq.yjb2" { return Some(1003421); }
    if app == "西南金点子" || app == "com.hexin.plat.android.XinanSecurity" { return Some(1003422); }
    if app == "东方赢家" || app == "com.dfzq.winner" { return Some(1003423); }
    if app == "恒泰金玉管家" || app == "com.hexin.plat.android.HengtaiSecurity" { return Some(1003424); }
    if app == "华彩人生（新版）" || app == "com.hcrs" { return Some(1003425); }
    if app == "腾讯视频·云视听" || app == "com.ktcp.video" { return Some(1003426); }
    if app == "美团打车" || app == "com.meituan.qcs.c.android" { return Some(1003427); }
    if app == "腾讯视频HD" || app == "com.tencent.qqlivepad" { return Some(1003428); }
    if app == "腾讯视频极速版" || app == "com.tencent.videolite.android" { return Some(1003429); }
    if app == "高德地图" || app == "com.autonavi.minimap" { return Some(1003430); }
    if app == "HeyTap健康" || app == "com.heytap.health" { return Some(1003431); }
    if app == "享道出行" || app == "com.saicmobility.user" { return Some(1003432); }
    if app == "曹操出行" || app == "cn.caocaokeji.user" { return Some(1003433); }
    if app == "首汽约车" || app == "com.ichinait.gbpassenger" { return Some(1003434); }
    if app == "万顺叫车" || app == "com.wsecar.wsjc" { return Some(1003435); }
    if app == "前程无忧51Job" || app == "com.job.android" { return Some(1003436); }
    if app == "前程无忧学生版" || app == "com.campus.android" { return Some(1003437); }
    if app == "T3出行" || app == "com.t3go.passenger" { return Some(1003438); }
    if app == "神州专车" || app == "com.szzc.ucar.pilot" { return Some(1003439); }
    if app == "嘀嗒出行" || app == "com.didapinche.booking" { return Some(1003440); }
    if app == "花小猪打车" || app == "com.huaxiaozhu.rider" { return Some(1003441); }
    if app == "猎聘" || app == "com.lietou.mishu" { return Some(1003442); }
    if app == "腾讯自选股" || app == "com.tencent.portfolio" { return Some(1003443); }
    if app == "京东健康" || app == "com.jd.jdhealth" { return Some(1003444); }
    if app == "腾讯游戏助手" || app == "com.tencent.gamehelper" { return Some(1003445); }
    if app == "腾讯游戏管家" || app == "com.tencent.gamestick" { return Some(1003446); }
    if app == "腾讯英语君小学版" || app == "com.tencent.aieducation" { return Some(1003447); }
    if app == "腾讯体育" || app == "com.tencent.qqsports" { return Some(1003448); }
    if app == "腾讯先锋" || app == "com.tencent.gamereva" { return Some(1003449); }
    if app == "腾讯手游加速器" || app == "com.tencent.cmocmna" { return Some(1003450); }
    if app == "腾讯搜活帮" || app == "com.tencent.csapp" { return Some(1003451); }
    if app == "腾讯文档" || app == "com.tencent.docs" { return Some(1003452); }
    if app == "腾讯微云" || app == "com.qq.qcloud" { return Some(1003453); }
    if app == "腾讯开心鼠" || app == "com.tencent.abcmouse" { return Some(1003454); }
    if app == "腾讯文件管理器" || app == "com.tencent.FileManager" { return Some(1003455); }
    if app == "腾讯广东麻将" || app == "com.tencent.ggame" { return Some(1003456); }
    if app == "腾讯加速器" || app == "com.tencent.xriver" { return Some(1003457); }
    if app == "腾讯动漫" || app == "com.qq.ac.android" { return Some(1003458); }
    if app == "腾讯WiFi管家" || app == "com.tencent.wifimanager" { return Some(1003459); }
    if app == "腾讯清理大师" || app == "android.lite.clean" { return Some(1003460); }
    if app == "腾讯企鹅辅导" || app == "com.tencent.k12" { return Some(1003461); }
    if app == "圣斗士星矢（腾讯）" || app == "com.tencent.tmgp.sskgame" { return Some(1003462); }
    if app == "腾讯医典" || app == "com.tencent.mymedinfo" { return Some(1003463); }
    if app == "腾讯NOW直播" || app == "com.tencent.now" { return Some(1003464); }
    if app == "腾讯企点" || app == "com.tencent.qidian" { return Some(1003465); }
    if app == "腾讯翻译君" || app == "com.qb.qtranslator" { return Some(1003466); }
    if app == "TXT全本免费阅读" || app == "cn.txtqbmfyd.reader" { return Some(1003467); }
    if app == "TXT免费全本阅读器" || app == "com.mianfeinovel" { return Some(1003468); }
    if app == "京东读书" || app == "com.jd.app.reader" { return Some(1003469); }
    if app == "看书神器阅读器" || app == "com.kanshushenqi.ebook.app" { return Some(1003470); }
    if app == "TXT文本听书" || app == "com.wyfc.itingtxt2" { return Some(1003471); }
    if app == "TXT全本免费电子书" || app == "cc.remennovel" { return Some(1003472); }
    if app == "咿啦看书绘本故事" || app == "com.ellabook" { return Some(1003473); }
    if app == "饭团看书Pro" || app == "com.fantuankanshu.ftpro" { return Some(1003474); }
    if app == "饭团看书" || app == "com.fantuankanshujbk" { return Some(1003475); }
    if app == "博看书苑" || app == "cn.com.bookan" { return Some(1003476); }
    if app == "58好借" || app == "com.wuba.borrowfinancials" { return Some(1003477); }
    if app == "宜人贷借款" || app == "com.yirendai" { return Some(1003478); }
    if app == "广场舞视频初级教学大全" || app == "com.live.wallpaperang" { return Some(1003479); }
    if app == "天猫读书" || app == "com.aliwx.tmreader" { return Some(1003480); }
    if app == "贵健康" || app == "cn.longmaster.health" { return Some(1003481); }
    if app == "孕健康" || app == "com.hbty.yjk.people" { return Some(1003482); }
    if app == "健康廊坊" || app == "app.can" { return Some(1003483); }
    if app == "小荷健康" || app == "com.lvsongguo" { return Some(1003484); }
    if app == "北京昌平健康云" || app == "com.wondersgroup.hs.healthcloudcp.patient" { return Some(1003485); }
    if app == "S 健康" || app == "com.sec.android.app.shealth" { return Some(1003486); }
    if app == "居民健康" || app == "com.jkx4rh.client" { return Some(1003487); }
    if app == "左点健康" || app == "com.zdeer.earpick" { return Some(1003488); }
    if app == "健康160" || app == "cn.kidyn.qdmedical160" { return Some(1003489); }
    if app == "vivo运动健康" || app == "com.vivo.health" { return Some(1003490); }
    if app == "健康乐" || app == "com.mhealth.app" { return Some(1003491); }
    if app == "复星健康" || app == "com.wanbangcloudhelth.fengyouhui" { return Some(1003492); }
    if app == "万步健康" || app == "com.wanbu.dascom" { return Some(1003493); }
    if app == "健康甘肃" || app == "com.gsww.gsrhc.jkgs" { return Some(1003494); }
    if app == "健康苏州掌上行" || app == "com.sz.health" { return Some(1003495); }
    if app == "健康优加" || app == "com.yinhai.jiankanghui" { return Some(1003496); }
    if app == "健康南京" || app == "com.focustech.medical" { return Some(1003497); }
    if app == "华西健康" || app == "com.scics.huaxi" { return Some(1003498); }
    if app == "健康中山" || app == "com.bsoft.mhealthp.zhongshan" { return Some(1003499); }
    if app == "健康守护者" || app == "com.quliang.jkshz" { return Some(1003500); }
    if app == "国寿AI健康" || app == "com.borui.gsjk" { return Some(1003501); }
    if app == "薄荷健康" || app == "com.boohee.one" { return Some(1003502); }
    if app == "轻牛健康" || app == "com.qingniu.plus" { return Some(1003503); }
    if app == "健康走路宝" || app == "com.jiankang.zoulubao" { return Some(1003504); }
    if app == "翼健康" || app == "com.gdhbgh.activity" { return Some(1003505); }
    if app == "健康运动宝" || app == "com.gds.jkydb" { return Some(1003506); }
    if app == "健康东莞" || app == "com.sms.smsmemberappjkdg" { return Some(1003507); }
    if app == "优健康" || app == "com.ihaozhuo.youjiankang" { return Some(1003508); }
    if app == "心脏健康研究" || app == "com.plagh.heartstudy" { return Some(1003509); }
    if app == "平安健康" || app == "com.pingan.papd" { return Some(1003510); }
    if app == "平安健康保" || app == "com.pajk.bd" { return Some(1003511); }
    if app == "平安健康保险" || app == "com.pa.health" { return Some(1003512); }
    if app == "乐动健康生活" || app == "com.tjd.lelife" { return Some(1003513); }
    if app == "乐动健康" || app == "com.tjd.tjdmainS2" { return Some(1003514); }
    if app == "健康界" || app == "com.hmkx.zgjkj" { return Some(1003515); }
    if app == "京视健康" || app == "com.helichuanmei.jinshijiankang" { return Some(1003516); }
    if app == "职业健康培训" || app == "com.huayi.zyjk" { return Some(1003517); }
    if app == "健康绍兴" || app == "com.ucmed.shaoxing.pt" { return Some(1003518); }
    if app == "健康山西" || app == "com.uh.rdsp" { return Some(1003519); }
    if app == "香山健康" || app == "com.senssun.senssuncloud" { return Some(1003520); }
    if app == "飞利浦水健康" || app == "com.mxchip.philips" { return Some(1003521); }
    if app == "派健康" || app == "com.lstech.rehealth" { return Some(1003522); }
    if app == "杭州健康通" || app == "com.hzhealth.medicalcare" { return Some(1003523); }
    if app == "读书瞳" || app == "com.k123du.huiben" { return Some(1003524); }
    if app == "微信读书" || app == "com.tencent.weread" { return Some(1003525); }
    if app == "连城读书" || app == "com.nine.NovelReader" { return Some(1003526); }
    if app == "红袖读书" || app == "com.hongxiu.app" { return Some(1003527); }
    if app == "连尚免费读书" || app == "com.wifi.reader.free" { return Some(1003528); }
    if app == "网易蜗牛读书" || app == "com.netease.snailread" { return Some(1003529); }
    if app == "连尚读书" || app == "com.wifi.reader" { return Some(1003530); }
    if app == "连尚读书极速版" || app == "com.wifi.reader.lite" { return Some(1003531); }
    if app == "追书神器免费版" || app == "com.ushaqi.zhuishushenqi.adfree" { return Some(1003532); }
    if app == "免费追书" || app == "com.mianfeizs.book" { return Some(1003533); }
    if app == "申怡读书" || app == "com.dyw" { return Some(1003534); }
    if app == "百词斩爱阅读" || app == "com.baicizhan.ireading" { return Some(1003535); }
    if app == "网易云音乐" || app == "com.netease.cloudmusic" { return Some(1003536); }
    if app == "京东金融" || app == "com.jd.jrapp" { return Some(1003537); }
    if app == "淘宝联盟" || app == "com.alimama.moon" { return Some(1003538); }
    if app == "淘宝主播" || app == "com.taobao.live4anchor" { return Some(1003539); }
    if app == "相声评书戏曲大全" || app == "com.qinqinxiong.apps.qqxbook" { return Some(1003540); }
    if app == "今日头条大字版" || app == "com.ss.android.article.daziban" { return Some(1003541); }
    if app == "驾考宝典摩托车" || app == "moto.app.good" { return Some(1003542); }
    if app == "驾考宝典3D练车" || app == "jiakaokeyi.app.good" { return Some(1003543); }
    if app == "驾考宝典极速版" || app == "jiakaokesi.app.good" { return Some(1003544); }
    if app == "全民K歌极速版" || app == "com.tencent.kg.android.lite" { return Some(1003545); }
    if app == "UPTsmAddon" || app == "com.unionpay.tsmservice" { return Some(1003546); }
    if app == "锤子音乐播放器" || app == "com.smartisanos.music" { return Some(1003547); }
    if app == "Nokia X 音乐播放器" || app == "com.android.music" { return Some(1003548); }
    if app == "索尼音乐播放器" || app == "com.sonyericsson.music" { return Some(1003549); }
    if app == "Google Play 游戏" || app == "com.google.android.play.games" { return Some(1003550); }
    if app == "OPPO商城" || app == "com.oppo.store" { return Some(1003551); }
    if app == "东方财富期货" || app == "com.eastmoney.app.qhsjkh" { return Some(1003552); }
    if app == "徽行信用卡" || app == "cn.com.huishangbank.mbank" { return Some(1003553); }
    if app == "广银信用卡" || app == "cn.com.gzbank.mbank" { return Some(1003554); }
    if app == "上海农商银行信用卡" || app == "cn.com.srcb.mbank" { return Some(1003555); }
    if app == "哈行信用卡" || app == "cn.com.hebbank.mbank" { return Some(1003556); }
    if app == "浦发信用卡" || app == "com.spdbccc.app" { return Some(1003557); }
    if app == "包商信用卡" || app == "cn.com.bsbank.mbank" { return Some(1003558); }
    if app == "快搜搜题" || app == "com.questions.liteai" { return Some(1003559); }
    if app == "货拉拉司机版" || app == "com.lalamove.huolala.driver" { return Some(1003560); }
    if app == "猿辅导" || app == "com.yuantiku.tutor" { return Some(1003561); }
    if app == "Google 通讯录同步" || app == "com.google.android.syncadapters.contacts" { return Some(1003562); }
    if app == "哔哩哔哩漫画" || app == "com.bilibili.comic" { return Some(1003563); }
    if app == "哔哩哔哩直播姬" || app == "com.bilibili.bilibililive" { return Some(1003564); }
    if app == "哔哩哔哩概念" || app == "com.bilibili.app.blue" { return Some(1003565); }
    if app == "Google Play 服务" || app == "com.google.android.gms" { return Some(1003566); }
    if app == "阿里云盘" || app == "com.alicloud.databox" { return Some(1003567); }
    if app == "闪电搜题" || app == "com.lightning.edu.ei" { return Some(1003568); }
    if app == "Google PDF 查看器" || app == "com.google.android.apps.pdfviewer" { return Some(1003569); }
    if app == "小包搜题" || app == "com.ixyzh.question" { return Some(1003570); }
    if app == "PP助手" || app == "com.pp.assistant" { return Some(1003571); }
    if app == "火星搜题" || app == "com.fenbi.android.souti" { return Some(1003572); }
    if app == "小猿搜题" || app == "com.fenbi.android.solar" { return Some(1003573); }
    if app == "中国银行缤纷生活" || app == "com.forms" { return Some(1003574); }
    if app == "OPPO社区" || app == "com.oppo.community" { return Some(1003575); }
    if app == "买单吧" || app == "com.bankcomm.maidanba" { return Some(1003576); }
    if app == "海南农信个人手机银行" || app == "com.nxy.hn" { return Some(1003577); }
    if app == "作业帮家长版" || app == "com.zuoyebang.knowledge" { return Some(1003578); }
    if app == "作业帮直播课" || app == "com.zuoyebang.airclass" { return Some(1003579); }
    if app == "E听说中学" || app == "com.ets100.secondary" { return Some(1003580); }
    if app == "快狗打车" || app == "com.wuba.huoyun" { return Some(1003581); }
    if app == "个人手机银行" || app == "cn.com.shbank.mper" { return Some(1003582); }
    if app == "大学搜题酱" || app == "com.zmzx.college.search" { return Some(1003583); }
    if app == "知乎日报" || app == "com.zhihu.daily.android" { return Some(1003584); }
    if app == "百词斩词典" || app == "com.baicizhan.dict" { return Some(1003585); }
    if app == "蜻蜓FM" || app == "fm.qingting.qtradio" { return Some(1003586); }
    if app == "快狗打车司机版" || app == "com.cxyw.suyun.ui" { return Some(1003587); }
    if app == "货拉拉搬家小哥" || app == "com.lalamove.huolala.porter" { return Some(1003588); }
    if app == "Yesoul野小兽系统" || app == "yesoul.yesoulmobile" { return Some(1003589); }
    if app == "货拉拉企业版" || app == "com.lalamove.huolala.eclient" { return Some(1003590); }
    if app == "手机天猫" || app == "com.tmall.wireless" { return Some(1003591); }
    if app == "Wind金融终端" || app == "windinfo.android" { return Some(1003592); }
    if app == "苏宁消费金融" || app == "com.csii.sncfc" { return Some(1003593); }
    if app == "苏宁金融" || app == "com.suning.mobile.epa" { return Some(1003594); }
    if app == "顺丰金融" || app == "com.sfpay.mobile" { return Some(1003595); }
    if app == "马上金融" || app == "com.msxf.loan" { return Some(1003596); }
    if app == "捷信金融" || app == "com.hcc.app" { return Some(1003597); }
    if app == "国美金融" || app == "com.gomemyc.mylc.android" { return Some(1003598); }
    if app == "晋商消费金融" || app == "io.dcloud.H59193852" { return Some(1003599); }
    if app == "中银消费金融" || app == "cn.boccfc.loan.finance" { return Some(1003600); }
    if app == "中国人寿综合金融" || app == "com.sinosoft.mobilebiz.chinalife" { return Some(1003601); }
    if app == "小米金融" || app == "com.xiaomi.jr" { return Some(1003602); }
    if app == "中原消费金融" || app == "com.hnzycfc.zyxj" { return Some(1003603); }
    if app == "招联金融" || app == "com.zl.fqbao" { return Some(1003604); }
    if app == "杭银金融" || app == "tomcat360.com.hyxfjr" { return Some(1003605); }
    if app == "石化金融" || app == "com.sinopec.fintech" { return Some(1003606); }
    if app == "京东极速版" || app == "com.jd.jdlite" { return Some(1003607); }
    if app == "京东钱包" || app == "com.wangyin.payment" { return Some(1003608); }
    if app == "京东医生" || app == "com.jd.dh" { return Some(1003609); }
    if app == "御风云视频" || app == "com.yufengyun.video.android" { return Some(1003610); }
    if app == "龙岗视频门禁" || app == "cn.xinyi.lgspmj" { return Some(1003611); }
    if app == "视频去水印免费" || app == "com.spqsymf.app.them" { return Some(1003612); }
    if app == "时光小视频" || app == "com.tencent.ipai" { return Some(1003613); }
    if app == "大鱼视频助手" || app == "com.jianpian.xiaoxigua" { return Some(1003614); }
    if app == "迅捷视频转换器" || app == "com.hudun.androidvideochanger" { return Some(1003615); }
    if app == "酷秀短视频" || app == "com.banban.kuxiu" { return Some(1003616); }
    if app == "清爽视频编辑" || app == "com.hudun.videoedit" { return Some(1003617); }
    if app == "华为 VR 视频" || app == "com.huawei.himovie.vr" { return Some(1003618); }
    if app == "抖看短视频" || app == "com.duoyutech.doukan" { return Some(1003619); }
    if app == "视频格式转换工厂" || app == "com.cjtec.videoformat" { return Some(1003620); }
    if app == "MX播放器专业版 MX Video Player Pro" || app == "com.mxtech.videoplayer.pro" { return Some(1003621); }
    if app == "沃视频" || app == "com.unicom.woshipin" { return Some(1003622); }
    if app == "Mx播放器解码包 Player Codec ARMv7 NEON" || app == "com.mxtech.ffmpeg.v7_neon" { return Some(1003623); }
    if app == "音视频转换" || app == "com.mvtrail.xiaomi.videotomp3converter" { return Some(1003624); }
    if app == "VLC多媒体播放器" || app == "org.videolan.vlc" { return Some(1003625); }
    if app == "唯美视频剪辑" || app == "com.video.fclip" { return Some(1003626); }
    if app == "视频剪辑猫" || app == "com.clipzz.media" { return Some(1003627); }
    if app == "秘乐短视频" || app == "com.milecn.milevideo" { return Some(1003628); }
    if app == "V8音视频" || app == "com.melon.lazymelon" { return Some(1003629); }
    if app == "3D魔秀AE视频制作" || app == "com.iMMcque.moshow" { return Some(1003630); }
    if app == "视频免费去水印" || app == "com.wuxibeierbangzeren" { return Some(1003631); }
    if app == "恒星播放器" || app == "cn.coldlake.stellarmobile" { return Some(1003632); }
    if app == "书单视频助手" || app == "com.lingbao.booklistvideo" { return Some(1003633); }
    if app == "联通视频彩铃" || app == "com.iflytek.womusicclient" { return Some(1003634); }
    if app == "悦颜视频美颜" || app == "com.leshu.facebeauty" { return Some(1003635); }
    if app == "爱剪辑视频编辑器" || app == "com.aijianji.clip" { return Some(1003636); }
    if app == "迅捷视频剪辑" || app == "com.fresh.light" { return Some(1003637); }
    if app == "搜狐视频HD" || app == "com.sohu.tv" { return Some(1003638); }
    if app == "快码万能播放器" || app == "com.kmsoft.fvplayer" { return Some(1003639); }
    if app == "PrettyUp视频人像美化" || app == "com.accordion.prettyo.cn" { return Some(1003640); }
    if app == "动效忍者AE视频特效制作" || app == "com.accarunit.motionvideoeditor.cn" { return Some(1003641); }
    if app == "央视影音HD" || app == "cn.cntvhd" { return Some(1003642); }
    if app == "手机QQ影音" || app == "com.tencent.research.drop" { return Some(1003643); }
    if app == "拼多多商家版" || app == "com.xunmeng.merchant" { return Some(1003644); }
    if app == "拼多多快递员版" || app == "com.xunmeng.deliver" { return Some(1003645); }
    if app == "京东到家" || app == "com.jingdong.pdj" { return Some(1003646); }
    if app == "京东掌柜宝" || app == "com.jd.b2b" { return Some(1003647); }
    if app == "京东慧采" || app == "com.jd.cdyjy.vsp" { return Some(1003648); }
    if app == "京东快递" || app == "com.jd.jdlogistic" { return Some(1003649); }
    if app == "京东云无线宝" || app == "com.jdcloud.mt.smartrouter" { return Some(1003650); }
    if app == "京东京车会" || app == "com.jdcar.jch" { return Some(1003651); }
    if app == "京东云企业管家" || app == "com.jd.tobs" { return Some(1003652); }
    if app == "京东商选" || app == "com.jd.hyt" { return Some(1003653); }
    if app == "京东小家" || app == "com.jd.iots" { return Some(1003654); }
    if app == "西瓜视频横屏版" || app == "com.ss.android.article.video.landscape" { return Some(1003655); }
    if app == "全本免费阅读器" || app == "com.quanben.novel" { return Some(1003656); }
    if app == "美团配送" || app == "com.meituan.banma.errand" { return Some(1003657); }
    if app == "美团优选" || app == "com.sankuai.youxuan" { return Some(1003658); }
    if app == "美团买菜" || app == "com.meituan.retail.v.android" { return Some(1003659); }
    if app == "京东饭粒" || app == "com.jd.fanli" { return Some(1003660); }
    if app == "知网阅读" || app == "com.cnki.reader" { return Some(1003661); }
    if app == "息壤阅读" || app == "com.readunion.ireader" { return Some(1003662); }
    if app == "儿童阅读训练营" || app == "com.jojoread.readcamp" { return Some(1003663); }
    if app == "薄荷阅读" || app == "com.chaoui.mintreading" { return Some(1003664); }
    if app == "卢卡Luka阅读养成" || app == "ai.ling.luka.app" { return Some(1003665); }
    if app == "红薯阅读" || app == "com.hongshu" { return Some(1003666); }
    if app == "呱呱阅读" || app == "com.jiliguala.library" { return Some(1003667); }
    if app == "极速PDF阅读器" || app == "com.jisu.pdf" { return Some(1003668); }
    if app == "锤子阅读" || app == "com.smartisan.reader" { return Some(1003669); }
    if app == "中油阅读" || app == "com.jiaoyu.shiyou" { return Some(1003670); }
    if app == "酷匠阅读" || app == "com.dpx.kujiang" { return Some(1003671); }
    if app == "Anyview阅读器" || app == "com.anyview" { return Some(1003672); }
    if app == "向日葵阅读" || app == "com.esread.sunflowerstudent" { return Some(1003673); }
    if app == "布咕阅读" || app == "uni.UNI001CBA1" { return Some(1003674); }
    if app == "有兔阅读" || app == "com.tomato.bookreader" { return Some(1003675); }
    if app == "网易云阅读" || app == "com.netease.pris" { return Some(1003676); }
    if app == "多看阅读HD" || app == "com.duokan.hdreader" { return Some(1003677); }
    if app == "文档阅读器PPT" || app == "com.poi.poiandroid" { return Some(1003678); }
    if app == "淘特商家版" || app == "com.taobao.qianniutjb" { return Some(1003679); }
    if app == "乐视视频" || app == "com.letv.android.client" { return Some(1003680); }
    if app == "央视频" || app == "com.cctv.yangshipin.app.androidp" { return Some(1003681); }
    if app == "InShot视频和照片编辑软件" || app == "com.camerasideas.instashot" { return Some(1003682); }
    if app == "视频转换压缩" || app == "com.shulantech.video" { return Some(1003683); }
    if app == "有趣小视频" || app == "com.shortplay.video" { return Some(1003684); }
    if app == "红包视频" || app == "com.sanmiao.sound" { return Some(1003685); }
    if app == "星空视频壁纸" || app == "yyc.xk" { return Some(1003686); }
    if app == "巧虎视频乐园" || app == "com.qh.tesla" { return Some(1003687); }
    if app == "PP视频" || app == "com.pplive.androidphone" { return Some(1003688); }
    if app == "视频美颜精灵" || app == "com.jhsoft.spmyjl" { return Some(1003689); }
    if app == "火萤视频壁纸" || app == "com.znxh.hyhuo" { return Some(1003690); }
    if app == "视频剪辑编辑软件" || app == "com.zkhcsoft.jianyi" { return Some(1003691); }
    if app == "好看视频" || app == "com.baidu.haokan" { return Some(1003692); }
    if app == "小米视频" || app == "com.miui.video" { return Some(1003693); }
    if app == "萤石云视频" || app == "com.videogo" { return Some(1003694); }
    if app == "花生免费视频" || app == "com.microx.peanut" { return Some(1003695); }
    if app == "小米视频电话" || app == "com.mi.vtalk" { return Some(1003696); }
    if app == "VUE视频剪辑" || app == "com.vue.edit" { return Some(1003697); }
    if app == "muse短视频" || app == "com.zhiliaoapp.musically" { return Some(1003698); }
    if app == "美册视频编辑制作" || app == "com.iMMcque.VCore" { return Some(1003699); }
    if app == "咪咕视频爱看版" || app == "com.wondertek.miguaikan" { return Some(1003700); }
    if app == "完美视频播放器" || app == "com.kk.xx.newplayer" { return Some(1003701); }
    if app == "乐秀视频编辑器" || app == "com.xvideostudio.videoeditor" { return Some(1003702); }
    if app == "刷宝短视频" || app == "com.jm.video" { return Some(1003703); }
    if app == "咪咕音乐" || app == "cmccwm.mobilemusic" { return Some(1003704); }
    if app == "无损音乐下载器" || app == "info.zzjdev.musicdownload" { return Some(1003705); }
    if app == "酷我音乐HD" || app == "cn.kuwo.kwmusichd" { return Some(1003706); }
    if app == "波点音乐" || app == "cn.wenyu.bodian" { return Some(1003707); }
    if app == "板凳音乐" || app == "cn.zevun.hb.music" { return Some(1003708); }
    if app == "葫芦音乐" || app == "com.xiaoniu.hulumusic" { return Some(1003709); }
    if app == "千千音乐" || app == "com.ting.mp3.android" { return Some(1003710); }
    if app == "超级音乐编辑器" || app == "com.tianxingjian.supersound" { return Some(1003711); }
    if app == "QQ音乐HD" || app == "com.tencent.qqmusicpad" { return Some(1003712); }
    if app == "Bose音乐" || app == "com.bose.bosemusic.china" { return Some(1003713); }
    if app == "MOO音乐" || app == "com.tencent.blackkey" { return Some(1003714); }
    if app == "音乐剪辑助手" || app == "com.smallyin.musiceditor" { return Some(1003715); }
    if app == "5sing原创音乐" || app == "com.sing.client" { return Some(1003716); }
    if app == "DJ音乐盒" || app == "com.djbox.product" { return Some(1003717); }
    if app == "OPPO音乐" || app == "com.oppo.music" { return Some(1003718); }
    if app == "车载DJ音乐盒" || app == "com.e5837972.kgt" { return Some(1003719); }
    if app == "小熊音乐" || app == "com.msb.bear.music" { return Some(1003720); }
    if app == "恩雅音乐" || app == "com.enya.enyamusic" { return Some(1003721); }
    if app == "咪咕音乐极速版" || app == "com.migu.music.mini" { return Some(1003722); }
    if app == "Flyme音乐" || app == "com.meizu.media.music" { return Some(1003723); }
    if app == "汽水音乐" || app == "com.luna.music" { return Some(1003724); }
    if app == "E歌蒙古音乐" || app == "com.ezen.ehshig" { return Some(1003725); }
    if app == "酷狗音乐" || app == "com.kugou.android" { return Some(1003726); }
    if app == "YouTube 音乐" || app == "com.google.android.apps.youtube.music" { return Some(1003727); }
    if app == "音乐CP" || app == "com.gt.musicalcp" { return Some(1003728); }
    if app == "AI音乐学园" || app == "com.immusician.music" { return Some(1003729); }
    if app == "海贝音乐" || app == "com.hiby.music" { return Some(1003730); }
    if app == "索尼精选HiRes音乐" || app == "com.hiresmusic" { return Some(1003731); }
    if app == "屏幕阅读Talkback" || app == "com.google.android.marvin.talkback" { return Some(1003732); }
    if app == "流利说-阅读" || app == "com.liulishuo.vira" { return Some(1003733); }
    if app == "三星阅读" || app == "com.mci.smagazine" { return Some(1003734); }
    if app == "阅站漫画阅读器" || app == "com.nw.cleansite.novel" { return Some(1003735); }
    if app == "豆瓣阅读" || app == "com.douban.book.reader" { return Some(1003736); }
    if app == "道客阅读" || app == "com.doc88.reader" { return Some(1003737); }
    if app == "点众阅读" || app == "com.dianzhong.reader" { return Some(1003738); }
    if app == "当当云阅读" || app == "com.dangdang.reader" { return Some(1003739); }
    if app == "扇贝阅读" || app == "com.shanbay.news" { return Some(1003740); }
    if app == "快点阅读" || app == "com.skyplatanus.crucio" { return Some(1003741); }
    if app == "广州智慧阅读" || app == "com.smart.reading.app" { return Some(1003742); }
    if app == "一米阅读" || app == "com.student.yuwen.yimilan" { return Some(1003743); }
    if app == "百度阅读" || app == "com.baidu.yuedu" { return Some(1003744); }
    if app == "Kindle阅读" || app == "com.amazon.kindlefc" { return Some(1003745); }
    if app == "PDF阅读器 Adobe Reader" || app == "com.adobe.reader" { return Some(1003746); }
    if app == "一米阅读家长" || app == "com.yimilan.yuwen.parent" { return Some(1003747); }
    if app == "长佩阅读" || app == "net.cpwxw.cpfiction" { return Some(1003748); }
    if app == "KaDa阅读" || app == "com.hhdd.kada" { return Some(1003749); }
    if app == "黑岩阅读" || app == "com.heiyan.reader" { return Some(1003750); }
    if app == "步步阅读" || app == "com.putao.KidReading.bookbook" { return Some(1003751); }
    if app == "美时视频美颜" || app == "com.qyapp.msbeauty" { return Some(1003752); }
    if app == "天猫魔投" || app == "com.taobao.motou" { return Some(1003753); }
    if app == "天猫养车" || app == "com.ncarzone.tmyc" { return Some(1003754); }
    if app == "天猫精灵" || app == "com.alibaba.ailabs.tg" { return Some(1003755); }
    if app == "牛咔视频" || app == "com.xweisoft.nbs" { return Some(1003756); }
    if app == "抖音火山版" || app == "com.ss.android.ugc.live" { return Some(1003757); }
    if app == "美团打车司机" || app == "com.meituan.qcs.r.android" { return Some(1003758); }
    if app == "美团优选物流" || app == "com.meituan.android.grocery.gms" { return Some(1003759); }
    if app == "美团优选团长" || app == "com.meituan.grocery.gh" { return Some(1003760); }
    if app == "美团民宿" || app == "com.meituan.phoenix" { return Some(1003761); }
    if app == "美团极速版" || app == "com.meituan.turbo" { return Some(1003762); }
    if app == "美团众包" || app == "com.sankuai.meituan.dispatch.crowdsource" { return Some(1003763); }
    if app == "美团骑手" || app == "com.sankuai.meituan.dispatch.homebrew" { return Some(1003764); }
    if app == "美团外卖商家版" || app == "com.sankuai.meituan.meituanwaimaibusiness" { return Some(1003765); }
    if app == "美团开店宝" || app == "com.sankuai.meituan.merchant" { return Some(1003766); }
    if app == "美团拍店" || app == "com.sankuai.meituan.pai" { return Some(1003767); }
    if app == "美团酒店商家" || app == "com.sankuai.mhotel" { return Some(1003768); }
    if app == "世纪证券小薇" || app == "com.zztzt.sjscgen" { return Some(1003769); }
    if app == "东吴证券同花顺" || app == "com.hexin.plat.android.DongwuSecurity" { return Some(1003770); }
    if app == "财通证券" || app == "com.hexin.plat.android.CaitongSecurity" { return Some(1003771); }
    if app == "京东股票" || app == "com.jd.stock" { return Some(1003772); }
    if app == "安信手机证券" || app == "cn.com.essence.stock" { return Some(1003773); }
    if app == "股市教练" || app == "com.hexin.android.stocktrain" { return Some(1003774); }
    if app == "华宝证券" || app == "com.hbscphone" { return Some(1003775); }
    if app == "开源手机证券" || app == "com.hexin.plat.android.KaiyuanSecurity" { return Some(1003776); }
    if app == "模拟炒股票" || app == "com.winner.simulatetrade" { return Some(1003777); }
    if app == "广证股涨" || app == "com.gangzhousc" { return Some(1003778); }
    if app == "大同证券大众版" || app == "com.hexin.plat.android.DatongSecurity" { return Some(1003779); }
    if app == "优顾炒股" || app == "com.jhss.youguu" { return Some(1003780); }
    if app == "中原证券掌中网超享版" || app == "com.zysc" { return Some(1003781); }
    if app == "同花顺股票开户" || app == "com.hexin.plat.kaihu" { return Some(1003782); }
    if app == "东莞证券大智慧" || app == "com.dongguanzq.dzh" { return Some(1003783); }
    if app == "牛股宝" || app == "com.ngb.stock" { return Some(1003784); }
    if app == "大咖问股" || app == "com.shendeng.note" { return Some(1003785); }
    if app == "干股帮" || app == "com.hyxk.ggmall.personal" { return Some(1003786); }
    if app == "民生证券" || app == "qianlong.qlmobile.minsheng" { return Some(1003787); }
    if app == "申银万国赢家理财手机证券股票软件" || app == "com.sywg.stock" { return Some(1003788); }
    if app == "云财经股票" || app == "com.pointercn.yunvs" { return Some(1003789); }
    if app == "华福证券掌乐" || app == "com.huafuzq.dzh" { return Some(1003790); }
    if app == "华鑫证券鑫e代" || app == "com.zztzt.huaxin" { return Some(1003791); }
    if app == "同花顺?HD炒股必备" || app == "com.hexin.plat.android.gpad" { return Some(1003792); }
    if app == "湘财股掌乐" || app == "com.xiangcaizq.dzh" { return Some(1003793); }
    if app == "i问财选股" || app == "com.hexin.android.stockassistant" { return Some(1003794); }
    if app == "东方悦享股票开户" || app == "com.orientsec.zjbl" { return Some(1003795); }
    if app == "华融证券大智慧" || app == "com.huarongzq.dzh" { return Some(1003796); }
    if app == "览益股市" || app == "com.lanyi.live" { return Some(1003797); }
    if app == "手机证券大智慧版" || app == "com.dazhihui.ydzq" { return Some(1003798); }
    if app == "天天慧选股" || app == "com.baidao.nugget" { return Some(1003799); }
    if app == "股票牛港股美股" || app == "com.hyhk.stock" { return Some(1003800); }
    if app == "广发手机证券至慧版" || app == "com.gfjgj.dzh" { return Some(1003801); }
    if app == "东北证券大智慧" || app == "com.dongbeizq.dzh" { return Some(1003802); }
    if app == "广州证券" || app == "com.hexin.plat.android.GuangzhouAMSecurity" { return Some(1003803); }
    if app == "优投顾股票" || app == "com.upchina.advisor" { return Some(1003804); }
    if app == "广发手机证券旧版" || app == "gf.king.app" { return Some(1003805); }
    if app == "民族证券(专业版)" || app == "com.hexin.plat.android.MzzqSecurity" { return Some(1003806); }
    if app == "淘股王炒股票" || app == "com.qifuxiang.tgw" { return Some(1003807); }
    if app == "山西证券" || app == "com.hexin.plat.android.ShanxiSecurity" { return Some(1003808); }
    if app == "渤海证券手机炒股" || app == "com.hexin.plat.android.BohaiSecurity" { return Some(1003809); }
    if app == "中山证券" || app == "com.zssc" { return Some(1003810); }
    if app == "东方赢家手机炒股" || app == "com.hundsun.stockwinner.dfzq" { return Some(1003811); }
    if app == "宏信证券投资赢家高端版" || app == "com.hundsun.stockwinner.hxzqgdb" { return Some(1003812); }
    if app == "长城随身股" || app == "com.hexin.plat.android.ChangchengSecurity" { return Some(1003813); }
    if app == "首创证券" || app == "com.hundsun.stockwinner.sczq" { return Some(1003814); }
    if app == "英大证券大智慧" || app == "com.yingdazq.dzh" { return Some(1003815); }
    if app == "一米阅读老师" || app == "com.yimilan.yuwen.teacher" { return Some(1003816); }
    if app == "影视大全快看" || app == "cn.quicktv.android" { return Some(1003817); }
    if app == "笔顺笔画大全" || app == "com.syyh.bishun" { return Some(1003818); }
    if app == "图片大全搜图" || app == "com.myd.picsearch" { return Some(1003819); }
    if app == "书法碑帖大全" || app == "com.xpz.shufaapp.free" { return Some(1003820); }
    if app == "地图大全" || app == "com.cutler.dragonmap" { return Some(1003821); }
    if app == "看看影视大全" || app == "com.rumtel.mobiletv" { return Some(1003822); }
    if app == "表情包大全" || app == "com.liou.doutu" { return Some(1003823); }
    if app == "古文典籍大全" || app == "com.qxc.gwdjdq" { return Some(1003824); }
    if app == "免费手机铃声大全" || app == "nilframe.app.mfsjlsdq" { return Some(1003825); }
    if app == "书法字典大全" || app == "com.zivn.cloudbrush3" { return Some(1003826); }
    if app == "小啄赚钱" || app == "com.xzzq.xiaozhuo" { return Some(1003827); }
    if app == "妙看赚钱" || app == "com.taige.mygold" { return Some(1003828); }
    if app == "小白赚钱" || app == "com.jike.noobmoney" { return Some(1003829); }
    if app == "华为荣耀钱包" || app == "com.huawei.wallet" { return Some(1003830); }
    if app == "1688工业品" || app == "com.alibaba.mro" { return Some(1003831); }
    if app == "1688微商货源" || app == "com.ykkj.ylbb" { return Some(1003832); }
    if app == "1688商家版" || app == "com.alibaba.wireless.seller" { return Some(1003833); }
    if app == "酷狗唱唱斗歌版" || app == "com.kugou.android.douge" { return Some(1003834); }
    if app == "音乐制作工坊" || app == "app.allergic.musicfactory" { return Some(1003835); }
    if app == "酷狗唱唱" || app == "com.kugou.android.ktvapp" { return Some(1003836); }
    if app == "多唱" || app == "com.evideo.duochang.phone" { return Some(1003837); }
    if app == "唱鸭" || app == "com.rockets.chang" { return Some(1003838); }
    if app == "酷我畅听" || app == "cn.kuwo.tingshu" { return Some(1003839); }
    if app == "高德地图HD" || app == "com.autonavi.minimap.custom" { return Some(1003840); }
    if app == "抖音盒子" || app == "com.ss.android.jumanji" { return Some(1003841); }
    if app == "安全支付服务支付宝插件" || app == "com.alipay.android.app" { return Some(1003842); }
    if app == "抖音来客" || app == "com.bytedance.ls.merchant" { return Some(1003843); }
    if app == "快手概念版" || app == "com.kwai.thanos" { return Some(1003844); }
    if app == "快手小店商家版" || app == "com.kuaishou.merchantshop" { return Some(1003845); }
    if app == "农行掌上银行" || app == "com.android.bankabc" { return Some(1003846); }
    if app == "快手直播伴侣" || app == "com.kwai.livepartner" { return Some(1003847); }
    if app == "华为智能助手" || app == "com.huawei.intelligent" { return Some(1003848); }
    if app == "主题动态壁纸" || app == "com.themewallpaper.douping" { return Some(1003849); }
    if app == "HiCare" || app == "com.huawei.phoneservice" { return Some(1003850); }
    if app == "手机克隆大师" || app == "com.klst.app.clone" { return Some(1003851); }
    if app == "手机克隆专家" || app == "com.tian.phonebak" { return Some(1003852); }
    if app == "中国建设银行" || app == "com.chinamworld.main" { return Some(1003853); }
    if app == "华为浏览器" || app == "com.huawei.browser" { return Some(1003854); }
    if app == "兼职咸鱼学生赚钱" || app == "cc.gara.fish.jj_fish" { return Some(1003855); }
    if app == "疯狂猜成语（单机版）" || app == "com.kamitu.drawsth.standalone.free.android" { return Some(1003856); }
    if app == "火山极速版" || app == "com.ss.android.ugc.livelite" { return Some(1003857); }
    if app == "西瓜皮" || app == "com.enqualcomm.kids.cyp" { return Some(1003858); }
    if app == "爱上消消消红包版" || app == "com.liuliuwan.ttxxl.xkx.qbsmss" { return Some(1003859); }
    if app == "东方头条极速版" || app == "com.songheng.fasteastnews" { return Some(1003860); }
    if app == "步多多" || app == "com.qsmy.walkmonkey" { return Some(1003861); }
    if app == "华为帐号" || app == "com.huawei.hwid" { return Some(1003862); }
    if app == "华为应用市场" || app == "com.huawei.appmarket" { return Some(1003863); }
    if app == "智慧搜索" || app == "com.huawei.search" { return Some(1003864); }
    if app == "华为讯息服务" || app == "com.huawei.android.pushagent" { return Some(1003865); }
    if app == "指南针极速版" || app == "com.duoku.compass" { return Some(1003866); }
    if app == "Amigo指南针" || app == "jlzn.com.android.compass" { return Some(1003867); }
    if app == "ZUI指南针" || app == "com.zui.compass" { return Some(1003868); }
    if app == "OPPO指南针" || app == "com.oppo.compass" { return Some(1003869); }
    if app == "手机罗盘指南针软件" || app == "com.biggerlens.compass" { return Some(1003870); }
    if app == "超级罗盘指南针" || app == "com.chaohai.compass" { return Some(1003871); }
    if app == "京喜" || app == "com.jd.pingou" { return Some(1003872); }
    if app == "蜜源" || app == "com.jf.my" { return Some(1003873); }
    if app == "识货" || app == "com.hupu.shihuo" { return Some(1003874); }
    if app == "实惠喵" || app == "com.xihu.shihuimiao" { return Some(1003875); }
    if app == "买什么都省" || app == "com.zhichan.msmds" { return Some(1003876); }
    if app == "工行手机银行" || app == "com.icbc" { return Some(1003877); }
    if app == "山姆会员商店" || app == "cn.samsclub.app" { return Some(1003878); }
    if app == "智慧好医院" || app == "cn.swifthealth.patientApp" { return Some(1003879); }
    if app == "医院上网助手" || app == "com.helian.wifi" { return Some(1003880); }
    if app == "润钱包" || app == "com.crbank.vas" { return Some(1003881); }
    if app == "爱钱进" || app == "com.iqianjin.client" { return Some(1003882); }
    if app == "钱大掌柜" || app == "com.cib.qdzg" { return Some(1003883); }
    if app == "招钱进宝APP" || app == "com.example.mposstandard" { return Some(1003884); }
    if app == "赚钱了" || app == "com.youchen.mh" { return Some(1003885); }
    if app == "招钱宝贝APP" || app == "com.fortunebill.dreampay" { return Some(1003886); }
    if app == "钱儿频道" || app == "fm.qian.michael" { return Some(1003887); }
    if app == "兼职日日赚钱" || app == "com.taotao.jzrrz" { return Some(1003888); }
    if app == "赚钱日记" || app == "ctp.ncb.uykj" { return Some(1003889); }
    if app == "钱迹" || app == "com.mutangtech.qianji" { return Some(1003890); }
    if app == "易钱包" || app == "com.yeepay.mpos.money" { return Some(1003891); }
    if app == "壹钱包" || app == "com.paic.zhifu.wallet.activity" { return Some(1003892); }
    if app == "KOO钱包" || app == "com.koo.koou" { return Some(1003893); }
    if app == "富国富钱包" || app == "com.fullgoal.android" { return Some(1003894); }
    if app == "鲸钱包" || app == "com.avictc.jingqbnative" { return Some(1003895); }
    if app == "沃钱包" || app == "com.unicom.wopay" { return Some(1003896); }
    if app == "羊毛省钱" || app == "com.meiyou.sheep" { return Some(1003897); }
    if app == "钱盾" || app == "com.ali.money.shield" { return Some(1003898); }
    if app == "豆豆钱" || app == "com.vcredit.ddcash" { return Some(1003899); }
    if app == "e钱包" || app == "com.efund.jqb" { return Some(1003900); }
    if app == "e钱庄" || app == "cn.com.csbank" { return Some(1003901); }
    if app == "小花钱包" || app == "com.xhqb.app" { return Some(1003902); }
    if app == "收钱吧" || app == "com.wosai.cashbar" { return Some(1003903); }
    if app == "斗鱼" || app == "air.tv.douyu.android" { return Some(1003904); }
    if app == "美柚孕期" || app == "com.meiyou.yunqi" { return Some(1003905); }
    if app == "好大夫医生版" || app == "com.haodf.android.doctor" { return Some(1003906); }
    if app == "微医" || app == "com.greenline.guahao" { return Some(1003907); }
    if app == "新氧医美" || app == "com.youxiang.soyoungapp" { return Some(1003908); }
    if app == "健客医生" || app == "com.jianke.doctor" { return Some(1003909); }
    if app == "春雨医生" || app == "me.chunyu.ChunyuDoctor" { return Some(1003910); }
    if app == "美柚宝宝记" || app == "com.meiyou.seeyoubaby" { return Some(1003911); }
    if app == "美柚" || app == "com.lingan.seeyou" { return Some(1003912); }
    if app == "百度拇指医生" || app == "com.baidu.doctor.doctoranswer" { return Some(1003913); }
    if app == "丁香医生" || app == "cn.dxy.android.aspirin" { return Some(1003914); }
    if app == "浙二好医生" || app == "com.gj.patient" { return Some(1003915); }
    if app == "贵州社保" || app == "com.yinhai.rst" { return Some(1003916); }
    if app == "1药网" || app == "com.yiwang" { return Some(1003917); }
    if app == "大家中医" || app == "com.dajiazhongyi.dajia" { return Some(1003918); }
    if app == "北京协和医院" || app == "com.hundsun.qy.hospitalcloud.bj.xhhosp.hsyy" { return Some(1003919); }
    if app == "上海中山医院" || app == "com.ihygeia.shzs" { return Some(1003920); }
    if app == "掌上阜外医院" || app == "com.fuwaihospital.fwapp" { return Some(1003921); }
    if app == "南京鼓楼医院" || app == "com.focustech.mmgl" { return Some(1003922); }
    if app == "北京医院挂号网" || app == "com.dengtadoctor.bj114" { return Some(1003923); }
    if app == "广安门医院" || app == "com.ewell.guahao.beijingguanganmen" { return Some(1003924); }
    if app == "瑞金医院" || app == "com.rjh.rjhospital" { return Some(1003925); }
    if app == "掌上云医院" || app == "xikang.hygea.client" { return Some(1003926); }
    if app == "掌上苏北医院" || app == "com.bdtl.mobilehospital" { return Some(1003927); }
    if app == "北京大学人民医院" || app == "com.pkuhit.phmm" { return Some(1003928); }
    if app == "掌上宣武医院" || app == "com.founder.xwyypatientapp" { return Some(1003929); }
    if app == "康强医疗人才网" || app == "com.kq.kq36.com" { return Some(1003930); }
    if app == "天翼云游戏" || app == "cn.egame.terminal.cloud5g" { return Some(1003931); }
    if app == "左游游戏厅" || app == "com.zuoyou.center" { return Some(1003932); }
    if app == "DD373游戏交易平台" || app == "com.dd373.app" { return Some(1003933); }
    if app == "网易云游戏" || app == "com.netease.android.cloudgame" { return Some(1003934); }
    if app == "YOWA云游戏" || app == "com.huya.fig" { return Some(1003935); }
    if app == "悟饭游戏电玩辅助" || app == "com.wufan.dianwan" { return Some(1003936); }
    if app == "4399游戏盒" || app == "com.m4399.gamecenter" { return Some(1003937); }
    if app == "咪噜游戏" || app == "com.maiyou.milu" { return Some(1003938); }
    if app == "小米游戏安全插件" || app == "com.xiaomi.gamecenter.sdk.service" { return Some(1003939); }
    if app == "PlayStation游戏资讯" || app == "com.scee.psxandroid" { return Some(1003940); }
    if app == "360手机游戏" || app == "com.qihoo.gameunion" { return Some(1003941); }
    if app == "狐狸游戏" || app == "com.adfox.games" { return Some(1003942); }
    if app == "果盘游戏" || app == "com.flamingo.gpgame" { return Some(1003943); }
    if app == "啪啪游戏厅" || app == "com.join.android.app.mgsim" { return Some(1003944); }
    if app == "随乐游云游戏" || app == "com.stnts.iyoucloud" { return Some(1003945); }
    if app == "指趣游戏盒" || app == "com.beieryouxi.zqyxh" { return Some(1003946); }
    if app == "我的世界（全球第一沙盒游戏）" || app == "com.tencent.tmgp.wdsj666" { return Some(1003947); }
    if app == "游戏键盘[安智汉化]" || app == "com.locnet.gamekeyboard2" { return Some(1003948); }
    if app == "7881游戏交易" || app == "w2a.app7881.com" { return Some(1003949); }
    if app == "洋码头" || app == "com.ymatou.shop" { return Some(1003950); }
    if app == "微淼商学院" || app == "cn.weimiao.mobile" { return Some(1003951); }
    if app == "WiFi万能钥匙浏览器" || app == "com.link.browser.app" { return Some(1003952); }
    if app == "Share微博" || app == "com.hengye.share" { return Some(1003953); }
    if app == "智能诊断" || app == "com.huawei.hwdiagnosis" { return Some(1003954); }
    if app == "WiFi万能钥匙 海外版" || app == "com.halo.wifikey.wifilocating" { return Some(1003955); }
    if app == "华为生活服务" || app == "com.huawei.lives" { return Some(1003956); }
    if app == "微博动漫" || app == "com.weibo.comic" { return Some(1003957); }
    if app == "微博轻享版" || app == "com.weico.international" { return Some(1003958); }
    if app == "ARCore(测试版)" || app == "com.google.ar.core" { return Some(1003959); }
    if app == "微博极速版" || app == "com.sina.weibolite" { return Some(1003960); }
    if app == "微博超话" || app == "com.sina.wbsupergroup" { return Some(1003961); }
    if app == "新浪微博4G版" || app == "com.sina.weibog3" { return Some(1003962); }
    if app == "WiFi万能钥匙极速版" || app == "com.snda.lantern.wifilocating" { return Some(1003963); }
    if app == "小艺输入法" || app == "com.huawei.ohos.inputmethod" { return Some(1003964); }
    if app == "国家反诈中心" || app == "com.hicorenational.antifraud" { return Some(1003965); }
    if app == "五矿手机证券" || app == "com.wkscuiphone" { return Some(1003966); }
    if app == "汇炒股" || app == "cn.gzhzcj.hcg" { return Some(1003967); }
    if app == "SAC证券培训" || app == "cn.com.whaty.zqxh" { return Some(1003968); }
    if app == "证券从业考试随身学" || app == "com.onesoft.app.Tiiku.Duia.ZQSSX" { return Some(1003969); }
    if app == "陀螺财经" || app == "com.tuoluocaijing" { return Some(1003970); }
    if app == "金贝壳手机证券智慧版通用版" || app == "com.guohaizq.dzh" { return Some(1003971); }
    if app == "火星财经" || app == "com.linekong.mars24" { return Some(1003972); }
    if app == "五花牛股票" || app == "com.chenxikeji.cwp.whngp" { return Some(1003973); }
    if app == "股票通" || app == "com.upchina" { return Some(1003974); }
    if app == "上海证券报" || app == "com.cnstock.newsapp" { return Some(1003975); }
    if app == "乐居财经" || app == "com.eju.mobile.leju.finance" { return Some(1003976); }
    if app == "团贷网" || app == "com.junte" { return Some(1003977); }
    if app == "捷信金融商家" || app == "com.homecredit.hccn.hcpay.mapp" { return Some(1003978); }
    if app == "人人贷借款" || app == "com.ucredit.financial.android" { return Some(1003979); }
    if app == "房贷计算器LPR" || app == "com.calculatorm.lprh" { return Some(1003980); }
    if app == "金汇金融" || app == "com.rxhui.pay" { return Some(1003981); }
    if app == "及贷" || app == "com.pp.TimelyLoan" { return Some(1003982); }
    if app == "大地影院" || app == "com.ddcinemaapp" { return Some(1003983); }
    if app == "完美播放720PRMVB播放…" || app == "android.rk.RockVideoPlayer" { return Some(1003984); }
    if app == "Amazfit手表" || app == "com.huami.watch.hmwatchmanager" { return Some(1003985); }
    if app == "小当严选" || app == "com.danghuan.xiaodangyanxuan" { return Some(1003986); }
    if app == "Wear OS by Google 智能手表" || app == "com.google.android.wearable.app.cn" { return Some(1003987); }
    if app == "米兔手表" || app == "com.imibaby.client" { return Some(1003988); }
    if app == "掌证宝" || app == "dongzheng.szkingdom.android.phone" { return Some(1003989); }
    if app == "树莓阅读" || app == "com.sshumeiydu" { return Some(1003990); }
    if app == "人教英语点读软件" || app == "com.xminc.bookreader" { return Some(1003991); }
    if app == "典读" || app == "top.cloudfun.read" { return Some(1003992); }
    if app == "七读" || app == "com.dj.sevenRead" { return Some(1003993); }
    if app == "精读圣经" || app == "com.cz.bible2" { return Some(1003994); }
    if app == "点读通" || app == "DDT.QQ78551393" { return Some(1003995); }
    if app == "三联中读" || app == "com.sanlian.zhongdu" { return Some(1003996); }
    if app == "为你诵读" || app == "com.ss.readpoem" { return Some(1003997); }
    if app == "米读极速版" || app == "com.lechuan.mdwz" { return Some(1003998); }
    if app == "攀登悦读" || app == "com.wutongtech.climbingreading" { return Some(1003999); }
    if app == "文字朗读神器" || app == "com.wyfc.txtreader" { return Some(1004000); }
    if app == "樊登小读者" || app == "com.xfanread.xfanread" { return Some(1004001); }
    if app == "迅读PDF" || app == "com.xundupdf.reader" { return Some(1004002); }
    if app == "有道乐读" || app == "com.youdao.yread" { return Some(1004003); }
    if app == "岛读" || app == "io.moreless.islanding" { return Some(1004004); }
    if app == "读写客" || app == "com.doxent.mobile.doxent_app" { return Some(1004005); }
    if app == "伴鱼自然拼读" || app == "com.duwo.phonics" { return Some(1004006); }
    if app == "TED英语演讲" || app == "com.iyuba.TEDVideo" { return Some(1004007); }
    if app == "人教口语" || app == "com.gumi.spokenenglish" { return Some(1004008); }
    if app == "佳音英语" || app == "com.joy.joytalk" { return Some(1004009); }
    if app == "口语易" || app == "com.kouyuyi.kyystuapp" { return Some(1004010); }
    if app == "大象英语" || app == "com.lansi.reading" { return Some(1004011); }
    if app == "FiF口语训练学生版" || app == "com.fifedu.tsdx" { return Some(1004012); }
    if app == "趣听英语绘本" || app == "com.lewanabc.bookfun" { return Some(1004013); }
    if app == "粉笔四六级" || app == "com.fenbi.android.yingyu" { return Some(1004014); }
    if app == "流利说少儿英语" || app == "com.liulishuo.sprout" { return Some(1004015); }
    if app == "瓜瓜龙英语" || app == "com.eykid.android.ey" { return Some(1004016); }
    if app == "简背单词" || app == "com.maimemo.momolist.android" { return Some(1004017); }
    if app == "英语魔方秀" || app == "com.memory.me" { return Some(1004018); }
    if app == "开言英语" || app == "com.openlanguage.kaiyan" { return Some(1004019); }
    if app == "海豚儿童英语" || app == "com.dolphinmedia.english" { return Some(1004020); }
    if app == "英语帮" || app == "com.pldx.engang" { return Some(1004021); }
    if app == "E英语宝" || app == "com.dinoenglish.yyb" { return Some(1004022); }
    if app == "英语语法精讲" || app == "com.qilesoft.en.grammar" { return Some(1004023); }
    if app == "大卫熊英语" || app == "com.seewo.picbook.pro" { return Some(1004024); }
    if app == "扇贝听力口语" || app == "com.shanbay.listen" { return Some(1004025); }
    if app == "扇贝单词英语版" || app == "com.shanbay.sentence" { return Some(1004026); }
    if app == "扇贝单词" || app == "com.shanbay.words" { return Some(1004027); }
    if app == "傻瓜英语" || app == "com.sprite.foreigners" { return Some(1004028); }
    if app == "咸蛋口语" || app == "com.chaoui.xd" { return Some(1004029); }
    if app == "芝士派英语" || app == "com.chaoui.cheesepie" { return Some(1004030); }
    if app == "51Talk青少儿英语" || app == "com.talk51.kid" { return Some(1004031); }
    if app == "VIPKID英语" || app == "com.vipkid.app" { return Some(1004032); }
    if app == "一点英语" || app == "com.wumii.android.athena" { return Some(1004033); }
    if app == "乐词新东方背单词" || app == "com.xdf.recite" { return Some(1004034); }
    if app == "词根单词" || app == "com.xfs.rootwords" { return Some(1004035); }
    if app == "有道背单词" || app == "com.youdao.reciteword" { return Some(1004036); }
    if app == "伴鱼少儿英语" || app == "cn.xckj.talk_junior" { return Some(1004037); }
    if app == "知米背单词" || app == "cn.edu.zjicm.wordsnet_d" { return Some(1004038); }
    if app == "朗文当代高级英语词典" || app == "cn.dictcn.android.digitize.wys_lwddgjyycd_8027" { return Some(1004039); }
    if app == "轻听英语" || app == "cn.com.langeasy.EasyListen" { return Some(1004040); }
    if app == "蜜果好孕" || app == "com.cdonyc.menstruation" { return Some(1004041); }
    if app == "疯狂造人备孕怀孕" || app == "com.bozhong.crazy" { return Some(1004042); }
    if app == "宝宝树孕育" || app == "com.babytree.apps.pregnancy" { return Some(1004043); }
    if app == "妈妈网孕育" || app == "cn.mama.pregnant" { return Some(1004044); }
    if app == "大卫优孕" || app == "com.runbio.ovulation.app" { return Some(1004045); }
    if app == "烘焙成本计算器" || app == "com.qiuguqiugu.hbcbjsq" { return Some(1004046); }
    if app == "e烘焙" || app == "com.miniice.ehongbei" { return Some(1004047); }
    if app == "手工烘焙坊" || app == "com.hudee.mama4efeabca5d9b8086872549ca" { return Some(1004048); }
    if app == "生日蛋糕派对 – 烘焙、装饰、设计生日蛋糕！" || app == "net.makerlabs.birthday_cake_party" { return Some(1004049); }
    if app == "我的小镇：烘焙坊" || app == "mytown.bakery" { return Some(1004050); }
    if app == "烘焙有Fun" || app == "com.idelan.midea.oven.app" { return Some(1004051); }
    if app == "小说家模拟2" || app == "cs.xsjmn2.com" { return Some(1004052); }
    if app == "链家" || app == "com.homelink.android" { return Some(1004053); }
    if app == "烘焙帮" || app == "com.hongbeibang.app" { return Some(1004054); }
    if app == "七喵小说阅读器" || app == "com.zhnovel.sevencats" { return Some(1004055); }
    if app == "追更小说" || app == "com.zhaoxitech.cbook" { return Some(1004056); }
    if app == "梦想烘焙厨房" || app == "com.yunbu.cupcake.uc" { return Some(1004057); }
    if app == "饼干烘焙" || app == "com.jimmy.cookie" { return Some(1004058); }
    if app == "烘焙课堂" || app == "com.hpyshark.baking" { return Some(1004059); }
    if app == "abc小说" || app == "com.abcxs.xiaoshuo" { return Some(1004060); }
    if app == "我的烘焙小屋 - 蛋糕制作餐厅烹饪游戏" || app == "com.joyjourney.CakeBakeShop" { return Some(1004061); }
    if app == "我爱烘焙" || app == "com.douguo.baking" { return Some(1004062); }
    if app == "烘焙达人" || app == "com.wumii.android.USER.app_2qBDDay" { return Some(1004063); }
    if app == "烘焙先锋" || app == "com.wta.NewCloudApp.jiuwei101949" { return Some(1004064); }
    if app == "510房产网" || app == "com.jy510.house" { return Some(1004065); }
    if app == "豆果烘焙坊助手" || app == "com.aonvk10099202" { return Some(1004066); }
    if app == "美味蛋糕的烘焙时刻" || app == "air.com.candyoyo.CupcakeBakeTime" { return Some(1004067); }
    if app == "彩虹甜品烘焙屋 – 甜點天堂" || app == "com.kidsfoodinc.android_rainbowdesserts" { return Some(1004068); }
    if app == "快乐烘焙网" || app == "com.klhpw.appcan.app11420784" { return Some(1004069); }
    if app == "草莓甜心烘焙店 (Strawberry Shortcake)" || app == "com.budgestudios.StrawberryShortcakeBakeShare" { return Some(1004070); }
    if app == "新手烘焙入门" || app == "com.cbtech.cbapp177" { return Some(1004071); }
    if app == "烘焙食品" || app == "com.changyang319.baking" { return Some(1004072); }
    if app == "房天下" || app == "com.soufun.app" { return Some(1004073); }
    if app == "i烘焙" || app == "com.limitstudio.nova" { return Some(1004074); }
    if app == "微微免费小说" || app == "com.shuqi.cont2" { return Some(1004075); }
    if app == "书旗小说云版" || app == "com.shuqi.aliyun" { return Some(1004076); }
    if app == "房产超市" || app == "com.fccs.app" { return Some(1004077); }
    if app == "一块烘焙" || app == "com.damai.together" { return Some(1004078); }
    if app == "汽车之家二手车" || app == "com.che168.usedcar" { return Some(1004079); }
    if app == "汽车之家论坛" || app == "com.autohome.autoclub" { return Some(1004080); }
    if app == "驾校宝典" || app == "com.mili.drivingtest" { return Some(1004081); }
    if app == "驾考通驾照考试" || app == "com.ggeye.jiakao.api" { return Some(1004082); }
    if app == "驾照一点通" || app == "com.johong.jiazhaobaodian" { return Some(1004083); }
    if app == "车轮驾考通" || app == "cn.eclicks.drivingtest" { return Some(1004084); }
    if app == "易车行" || app == "com.classfish.obd" { return Some(1004085); }
    if app == "易车二手车" || app == "com.yiche.usedcar" { return Some(1004086); }
    if app == "驾考精灵" || app == "com.iningke.jiakaojl" { return Some(1004087); }
    if app == "驾校一点通" || app == "com.jxedt" { return Some(1004088); }
    if app == "元贝驾考" || app == "com.runbey.ybjk" { return Some(1004089); }
    if app == "UC浏览器冲浪版" || app == "com.UCMobile.cmcc" { return Some(1004090); }
    if app == "个人所得税" || app == "cn.gov.tax.its" { return Some(1004091); }
    if app == "中国移动" || app == "com.greenpoint.android.mc10086.activity" { return Some(1004092); }
    if app == "邮储银行" || app == "com.yitong.mbank.psbc" { return Some(1004093); }
    if app == "12306官方版" || app == "com.MobileTicket" { return Some(1004094); }
    if app == "工银融e联" || app == "com.icbc.im" { return Some(1004095); }
    if app == "QQ" || app == "com.tencent.mobileqq" { return Some(1004096); }
    if app == "系统更新" || app == "com.huawei.android.hwouc" { return Some(1004097); }
    if app == "翼支付" || app == "com.chinatelecom.bestpayclient" { return Some(1004098); }
    if app == "WPS Office" || app == "cn.wps.moffice_eng" { return Some(1004099); }
    if app == "FitTime" || app == "com.rjfittime.app" { return Some(1004100); }
    if app == "咪咕善跑" || app == "com.imohoo.shanpao" { return Some(1004101); }
    if app == "乐刻运动" || app == "com.leoao.fitness" { return Some(1004102); }
    if app == "乐动力跑步" || app == "cn.ledongli.runner" { return Some(1004103); }
    if app == "乐动力" || app == "cn.ledongli.ldl" { return Some(1004104); }
    if app == "Fit健身" || app == "com.sportq.fit" { return Some(1004105); }
    if app == "找乐运动计步器" || app == "com.lipian.gcwds" { return Some(1004106); }
    if app == "每日瑜伽" || app == "com.dailyyoga.cn" { return Some(1004107); }
    if app == "即刻运动" || app == "com.fittimellc.fittime" { return Some(1004108); }
    if app == "小米运动" || app == "com.xiaomi.hm.health" { return Some(1004109); }
    if app == "悦动圈" || app == "com.yuedong.sport" { return Some(1004110); }
    if app == "健康运动计步器" || app == "com.yundong.jibuqid" { return Some(1004111); }
    if app == "咕咚" || app == "com.codoon.gps" { return Some(1004112); }
    if app == "LinkedIn领英" || app == "com.linkedin.android" { return Some(1004113); }
    if app == "马蜂窝" || app == "com.mfw.roadbook" { return Some(1004114); }
    if app == "去哪儿旅行" || app == "com.Qunar" { return Some(1004115); }
    if app == "Wake瑜伽" || app == "com.wakeyoga.wakeyoga" { return Some(1004116); }
    if app == "Change健身" || app == "com.qianji.change" { return Some(1004117); }
    if app == "运动健身速成fit" || app == "com.ruofeng.sport.quick" { return Some(1004118); }
    if app == "健身减肥" || app == "com.jsjf.jsjftry" { return Some(1004119); }
    if app == "Relax Melodies: 睡眠与瑜伽" || app == "ipnossoft.rma.free" { return Some(1004120); }
    if app == "YogaEasy瑜伽" || app == "com.defshare.yogaeasy" { return Some(1004121); }
    if app == "赛普健身" || app == "saipujianshen.com" { return Some(1004122); }
    if app == "郑多燕健身操视频" || app == "com.waqu.android.vertical_zhenggym" { return Some(1004123); }
    if app == "每日瑜伽视频" || app == "com.waqu.android.vertical_yoga" { return Some(1004124); }
    if app == "Try健身减肥" || app == "com.sports.tryfits" { return Some(1004125); }
    if app == "金吉鸟健身" || app == "com.luckybird.sport" { return Some(1004126); }
    if app == "趣运动" || app == "com.gosport" { return Some(1004127); }
    if app == "我家瑜伽" || app == "cn.org.rar.iwantyoga" { return Some(1004128); }
    if app == "去哪儿攻略" || app == "com.qunar.travelplan" { return Some(1004129); }
    if app == "携程商旅" || app == "com.ctrip.ct" { return Some(1004130); }
    if app == "携程V-Booking" || app == "ctrip.vbooking.link" { return Some(1004131); }
    if app == "携程攻略" || app == "com.android.ctrip.gs" { return Some(1004132); }
    if app == "周末去哪儿" || app == "com.xisue.zhoumo" { return Some(1004133); }
    if app == "艺龙旅行" || app == "com.dp.android.elong" { return Some(1004134); }
    if app == "驴妈妈旅游" || app == "com.gift.android" { return Some(1004135); }
    if app == "途牛旅游" || app == "com.tuniu.app.ui" { return Some(1004136); }
    if app == "爱奇艺TV" || app == "com.qiyi.tv" { return Some(1004137); }
    if app == "爱奇艺票务" || app == "com.qiyi.android.ticket" { return Some(1004138); }
    if app == "爱奇艺泡泡" || app == "com.iqiyi.paopao" { return Some(1004139); }
    if app == "爱奇艺播播机" || app == "com.qiyi.game.live" { return Some(1004140); }
    if app == "爱奇艺漫画" || app == "com.iqiyi.comic" { return Some(1004141); }
    if app == "开卷小说" || app == "com.kingreader.framework" { return Some(1004142); }
    if app == "追言小说" || app == "com.wairead.book" { return Some(1004143); }
    if app == "全本免费言情小说" || app == "cn.xingread.free" { return Some(1004144); }
    if app == "有声小说" || app == "com.mht.mkl.voice" { return Some(1004145); }
    if app == "和讯股票" || app == "com.hexun.mobile" { return Some(1004146); }
    if app == "淘牛邦炒股票" || app == "cn.uniwa.uniwa" { return Some(1004147); }
    if app == "彩贝股票" || app == "net.icaibei.live" { return Some(1004148); }
    if app == "仙人掌股票" || app == "com.icaikee.xrzgp" { return Some(1004149); }
    if app == "财经股票头条" || app == "com.eastmoney.android.tokyo" { return Some(1004150); }
    if app == "快涨股票" || app == "cn.sogukj.stockalert" { return Some(1004151); }
    if app == "借钱能手贷款" || app == "com.lightpalm.daidai" { return Some(1004152); }
    if app == "好易借" || app == "com.haoyidai" { return Some(1004153); }
    if app == "秒借" || app == "com.ddshenbian.miaojie" { return Some(1004154); }
    if app == "闪电借款" || app == "cn.com.weshare.jiekuan" { return Some(1004155); }
    if app == "花无尽借款" || app == "com.qianhaishuliang.shop" { return Some(1004156); }
    if app == "悠融借贷" || app == "com.shoushan.zhenxin" { return Some(1004157); }
    if app == "借啊" || app == "com.hrtx.jiea" { return Some(1004158); }
    if app == "易借款" || app == "com.mstaz.app.toolset" { return Some(1004159); }
    if app == "钱秒借" || app == "com.qianchang.microfinance" { return Some(1004160); }
    if app == "无忧借条" || app == "com.yxbao.faith" { return Some(1004161); }
    if app == "白白贷款" || app == "com.iqueqian.bbdk" { return Some(1004162); }
    if app == "帮你借" || app == "com.app.bangnijie" { return Some(1004163); }
    if app == "简借贷款" || app == "com.express.wallet.walletexpress" { return Some(1004164); }
    if app == "借钱用" || app == "com.znsb.lendmoneyso" { return Some(1004165); }
    if app == "借得快" || app == "com.example.jdk" { return Some(1004166); }
    if app == "借乐花" || app == "com.vcredit.jlh_app" { return Some(1004167); }
    if app == "分期借钱贷款" || app == "com.xncredit.fqd" { return Some(1004168); }
    if app == "轻松借" || app == "com.sinaif.easy" { return Some(1004169); }
    if app == "有借" || app == "com.iask.finance" { return Some(1004170); }
    if app == "极贷管家贷款借钱" || app == "com.thering.loanmarket" { return Some(1004171); }
    if app == "急借通" || app == "com.zyjr.emergencylending" { return Some(1004172); }
    if app == "借点钱贷款" || app == "com.wdzj.borrowmoney" { return Some(1004173); }
    if app == "快速借款" || app == "com.ucredit.paydayloan" { return Some(1004174); }
    if app == "借钱花吧" || app == "com.rybring.huabei" { return Some(1004175); }
    if app == "简单借款" || app == "com.ufenqi.bajieloan" { return Some(1004176); }
    if app == "今借到" || app == "com.renrenxin.jjd" { return Some(1004177); }
    if app == "还借钱" || app == "com.htouhui.pdl" { return Some(1004178); }
    if app == "雷速体育" || app == "com.leisu.sports" { return Some(1004179); }
    if app == "足球财富" || app == "com.rrc.footballwealth" { return Some(1004180); }
    if app == "7M即时比分" || app == "com.sevenmmobile" { return Some(1004181); }
    if app == "疯狂红单" || app == "com.vodone.know" { return Some(1004182); }
    if app == "新新贷金融" || app == "com.xinxindai.fiance" { return Some(1004183); }
    if app == "好车贷-金融投资工具" || app == "com.yxr.haochedai" { return Some(1004184); }
    if app == "马上分期" || app == "com.msxf.msg" { return Some(1004185); }
    if app == "奢分期" || app == "com.shefenqi.mall" { return Some(1004186); }
    if app == "网贷天眼" || app == "com.example.ltest" { return Some(1004187); }
    if app == "新浪金融" || app == "com.sina.licaishi" { return Some(1004188); }
    if app == "可溯金融" || app == "com.guoding.kesudai" { return Some(1004189); }
    if app == "金山金融" || app == "com.kingsoft.loan" { return Some(1004190); }
    if app == "速贷之家" || app == "com.yeer.sdzj" { return Some(1004191); }
    if app == "房贷计算" || app == "com.example.baidu.fangdaicalculator" { return Some(1004192); }
    if app == "神灯小贷" || app == "com.shrxc.app" { return Some(1004193); }
    if app == "凤凰金融" || app == "com.fengjr.mobile" { return Some(1004194); }
    if app == "现金贷" || app == "com.xinhe.cashloan" { return Some(1004195); }
    if app == "房贷提前还款计算器" || app == "com.huishuaka.fangdaihuankuan" { return Some(1004196); }
    if app == "蜡笔分期" || app == "com.bank9f.staging" { return Some(1004197); }
    if app == "人人贷财富" || app == "com.renrendai.finance" { return Some(1004198); }
    if app == "贷款宝借款" || app == "com.huijiekuan.daikuanbao" { return Some(1004199); }
    if app == "宜贷网" || app == "com.edai.p2p.app" { return Some(1004200); }
    if app == "摩尔金融" || app == "com.moer.moerfinance" { return Some(1004201); }
    if app == "零零期分期" || app == "com.llq.linglingqihd" { return Some(1004202); }
    if app == "白领贷" || app == "com.mxd.office" { return Some(1004203); }
    if app == "买单侠分期" || app == "im.fenqi.mall" { return Some(1004204); }
    if app == "喜鹊快贷" || app == "com.fengwd.android.mobile" { return Some(1004205); }
    if app == "和信贷财富" || app == "com.hexindai.hxd" { return Some(1004206); }
    if app == "分期花" || app == "com.fenqi.loan" { return Some(1004207); }
    if app == "百金贷" || app == "com.yisheng.baijindai" { return Some(1004208); }
    if app == "融易贷钱" || app == "com.app.rongyidai" { return Some(1004209); }
    if app == "易通贷" || app == "com.stateunion.p2p.etongdai" { return Some(1004210); }
    if app == "魔借" || app == "com.dianrong.speedloan1" { return Some(1004211); }
    if app == "51公积金借款" || app == "com.balance6game.loanapp" { return Some(1004212); }
    if app == "屌丝贷" || app == "com.loserbank.loserbank.loserbankproject" { return Some(1004213); }
    if app == "拍分期" || app == "com.ppdai.installment" { return Some(1004214); }
    if app == "卡牛瑞贷贷款" || app == "com.mymoney.sms.billmanager" { return Some(1004215); }
    if app == "金融工场" || app == "com.eten.myriches" { return Some(1004216); }
    if app == "创客金融" || app == "com.ckjr.context" { return Some(1004217); }
    if app == "尊嘉金融" || app == "com.juniorchina.jcstock" { return Some(1004218); }
    if app == "小米贷款极速版" || app == "com.xiaomi.loanx" { return Some(1004219); }
    if app == "中银消费钱包" || app == "com.boccfc.wallet" { return Some(1004220); }
    if app == "信用管家借钱" || app == "com.greate.myapplication" { return Some(1004221); }
    if app == "一点分期" || app == "com.sogou.installmentloan" { return Some(1004222); }
    if app == "牛呗借钱借款" || app == "com.niuwa.niubei.android" { return Some(1004223); }
    if app == "新浪卡贷" || app == "com.sinaif.credit" { return Some(1004224); }
    if app == "维信闪贷" || app == "com.vcredit.ddflower" { return Some(1004225); }
    if app == "薄荷好借" || app == "com.mintq.bhfq" { return Some(1004226); }
    if app == "微盟贷款王" || app == "com.weimob.loanking" { return Some(1004227); }
    if app == "贷款123" || app == "com.daikuan123.android" { return Some(1004228); }
    if app == "天神贷" || app == "com.tianshen.cash" { return Some(1004229); }
    if app == "91社保查询贷款" || app == "com.xncredit.ssd" { return Some(1004230); }
    if app == "小雨点闪贷" || app == "com.xyd.raincredit" { return Some(1004231); }
    if app == "信贷助手" || app == "com.rong360.xindaizhushou" { return Some(1004232); }
    if app == "分期族" || app == "com.fqh.loan" { return Some(1004233); }
    if app == "优分期" || app == "com.ufenqi.app" { return Some(1004234); }
    if app == "富勤金融" || app == "com.fqhy.cfb" { return Some(1004235); }
    if app == "你我嘉选" || app == "com.chinaideal.bkclient.tabmain" { return Some(1004236); }
    if app == "中安信业贷款" || app == "cn.zac.esd" { return Some(1004237); }
    if app == "拍拍贷" || app == "com.ppdai.lender" { return Some(1004238); }
    if app == "麦子借款" || app == "com.maizijf.finance" { return Some(1004239); }
    if app == "微贷网" || app == "com.renrun.aphone.app" { return Some(1004240); }
    if app == "泰然金融" || app == "com.tairanchina.taiheapp" { return Some(1004241); }
    if app == "亲亲小贷借款" || app == "com.android.qqxd.loan" { return Some(1004242); }
    if app == "大小贷" || app == "com.dxd.shengdxd" { return Some(1004243); }
    if app == "贷你嗨" || app == "com.loanhigh.cash" { return Some(1004244); }
    if app == "笑脸金融" || app == "com.ghph.smile" { return Some(1004245); }
    if app == "花借" || app == "com.yqh.yaoqianhua" { return Some(1004246); }
    if app == "全能借款" || app == "com.xiaoV.VWallet" { return Some(1004247); }
    if app == "金融苑" || app == "xybank.com.rainbowcredit" { return Some(1004248); }
    if app == "期待合伙人-贷款借钱" || app == "com.dafy.sevend" { return Some(1004249); }
    if app == "民贷天下" || app == "com.mdcn.mdonline" { return Some(1004250); }
    if app == "宏亚金融" || app == "com.hyjr.hy_app" { return Some(1004251); }
    if app == "温商贷理财" || app == "com.wsloan" { return Some(1004252); }
    if app == "雪山贷" || app == "com.xueshanjinrong.snowmountainloan" { return Some(1004253); }
    if app == "网贷之家" || app == "com.wdzj.app" { return Some(1004254); }
    if app == "月光侠分期" || app == "com.yueguangxia.ygxknight" { return Some(1004255); }
    if app == "急现贷" || app == "com.dataseed.cashnow" { return Some(1004256); }
    if app == "立时贷" || app == "com.xiaogelicai.jianhuiyi" { return Some(1004257); }
    if app == "信贷圈" || app == "com.haodai.app" { return Some(1004258); }
    if app == "达飞云贷" || app == "com.hebei.dafy.c2c" { return Some(1004259); }
    if app == "闪贷" || app == "com.haodai.flashloan" { return Some(1004260); }
    if app == "一信贷" || app == "com.tongniu.tongniudai" { return Some(1004261); }
    if app == "包银消费" || app == "com.baoyin.credit" { return Some(1004262); }
    if app == "熊猫贷款" || app == "com.mg.pandaloan" { return Some(1004263); }
    if app == "合时代金融" || app == "com.heshidai.app" { return Some(1004264); }
    if app == "厚本金融" || app == "com.houbank.houbankfinance" { return Some(1004265); }
    if app == "合众e贷财富" || app == "com.hzcf" { return Some(1004266); }
    if app == "晋商贷" || app == "com.rd.zdbao.jinshangdai" { return Some(1004267); }
    if app == "功夫贷" || app == "com.treefinance.gongfudai" { return Some(1004268); }
    if app == "云科贷" || app == "com.chinacreditech.client" { return Some(1004269); }
    if app == "财小仙小额贷" || app == "com.caixiaoxian" { return Some(1004270); }
    if app == "大白汽车分期" || app == "com.qudian.android.dabaicar" { return Some(1004271); }
    if app == "闪电贷" || app == "cn.com.weshare.android.shandiandai" { return Some(1004272); }
    if app == "PPmoney出借" || app == "com.pmp.ppmoney" { return Some(1004273); }
    if app == "指尖贷" || app == "com.android.yzloan" { return Some(1004274); }
    if app == "米米贷" || app == "com.mimidai" { return Some(1004275); }
    if app == "正好贷" || app == "com.zhph.creditandloanappu" { return Some(1004276); }
    if app == "微贷" || app == "com.weidaiwang.creditloan" { return Some(1004277); }
    if app == "信富期贷" || app == "com.crfchina.agora" { return Some(1004278); }
    if app == "浙江金融资产交易中心" || app == "com.hundsun.zjfae" { return Some(1004279); }
    if app == "翼龙贷财富" || app == "com.eloancn.mclient" { return Some(1004280); }
    if app == "普融花" || app == "com.hengchang.client" { return Some(1004281); }
    if app == "轻易贷" || app == "com.autochina.p2p" { return Some(1004282); }
    if app == "你我金融" || app == "com.junte.onlinefinance" { return Some(1004283); }
    if app == "快贷" || app == "com.caimi.kuaidai" { return Some(1004284); }
    if app == "金融圈理财" || app == "com.formaxcopymaster.activitys" { return Some(1004285); }
    if app == "汽车金融大全" || app == "com.kaopujinfu.app" { return Some(1004286); }
    if app == "钱包金融" || app == "com.haodaibao.android" { return Some(1004287); }
    if app == "马上贷" || app == "com.msxf.cash" { return Some(1004288); }
    if app == "趣花分期" || app == "com.shcc.microcredit" { return Some(1004289); }
    if app == "Pure浏览器" || app == "com.pure.lite.browser" { return Some(1004290); }
    if app == "鑫格理财" || app == "com.puyue.www.xinge" { return Some(1004291); }
    if app == "爱学班班家长端" || app == "com.hht.bbparent" { return Some(1004292); }
    if app == "钱宝有票" || app == "com.qbao.ticket" { return Some(1004293); }
    if app == "清北网校" || app == "com.qbhsnetschool" { return Some(1004294); }
    if app == "青岛新闻" || app == "com.qdnews.qd" { return Some(1004295); }
    if app == "特来电" || app == "com.qdtevc.teld.app" { return Some(1004296); }
    if app == "知己交友" || app == "air.zhiji.app" { return Some(1004297); }
    if app == "家长学校" || app == "com.qiandl.android.nops" { return Some(1004298); }
    if app == "华为网盘" || app == "com.huawei.dbank.v7" { return Some(1004299); }
    if app == "滴滴出行极简版" || app == "com.didi.mini.passenger" { return Some(1004300); }
    if app == "钱金金贷款" || app == "com.qianjinjin.android" { return Some(1004301); }
    if app == "巧房助手" || app == "com.qiaofang.assistant" { return Some(1004302); }
    if app == "出国翻译官" || app == "com.qicai.translate" { return Some(1004303); }
    if app == "华为打印机" || app == "com.huawei.cv80.printer_huawei" { return Some(1004304); }
    if app == "U钱包" || app == "com.dianrong.uloan" { return Some(1004305); }
    if app == "360手机助手" || app == "com.qihoo.appstore" { return Some(1004306); }
    if app == "360浏览器" || app == "com.qihoo.browser" { return Some(1004307); }
    if app == "360清理大师" || app == "com.qihoo.cleandroid_cn" { return Some(1004308); }
    if app == "360极速浏览器" || app == "com.qihoo.contents" { return Some(1004309); }
    if app == "360行车记录仪" || app == "com.qihoo.dr" { return Some(1004310); }
    if app == "360浏览器极速版" || app == "com.qihoo.expressbrowser" { return Some(1004311); }
    if app == "360免费WiFi" || app == "com.qihoo.freewifi" { return Some(1004312); }
    if app == "指间小说" || app == "com.qihoo.ftreade" { return Some(1004313); }
    if app == "热点资讯" || app == "com.martian.hotnews" { return Some(1004314); }
    if app == "大众点评" || app == "com.dianping.v1" { return Some(1004315); }
    if app == "大众点评极速版" || app == "com.dianping.lite" { return Some(1004316); }
    if app == "360信用生活" || app == "com.qihoo.miaojie" { return Some(1004317); }
    if app == "360安全浏览器HD Pad" || app == "com.qihoo.padbrowser" { return Some(1004318); }
    if app == "MarryU相亲交友" || app == "com.marryu" { return Some(1004319); }
    if app == "360手机卫士" || app == "com.qihoo360.mobilesafe" { return Some(1004320); }
    if app == "齐家" || app == "com.qijia.o2o" { return Some(1004321); }
    if app == "好司机" || app == "com.dheaven.mscapp.SIGFFCNFN" { return Some(1004322); }
    if app == "解放行司机版" || app == "com.mapbar.qingqi.driver" { return Some(1004323); }
    if app == "东风出行" || app == "com.dfxny.dfcxrenter.beta" { return Some(1004324); }
    if app == "英语口语8000句" || app == "com.qilesoft.en.eights" { return Some(1004325); }
    if app == "华为全景相机" || app == "com.huawei.cv60" { return Some(1004326); }
    if app == "嘉铭宝宝起名取名" || app == "com.qiming.babyname" { return Some(1004327); }
    if app == "掌上药店" || app == "com.manle.phone.android.yaodian" { return Some(1004328); }
    if app == "新宝骏车联" || app == "com.qinggan.app.arielapp" { return Some(1004329); }
    if app == "满口袋贷款" || app == "com.mankoudai88.p2p" { return Some(1004330); }
    if app == "微光" || app == "com.qingqi.dianbo" { return Some(1004331); }
    if app == "青桔" || app == "com.qingqikeji.blackhorse.passenger" { return Some(1004332); }
    if app == "轻轻1对1" || app == "com.qingqing.student" { return Some(1004333); }
    if app == "翻译狗" || app == "com.qingxun.translationdog" { return Some(1004334); }
    if app == "北京挂号网" || app == "com.dengtadoctor.bjyuyue" { return Some(1004335); }
    if app == "滴滴小巴司机" || app == "com.didichuxing.provider" { return Some(1004336); }
    if app == "奇热免费小说" || app == "com.qixiao.qrxs" { return Some(1004337); }
    if app == "万能WiFi" || app == "com.qixiao.wifiartifact" { return Some(1004338); }
    if app == "六爻排盘宝" || app == "com.example.mls.mdsliuyao" { return Some(1004339); }
    if app == "盘丝洞直播" || app == "com.psd.live" { return Some(1004340); }
    if app == "滴滴司机部落" || app == "com.didichuxing.rainbow" { return Some(1004341); }
    if app == "人人影音" || app == "com.makerx.rrkp" { return Some(1004342); }
    if app == "koobee浏览器" || app == "com.prize.browser" { return Some(1004343); }
    if app == "东电微校家长端" || app == "com.ddwx.family" { return Some(1004344); }
    if app == "叮当快药" || app == "com.ddsy.songyao" { return Some(1004345); }
    if app == "音乐剪切器" || app == "com.makeringtone.mp3" { return Some(1004346); }
    if app == "街机达人捕鱼" || app == "com.prgame5.fish2.online" { return Some(1004347); }
    if app == "八字排盘宝" || app == "com.example.mls.mdspaipan" { return Some(1004348); }
    if app == "爱用商城" || app == "com.qkkj.wallet" { return Some(1004349); }
    if app == "iSilo阅读工具" || app == "com.dcco.app.iSilo" { return Some(1004350); }
    if app == "恒易分期借" || app == "com.daye.sudai360" { return Some(1004351); }
    if app == "TXT小说阅读器" || app == "com.maiton.xsreader" { return Some(1004352); }
    if app == "千米红包" || app == "com.qmsh.hbq" { return Some(1004353); }
    if app == "音乐宝" || app == "com.qmth.music" { return Some(1004354); }
    if app == "大王直播" || app == "com.dawang.live" { return Some(1004355); }
    if app == "兔小贝拼音" || app == "com.qpx.pinying" { return Some(1004356); }
    if app == "家长100" || app == "com.davik.jiazhan100" { return Some(1004357); }
    if app == "无忧课堂" || app == "com.dasheng.b2s" { return Some(1004358); }
    if app == "交往吧婚恋交友" || app == "org.zywx.wbpalmstar.widgetone.uex11535432" { return Some(1004359); }
    if app == "欢乐斗地主" || app == "com.qqgame.hlddz" { return Some(1004360); }
    if app == "专车司机助手" || app == "com.didichuxing.supervise" { return Some(1004361); }
    if app == "钱时代理财" || app == "com.qsdjf.demo" { return Some(1004362); }
    if app == "乐贷款" || app == "com.maimob.loan" { return Some(1004363); }
    if app == "钱牛牛理财" || app == "com.qsq.qianshengqian" { return Some(1004364); }
    if app == "萌小明租车" || app == "com.maimi.meng" { return Some(1004365); }
    if app == "当当" || app == "com.dangdang.buy2" { return Some(1004366); }
    if app == "青果阅读" || app == "com.quduquxie" { return Some(1004367); }
    if app == "神州买买车" || app == "com.maimaiche.ucecapp" { return Some(1004368); }
    if app == "汽车保姆" || app == "com.digienginetek.rccsec" { return Some(1004369); }
    if app == "77分期" || app == "com.qufenqi.wallet" { return Some(1004370); }
    if app == "WIFI网速测试" || app == "com.quickbird.speedtest" { return Some(1004371); }
    if app == "Quizii" || app == "com.quizii" { return Some(1004372); }
    if app == "极速贷" || app == "com.example.pro_phone" { return Some(1004373); }
    if app == "贝多多理财记账" || app == "com.daiyoubang" { return Some(1004374); }
    if app == "魔力WiFi管家" || app == "com.magic.wifigj" { return Some(1004375); }
    if app == "去哪儿机票" || app == "com.qunar.flight" { return Some(1004376); }
    if app == "叮嗒出行" || app == "com.dingda.app" { return Some(1004377); }
    if app == "酷家乐设计师" || app == "com.qunhe.designer" { return Some(1004378); }
    if app == "设计家装修" || app == "com.qunhe.designhome" { return Some(1004379); }
    if app == "酷家乐装修业主版" || app == "com.qunhe.rendershow" { return Some(1004380); }
    if app == "梅赛德斯-奔驰导航" || app == "com.daimler.moba.kundenapp.android" { return Some(1004381); }
    if app == "叮叮易建" || app == "com.dingdingyijian.ddyj" { return Some(1004382); }
    if app == "葡萄浏览器" || app == "com.qwh.grapebrowser" { return Some(1004383); }
    if app == "超准星座运势" || app == "com.m3.yunshi" { return Some(1004384); }
    if app == "3D风水罗盘" || app == "com.qyz.fengshuicompass" { return Some(1004385); }
    if app == "钱庄理财" || app == "com.qz.qian" { return Some(1004386); }
    if app == "首汽租车" || app == "com.example.rentalcarapp" { return Some(1004387); }
    if app == "QQ空间" || app == "com.qzone" { return Some(1004388); }
    if app == "城市飞车（奥运喝彩版）" || app == "com.racergame.cityracing3d" { return Some(1004389); }
    if app == "够谱司机端" || app == "com.ly.domestic.driver" { return Some(1004390); }
    if app == "彩虹小说" || app == "com.rainbow.xiqidao" { return Some(1004391); }
    if app == "必看小说" || app == "com.lwby.breader" { return Some(1004392); }
    if app == "钱内助金服" || app == "com.rd.qnz" { return Some(1004393); }
    if app == "陆金所" || app == "com.lufax.android" { return Some(1004394); }
    if app == "PP基金理财" || app == "com.ppjijin.ppjijin" { return Some(1004395); }
    if app == "爱阅读HD" || app == "com.readingjoy.read.hd" { return Some(1004396); }
    if app == "瑞幸咖啡" || app == "com.lucky.luckyclient" { return Some(1004397); }
    if app == "爱尚浏览器" || app == "com.lu.browser" { return Some(1004398); }
    if app == "绿色童年-家长" || app == "com.lstn.protect2" { return Some(1004399); }
    if app == "一个浏览器" || app == "com.example.zzb.txweblibrary" { return Some(1004400); }
    if app == "DaDa英语" || app == "com.dadaabc.zhuozan.dadaabcstudent" { return Some(1004401); }
    if app == "懂表帝" || app == "com.cztec.watch" { return Some(1004402); }
    if app == "红星美凯龙" || app == "com.redstar.mainapp" { return Some(1004403); }
    if app == "无线网络密码:WiFi Password" || app == "com.czbix.xposed.wifipassword" { return Some(1004404); }
    if app == "人人车二手车" || app == "com.renrenche.carapp" { return Some(1004405); }
    if app == "人人车" || app == "com.renrenche.renrenche" { return Some(1004406); }
    if app == "团油企业司机" || app == "com.czb.fleet" { return Some(1004407); }
    if app == "瑞卡租车" || app == "com.reocar.reocar" { return Some(1004408); }
    if app == "元力町轻小说" || app == "com.resou.reader" { return Some(1004409); }
    if app == "文墨天机●紫微斗数" || app == "air.com.ziwei001.zwmobilenui" { return Some(1004410); }
    if app == "哒哒充电" || app == "com.czb.charge" { return Some(1004411); }
    if app == "Starfall 学着阅读" || app == "air.com.starfall.ltr" { return Some(1004412); }
    if app == "幸福里" || app == "com.f100.android" { return Some(1004413); }
    if app == "嗨住租房" || app == "com.loulifang.house" { return Some(1004414); }
    if app == "每日星座运程" || app == "com.culiu.horoscope" { return Some(1004415); }
    if app == "汽车报价" || app == "com.cubic.choosecar" { return Some(1004416); }
    if app == "央视音乐" || app == "com.ctvit.cctvmusic" { return Some(1004417); }
    if app == "携程租车" || app == "com.ctrip.izuche" { return Some(1004418); }
    if app == "拍拍钱庄" || app == "com.ppdai.loan.caocao" { return Some(1004419); }
    if app == "富力好房" || app == "com.rfchina.app.wqhouse.client" { return Some(1004420); }
    if app == "龙珠直播" || app == "com.longzhu.tga" { return Some(1004421); }
    if app == "借钱花" || app == "com.ppdai.loan.buy" { return Some(1004422); }
    if app == "星座游戏大全" || app == "com.fairytale.fortune" { return Some(1004423); }
    if app == "时光韩剧" || app == "com.rn.hanju" { return Some(1004424); }
    if app == "WiFi连接管理器" || app == "com.roamingsoft.manager" { return Some(1004425); }
    if app == "4G浏览器" || app == "com.roboo.explorer" { return Some(1004426); }
    if app == "金庸听书" || app == "com.longrundmt.jinyong" { return Some(1004427); }
    if app == "康康买药（商户版）" || app == "com.rogrand.kkmy.merchants" { return Some(1004428); }
    if app == "BL小说" || app == "com.dingsheng.novel_bl" { return Some(1004429); }
    if app == "95美女秀" || app == "com.lokinfo.android.gamemarket.mmshow" { return Some(1004430); }
    if app == "携程通" || app == "com.ctrip.b.welfare" { return Some(1004431); }
    if app == "融360贷款" || app == "com.rong360.loans" { return Some(1004432); }
    if app == "电信实名制" || app == "com.ct.realname" { return Some(1004433); }
    if app == "旺财猫理财" || app == "com.rongfinance.wangcaicat" { return Some(1004434); }
    if app == "融金所理财" || app == "com.rongjinsuo.android" { return Some(1004435); }
    if app == "金储宝理财" || app == "com.rongxun.JingChuBao" { return Some(1004436); }
    if app == "晴天助理财" || app == "com.rongxun.QingTianZhu" { return Some(1004437); }
    if app == "招商基金" || app == "com.logansoft.zcbao" { return Some(1004438); }
    if app == "中国电信" || app == "com.ct.client" { return Some(1004439); }
    if app == "儿童游戏弹钢琴" || app == "com.cslm.music" { return Some(1004440); }
    if app == "运花花" || app == "com.loanhome.yunhuahua" { return Some(1004441); }
    if app == "旗鱼浏览器" || app == "com.ruanmei.qiyubrowser" { return Some(1004442); }
    if app == "儿童教育游戏乐园" || app == "com.cslm.edugamepark" { return Some(1004443); }
    if app == "宝宝学数字加减法" || app == "com.ruibao.babyjiajian" { return Some(1004444); }
    if app == "宝宝幼儿园学知识" || app == "com.ruibao.babyyoueryuan" { return Some(1004445); }
    if app == "快乐宝宝学数字" || app == "com.ruibao.shuzigame" { return Some(1004446); }
    if app == "WiFi魔盒" || app == "com.ruijie.wifim" { return Some(1004447); }
    if app == "熊花花" || app == "com.loanhome.bearcost" { return Some(1004448); }
    if app == "电视直播" || app == "com.rumtel.pandatv" { return Some(1004449); }
    if app == "微课传奇" || app == "android.app.wkcq" { return Some(1004450); }
    if app == "newifi" || app == "com.diting.newifi.bridge" { return Some(1004451); }
    if app == "如祺出行" || app == "com.ruqi.travel" { return Some(1004452); }
    if app == "房车生活家" || app == "com.rv2go.rvlife" { return Some(1004453); }
    if app == "小学语文课堂" || app == "com.csliyu.primary" { return Some(1004454); }
    if app == "初中语文课堂" || app == "com.csliyu.junior" { return Some(1004455); }
    if app == "阳光快线家长" || app == "com.hjw.videoparent" { return Some(1004456); }
    if app == "华为WLAN" || app == "com.huawei.cloudwifi" { return Some(1004457); }
    if app == "微粒贷借钱" || app == "com.rybring.jie" { return Some(1004458); }
    if app == "小学英语课堂" || app == "com.csliyu.englishprimary" { return Some(1004459); }
    if app == "外快理财" || app == "com.ryw.waikuai" { return Some(1004460); }
    if app == "贷款大师借款" || app == "com.rzj.dkds" { return Some(1004461); }
    if app == "发现精彩" || app == "com.cs_credit_bank" { return Some(1004462); }
    if app == "易居房友经纪" || app == "com.cric.fangyou.agent" { return Some(1004463); }
    if app == "信而富" || app == "com.crfchina.market" { return Some(1004464); }
    if app == "宜人优选" || app == "com.creditwealth.client" { return Some(1004465); }
    if app == "上汽荣威" || app == "com.saicmotor.tocapp" { return Some(1004466); }
    if app == "赛学霸初中物理" || app == "com.saixueba.physics" { return Some(1004467); }
    if app == "捕鱼欢乐颂" || app == "com.saiyun.qpbylwb.uc" { return Some(1004468); }
    if app == "星火理财服务" || app == "com.creditease.xinghuoinvest" { return Some(1004469); }
    if app == "省呗" || app == "com.samoyed.credit" { return Some(1004470); }
    if app == "牛奶音乐" || app == "com.samsung.mdl.radio" { return Some(1004471); }
    if app == "三好网" || app == "com.sanhao.app" { return Some(1004472); }
    if app == "宜信财富" || app == "com.creditease.android" { return Some(1004473); }
    if app == "微当钱包" || app == "com.loan7" { return Some(1004474); }
    if app == "AR星座" || app == "com.fancyar.star" { return Some(1004475); }
    if app == "IS智慧平台" || app == "com.example.is" { return Some(1004476); }
    if app == "周公解梦软件" || app == "com.divination.dream1518" { return Some(1004477); }
    if app == "WiFi无线连" || app == "zte.com.wilink" { return Some(1004478); }
    if app == "辽宁移动" || app == "com.lncmcc.sjyyt" { return Some(1004479); }
    if app == "重庆移动" || app == "com.cqmc.client" { return Some(1004480); }
    if app == "美团团购Pad版_美食电影团购优惠" || app == "com.sankuai.meituanhd" { return Some(1004481); }
    if app == "立马理财" || app == "com.lmlc.android" { return Some(1004482); }
    if app == "10086" || app == "zz.dela.cmcc.traffic" { return Some(1004483); }
    if app == "行车卫士" || app == "com.cpsdna.xingcheweishi" { return Some(1004484); }
    if app == "充电圈" || app == "com.potevio.icharge" { return Some(1004485); }
    if app == "冲浪浏览器" || app == "com.cplatform.android.cmsurfclient" { return Some(1004486); }
    if app == "群策起名" || app == "com.sanshi_td.qiming" { return Some(1004487); }
    if app == "英汉词典" || app == "com.sbaike.client.yinghan.zidian.meizu" { return Some(1004488); }
    if app == "看房日记" || app == "com.fanggeek.agent" { return Some(1004489); }
    if app == "装修图库" || app == "android.decorate.gallery.jiajuol.com" { return Some(1004490); }
    if app == "全国旅游景点" || app == "com.scenic.ego.view" { return Some(1004491); }
    if app == "小鹿选房" || app == "com.fanggeek.shikamaru" { return Some(1004492); }
    if app == "家长网络学院" || app == "com.sd.parentsofnetwork" { return Some(1004493); }
    if app == "汽车在线" || app == "com.coomix.app.car" { return Some(1004494); }
    if app == "滴滴顺风车" || app == "com.sdu.didi.beatles" { return Some(1004495); }
    if app == "滴滴车主" || app == "com.sdu.didi.gsui" { return Some(1004496); }
    if app == "Weather" || app == "com.coolwind.weather" { return Some(1004497); }
    if app == "返利优惠券联盟" || app == "com.fanliyouhuiquanlmw" { return Some(1004498); }
    if app == "三星Galaxy S3相机增强" || app == "com.sec.android.app.camera" { return Some(1004499); }
    if app == "淘新闻" || app == "com.coohua.xinwenzhuan" { return Some(1004500); }
    if app == "三星应用商店" || app == "com.sec.android.app.samsungapps" { return Some(1004501); }
    if app == "三星浏览器" || app == "com.sec.android.app.sbrowser" { return Some(1004502); }
    if app == "中华老黄历" || app == "com.fanyue.laohuangli" { return Some(1004503); }
    if app == "三星推送服务" || app == "com.sec.spp.push" { return Some(1004504); }
    if app == "寺库奢侈品" || app == "com.secoo" { return Some(1004505); }
    if app == "单词锁屏" || app == "com.secretlisa.beidanci" { return Some(1004506); }
    if app == "掌通家园" || app == "com.seebaby" { return Some(1004507); }
    if app == "班级优化大师" || app == "com.seewo.easicare.pro" { return Some(1004508); }
    if app == "宝宝起名取名" || app == "predictor.namer" { return Some(1004509); }
    if app == "灵占天下算命占卜" || app == "predictor.tw.ui" { return Some(1004510); }
    if app == "小南充电" || app == "com.comtop.charge" { return Some(1004511); }
    if app == "灵占算命八字星座" || app == "predictor.ui" { return Some(1004512); }
    if app == "乐教乐学" || app == "com.lj.ljshell" { return Some(1004513); }
    if app == "就要玩捕鱼" || app == "com.lixxix.HappyFishFour" { return Some(1004514); }
    if app == "全民财富理财" || app == "com.fanyue.peoplewealth" { return Some(1004515); }
    if app == "自动浏览器" || app == "com.fax.zdllq" { return Some(1004516); }
    if app == "波克捕鱼-单机版" || app == "com.pokercity.bydrqp.uc" { return Some(1004517); }
    if app == "丰趣海淘" || app == "com.sfht.m" { return Some(1004518); }
    if app == "325棋牌捕鱼" || app == "com.poker325.game.uc" { return Some(1004519); }
    if app == "e充电" || app == "com.sgcc.evs.echarge" { return Some(1004520); }
    if app == "小学英语助手" || app == "com.liuyang.pephelp" { return Some(1004521); }
    if app == "捕鱼欢乐季" || app == "com.sgw.fishdaren.uc" { return Some(1004522); }
    if app == "欢乐捕鱼人" || app == "com.sgw.hlbyr.aligames" { return Some(1004523); }
    if app == "上海移动掌上营业厅" || app == "com.sh.cm.busihall" { return Some(1004524); }
    if app == "上海移动和你" || app == "com.sh.cm.shydhn" { return Some(1004525); }
    if app == "美女报告" || app == "com.sh.mww.beautyreport" { return Some(1004526); }
    if app == "麦当劳Pro" || app == "com.mcdonalds.gma.cn" { return Some(1004527); }
    if app == "零基础学英语" || app == "com.liuyang.adultzero" { return Some(1004528); }
    if app == "钱龙" || app == "qianlong.qlmobile" { return Some(1004529); }
    if app == "华为云电脑" || app == "com.huawei.cloud" { return Some(1004530); }
    if app == "华为穿戴" || app == "com.huawei.bone" { return Some(1004531); }
    if app == "小学英语三年级上" || app == "com.shandong.english.threegradeenup" { return Some(1004532); }
    if app == "稳赚宝理财" || app == "com.shangchao.wzb" { return Some(1004533); }
    if app == "WiFi防蹭网大师" || app == "com.fcwds.wifiprotect" { return Some(1004534); }
    if app == "贵州校讯通" || app == "qtone.xxt.gz" { return Some(1004535); }
    if app == "智能风水罗盘 (专业版)" || app == "com.compass.full" { return Some(1004536); }
    if app == "Shazam音乐神搜" || app == "com.shazam.android" { return Some(1004537); }
    if app == "菠萝街直播" || app == "com.shejiao.boluojie" { return Some(1004538); }
    if app == "一起秀直播" || app == "com.shejiao.yueyue" { return Some(1004539); }
    if app == "姓名测试大师" || app == "com.divination.name1518" { return Some(1004540); }
    if app == "老黄历万年历" || app == "com.shengliulaohuangli" { return Some(1004541); }
    if app == "光速贷" || app == "com.example.guangsudai" { return Some(1004542); }
    if app == "柚子相机" || app == "com.commsource.pomelo" { return Some(1004543); }
    if app == "少儿英语点读" || app == "com.shengxue.echild" { return Some(1004544); }
    if app == "懒人英语阅读" || app == "com.shengxue.lazyread" { return Some(1004545); }
    if app == "新概念英语100分" || app == "com.shengxue.xgnyybest" { return Some(1004546); }
    if app == "一起中学家长" || app == "com.shensz.parents" { return Some(1004547); }
    if app == "环球新军事" || app == "com.shenyuan.militarynews" { return Some(1004548); }
    if app == "乐贝通家长版" || app == "com.shenzhou.lbt_jz" { return Some(1004549); }
    if app == "居理新房" || app == "com.comjia.kanjiaestate" { return Some(1004550); }
    if app == "移动彩云" || app == "com.shinemo.qoffice.zjcc" { return Some(1004551); }
    if app == "爆米花视频" || app == "com.com.baomihuawang.androidclient" { return Some(1004552); }
    if app == "听书大全" || app == "com.shipook.reader.tsdq" { return Some(1004553); }
    if app == "OPPO相册隐藏照片恢复工具" || app == "com.coloros.photorestore" { return Some(1004554); }
    if app == "VIP陪练" || app == "com.pnlyy.pnlclass.pnlclass_student" { return Some(1004555); }
    if app == "音乐DJ混音器" || app == "com.djit.apps.edjing.expert" { return Some(1004556); }
    if app == "BKBY" || app == "com.shiyi.aliby.aligames" { return Some(1004557); }
    if app == "波克捕鱼（捕鱼达人千炮版）" || app == "com.shiyi.bkby" { return Some(1004558); }
    if app == "阅赞家长" || app == "com.codans.goodreadingparents" { return Some(1004559); }
    if app == "捕鱼达人4" || app == "com.cocos2d.fishingfun.uc" { return Some(1004560); }
    if app == "邻里WiFi密码" || app == "com.linli.llwifi" { return Some(1004561); }
    if app == "铃声多多" || app == "com.shoujiduoduo.ringtone" { return Some(1004562); }
    if app == "WiFi万能钥匙主人版" || app == "com.linksure.wifimaster" { return Some(1004563); }
    if app == "的士联盟司机端" || app == "com.shouyue.taxi.driver" { return Some(1004564); }
    if app == "秀色直播" || app == "com.showself.ui" { return Some(1004565); }
    if app == "WiFi万能钥匙女生版" || app == "com.linksure.girlkey" { return Some(1004566); }
    if app == "车e族" || app == "com.shuchuang.shihua" { return Some(1004567); }
    if app == "还呗" || app == "com.shuhekeji" { return Some(1004568); }
    if app == "中国新闻网" || app == "com.cns.mc.activity" { return Some(1004569); }
    if app == "即阅免费小说" || app == "com.shuman.jymfxs" { return Some(1004570); }
    if app == "华为教育中心" || app == "com.huawei.educenter" { return Some(1004571); }
    if app == "手机电影" || app == "com.cnlive.movie" { return Some(1004572); }
    if app == "追阅小说" || app == "com.shunmao.zymfxs" { return Some(1004573); }
    if app == "CAJ云阅读" || app == "com.cnki.android.cajviewercloud" { return Some(1004574); }
    if app == "DJKK音乐" || app == "com.djkk.music" { return Some(1004575); }
    if app == "豆丁免费小说" || app == "com.shuqi.cont1" { return Some(1004576); }
    if app == "免费小说电子书城" || app == "com.shuqi.contq1" { return Some(1004577); }
    if app == "共享钱庄" || app == "com.linkfin.lianzhidai" { return Some(1004578); }
    if app == "捕鱼多多" || app == "com.djl28.fish3d" { return Some(1004579); }
    if app == "批批网服装女装批发" || app == "com.pipipifa.pilaipiwang" { return Some(1004580); }
    if app == "气泡阅读" || app == "com.cmyd.xuetang" { return Some(1004581); }
    if app == "广西校讯通安卓客户端" || app == "com.linkage.mobile72.gx" { return Some(1004582); }
    if app == "爱看阅读" || app == "com.mdroid.read" { return Some(1004583); }
    if app == "小优视频编辑" || app == "com.lingyun.ydd" { return Some(1004584); }
    if app == "书山宝" || app == "com.shushan.shushanbao" { return Some(1004585); }
    if app == "wifi网络电话" || app == "com.feiin.wldh" { return Some(1004586); }
    if app == "洋钱罐理财" || app == "com.lingyue.YqgAndroid" { return Some(1004587); }
    if app == "清理大师" || app == "com.shyz.toutiao" { return Some(1004588); }
    if app == "四川移动和生活" || app == "com.sichuan.iwant" { return Some(1004589); }
    if app == "金华新闻" || app == "com.cmstop.jhrb" { return Some(1004590); }
    if app == "和宝贝家长端" || app == "com.cmcc.hbb.android.phone.parents" { return Some(1004591); }
    if app == "中国移动甘肃" || app == "com.cmcc.gs" { return Some(1004592); }
    if app == "深圳预约挂号" || app == "com.cmcc.aregister" { return Some(1004593); }
    if app == "新浪音乐" || app == "com.sina.music" { return Some(1004594); }
    if app == "新浪新闻" || app == "com.sina.news" { return Some(1004595); }
    if app == "新浪新闻极速版" || app == "com.sina.news.lite" { return Some(1004596); }
    if app == "绿洲" || app == "com.sina.oasis" { return Some(1004597); }
    if app == "对缘" || app == "com.lingxi.cupid" { return Some(1004598); }
    if app == "窝牛装修" || app == "com.lingduo.acorn" { return Some(1004599); }
    if app == "飞鱼小说阅读器" || app == "com.feiyu.fynovel" { return Some(1004600); }
    if app == "猿题库" || app == "com.fenbi.android.gaozhong" { return Some(1004601); }
    if app == "周易万年历" || app == "com.lin.wannianli" { return Some(1004602); }
    if app == "新浪贷款" || app == "com.sinaif.credit5" { return Some(1004603); }
    if app == "大王贷款" || app == "com.sinaif.suploan" { return Some(1004604); }
    if app == "新浪SHOW直播" || app == "com.sinashow.live" { return Some(1004605); }
    if app == "招商银行掌上生活" || app == "com.cmbchina.ccd.pluto.cmbActivity" { return Some(1004606); }
    if app == "钢琴教练" || app == "com.meet.pianolearn" { return Some(1004607); }
    if app == "读秒钱包" || app == "com.pintec.dumiao" { return Some(1004608); }
    if app == "Lily学生" || app == "com.lilyenglish.homework_student" { return Some(1004609); }
    if app == "WiFi助手" || app == "com.cloudsynch.wifihelper" { return Some(1004610); }
    if app == "猎豹安全大师" || app == "com.cleanmaster.security_cn" { return Some(1004611); }
    if app == "阳光钱袋" || app == "com.sinosig.ygqd" { return Some(1004612); }
    if app == "小猿口算" || app == "com.fenbi.android.leo" { return Some(1004613); }
    if app == "爱钱记小额贷款" || app == "com.like.loanmarket" { return Some(1004614); }
    if app == "猎豹清理大师" || app == "com.cleanmaster.mguard_cn" { return Some(1004615); }
    if app == "老司机汽车" || app == "com.feng.car" { return Some(1004616); }
    if app == "疯播直播" || app == "com.fengbo.live" { return Some(1004617); }
    if app == "风车理财" || app == "com.fengchelicai.fclc" { return Some(1004618); }
    if app == "肯德基(KFC)优惠券" || app == "com.life.sharekfc" { return Some(1004619); }
    if app == "猎鹰浏览器" || app == "com.lieying.browser" { return Some(1004620); }
    if app == "优易学车" || app == "com.sinotech.ln_student" { return Some(1004621); }
    if app == "中国移动广西" || app == "com.sinovatech.gxmobile.ui" { return Some(1004622); }
    if app == "联通手机营业厅" || app == "com.sinovatech.unicom.ui" { return Some(1004623); }
    if app == "宝宝学ABC" || app == "com.sinyee.babybus.abc" { return Some(1004624); }
    if app == "奇妙逻辑冒险" || app == "com.sinyee.babybus.adventure" { return Some(1004625); }
    if app == "宝宝机场" || app == "com.sinyee.babybus.airport" { return Some(1004626); }
    if app == "奇妙游乐园世界" || app == "com.sinyee.babybus.amusement" { return Some(1004627); }
    if app == "宝宝认蚂蚁" || app == "com.sinyee.babybus.ant" { return Some(1004628); }
    if app == "宝宝汽车城市" || app == "com.sinyee.babybus.automobilecity" { return Some(1004629); }
    if app == "宝宝小画板 - 宝宝巴士" || app == "com.sinyee.babybus.babydrawing.global" { return Some(1004630); }
    if app == "人教小学英语四上点读" || app == "com.lida.diandusishang" { return Some(1004631); }
    if app == "奇妙蛋糕店 - 宝宝巴士" || app == "com.sinyee.babybus.bakery" { return Some(1004632); }
    if app == "宝宝小船长" || app == "com.sinyee.babybus.boat" { return Some(1004633); }
    if app == "宝宝房屋设计师" || app == "com.sinyee.babybus.build" { return Some(1004634); }
    if app == "奇妙蛋糕店" || app == "com.sinyee.babybus.cake" { return Some(1004635); }
    if app == "糖果工厂" || app == "com.sinyee.babybus.candy" { return Some(1004636); }
    if app == "宝宝巴士儿歌" || app == "com.sinyee.babybus.chants" { return Some(1004637); }
    if app == "宝宝巴士儿歌HD" || app == "com.sinyee.babybus.chants.pad" { return Some(1004638); }
    if app == "奇妙咖啡餐厅" || app == "com.sinyee.babybus.coffee" { return Some(1004639); }
    if app == "宝宝音乐启蒙" || app == "com.sinyee.babybus.concert" { return Some(1004640); }
    if app == "宝宝音乐多多" || app == "com.sinyee.babybus.cookingpercussion" { return Some(1004641); }
    if app == "饼干消消乐 - 幼儿教育游戏 - 宝宝巴士" || app == "com.sinyee.babybus.cooky" { return Some(1004642); }
    if app == "奇妙宠物美妆店" || app == "com.sinyee.babybus.cosmetology" { return Some(1004643); }
    if app == "宝宝日常安全" || app == "com.sinyee.babybus.dailysafety" { return Some(1004644); }
    if app == "宝宝小牙医" || app == "com.sinyee.babybus.dentist" { return Some(1004645); }
    if app == "宝宝小牙医 - 宝宝巴士" || app == "com.sinyee.babybus.dentist.global" { return Some(1004646); }
    if app == "宝宝恐龙家园" || app == "com.sinyee.babybus.dinosaur" { return Some(1004647); }
    if app == "宝宝地震安全" || app == "com.sinyee.babybus.earthquake" { return Some(1004648); }
    if app == "宝宝地震安全2" || app == "com.sinyee.babybus.earthquakeII" { return Some(1004649); }
    if app == "宝宝地震安全3" || app == "com.sinyee.babybus.earthquakeIII" { return Some(1004650); }
    if app == "宝宝冰淇淋工厂" || app == "com.sinyee.babybus.factory" { return Some(1004651); }
    if app == "中华美食" || app == "com.sinyee.babybus.food" { return Some(1004652); }
    if app == "宝宝爱水果蔬菜" || app == "com.sinyee.babybus.foodstuff" { return Some(1004653); }
    if app == "奇妙农场" || app == "com.sinyee.babybus.garden" { return Some(1004654); }
    if app == "宝宝职业梦工厂" || app == "com.sinyee.babybus.hero" { return Some(1004655); }
    if app == "宝宝巴士汉字" || app == "com.sinyee.babybus.homeland" { return Some(1004656); }
    if app == "宝宝居家安全" || app == "com.sinyee.babybus.homesafe" { return Some(1004657); }
    if app == "奇妙怪物医院" || app == "com.sinyee.babybus.hospital" { return Some(1004658); }
    if app == "雪糕工厂" || app == "com.sinyee.babybus.icecream" { return Some(1004659); }
    if app == "万花筒之旅（益智认知）" || app == "com.sinyee.babybus.kaleidoscope" { return Some(1004660); }
    if app == "宝宝开车大冒险" || app == "com.sinyee.babybus.karting" { return Some(1004661); }
    if app == "宝宝欢乐教室" || app == "com.sinyee.babybus.kindergarten" { return Some(1004662); }
    if app == "宝宝星际厨房" || app == "com.sinyee.babybus.kitchens" { return Some(1004663); }
    if app == "宝宝调色屋" || app == "com.sinyee.babybus.magichouse" { return Some(1004664); }
    if app == "小魔女传奇" || app == "com.sinyee.babybus.magician" { return Some(1004665); }
    if app == "宝宝庄园" || app == "com.sinyee.babybus.manor" { return Some(1004666); }
    if app == "宝宝数字世界" || app == "com.sinyee.babybus.math" { return Some(1004667); }
    if app == "宝宝宇航员 - 宝宝巴士" || app == "com.sinyee.babybus.moonexplorer.global" { return Some(1004668); }
    if app == "宝宝音乐派对" || app == "com.sinyee.babybus.musicparty" { return Some(1004669); }
    if app == "宝宝玩数字" || app == "com.sinyee.babybus.number" { return Some(1004670); }
    if app == "宝宝数字书写" || app == "com.sinyee.babybus.numberwriter" { return Some(1004671); }
    if app == "宝宝大扫除" || app == "com.sinyee.babybus.organized" { return Some(1004672); }
    if app == "奇妙春游日" || app == "com.sinyee.babybus.picnic" { return Some(1004673); }
    if app == "宝宝小警察" || app == "com.sinyee.babybus.policemen" { return Some(1004674); }
    if app == "奇妙百变公主" || app == "com.sinyee.babybus.princess" { return Some(1004675); }
    if app == "宝宝隐私安全" || app == "com.sinyee.babybus.privacy" { return Some(1004676); }
    if app == "人教小学英语三下点读" || app == "com.lida.diandusanxia" { return Some(1004677); }
    if app == "人教小学英语三上点读" || app == "com.lida.diandusanshang" { return Some(1004678); }
    if app == "八字万年历" || app == "com.licai.bzwnl" { return Some(1004679); }
    if app == "风韵出行司机" || app == "com.fengyuncx.yydriver" { return Some(1004680); }
    if app == "纷极阅读" || app == "com.fenjiread.learner" { return Some(1004681); }
    if app == "宝宝巴士大全 - 幼儿教育游戏合集" || app == "com.sinyee.babybus.recommendInter.adp" { return Some(1004682); }
    if app == "宝宝巴士" || app == "com.sinyee.babybus.recommendapp" { return Some(1004683); }
    if app == "宝宝修车店" || app == "com.sinyee.babybus.repair" { return Some(1004684); }
    if app == "奇妙料理餐厅" || app == "com.sinyee.babybus.restaurant" { return Some(1004685); }
    if app == "宝宝超市" || app == "com.sinyee.babybus.shopping" { return Some(1004686); }
    if app == "宝宝小超市" || app == "com.sinyee.babybus.shopping.adp" { return Some(1004687); }
    if app == "宝宝服装秀" || app == "com.sinyee.babybus.show" { return Some(1004688); }
    if app == "宝宝手工零食" || app == "com.sinyee.babybus.snacks" { return Some(1004689); }
    if app == "宝宝果汁商店" || app == "com.sinyee.babybus.soda" { return Some(1004690); }
    if app == "宝宝乐器" || app == "com.sinyee.babybus.songIV" { return Some(1004691); }
    if app == "宝宝巴士故事" || app == "com.sinyee.babybus.story" { return Some(1004692); }
    if app == "奇妙超级英雄" || app == "com.sinyee.babybus.superman" { return Some(1004693); }
    if app == "宝宝时尚设计师" || app == "com.sinyee.babybus.tailor" { return Some(1004694); }
    if app == "宝宝巴士奇妙屋" || app == "com.sinyee.babybus.talk2kiki" { return Some(1004695); }
    if app == "宝宝神奇汽车" || app == "com.sinyee.babybus.taxi" { return Some(1004696); }
    if app == "宝宝上厕所" || app == "com.sinyee.babybus.toilet" { return Some(1004697); }
    if app == "宝宝出行安全" || app == "com.sinyee.babybus.travelsafety" { return Some(1004698); }
    if app == "宝宝认工程车" || app == "com.sinyee.babybus.truck" { return Some(1004699); }
    if app == "宝宝梦想小镇" || app == "com.sinyee.babybus.village" { return Some(1004700); }
    if app == "宝宝魔力乐园" || app == "com.sinyee.babybus.wonderland" { return Some(1004701); }
    if app == "宝宝动物世界" || app == "com.sinyee.babybus.zoo" { return Some(1004702); }
    if app == "中国移动安徽" || app == "com.sitech.ac" { return Some(1004703); }
    if app == "掌上营业厅" || app == "com.sitech.palmbusinesshall4imbtvn" { return Some(1004704); }
    if app == "51借钱贷款王" || app == "com.dk.yuchendai" { return Some(1004705); }
    if app == "小学三年级英语上册" || app == "com.liangshan.wbgA" { return Some(1004706); }
    if app == "手机浏览器" || app == "com.sj.sjbrowser" { return Some(1004707); }
    if app == "辰龙捕鱼3D千炮版" || app == "com.cl0579.fish3d" { return Some(1004708); }
    if app == "天悦捕鱼" || app == "com.cl0579.FishYqs" { return Some(1004709); }
    if app == "UA浏览器" || app == "com.sljh.uabrowser" { return Some(1004710); }
    if app == "赚钱小视频" || app == "com.sljh.zqxsp" { return Some(1004711); }
    if app == "零花钱" || app == "com.lhq8.app" { return Some(1004712); }
    if app == "超级WiFi钥匙开启器" || app == "com.cjwfyaoskqiqixx.haowifi" { return Some(1004713); }
    if app == "动卡空间" || app == "com.citiccard.mobilebank" { return Some(1004714); }
    if app == "医鹿" || app == "com.citic21.user" { return Some(1004715); }
    if app == "我有外卖" || app == "com.citaq.ideliver" { return Some(1004716); }
    if app == "白菜优惠券" || app == "com.lf.coupon" { return Some(1004717); }
    if app == "顺风车司机端" || app == "com.pinke.driver" { return Some(1004718); }
    if app == "行车记录仪" || app == "com.smartcar.carrecorder" { return Some(1004719); }
    if app == "锤子便签" || app == "com.smartisan.notes" { return Some(1004720); }
    if app == "妈妈社区" || app == "com.ci123.pregnancywap" { return Some(1004721); }
    if app == "Chrome浏览器测试版" || app == "com.chrome.beta" { return Some(1004722); }
    if app == "英语电台 雅思托福听力口语" || app == "com.smartray.englishradio" { return Some(1004723); }
    if app == "WiFi密码查看神器" || app == "com.smartzone.wifitools" { return Some(1004724); }
    if app == "WiFi-UFO" || app == "com.lewei.multiple.lewei" { return Some(1004725); }
    if app == "联通实名制" || app == "com.chinaunicom.custinforegist" { return Some(1004726); }
    if app == "美呗医美" || app == "com.meibei.app" { return Some(1004727); }
    if app == "桔子理财" || app == "com.fenqile.licai" { return Some(1004728); }
    if app == "花秀直播" || app == "com.smithereens.android.kan.huaxiu" { return Some(1004729); }
    if app == "速刷理财版" || app == "com.chinapnr.android.supay.activity" { return Some(1004730); }
    if app == "乐视体育" || app == "com.lesports.glivesports" { return Some(1004731); }
    if app == "06成长汇" || app == "com.snbc.Main" { return Some(1004732); }
    if app == "积目" || app == "com.fenzotech.jimu" { return Some(1004733); }
    if app == "宝宝幼儿园英语" || app == "com.leqi.englishcard" { return Some(1004734); }
    if app == "小玛丽捕鱼" || app == "com.leqi.buyu.uc" { return Some(1004735); }
    if app == "随e行WiFi" || app == "com.chinamobile.cmccwifi" { return Some(1004736); }
    if app == "电玩城捕鱼" || app == "com.leqi.buyu" { return Some(1004737); }
    if app == "360新闻" || app == "com.so.news.activity" { return Some(1004738); }
    if app == "平安WiFi" || app == "com.pingan.pinganwifi" { return Some(1004739); }
    if app == "搜房网" || app == "com.sofang.net.buz" { return Some(1004740); }
    if app == "联想浏览器" || app == "com.lenovo.browser" { return Some(1004741); }
    if app == "战略军事" || app == "com.chinaiiss.strate" { return Some(1004742); }
    if app == "还付宝" || app == "com.sofupay.hfb" { return Some(1004743); }
    if app == "Bingo" || app == "com.sogou.activity.src" { return Some(1004744); }
    if app == "搜狗手机助手" || app == "com.sogou.androidtool" { return Some(1004745); }
    if app == "搜狗海淘" || app == "com.sogou.haitao" { return Some(1004746); }
    if app == "搜狗地图" || app == "com.sogou.map.android.maps" { return Some(1004747); }
    if app == "美丽说" || app == "com.meilishuo" { return Some(1004748); }
    if app == "华为 VR" || app == "com.huawei.appmarket.vr" { return Some(1004749); }
    if app == "搜狗翻译" || app == "com.sogou.translator" { return Some(1004750); }
    if app == "焦点好房" || app == "com.sohu.focus.live" { return Some(1004751); }
    if app == "搜狐资讯" || app == "com.sohu.infonews" { return Some(1004752); }
    if app == "搜狗输入法" || app == "com.sohu.inputmethod.sogou" { return Some(1004753); }
    if app == "搜狐新闻" || app == "com.sohu.newsclient" { return Some(1004754); }
    if app == "101辅导" || app == "com.chinaedu.blessonstu" { return Some(1004755); }
    if app == "春秋旅游" || app == "com.china3s.strip" { return Some(1004756); }
    if app == "12306掌上火车票" || app == "com.ffgamestudio.fast12306" { return Some(1004757); }
    if app == "中华军事" || app == "com.china.mobile.chinamilitary" { return Some(1004758); }
    if app == "宝宝学英语ABC" || app == "com.songbei.enAbc" { return Some(1004759); }
    if app == "东方头条" || app == "com.songheng.eastnews" { return Some(1004760); }
    if app == "松鼠资讯" || app == "com.songshu.jucai" { return Some(1004761); }
    if app == "找室友租房" || app == "com.songshulin.android.roommate" { return Some(1004762); }
    if app == "生财喵" || app == "com.songwo.luckycat" { return Some(1004763); }
    if app == "车况检测大师 OBD" || app == "rocket.vehiclemgr.android.obd2" { return Some(1004764); }
    if app == "车易拍商户版" || app == "com.cheyipai.ui" { return Some(1004765); }
    if app == "飞傲音乐" || app == "com.fiio.music" { return Some(1004766); }
    if app == "弹个车商家版" || app == "com.souche.apps.brace" { return Some(1004767); }
    if app == "弹个车" || app == "com.souche.apps.destiny" { return Some(1004768); }
    if app == "樱桃聊天交友" || app == "com.cherry.chat" { return Some(1004769); }
    if app == "中国移动浙江" || app == "com.example.businesshall" { return Some(1004770); }
    if app == "房天下装修" || app == "com.soufun.decoration.app" { return Some(1004771); }
    if app == "租房帮" || app == "com.soufun.zf" { return Some(1004772); }
    if app == "盖特浏览器" || app == "com.souget.get" { return Some(1004773); }
    if app == "美名腾智能宝宝起名" || app == "com.meimingteng.naming" { return Some(1004774); }
    if app == "维汉双语词典" || app == "com.soul.wh" { return Some(1004775); }
    if app == "羊小咩" || app == "com.financial.quantgroup" { return Some(1004776); }
    if app == "剪映" || app == "com.lemon.lv" { return Some(1004777); }
    if app == "走多多" || app == "com.cheetah.stepformoney" { return Some(1004778); }
    if app == "Faceu激萌" || app == "com.lemon.faceu" { return Some(1004779); }
    if app == "湛庐阅读" || app == "com.cheersedu.app" { return Some(1004780); }
    if app == "大大红包" || app == "com.sponsor.hbhunter" { return Some(1004781); }
    if app == "平安金管家" || app == "com.pingan.lifeinsurance" { return Some(1004782); }
    if app == "平安健康医生版" || app == "com.pingan.doctor" { return Some(1004783); }
    if app == "看点资讯" || app == "com.spotlightcolor.news" { return Some(1004784); }
    if app == "典典养车" || app == "com.chediandian.customer" { return Some(1004785); }
    if app == "平安好车主" || app == "com.pingan.carowner" { return Some(1004786); }
    if app == "车点点" || app == "com.chediandian.app" { return Some(1004787); }
    if app == "车到加油" || app == "com.chedao.app" { return Some(1004788); }
    if app == "车300专业版" || app == "com.che300.price" { return Some(1004789); }
    if app == "万能相机" || app == "com.meitu.boxxcam" { return Some(1004790); }
    if app == "星火免费小说" || app == "com.squirrel.reader" { return Some(1004791); }
    if app == "先锋浏览器" || app == "com.charlie.a07073.thunderbirdsbrowser" { return Some(1004792); }
    if app == "散人传奇" || app == "com.srcqleiting.ttaffzd" { return Some(1004793); }
    if app == "柠檬浏览器" || app == "com.ss.android.article.browser" { return Some(1004794); }
    if app == "极速充电" || app == "com.charge.matrix_battery" { return Some(1004795); }
    if app == "星尘浏览器HD" || app == "com.chaozhuo.browser" { return Some(1004796); }
    if app == "全本小说书城" || app == "com.lemi.freebook" { return Some(1004797); }
    if app == "儿童音乐" || app == "com.lemantang.gamehall" { return Some(1004798); }
    if app == "乐居买房" || app == "com.leju.platform" { return Some(1004799); }
    if app == "掌阅" || app == "com.chaozh.iReaderFree" { return Some(1004800); }
    if app == "懂车帝极速版" || app == "com.ss.android.autoprice" { return Some(1004801); }
    if app == "千炮版捕鱼达人" || app == "ruiyu.QPCatchFishJoy.uc" { return Some(1004802); }
    if app == "GOGOKID英语" || app == "com.ss.android.ex.parent" { return Some(1004803); }
    if app == "Magic Music Tiles: Piano Song" || app == "com.piano.music_tiles" { return Some(1004804); }
    if app == "360你财富" || app == "com.financial360.nicaifu" { return Some(1004805); }
    if app == "阿凡题搜题" || app == "com.lejent.zuoyeshenqi.afanti" { return Some(1004806); }
    if app == "阿凡题1对1" || app == "com.lejent.toptutor" { return Some(1004807); }
    if app == "物理大师" || app == "com.physicmaster" { return Some(1004808); }
    if app == "海狐海淘" || app == "com.leixun.haitao" { return Some(1004809); }
    if app == "传奇单机版：1.76" || app == "com.ssby.cqdjb.d" { return Some(1004810); }
    if app == "乐嗨直播" || app == "com.lehai.ui" { return Some(1004811); }
    if app == "真人欢乐捕鱼" || app == "com.fish.cooee.uc" { return Some(1004812); }
    if app == "w浏览器" || app == "com.ledu.wbrowser" { return Some(1004813); }
    if app == "汇养车" || app == "com.cgacar.hyc" { return Some(1004814); }
    if app == "e浏览器" || app == "com.ledu.ebrowser" { return Some(1004815); }
    if app == "天翼阅读" || app == "com.lectek.android.sfreader" { return Some(1004816); }
    if app == "车主无忧查违章" || app == "com.starbaba.starbaba" { return Some(1004817); }
    if app == "星巴克" || app == "com.starbucks.cn" { return Some(1004818); }
    if app == "捕鱼大师" || app == "com.fish.master" { return Some(1004819); }
    if app == "放假旅游网" || app == "com.cfd.travel.ui" { return Some(1004820); }
    if app == "车置宝二手车" || app == "com.lebo.mychebao.personaledition" { return Some(1004821); }
    if app == "奥特曼格斗进化0" || app == "com.stemu.psp.atmgdjh" { return Some(1004822); }
    if app == "乐宝家长版" || app == "com.lebaoedu.parent" { return Some(1004823); }
    if app == "音乐照片播放器" || app == "com.photo.easyphotoplayer" { return Some(1004824); }
    if app == "中原找房" || app == "com.centaline.androidsalesblog" { return Some(1004825); }
    if app == "一起玩捕鱼" || app == "com.phoneu.goldtoadfish.aligames" { return Some(1004826); }
    if app == "啄木鸟作业批改" || app == "com.suanshubang.math" { return Some(1004827); }
    if app == "3D电子罗盘" || app == "com.cdjm.d3compass" { return Some(1004828); }
    if app == "美妆相机" || app == "com.meitu.makeup" { return Some(1004829); }
    if app == "捕鱼世界" || app == "com.suishenyou.fishworld" { return Some(1004830); }
    if app == "儿童宝宝学数字" || app == "com.cdbabyjoy.xshuzi" { return Some(1004831); }
    if app == "随阅小说大全" || app == "com.suiyue.xiaoshuo" { return Some(1004832); }
    if app == "儿童宝宝学画画" || app == "com.cdbabyjoy.xhh" { return Some(1004833); }
    if app == "儿童宝宝幼儿园" || app == "com.cdbabyjoy.school" { return Some(1004834); }
    if app == "苏宁阅读HD" || app == "com.suning.mobile.magazine" { return Some(1004835); }
    if app == "四川移动掌上营业厅" || app == "com.sunrise.scmbhc" { return Some(1004836); }
    if app == "央视体育" || app == "com.cctv.cctv5ultimate" { return Some(1004837); }
    if app == "Raz-Kids" || app == "com.learninga_z.onyourown" { return Some(1004838); }
    if app == "AllBackup全备份" || app == "com.huawei.KoBackup" { return Some(1004839); }
    if app == "and-free免费WiFi" || app == "com.surf.jsandfree" { return Some(1004840); }
    if app == "最美装修" || app == "com.suryani.jiagallery" { return Some(1004841); }
    if app == "美诱直播" || app == "com.sutond.app.beautyshow.maiyu" { return Some(1004842); }
    if app == "Amaze音乐表演" || app == "com.leappmusic.amaze" { return Some(1004843); }
    if app == "上汽大众" || app == "com.svw.sc.mos" { return Some(1004844); }
    if app == "长安出行" || app == "com.ccclubs.changan" { return Some(1004845); }
    if app == "迪士尼英语家长服务" || app == "com.swordfishsoft.android.disney.education" { return Some(1004846); }
    if app == "拼车" || app == "com.sxm.farpoolcar" { return Some(1004847); }
    if app == "一喂顺风车" || app == "com.sxm.yiwei" { return Some(1004848); }
    if app == "生学堂家长端" || app == "com.sxt.parent" { return Some(1004849); }
    if app == "鹏华A加钱" || app == "com.phfund.wallet" { return Some(1004850); }
    if app == "WiFi安全助手" || app == "com.syezon.lab.wifi_manager" { return Some(1004851); }
    if app == "WiFi密码神器" || app == "com.syezon.wifikey" { return Some(1004852); }
    if app == "美颜相机" || app == "com.meitu.meiyancamera" { return Some(1004853); }
    if app == "看点小视频" || app == "com.perfect.video" { return Some(1004854); }
    if app == "氧秀直播" || app == "com.syoogame.yangba" { return Some(1004855); }
    if app == "闪银" || app == "com.syqy.wecash" { return Some(1004856); }
    if app == "联动云租车" || app == "com.ldygo.qhzc" { return Some(1004857); }
    if app == "惠头条" || app == "com.cashtoutiao" { return Some(1004858); }
    if app == "装修设计" || app == "android.decorate.jiajuol.com" { return Some(1004859); }
    if app == "宝沃汽车" || app == "com.sz.bw.borgwardcar" { return Some(1004860); }
    if app == "现金巴士" || app == "com.cashbus.android.swhj" { return Some(1004861); }
    if app == "宜停车" || app == "com.szchmtech.parkingfee" { return Some(1004862); }
    if app == "极鹰浏览器" || app == "com.szcx.browser" { return Some(1004863); }
    if app == "咚咚找房" || app == "com.szhome.dongdong" { return Some(1004864); }
    if app == "小猪佩奇儿童启蒙故事" || app == "com.sztfhd.mhxzpqqm" { return Some(1004865); }
    if app == "神州租车" || app == "com.szzc" { return Some(1004866); }
    if app == "车王二手车" || app == "com.carking.cn" { return Some(1004867); }
    if app == "车300二手车" || app == "com.car300.activity" { return Some(1004868); }
    if app == "怀孕期妈妈必备" || app == "com.ldm.pregnant.fortyweeks" { return Some(1004869); }
    if app == "专业风水罗盘" || app == "zhenghe.zhuanyefeng" { return Some(1004870); }
    if app == "租车服务 car2go" || app == "com.car2go" { return Some(1004871); }
    if app == "阅非-商铺购物抢红包赚钱" || app == "com.lcon.shangfei.shanfeishop" { return Some(1004872); }
    if app == "ABC Reading" || app == "com.tal.abctimelibrary" { return Some(1004873); }
    if app == "她理财" || app == "com.talicai.talicaiclient" { return Some(1004874); }
    if app == "美家量房" || app == "air.com.jiamm.homedraw" { return Some(1004875); }
    if app == "云宝贝家长版" || app == "com.talkweb.cloudbaby_p" { return Some(1004876); }
    if app == "Wifi小叮当" || app == "com.tangerine.wifi" { return Some(1004877); }
    if app == "直播帝" || app == "com.tango.zhibodi" { return Some(1004878); }
    if app == "陌友闲聊交友" || app == "com.tanke.aimoyou" { return Some(1004879); }
    if app == "传奇单机1.76" || app == "com.lcby.cqdj.k.dkm" { return Some(1004880); }
    if app == "B612咔叽" || app == "com.campmobile.snowcamera" { return Some(1004881); }
    if app == "天翼手机" || app == "com.lasun.mobile.client.activity" { return Some(1004882); }
    if app == "Camera51智能相机" || app == "com.camera51.android" { return Some(1004883); }
    if app == "天天吉历" || app == "com.calendar2345" { return Some(1004884); }
    if app == "黄历天气" || app == "com.calendar.UI" { return Some(1004885); }
    if app == "包有钱" || app == "com.tao.product" { return Some(1004886); }
    if app == "淘宝HD" || app == "com.taobao.apad" { return Some(1004887); }
    if app == "翻易通" || app == "com.flitto.app" { return Some(1004888); }
    if app == "车主通" || app == "com.lanxin" { return Some(1004889); }
    if app == "凯撒旅游" || app == "com.caissa.teamtouristic" { return Some(1004890); }
    if app == "证券开户" || app == "com.cairh.khapp.htsec.fuzhoulu" { return Some(1004891); }
    if app == "兰迪少儿英语" || app == "com.landi.landiclassplatform" { return Some(1004892); }
    if app == "菜鸟" || app == "com.cainiao.wireless" { return Some(1004893); }
    if app == "淘票票" || app == "com.taobao.movie.android" { return Some(1004894); }
    if app == "挖财信用卡管家" || app == "com.caimi.creditcard" { return Some(1004895); }
    if app == "萌伴小学堂" || app == "com.cacloud.ar" { return Some(1004896); }
    if app == "淘WiFi" || app == "com.taobao.wifi" { return Some(1004897); }
    if app == "淘车" || app == "com.taoche.yixin.app" { return Some(1004898); }
    if app == "天机八字排盘" || app == "com.bzPaiPan" { return Some(1004899); }
    if app == "天天电玩捕鱼" || app == "com.byzzb.lb.uc" { return Some(1004900); }
    if app == "返利赚钱联盟" || app == "com.taomibaw" { return Some(1004901); }
    if app == "懒财金服" || app == "com.lancai.main" { return Some(1004902); }
    if app == "脉脉" || app == "com.taou.maimai" { return Some(1004903); }
    if app == "奇悠阅读" || app == "com.bytetech1" { return Some(1004904); }
    if app == "豆腐阅读" || app == "com.taptech.doufu" { return Some(1004905); }
    if app == "WiFi加速器" || app == "com.flyspeed.wifispeed" { return Some(1004906); }
    if app == "作业答案大全" || app == "com.tataera.daquanhomework" { return Some(1004907); }
    if app == "懒人英语听力" || app == "com.tataera.lazylisten" { return Some(1004908); }
    if app == "华为云会议" || app == "com.huawei.CloudLink" { return Some(1004909); }
    if app == "塔塔小学英语" || app == "com.tataera.xiaoxue" { return Some(1004910); }
    if app == "GPS查车" || app == "com.tbit.tbituser" { return Some(1004911); }
    if app == "土拨鼠装修" || app == "com.tbs.tobosutype" { return Some(1004912); }
    if app == "拉卡拉" || app == "com.lakala.android" { return Some(1004913); }
    if app == "证太理财" || app == "com.tdx.AndroidTPY" { return Some(1004914); }
    if app == "教学钢琴" || app == "com.teaching.piano" { return Some(1004915); }
    if app == "比亚迪汽车" || app == "com.byd.aeri.caranywhere" { return Some(1004916); }
    if app == "特价惠" || app == "com.tejiahui" { return Some(1004917); }
    if app == "真人街机捕鱼千炮版" || app == "com.by.fishgame.zr.uc" { return Some(1004918); }
    if app == "街机金蟾捕鱼" || app == "com.by.fishgame.uc" { return Some(1004919); }
    if app == "四川电信掌上营业厅" || app == "com.telecom.sc.housekeeper" { return Some(1004920); }
    if app == "黄油相机" || app == "com.by.butter.camera" { return Some(1004921); }
    if app == "来看阅读" || app == "com.laikan.reader" { return Some(1004922); }
    if app == "蜜月直播" || app == "com.butterfly.jonson.live.miyue" { return Some(1004923); }
    if app == "WiFi密码钥匙显示器" || app == "com.buqie.wifipwdtools" { return Some(1004924); }
    if app == "QQ邮箱" || app == "com.tencent.androidqqmail" { return Some(1004925); }
    if app == "儿童学汉字游戏" || app == "com.example.babykownchinesecharacter" { return Some(1004926); }
    if app == "简理财" || app == "com.laijin.simplefinance" { return Some(1004927); }
    if app == "传奇世界" || app == "com.tencent.cqsj" { return Some(1004928); }
    if app == "悟空阅读" || app == "air.com.gongfubb.wkyd" { return Some(1004929); }
    if app == "UCC浏览器" || app == "com.browser2345_ucc" { return Some(1004930); }
    if app == "2345极速浏览器" || app == "com.browser2345_js" { return Some(1004931); }
    if app == "火锅视频" || app == "com.tencent.firevideo" { return Some(1004932); }
    if app == "乘车码" || app == "com.tencent.fit.ccm" { return Some(1004933); }
    if app == "相册管家" || app == "com.tencent.gallerymanager" { return Some(1004934); }
    if app == "天天飞车" || app == "com.tencent.game.SSGame" { return Some(1004935); }
    if app == "2345浏览器" || app == "com.browser2345" { return Some(1004936); }
    if app == "王者营地" || app == "com.tencent.gamehelper.smoba" { return Some(1004937); }
    if app == "掌上飞车" || app == "com.tencent.gamehelper.speed" { return Some(1004938); }
    if app == "绿网浏览器" || app == "com.browser.txtw" { return Some(1004939); }
    if app == "贪吃蛇浏览器" || app == "com.browser.tcs" { return Some(1004940); }
    if app == "久久浏览器" || app == "com.forever.browser" { return Some(1004941); }
    if app == "JOOX免费音乐" || app == "com.tencent.ibg.joox" { return Some(1004942); }
    if app == "Brave浏览器" || app == "com.brave.browser" { return Some(1004943); }
    if app == "小学英语伴读人教版" || app == "com.pengpengcj.cjypep" { return Some(1004944); }
    if app == "萌心小视频" || app == "com.boyibo.minivideo" { return Some(1004945); }
    if app == "企鹅电竞直播助手" || app == "com.tencent.liveassistant" { return Some(1004946); }
    if app == "蓝月传奇" || app == "com.tencent.lycqsh" { return Some(1004947); }
    if app == "BOXFiSH盒子鱼英语" || app == "com.boxfish.stu" { return Some(1004948); }
    if app == "跨越司机" || app == "com.kyepartner.express" { return Some(1004949); }
    if app == "腾讯听听" || app == "com.tencent.mia.speaker" { return Some(1004950); }
    if app == "服装八场" || app == "com.kwrzcu.kcfsod" { return Some(1004951); }
    if app == "快手小游戏" || app == "com.kwai.sogame" { return Some(1004952); }
    if app == "花粉俱乐部" || app == "com.huawei.fans" { return Some(1004953); }
    if app == "QQ浏览器" || app == "com.tencent.mtt" { return Some(1004954); }
    if app == "QQ浏览器Play版" || app == "com.tencent.mtt.intl" { return Some(1004955); }
    if app == "一甜相机" || app == "com.kwai.m2u" { return Some(1004956); }
    if app == "音乐圣经" || app == "com.boostfield.musicbible" { return Some(1004957); }
    if app == "有味" || app == "com.tencent.news.lite" { return Some(1004958); }
    if app == "香网小说" || app == "com.boetech.xiangread" { return Some(1004959); }
    if app == "QQ浏览器HD" || app == "com.tencent.padbrowser" { return Some(1004960); }
    if app == "北京移动手机营业厅" || app == "com.bmcc.ms.ui" { return Some(1004961); }
    if app == "企鹅电竞" || app == "com.tencent.qgame" { return Some(1004962); }
    if app == "Y2002电音" || app == "com.blueocean.musicplayer" { return Some(1004963); }
    if app == "应用宝HD" || app == "com.tencent.qqappmarket.hd" { return Some(1004964); }
    if app == "电喵直播" || app == "com.kwai.android.gzone" { return Some(1004965); }
    if app == "佛滔命理大师" || app == "com.fotao.fotaoapp" { return Some(1004966); }
    if app == "看房" || app == "com.tencent.qqhouse" { return Some(1004967); }
    if app == "幻音音乐" || app == "com.huanyin.magic" { return Some(1004968); }
    if app == "宝贝听听" || app == "com.kunpeng.babyting" { return Some(1004969); }
    if app == "小企鹅乐园" || app == "com.tencent.qqlivekid" { return Some(1004970); }
    if app == "北京医院预约挂号网" || app == "com.bjguahao.yyghw" { return Some(1004971); }
    if app == "猎豹贷款闪电借款" || app == "com.birdfenqi.liebaodai" { return Some(1004972); }
    if app == "音乐圈" || app == "com.tencent.qqmusic.samsung" { return Some(1004973); }
    if app == "学英语听力" || app == "com.binfenyingyu.bbc" { return Some(1004974); }
    if app == "ofo共享单车" || app == "so.ofo.labofo" { return Some(1004975); }
    if app == "QQ同步助手" || app == "com.tencent.qqpim" { return Some(1004976); }
    if app == "酷狗音乐TV版" || app == "com.kugou.tv.android" { return Some(1004977); }
    if app == "酷狗音乐PAD版" || app == "com.kugou.playerHD" { return Some(1004978); }
    if app == "繁星直播" || app == "com.kugou.fanxing.lite" { return Some(1004979); }
    if app == "爆点资讯" || app == "com.bigbang.news" { return Some(1004980); }
    if app == "快报" || app == "com.tencent.reading" { return Some(1004981); }
    if app == "便利蜂" || app == "com.bianlifeng.customer.android" { return Some(1004982); }
    if app == "看点" || app == "com.tencent.rijvideo" { return Some(1004983); }
    if app == "WiFi路由管家" || app == "com.bhu.wifioverlook" { return Some(1004984); }
    if app == "跑跑卡丁车官方竞速版" || app == "com.tencent.tmgp.WePop" { return Some(1004985); }
    if app == "捕鱼来了" || app == "com.tencent.tmgp.bydr3dx" { return Some(1004986); }
    if app == "穿越火线：枪战王者" || app == "com.tencent.tmgp.cf" { return Some(1004987); }
    if app == "一汽马自达购车助手" || app == "com.bh.cig.mazda" { return Some(1004988); }
    if app == "和平精英" || app == "com.tencent.tmgp.pubgmhd" { return Some(1004989); }
    if app == "绝地求生：刺激战场体验服" || app == "com.tencent.tmgp.pubgmhdce" { return Some(1004990); }
    if app == "热血传奇-跨服新大陆" || app == "com.tencent.tmgp.rxcq" { return Some(1004991); }
    if app == "千炮捕鱼联网版-疯狂爆金币" || app == "com.tencent.tmgp.saiyun.qpbylwb2" { return Some(1004992); }
    if app == "王者荣耀" || app == "com.tencent.tmgp.sgame" { return Some(1004993); }
    if app == "QQ飞车" || app == "com.tencent.tmgp.speedmobile" { return Some(1004994); }
    if app == "百分百考试" || app == "com.bfbksw.webapp" { return Some(1004995); }
    if app == "Beta理财师" || app == "com.betawm.baw" { return Some(1004996); }
    if app == "心语欣欣" || app == "com.bestbrand.xthk" { return Some(1004997); }
    if app == "酷狗直播" || app == "com.kugou.fanxing" { return Some(1004998); }
    if app == "百合生活" || app == "com.berchina.o2o.buyer.ui" { return Some(1004999); }
    if app == "捕鱼街机电玩城" || app == "com.tencent.tmgp.youxila" { return Some(1005000); }
    if app == "QQ安全中心" || app == "com.tencent.token" { return Some(1005001); }
    if app == "企鹅体育" || app == "com.tencent.tv.qie" { return Some(1005002); }
    if app == "无他相机" || app == "com.benqu.wuta" { return Some(1005003); }
    if app == "牛听听" || app == "com.benew.ntt" { return Some(1005004); }
    if app == "酷狗概念版" || app == "com.kugou.android.lite" { return Some(1005005); }
    if app == "微视" || app == "com.tencent.weishi" { return Some(1005006); }
    if app == "北斗地图" || app == "com.beidou.android.beidoumap" { return Some(1005007); }
    if app == "育儿宝" || app == "com.beibo.yuerbao" { return Some(1005008); }
    if app == "白熊阅读" || app == "com.bearead.app" { return Some(1005009); }
    if app == "九歌音乐" || app == "com.beansprout.music" { return Some(1005010); }
    if app == "传奇世界3D" || app == "com.tencent.woool3d" { return Some(1005011); }
    if app == "宽途洗车卡" || app == "com.kuanter.kuanterauto" { return Some(1005012); }
    if app == "水印相机" || app == "com.tencent.zebra" { return Some(1005013); }
    if app == "音乐速度调节器专业版[安智汉化]" || app == "com.tequnique.msc" { return Some(1005014); }
    if app == "Tesla(Beta版)" || app == "com.teslamotors.tesla" { return Some(1005015); }
    if app == "快对" || app == "com.kuaiduizuoye.scan" { return Some(1005016); }
    if app == "汽车大师" || app == "com.bcb.carmaster" { return Some(1005017); }
    if app == "vivo应用商店" || app == "com.bbk.appstore" { return Some(1005018); }
    if app == "互动作业答案快对" || app == "com.kuaiduizuoye.dprd" { return Some(1005019); }
    if app == "WiFi万能密码查看" || app == "com.baymax.wifitools" { return Some(1005020); }
    if app == "免费WiFi密码钥匙" || app == "com.baymax.wifipoint" { return Some(1005021); }
    if app == "长城证券营业部开户" || app == "com.thinkive.mobile.cgws.account" { return Some(1005022); }
    if app == "小豆苗" || app == "com.threegene.yeemiao" { return Some(1005023); }
    if app == "滴滴代驾司机" || app == "com.kuaidi.daijia.driver" { return Some(1005024); }
    if app == "快成司机" || app == "com.kuaichengwuliu.driver" { return Some(1005025); }
    if app == "甜橙直播" || app == "com.tiancapp.cn" { return Some(1005026); }
    if app == "葡萄美女直播" || app == "com.tiange.grape" { return Some(1005027); }
    if app == "喵播" || app == "com.tiange.miaolive" { return Some(1005028); }
    if app == "小蛮腰直播" || app == "com.tiange.waist" { return Some(1005029); }
    if app == "甜果资讯赚钱" || app == "com.tianguo.zxz" { return Some(1005030); }
    if app == "WiFi密码管理器" || app == "com.tianhao.partner.android.wifi" { return Some(1005031); }
    if app == "宝骏新能源" || app == "com.baojun.newterritory" { return Some(1005032); }
    if app == "一起阅读学生" || app == "com.tianqi.stu" { return Some(1005033); }
    if app == "买车宝典" || app == "com.baojiazhijia.qichebaojia" { return Some(1005034); }
    if app == "天天浏览器" || app == "com.tiantianmini.android.browser" { return Some(1005035); }
    if app == "天行听书" || app == "com.tianxing.voicebook" { return Some(1005036); }
    if app == "Genie音乐精灵" || app == "com.ktmusic.geniemusic" { return Some(1005037); }
    if app == "灵犀浏览器" || app == "sogou.mobile.explorer.streamline" { return Some(1005038); }
    if app == "小鲤鱼育儿" || app == "com.tiaolm.bhc" { return Some(1005039); }
    if app == "铁行火车票" || app == "com.tiexing" { return Some(1005040); }
    if app == "军事头条" || app == "com.tiexue.mobile.topnews" { return Some(1005041); }
    if app == "12306铁友火车票" || app == "com.tieyou.train.ark" { return Some(1005042); }
    if app == "星座大师" || app == "com.banma.astro" { return Some(1005043); }
    if app == "一汽大众服务" || app == "com.timanetworks.android.faw.vw.aftermarket.release" { return Some(1005044); }
    if app == "听世界听书" || app == "com.ting" { return Some(1005045); }
    if app == "淘集集" || app == "com.huanshou.taojj" { return Some(1005046); }
    if app == "音乐铃声剪辑" || app == "com.fragileheart.mp3editor" { return Some(1005047); }
    if app == "玖富钱包" || app == "com.bank9f.weilicai" { return Some(1005048); }
    if app == "千千静听百度音乐版" || app == "com.ting.mp3.qianqian.android" { return Some(1005049); }
    if app == "听戏" || app == "com.hualumedia.opera" { return Some(1005050); }
    if app == "姓名配对打分测试缘分" || app == "com.peidui.jiangxiaodong" { return Some(1005051); }
    if app == "凯叔讲故事" || app == "com.ks.kaishustory" { return Some(1005052); }
    if app == "带营音乐" || app == "com.bandcamp.android" { return Some(1005053); }
    if app == "WiFi密码显示器" || app == "com.tjf.wifiscanner.v1" { return Some(1005054); }
    if app == "半次元" || app == "com.banciyuan.bcywebview" { return Some(1005055); }
    if app == "奇乐直播" || app == "com.banbantv.show" { return Some(1005056); }
    if app == "记事本 Note Pad" || app == "com.example.android.notepad" { return Some(1005057); }
    if app == "文件浏览器" || app == "com.fs.browser" { return Some(1005058); }
    if app == "趣租房" || app == "com.baihe.pie" { return Some(1005059); }
    if app == "百合相亲" || app == "com.baihe.date" { return Some(1005060); }
    if app == "百合网" || app == "com.baihe" { return Some(1005061); }
    if app == "百度阅读Pro" || app == "com.baidu.yuedupro" { return Some(1005062); }
    if app == "有钱花Lite" || app == "com.baidu.umoney" { return Some(1005063); }
    if app == "贴吧极速版" || app == "com.baidu.tieba_mini" { return Some(1005064); }
    if app == "百度贴吧" || app == "com.baidu.tieba" { return Some(1005065); }
    if app == "百度医生" || app == "com.baidu.patient" { return Some(1005066); }
    if app == "快陪练" || app == "com.kpl.student" { return Some(1005067); }
    if app == "土巴兔装修" || app == "com.to8to.housekeeper" { return Some(1005068); }
    if app == "设计本装修" || app == "com.to8to.wireless.designroot" { return Some(1005069); }
    if app == "装修体验馆" || app == "com.to8to.zxtyg" { return Some(1005070); }
    if app == "百度新闻" || app == "com.baidu.news" { return Some(1005071); }
    if app == "百度音乐HD" || app == "com.baidu.music.pad" { return Some(1005072); }
    if app == "度小视" || app == "com.baidu.minivideo" { return Some(1005073); }
    if app == "车到哪" || app == "com.tonglu.app" { return Some(1005074); }
    if app == "学而思轻课" || app == "com.tongxue.tiku" { return Some(1005075); }
    if app == "PEP小学英语三下" || app == "com.tongzhou.pep_xiaoxue_yinyu_sannianji_xia" { return Some(1005076); }
    if app == "小学英语三年级（上）" || app == "com.tongzhou.xiaoxue_yinyu_sannianji_shang" { return Some(1005077); }
    if app == "小学英语四年级(下)" || app == "com.tongzhou.xiaoxue_yinyu_sinianji_xia" { return Some(1005078); }
    if app == "小学英语一年级(上)" || app == "com.tongzhou.xiaoxue_yinyu_yinianji_shang" { return Some(1005079); }
    if app == "宝宝知道" || app == "com.baidu.mbaby" { return Some(1005080); }
    if app == "新东方在线" || app == "com.koolearn.android" { return Some(1005081); }
    if app == "饿了么星选" || app == "com.baidu.lbs.waimai" { return Some(1005082); }
    if app == "小猪佩奇拼图" || app == "com.koodroid.puzzle.peppa" { return Some(1005083); }
    if app == "华为输入法" || app == "com.baidu.input_huawei" { return Some(1005084); }
    if app == "一汽丰田" || app == "com.toyota.ftmsApp" { return Some(1005085); }
    if app == "CHM阅读器" || app == "com.pdagate.chmreader" { return Some(1005086); }
    if app == "陆鲸司机" || app == "com.transfar56.project.uc" { return Some(1005087); }
    if app == "塔罗牌占卜" || app == "taluo.jumeng.com.tarot" { return Some(1005088); }
    if app == "百度输入法" || app == "com.baidu.input" { return Some(1005089); }
    if app == "Tripadvisor猫途鹰" || app == "com.tripadvisor.tripadvisor.daodao" { return Some(1005090); }
    if app == "卡车之家" || app == "com.truckhome.bbs" { return Some(1005091); }
    if app == "BOBO直播" || app == "com.example.BOBO" { return Some(1005092); }
    if app == "免费小说阅读神器" || app == "com.kong.app.book.fengniao" { return Some(1005093); }
    if app == "耳多资讯" || app == "com.fulihui.www.information" { return Some(1005094); }
    if app == "考拉阅读教师" || app == "com.koalareading.koalateacher" { return Some(1005095); }
    if app == "畅途汽车票" || app == "com.tts.hybird" { return Some(1005096); }
    if app == "松果出行" || app == "com.ttyongche.ttpinecone" { return Some(1005097); }
    if app == "作业帮" || app == "com.baidu.homework" { return Some(1005098); }
    if app == "土豆视频HD" || app == "com.tudou.xoom.android" { return Some(1005099); }
    if app == "兔几直播" || app == "com.tuji.live.tv" { return Some(1005100); }
    if app == "觅马出行" || app == "com.tulingweier.yw.minihorsetravelapp" { return Some(1005101); }
    if app == "途牛旅游HD" || app == "com.tuniu.HD.ui" { return Some(1005102); }
    if app == "作业精灵" || app == "com.pcncn.jj" { return Some(1005103); }
    if app == "考拉阅读" || app == "com.koalareading.koalareading" { return Some(1005104); }
    if app == "儿童学习钢琴" || app == "com.tuoniu.dokdoapps.mybabypiano" { return Some(1005105); }
    if app == "妥妥E行司机端" || app == "com.tuotuo.driver" { return Some(1005106); }
    if app == "学习帮" || app == "com.tupo.studygroup" { return Some(1005107); }
    if app == "平安车管家" || app == "com.pazl.qcjr.cgj" { return Some(1005108); }
    if app == "微家园家长版" || app == "com.tuxing.app.home" { return Some(1005109); }
    if app == "捕鱼大作战" || app == "com.tuyoo.fish.uc" { return Some(1005110); }
    if app == "度小满理财" || app == "com.baidu.finance" { return Some(1005111); }
    if app == "电视红包" || app == "com.tvmining.yao8" { return Some(1005112); }
    if app == "作业盒子中学学生端" || app == "com.knowbox.wb.student" { return Some(1005113); }
    if app == "儿童捕鱼游戏" || app == "com.twinbrige.birgefish" { return Some(1005114); }
    if app == "Boss直聘高薪版" || app == "com.twl.bosszhipin2" { return Some(1005115); }
    if app == "花椒直播" || app == "com.huajiao" { return Some(1005116); }
    if app == "天象黄历" || app == "com.tx.txalmanac" { return Some(1005117); }
    if app == "百度浏览器" || app == "com.baidu.browser.apps" { return Some(1005118); }
    if app == "百度翻译" || app == "com.baidu.baidutranslate" { return Some(1005119); }
    if app == "免费WiFi大全" || app == "com.txj.wifi.free" { return Some(1005120); }
    if app == "百度手机助手" || app == "com.baidu.appsearch" { return Some(1005121); }
    if app == "百度地图HD" || app == "com.baidu.BaiduMap.auto" { return Some(1005122); }
    if app == "WiFi_CAM" || app == "com.tzh.wifi.wificam.activity" { return Some(1005123); }
    if app == "百度地图" || app == "com.baidu.BaiduMap" { return Some(1005124); }
    if app == "小盒学习" || app == "com.knowbox.rc.student.pk" { return Some(1005125); }
    if app == "贝乐虎启蒙" || app == "com.ubestkid.collection.a" { return Some(1005126); }
    if app == "儿童益智识字" || app == "com.babywhere.learnwords" { return Some(1005127); }
    if app == "美女捕鱼" || app == "com.uc108.mobile.tcby.uc" { return Some(1005128); }
    if app == "淘车二手车" || app == "com.ucar.app" { return Some(1005129); }
    if app == "美团出租司机" || app == "com.meituan.taxi.android" { return Some(1005130); }
    if app == "宝宝树小时光" || app == "com.babytree.apps.lama" { return Some(1005131); }
    if app == "贝聊家长版" || app == "com.babychat" { return Some(1005132); }
    if app == "传奇塔防" || app == "com.babeltimeus.legendstd" { return Some(1005133); }
    if app == "小盒学生" || app == "com.knowbox.rc.student" { return Some(1005134); }
    if app == "美团旅行" || app == "com.meituan.tower" { return Some(1005135); }
    if app == "泡单词" || app == "com.paoword.www.paoword" { return Some(1005136); }
    if app == "旅游景点攻略" || app == "com.auyou.jingdian" { return Some(1005137); }
    if app == "达人直播" || app == "com.uelive.talentlive.activity" { return Some(1005138); }
    if app == "旅游结伴" || app == "com.auyou.jieban" { return Some(1005139); }
    if app == "WiFi钥匙万能查" || app == "com.autosee.wifitool" { return Some(1005140); }
    if app == "快的司机" || app == "com.funcity.taxi.driver" { return Some(1005141); }
    if app == "酷酷跑" || app == "com.kkptech.kkpsy" { return Some(1005142); }
    if app == "波波浏览器" || app == "com.unas.boo" { return Some(1005143); }
    if app == "KKBOX 音乐商店" || app == "com.kkbox.alc.android" { return Some(1005144); }
    if app == "和地图" || app == "com.autonavi.cmccmap" { return Some(1005145); }
    if app == "沃阅读" || app == "com.unicom.zworeader.ui" { return Some(1005146); }
    if app == "养车之家" || app == "com.autohome.vendor" { return Some(1005147); }
    if app == "二手车之家" || app == "com.autohome.usedcar" { return Some(1005148); }
    if app == "熊猫直播HD" || app == "com.panda.videolivehd" { return Some(1005149); }
    if app == "康爱多掌上药店" || app == "com.unique.app" { return Some(1005150); }
    if app == "UniToy智能" || app == "com.unisound.karrobot" { return Some(1005151); }
    if app == "盼达用车" || app == "com.panda.usecar" { return Some(1005152); }
    if app == "Flyme桌面" || app == "com.meizu.flyme.launcher" { return Some(1005153); }
    if app == "WiFi 连网神器" || app == "com.autoconnectwifi.app" { return Some(1005154); }
    if app == "向上网" || app == "com.up360.parents.android.activity" { return Some(1005155); }
    if app == "充电吧" || app == "com.up72.beiqi" { return Some(1005156); }
    if app == "透明房产网" || app == "com.funi.cloudcode" { return Some(1005157); }
    if app == "wifi分析助手" || app == "com.kk.xx.analyzer" { return Some(1005158); }
    if app == "金山背单词" || app == "com.kingsoft.wordback" { return Some(1005159); }
    if app == "弹吧钢琴陪练" || app == "com.autoapp.pianostave" { return Some(1005160); }
    if app == "趣直播" || app == "com.uqu.live" { return Some(1005161); }
    if app == "ETC车宝" || app == "com.uroad.carclub" { return Some(1005162); }
    if app == "金山词霸" || app == "com.kingsoft" { return Some(1005163); }
    if app == "有声读物阅读器 Audible" || app == "com.audible.application" { return Some(1005164); }
    if app == "华硕音乐播放器" || app == "com.asus.music" { return Some(1005165); }
    if app == "移动WiFi通" || app == "com.aspire.g3wlan.client" { return Some(1005166); }
    if app == "碰碰交友" || app == "com.asiainno.pengpeng" { return Some(1005167); }
    if app == "H5浏览器" || app == "org.noear.h5" { return Some(1005168); }
    if app == "广东移动智慧生活" || app == "com.kingpoint.gmcchh" { return Some(1005169); }
    if app == "小学英语单词同步学" || app == "com.uyutong.xxyydctbx" { return Some(1005170); }
    if app == "小学英语课本点读" || app == "com.uyutong.xxyykbdd" { return Some(1005171); }
    if app == "哈哈文库" || app == "com.v.junior" { return Some(1005172); }
    if app == "互助文档" || app == "com.v.zy" { return Some(1005173); }
    if app == "农场英雄传奇" || app == "com.king.farmheroessaga" { return Some(1005174); }
    if app == "海淘免税店" || app == "com.vanwell.module.zhefenglepink.app" { return Some(1005175); }
    if app == "生鲜传奇" || app == "com.kidswant.freshlegend" { return Some(1005176); }
    if app == "耽美小说" || app == "com.huabenapp.danmei" { return Some(1005177); }
    if app == "星星钱袋" || app == "com.vcredit.starcredit" { return Some(1005178); }
    if app == "悟空识字" || app == "air.com.gongfubb.wksz" { return Some(1005179); }
    if app == "车旺大卡" || app == "com.vehicles.activities" { return Some(1005180); }
    if app == "老黄历" || app == "com.veryapps.chinacalendar" { return Some(1005181); }
    if app == "袋鼠家-家长" || app == "com.viapalm.kcparent" { return Some(1005182); }
    if app == "动漫相机" || app == "com.dongman.camera" { return Some(1005183); }
    if app == "安全体检助手" || app == "com.appmall.box" { return Some(1005184); }
    if app == "快方送药" || app == "com.kf.kuaifang" { return Some(1005185); }
    if app == "快逗短视频" || app == "com.video.kd" { return Some(1005186); }
    if app == "安安用车司机端" || app == "com.keubano.zhdz" { return Some(1005187); }
    if app == "新趣小视频" || app == "com.video.newqu" { return Some(1005188); }
    if app == "火趣小视频" || app == "com.video.v8090" { return Some(1005189); }
    if app == "课课作业" || app == "com.kerkr.kerkrstudent.kerkrstudent" { return Some(1005190); }
    if app == "众鑫玩卡" || app == "com.appbyme.app239109" { return Some(1005191); }
    if app == "可可宝贝" || app == "com.kekenet.baby" { return Some(1005192); }
    if app == "天天捕鱼达人" || app == "tiantianbuyudaren.cocos2dx.game.gameplatformapp.aligames" { return Some(1005193); }
    if app == "天机六爻排盘" || app == "com.paipanapp" { return Some(1005194); }
    if app == "Flyme图库" || app == "com.meizu.media.gallery" { return Some(1005195); }
    if app == "球球直播" || app == "com.app.qqzb" { return Some(1005196); }
    if app == "亲朋捕鱼大乱斗" || app == "com.viking.BuYuDLD" { return Some(1005197); }
    if app == "车行易查违章" || app == "com.violationquery" { return Some(1005198); }
    if app == "无忧简单极速贷" || app == "com.vip.pinganedai" { return Some(1005199); }
    if app == "小学作业答案" || app == "com.keke.kerkrstudent2" { return Some(1005200); }
    if app == "平安好学英语" || app == "com.vipabc.vipmobile.phone" { return Some(1005201); }
    if app == "汇添富现金宝" || app == "com.htffund.mobile.ec.ui" { return Some(1005202); }
    if app == "vivo浏览器" || app == "com.vivo.browser" { return Some(1005203); }
    if app == "互传" || app == "com.vivo.easyshare" { return Some(1005204); }
    if app == "VC浏览器" || app == "com.funnylemon.browser" { return Some(1005205); }
    if app == "全民听书" || app == "com.app.lrlisten" { return Some(1005206); }
    if app == "vivo官网" || app == "com.vivo.space" { return Some(1005207); }
    if app == "vivo视频" || app == "com.vivo.video" { return Some(1005208); }
    if app == "风行视频HD" || app == "com.funshion.video.pad" { return Some(1005209); }
    if app == "小白来花" || app == "com.kdlc.xqb" { return Some(1005210); }
    if app == "话萌小说" || app == "com.vnovel" { return Some(1005211); }
    if app == "114预约挂号网" || app == "com.vodone.o2o.mingyi_guahao_114.demander" { return Some(1005212); }
    if app == "口袋理财" || app == "com.kdkj.koudailicai" { return Some(1005213); }
    if app == "蜀山浏览器" || app == "com.vqs.vip" { return Some(1005214); }
    if app == "触手直播" || app == "com.kascend.chushou" { return Some(1005215); }
    if app == "溜溜好运八字排盘" || app == "com.donhoo.bazipaipan" { return Some(1005216); }
    if app == "考拉海购" || app == "com.kaola" { return Some(1005217); }
    if app == "VV" || app == "com.vv51.mvbox" { return Some(1005218); }
    if app == "百果园" || app == "com.pagoda.buy" { return Some(1005219); }
    if app == "万贯街贷款" || app == "com.app.finance.afd" { return Some(1005220); }
    if app == "爱车生活" || app == "com.fw.gps.yiwenneutral" { return Some(1005221); }
    if app == "装修记账本" || app == "com.wacai.wjz.decoration" { return Some(1005222); }
    if app == "挖财记账" || app == "com.wacai365" { return Some(1005223); }
    if app == "新车报价之家" || app == "com.newcar.activity" { return Some(1005224); }
    if app == "平安普惠陆慧融" || app == "com.paem" { return Some(1005225); }
    if app == "孩子国家长" || app == "yuerhelper.com" { return Some(1005226); }
    if app == "星星充电" || app == "com.wanbangauto.chargepile" { return Some(1005227); }
    if app == "看看新闻" || app == "com.kankanews.kankanxinwen" { return Some(1005228); }
    if app == "埃安" || app == "com.gacne.www" { return Some(1005229); }
    if app == "快手看片" || app == "com.kandian.vodapp" { return Some(1005230); }
    if app == "快易花" || app == "com.wanda.kuaiyihua" { return Some(1005231); }
    if app == "学子斋作业答案" || app == "com.apicloud.A6992757665591" { return Some(1005232); }
    if app == "外研社小学英语一年级上册" || app == "air.com.byebyer.waiyanshe1A" { return Some(1005233); }
    if app == "轻芒阅读" || app == "com.wandoujia" { return Some(1005234); }
    if app == "豌豆荚" || app == "com.wandoujia.phoenix2" { return Some(1005235); }
    if app == "YY小视频" || app == "com.wangniu.fvc" { return Some(1005236); }
    if app == "爱学小学" || app == "com.aoyuan.aixue.stps.app" { return Some(1005237); }
    if app == "听力百分百" || app == "com.wanhe.eng100.listening" { return Some(1005238); }
    if app == "证券从业万题库" || app == "com.exam8.zhengquan" { return Some(1005239); }
    if app == "多彩便签" || app == "com.aohe.icodestar.notes" { return Some(1005240); }
    if app == "快手搞笑" || app == "com.kandian.shortgaoxiao" { return Some(1005241); }
    if app == "千炮彩金捕鱼" || app == "com.wanmei.qkfish.uc" { return Some(1005242); }
    if app == "全屋WIFI评测" || app == "com.wanwei.zhuangwei.wifiqualityevaluation" { return Some(1005243); }
    if app == "陕西移动云店" || app == "com.doone.sxyd" { return Some(1005244); }
    if app == "P2P理财" || app == "com.p2peye.manage" { return Some(1005245); }
    if app == "DOSS音乐" || app == "com.dossav.dossmusic" { return Some(1005246); }
    if app == "探探极速版" || app == "com.p1.mobile.light" { return Some(1005247); }
    if app == "万表全球名表" || app == "com.wbiao.wbapp" { return Some(1005248); }
    if app == "aWiFi小助手" || app == "com.wcare.telecom.wifi" { return Some(1005249); }
    if app == "万达普惠" || app == "com.wdjr.loan" { return Some(1005250); }
    if app == "唯爱交友" || app == "com.wealove.chat" { return Some(1005251); }
    if app == "不可能的世界小说" || app == "com.kana.reader" { return Some(1005252); }
    if app == "氧气育儿幼教" || app == "com.anysoft.tyyd.dz.m1my1" { return Some(1005253); }
    if app == "氧气听书" || app == "com.anysoft.tyyd" { return Some(1005254); }
    if app == "KakaoStory社区交友" || app == "com.kakao.story" { return Some(1005255); }
    if app == "变身奥特曼" || app == "com.kaizhengne.atmcamera" { return Some(1005256); }
    if app == "传奇霸业手游(正版)" || app == "com.game37.bayechuanqi" { return Some(1005257); }
    if app == "小学英语四年级下册" || app == "air.com.byebyer.renjiaosanqi4b" { return Some(1005258); }
    if app == "七彩抢红包" || app == "com.weikuaibupo.qicaihongbao" { return Some(1005259); }
    if app == "战旗直播" || app == "com.gameabc.zhanqiAndroid" { return Some(1005260); }
    if app == "奥维互动地图" || app == "com.ovital.ovitalMap" { return Some(1005261); }
    if app == "财鸟贷款" || app == "com.weizhong.cainiaodaikuan" { return Some(1005262); }
    if app == "蚂蚁财富" || app == "com.antfortune.wealth" { return Some(1005263); }
    if app == "学霸君" || app == "com.wenba.bangbang" { return Some(1005264); }
    if app == "学霸君家长" || app == "com.wenba.junjunparent" { return Some(1005265); }
    if app == "学霸君1对1HD" || app == "com.wenba.student" { return Some(1005266); }
    if app == "学霸君1对1" || app == "com.wenba.xbjtutor" { return Some(1005267); }
    if app == "车e兴" || app == "com.anshibo.activity" { return Some(1005268); }
    if app == "司机先生" || app == "com.ansangha.drdriving" { return Some(1005269); }
    if app == "小白贷款-极速贷" || app == "com.anniu.white" { return Some(1005270); }
    if app == "3A幼教助手" || app == "com.anke.app.activity" { return Some(1005271); }
    if app == "欧洲卡车司机-手游版" || app == "com.ovilex.eurotruckdriver.adp" { return Some(1005272); }
    if app == "我厨买菜" || app == "com.wicture.wochu" { return Some(1005273); }
    if app == "WiFi众联钥匙" || app == "com.wifi.key" { return Some(1005274); }
    if app == "WiFi免费钥匙" || app == "com.wifi.qx" { return Some(1005275); }
    if app == "天风同花顺" || app == "com.hexin.plat.android.TianfengSZSecurity" { return Some(1005276); }
    if app == "安全教育平台" || app == "com.jzzs.ParentsHelper" { return Some(1005277); }
    if app == "连尚读书女生版" || app == "com.wifi.reader.girl" { return Some(1005278); }
    if app == "WIFI通" || app == "com.wifi.tong" { return Some(1005279); }
    if app == "WiFi万能极速钥匙" || app == "com.wifidecode.mycompany" { return Some(1005280); }
    if app == "WiFi共享精灵" || app == "com.wifigx.wifishare" { return Some(1005281); }
    if app == "WIFI密码破解" || app == "com.wifipassword.wifimanager" { return Some(1005282); }
    if app == "wifi万能密码神器" || app == "com.wifiwnmsq.haowifi" { return Some(1005283); }
    if app == "WiFi信号分析仪" || app == "com.wind.wifianalyzer" { return Some(1005284); }
    if app == "欧洲卡车司机" || app == "com.ovilex.eurotruckdriver" { return Some(1005285); }
    if app == "开运风水罗盘" || app == "com.gamepans.compass" { return Some(1005286); }
    if app == "阳光阅读" || app == "com.winshare.sunshineread.activity" { return Some(1005287); }
    if app == "微课掌上通" || app == "com.winupon.weike.android" { return Some(1005288); }
    if app == "Google Play" || app == "com.android.vending" { return Some(1005289); }
    if app == "TronClass 畅课" || app == "com.wisdomgarden.trpc" { return Some(1005290); }
    if app == "家长慕课" || app == "com.wisdomparents.moocsapp" { return Some(1005291); }
    if app == "音乐计算器:Musicalculator" || app == "com.wjy50.app.MusiCalculator" { return Some(1005292); }
    if app == "精真估二手车" || app == "com.jzg.jzgoto.phone" { return Some(1005293); }
    if app == "HTC音乐" || app == "com.htc.music" { return Some(1005294); }
    if app == "货车帮" || app == "com.wlqq" { return Some(1005295); }
    if app == "货车帮货主" || app == "com.wlqq4consignor" { return Some(1005296); }
    if app == "理财范" || app == "com.wltx.licaifan" { return Some(1005297); }
    if app == "音乐节拍器" || app == "com.gamepans.metronomic" { return Some(1005298); }
    if app == "多点" || app == "com.wm.dmall" { return Some(1005299); }
    if app == "斗鱼TV" || app == "tv.douyu" { return Some(1005300); }
    if app == "坐车网" || app == "com.ourlinc" { return Some(1005301); }
    if app == "欧朋浏览器" || app == "com.oupeng.mini.android" { return Some(1005302); }
    if app == "SoulSense品质生活潮流品牌" || app == "com.android.soulsense" { return Some(1005303); }
    if app == "澎湃新闻" || app == "com.wondertek.paper" { return Some(1005304); }
    if app == "我能理财" || app == "com.wonenglicai.and" { return Some(1005305); }
    if app == "欧朋浏览器极速版" || app == "com.oupeng.browser" { return Some(1005306); }
    if app == "图片剪裁器" || app == "com.android.shell" { return Some(1005307); }
    if app == "栗子直播" || app == "tv.inhand.jclive" { return Some(1005308); }
    if app == "单词日记" || app == "com.wordaily" { return Some(1005309); }
    if app == "宝宝学数字数学启蒙" || app == "com.wordtiger.babyshuzi" { return Some(1005310); }
    if app == "宝宝学数字游戏" || app == "com.wordtiger.shuziGame" { return Some(1005311); }
    if app == "中草药宝典" || app == "com.worker.junjun.zcybd" { return Some(1005312); }
    if app == "小学六年级英语上册" || app == "com.jxdmxxw.tvw" { return Some(1005313); }
    if app == "我型穿衣搭配" || app == "com.woxingshiyi.app" { return Some(1005314); }
    if app == "红杉单词王" || app == "com.woxue.app" { return Some(1005315); }
    if app == "电竞捕鱼单机版" || app == "com.woyun.djbygame.danji.uc" { return Some(1005316); }
    if app == "北京现代bluemembers" || app == "com.wrd" { return Some(1005317); }
    if app == "赶集懒人找房" || app == "com.ganji.android.garield" { return Some(1005318); }
    if app == "Learn English" || app == "com.wsi.wallstreetenglish.nsechina.prodchina" { return Some(1005319); }
    if app == "豆瓣音乐人" || app == "com.douban.artist" { return Some(1005320); }
    if app == "便签" || app == "com.android.notes" { return Some(1005321); }
    if app == "梧桐阅读" || app == "com.wtzw.reader" { return Some(1005322); }
    if app == "58同城" || app == "com.wuba" { return Some(1005323); }
    if app == "家家支付" || app == "com.android.lft" { return Some(1005324); }
    if app == "白菜二手车" || app == "com.wuba.ercar" { return Some(1005325); }
    if app == "速贷宝" || app == "com.hrtx.sudaibao" { return Some(1005326); }
    if app == "安全浏览器" || app == "com.jx.safebrowser" { return Some(1005327); }
    if app == "盒马驾到" || app == "com.wudaokou.flyingfish" { return Some(1005328); }
    if app == "e海通财" || app == "com.android.haitong" { return Some(1005329); }
    if app == "私密浏览器" || app == "com.jx.privatebrowser" { return Some(1005330); }
    if app == "武汉停车" || app == "com.wuhanparking.whtc" { return Some(1005331); }
    if app == "神鸟资讯" || app == "com.wujia.birdnews" { return Some(1005332); }
    if app == "悟空找房" || app == "com.wukong.ua" { return Some(1005333); }
    if app == "悟空优选" || app == "com.wukonglicai.app" { return Some(1005334); }
    if app == "极速浏览器" || app == "com.jx.minibrowser" { return Some(1005335); }
    if app == "高速浏览器" || app == "com.jx.fastbrowser" { return Some(1005336); }
    if app == "我的咖啡店" || app == "com.melesta.coffeeshop" { return Some(1005337); }
    if app == "e学" || app == "com.juziwl.xiaoxin" { return Some(1005338); }
    if app == "吉柚小视频" || app == "com.android.genchaung.jishaoshortvideo" { return Some(1005339); }
    if app == "e学云" || app == "com.juziwl.exuecloud.parent" { return Some(1005340); }
    if app == "全国汽车票" || app == "com.wxws.myticket" { return Some(1005341); }
    if app == "阅听文学" || app == "com.wyfc.booknovel" { return Some(1005342); }
    if app == "看小说听书" || app == "com.wyfc.itingtxt" { return Some(1005343); }
    if app == "File Manager" || app == "com.android.filemanager" { return Some(1005344); }
    if app == "写书小说阅读" || app == "com.wyfc.lovenovel" { return Some(1005345); }
    if app == "桔子浏览器" || app == "com.juzi.browser" { return Some(1005346); }
    if app == "一直播" || app == "tv.xiaoka.live" { return Some(1005347); }
    if app == "WiFi看头条" || app == "com.juwan.market" { return Some(1005348); }
    if app == "军事武器" || app == "com.justoper.weapins.mil" { return Some(1005349); }
    if app == "GiWiFi校园助手" || app == "com.gbcom.gwifi.school" { return Some(1005350); }
    if app == "捕鱼无双OL" || app == "com.junyou.buyu.uc" { return Some(1005351); }
    if app == "蜜柚浏览器" || app == "com.melon.browser" { return Some(1005352); }
    if app == "WiFi连接助手" || app == "com.gctec.wifibox" { return Some(1005353); }
    if app == "招财猫理财" || app == "com.x1.ui" { return Some(1005354); }
    if app == "小牛借呗" || app == "com.x2427724635.nmg" { return Some(1005355); }
    if app == "掌通宝家长版" || app == "com.xby.ztb.android.parent" { return Some(1005356); }
    if app == "爱卡汽车" || app == "com.xcar.activity" { return Some(1005357); }
    if app == "谷歌浏览器Google Chrome" || app == "com.android.chrome" { return Some(1005358); }
    if app == "虹米浏览器" || app == "com.android.ch.browser" { return Some(1005359); }
    if app == "新东方" || app == "com.xdf.pocket" { return Some(1005360); }
    if app == "小学英语三年级上册" || app == "air.com.byebyer.renjiaosanqi3A1" { return Some(1005361); }
    if app == "学而思云学习" || app == "com.xes.cloudlearning" { return Some(1005362); }
    if app == "学而思" || app == "com.xes.jazhanghui.activity" { return Some(1005363); }
    if app == "学而思老师" || app == "com.xes.jazhanghui.teacher.activity" { return Some(1005364); }
    if app == "今日军事" || app == "com.junshi.jinrijunshi" { return Some(1005365); }
    if app == "本来生活" || app == "com.android.benlailife.activity" { return Some(1005366); }
    if app == "幸福家" || app == "com.xfj.customer" { return Some(1005367); }
    if app == "女人衣服穿搭" || app == "com.junling.gard" { return Some(1005368); }
    if app == "券妈妈优惠券" || app == "com.android.app.quanmama" { return Some(1005369); }
    if app == "先锋影音" || app == "com.xfyy.yax" { return Some(1005370); }
    if app == "西播影音" || app == "com.xghotplay.allplayss" { return Some(1005371); }
    if app == "翼先锋影音" || app == "com.xghotplay.bluedo" { return Some(1005372); }
    if app == "西瓜影音播放器" || app == "com.xgyybfq.uc.va" { return Some(1005373); }
    if app == "淘色秀场" || app == "com.jumei.ui" { return Some(1005374); }
    if app == "今日水印相机" || app == "com.xhey.xcamera" { return Some(1005375); }
    if app == "羚萌直播" || app == "com.julun.lingmeng" { return Some(1005376); }
    if app == "下厨房" || app == "com.xiachufang" { return Some(1005377); }
    if app == "阿布睡前故事" || app == "com.android.abustory" { return Some(1005378); }
    if app == "家庭理财" || app == "com.jtlctv.yyl" { return Some(1005379); }
    if app == "PDF阅读器ezPDF" || app == "udk.android.reader" { return Some(1005380); }
    if app == "英语消消乐" || app == "com.melostudio.wordcrush" { return Some(1005381); }
    if app == "及时用车专车司机端" || app == "com.jsyc.driver" { return Some(1005382); }
    if app == "江苏移动掌上营业厅" || app == "com.jsmcc" { return Some(1005383); }
    if app == "16WiFi" || app == "com.and.colourmedia.ewifi.nanjing" { return Some(1005384); }
    if app == "微探觅恋交友" || app == "com.xianmoliao.wtmljy" { return Some(1005385); }
    if app == "中国老黄历" || app == "com.ancient.calendar" { return Some(1005386); }
    if app == "智学家长端" || app == "com.xiao.parent" { return Some(1005387); }
    if app == "小学英语五年级上册" || app == "air.com.byebyer.renjiao5a" { return Some(1005388); }
    if app == "KK" || app == "com.melot.meshow" { return Some(1005389); }
    if app == "鲤鱼辅导" || app == "com.xiaohaizi.ui" { return Some(1005390); }
    if app == "抓钱猫理财" || app == "com.xiaojinniu.smalltaurus" { return Some(1005391); }
    if app == "滴滴外卖" || app == "com.xiaojukeji.didi.customer" { return Some(1005392); }
    if app == "米聊" || app == "com.xiaomi.channel" { return Some(1005393); }
    if app == "智慧幼儿园家长版" || app == "com.jshjw.preschool.mobile" { return Some(1005394); }
    if app == "智慧教育家长" || app == "com.jshjw.eschool.mobile" { return Some(1005395); }
    if app == "桔色直播" || app == "com.orange.live" { return Some(1005396); }
    if app == "阳光出行车主端" || app == "com.jryg.driver" { return Some(1005397); }
    if app == "Silk浏览器" || app == "com.amazon.cloud9" { return Some(1005398); }
    if app == "小米应用商店" || app == "com.xiaomi.market" { return Some(1005399); }
    if app == "小爱语音引擎" || app == "com.xiaomi.mibrain.speech" { return Some(1005400); }
    if app == "小米移动" || app == "com.xiaomi.mimobile" { return Some(1005401); }
    if app == "小米WIFI" || app == "com.xiaomi.mishare" { return Some(1005402); }
    if app == "小米便签" || app == "com.xiaomi.notes" { return Some(1005403); }
    if app == "小米生活" || app == "com.xiaomi.o2o" { return Some(1005404); }
    if app == "小米商城" || app == "com.xiaomi.shop" { return Some(1005405); }
    if app == "米家" || app == "com.xiaomi.smarthome" { return Some(1005406); }
    if app == "豆果家常菜谱" || app == "com.douguo.homerecipe" { return Some(1005407); }
    if app == "WiFi测速大师" || app == "com.xiaonanjiao.speedtest" { return Some(1005408); }
    if app == "拉勾招聘" || app == "com.alpha.lagouapk" { return Some(1005409); }
    if app == "小鹏汽车" || app == "com.xiaopeng.mycarinfo" { return Some(1005410); }
    if app == "爱乐奇家长" || app == "com.alo7.axt.parent" { return Some(1005411); }
    if app == "小说520" || app == "com.xiaoshuo520.reader" { return Some(1005412); }
    if app == "宙斯浏览器" || app == "com.allzeus.browser" { return Some(1005413); }
    if app == "华为学习" || app == "com.gearedu.honorstudy.huawei" { return Some(1005414); }
    if app == "火钱理财" || app == "com.allyoubank.huoq" { return Some(1005415); }
    if app == "全民优惠" || app == "com.allfree.cc" { return Some(1005416); }
    if app == "浙江联通" || app == "uni.jdxt.app" { return Some(1005417); }
    if app == "小蚁行车记录仪" || app == "com.xiaoyi.car.camera" { return Some(1005418); }
    if app == "高途学院" || app == "com.genshuixue.student" { return Some(1005419); }
    if app == "家有学霸" || app == "com.xiaoyu.com.xueba" { return Some(1005420); }
    if app == "小学课程名师辅导" || app == "com.xiaoyu.xiaoxue" { return Some(1005421); }
    if app == "小欧助手" || app == "com.oppo.speechassist" { return Some(1005422); }
    if app == "君融理财" || app == "com.jrd.loan" { return Some(1005423); }
    if app == "免费小说追书" || app == "com.jr.xiaoandushu" { return Some(1005424); }
    if app == "爱淘宝" || app == "com.alimama.bluestone" { return Some(1005425); }
    if app == "喜马拉雅极速版" || app == "com.ximalaya.ting.lite" { return Some(1005426); }
    if app == "优信新车" || app == "com.xin.dbm" { return Some(1005427); }
    if app == "新车评" || app == "com.xincheping.xincheping" { return Some(1005428); }
    if app == "巅峰捕鱼" || app == "com.alijiuyou.dfby.uc" { return Some(1005429); }
    if app == "星座进货宝" || app == "com.alidao.sjxz" { return Some(1005430); }
    if app == "医院预约挂号" || app == "com.xinglin.health_assistant.all" { return Some(1005431); }
    if app == "起名大师" || app == "com.ggeye.babymingzi" { return Some(1005432); }
    if app == "河南移动智慧生活" || app == "com.xinhang.mobileclient" { return Some(1005433); }
    if app == "豆果美食" || app == "com.douguo.recipe" { return Some(1005434); }
    if app == "e租车管理" || app == "com.jpdfh.video" { return Some(1005435); }
    if app == "OPPO软件商店" || app == "com.oppo.market" { return Some(1005436); }
    if app == "运满满司机" || app == "com.xiwei.logistics" { return Some(1005437); }
    if app == "万象贷小额借贷款" || app == "com.xiyoukeji.xinhuahua" { return Some(1005438); }
    if app == "音乐标签编辑器" || app == "com.xjcheng.musictageditor" { return Some(1005439); }
    if app == "无损高保真音乐播放器" || app == "com.xjcheng.simlosslessplay" { return Some(1005440); }
    if app == "去你大爷的内置浏览器" || app == "com.xloger.exlink.app" { return Some(1005441); }
    if app == "奥特曼传奇英雄" || app == "com.joym.legendhero.uc" { return Some(1005442); }
    if app == "OPPO桌面" || app == "com.oppo.launcher" { return Some(1005443); }
    if app == "金手指捕鱼" || app == "com.joygame.fish.aligames" { return Some(1005444); }
    if app == "么么直播" || app == "com.memezhibo.android" { return Some(1005445); }
    if app == "玩车之家" || app == "com.xmyunyou.wcd" { return Some(1005446); }
    if app == "天翼WiFi" || app == "com.akazam.android.wlandialer" { return Some(1005447); }
    if app == "PDF阅读器Xodo Docs" || app == "com.xodo.pdf.reader" { return Some(1005448); }
    if app == "金手指捕鱼-千炮版" || app == "com.joygame.fish" { return Some(1005449); }
    if app == "爱又米" || app == "com.aixuedai.axd" { return Some(1005450); }
    if app == "全民漂移狂野飙车" || app == "com.joyfort.merge.car.jrtt" { return Some(1005451); }
    if app == "免费小说城" || app == "com.aishukeem360.ledu" { return Some(1005452); }
    if app == "牛牛汽车" || app == "com.aiqing.niuniuqiche" { return Some(1005453); }
    if app == "日杂相机" || app == "com.mendon.riza" { return Some(1005454); }
    if app == "爱维宝贝" || app == "com.jovision.ivbaby" { return Some(1005455); }
    if app == "备胎好车商家版" || app == "com.aika.dealer" { return Some(1005456); }
    if app == "车主惠" || app == "com.aibaoxian.car.optimus" { return Some(1005457); }
    if app == "oppo相机" || app == "com.oppo.camera" { return Some(1005458); }
    if app == "学而思网校" || app == "com.xueersi.parentsmeeting" { return Some(1005459); }
    if app == "WIFI信号增强神器" || app == "com.ada.app.wifistrengthen" { return Some(1005460); }
    if app == "斗鱼极速版" || app == "com.douyu.rush" { return Some(1005461); }
    if app == "贷款管家" || app == "com.xulu.loanmanager" { return Some(1005462); }
    if app == "阿卡索英语" || app == "com.acadsoc.talkshow" { return Some(1005463); }
    if app == "尚WiFi" || app == "com.xunboda.iwifi" { return Some(1005464); }
    if app == "易用汇" || app == "com.gionee.aora.market" { return Some(1005465); }
    if app == "乐外卖商家" || app == "com.xunjoy.lewaimai.shop" { return Some(1005466); }
    if app == "阿卡索口语秀" || app == "com.acadsoc.learn" { return Some(1005467); }
    if app == "迅雷" || app == "com.xunlei.downloadprovider" { return Some(1005468); }
    if app == "看看视频" || app == "com.xunlei.kankan" { return Some(1005469); }
    if app == "迅雷直播" || app == "com.xunlei.tdlive" { return Some(1005470); }
    if app == "阿卡索少儿英语" || app == "com.acadsoc.english.children" { return Some(1005471); }
    if app == "智慧树网" || app == "com.able.wisdomtree.zs" { return Some(1005472); }
    if app == "Opera Mini web 浏览器" || app == "com.opera.mini.native" { return Some(1005473); }
    if app == "信用牛牛借钱贷款" || app == "com.a520daikuan.doc_xynn" { return Some(1005474); }
    if app == "盎盎理财" || app == "com.xuyao.anganglicai" { return Some(1005475); }
    if app == "金螳螂家装修" || app == "com.goldmantis.app.jia" { return Some(1005476); }
    if app == "陌遇交友" || app == "com.goldmelt.morse" { return Some(1005477); }
    if app == "网络小说家模拟" || app == "com.ZhajiPijiu.Writer" { return Some(1005478); }
    if app == "问学家长端" || app == "com.xweisoft.wx.family" { return Some(1005479); }
    if app == "麦田在线" || app == "com.jkrm.maitian" { return Some(1005480); }
    if app == "Opera Mini浏览器" || app == "com.opera.mini.android" { return Some(1005481); }
    if app == "测测" || app == "com.xxwolo.cc5" { return Some(1005482); }
    if app == "酷爱直播" || app == "com.Tiange.ChatRoom" { return Some(1005483); }
    if app == "相机360" || app == "vStudio.Android.Camera360" { return Some(1005484); }
    if app == "书耽" || app == "com.mengjun.DanNovel" { return Some(1005485); }
    if app == "作业答案搜题神器" || app == "com.eweingyou.vlgawa.aownd" { return Some(1005486); }
    if app == "汽车大全" || app == "com.xyauto.carcenter" { return Some(1005487); }
    if app == "小学英语四年级上册" || app == "air.byebyer.renjiao34A" { return Some(1005488); }
    if app == "钱站" || app == "aiqianjin.jiea" { return Some(1005489); }
    if app == "华为培训" || app == "com.huawei.hedex.mobile.enterprise.training" { return Some(1005490); }
    if app == "家长帮手" || app == "com.open.parentmanager" { return Some(1005491); }
    if app == "平安财富宝" || app == "com.PingAn.CaiFuBao" { return Some(1005492); }
    if app == "腐萌小说" || app == "com.mengjun.fumeng" { return Some(1005493); }
    if app == "领奇理财投资" || app == "com.LQhlw.lingqilicai" { return Some(1005494); }
    if app == "旅游电子合同" || app == "com.goldpalm.e_contract" { return Some(1005495); }
    if app == "ETCP停车" || app == "com.ETCPOwner.yc" { return Some(1005496); }
    if app == "充电评测" || app == "com.gombosdev.ampere" { return Some(1005497); }
    if app == "驿充电" || app == "com.evlink.evcharge" { return Some(1005498); }
    if app == "集结号捕鱼" || app == "com.jjh.Fish.aligames" { return Some(1005499); }
    if app == "蜜桃直播" || app == "com.jj.shows" { return Some(1005500); }
    if app == "洋葱学园 原洋葱数学" || app == "com.yangcong345.android.phone" { return Some(1005501); }
    if app == "贷款计算器" || app == "com.gongju.dkjsq" { return Some(1005502); }
    if app == "要出发周边游" || app == "com.yaochufa.app" { return Some(1005503); }
    if app == "药房网商城" || app == "com.yaofangwang.mall" { return Some(1005504); }
    if app == "捕鱼欢乐炸" || app == "com.yaoji.byhlz.uc" { return Some(1005505); }
    if app == "捕鱼炸翻天" || app == "com.yaoji.yaojibuyu.uc" { return Some(1005506); }
    if app == "妈妈帮" || app == "com.yaya.mmbang" { return Some(1005507); }
    if app == "凹凸租车" || app == "com.Autoyol.auto" { return Some(1005508); }
    if app == "每日红包" || app == "com.yc.mrhb" { return Some(1005509); }
    if app == "小学拼音学习" || app == "com.yc.pinyin" { return Some(1005510); }
    if app == "亿联会议" || app == "com.yealink.vc.mobile" { return Some(1005511); }
    if app == "公平价二手车" || app == "com.gongpingjia" { return Some(1005512); }
    if app == "谷歌新闻和天气" || app == "com.google.android.apps.genie.geniewidget" { return Some(1005513); }
    if app == "谷歌地图" || app == "com.google.android.apps.maps" { return Some(1005514); }
    if app == "肯德基" || app == "com.yek.android.kfc.activitys" { return Some(1005515); }
    if app == "优衣库" || app == "com.yek.android.uniqlo" { return Some(1005516); }
    if app == "啾哩直播" || app == "com.jiulizhibo.phonelive" { return Some(1005517); }
    if app == "来借贷款" || app == "com.jiujiuyun.laijie" { return Some(1005518); }
    if app == "夜嗨直播" || app == "com.yemeizhibo.android" { return Some(1005519); }
    if app == "龙易运势" || app == "com.Astro.UI" { return Some(1005520); }
    if app == "车来了" || app == "com.ygkj.chelaile.standard" { return Some(1005521); }
    if app == "口袋老师" || app == "com.ygtoo" { return Some(1005522); }
    if app == "一伴婚恋相亲交友" || app == "com.yiban1314.yiban" { return Some(1005523); }
    if app == "Google 翻译" || app == "com.google.android.apps.translate" { return Some(1005524); }
    if app == "司机宝" || app == "com.yicai.sijibao" { return Some(1005525); }
    if app == "一起作业" || app == "com.A17zuoye.mobile.homework" { return Some(1005526); }
    if app == "书虫小说电子书" || app == "com.jiubang.bookv4" { return Some(1005527); }
    if app == "小学英语趣配音" || app == "com.himi.english.qupeiyin.xiaoxue" { return Some(1005528); }
    if app == "基金从业随身学" || app == "com.onesoft.app.Tiiku.Duia.JJSSX" { return Some(1005529); }
    if app == "成长守护家长端" || app == "com.jinyuc.pcp.parent" { return Some(1005530); }
    if app == "新闻资讯" || app == "com.yidian.xiaomi" { return Some(1005531); }
    if app == "驿动汽车" || app == "com.yidong.travel.app" { return Some(1005532); }
    if app == "纳米盒" || app == "com.jinxin.namibox" { return Some(1005533); }
    if app == "一兜糖" || app == "com.yidoutang.app" { return Some(1005534); }
    if app == "日历同步服务" || app == "com.google.android.syncadapters.calendar" { return Some(1005535); }
    if app == "极搜浏览器" || app == "cn.zhangyoukeji.browser.jisou" { return Some(1005536); }
    if app == "快票出行" || app == "com.yijin.fastticket" { return Some(1005537); }
    if app == "亿联银行" || app == "com.yilianbank.yilian" { return Some(1005538); }
    if app == "儿童教育游戏" || app == "cn.ytsmwhc.bestmother" { return Some(1005539); }
    if app == "溢米辅导" || app == "com.yimifudao.student.mobile" { return Some(1005540); }
    if app == "YouTube" || app == "com.google.android.youtube" { return Some(1005541); }
    if app == "鲸鱼宝理财" || app == "com.jinr.core" { return Some(1005542); }
    if app == "牛角小说" || app == "com.yincheng.njread" { return Some(1005543); }
    if app == "作业通" || app == "com.jingyou.math" { return Some(1005544); }
    if app == "哈啰" || app == "com.jingyao.easybike" { return Some(1005545); }
    if app == "怡康到家" || app == "cn.yaoking" { return Some(1005546); }
    if app == "十元街" || app == "com.jingdong.secondkill" { return Some(1005547); }
    if app == "58交友" || app == "com.yingyu.huameng" { return Some(1005548); }
    if app == "肯德基宅急送" || app == "com.hp.mit.atmobile.kfc" { return Some(1005549); }
    if app == "小赢理财" || app == "com.yingzt.invest" { return Some(1005550); }
    if app == "内蒙古和校园家长版" || app == "cn.xxt.nm.app" { return Some(1005551); }
    if app == "氢相机" || app == "com.oneplus.camera" { return Some(1005552); }
    if app == "印象笔记" || app == "com.yinxiang" { return Some(1005553); }
    if app == "智行火车票" || app == "com.yipiao" { return Some(1005554); }
    if app == "黄历万年历" || app == "com.yiqi.calendar" { return Some(1005555); }
    if app == "一起学" || app == "com.yiqizuoye.jzt" { return Some(1005556); }
    if app == "一起小学老师" || app == "com.yiqizuoye.teacher" { return Some(1005557); }
    if app == "学习强国" || app == "cn.xuexi.android" { return Some(1005558); }
    if app == "宜人贷款" || app == "com.yirendai.rnloan" { return Some(1005559); }
    if app == "免费小说电子书" || app == "com.yirvana.book" { return Some(1005560); }
    if app == "一手服装批发" || app == "com.yishouapp.fumi" { return Some(1005561); }
    if app == "言情小说吧" || app == "cn.xs8.app" { return Some(1005562); }
    if app == "九库阅读" || app == "com.mengmengda.reader" { return Some(1005563); }
    if app == "金蛋理财" || app == "com.jindan.p2p" { return Some(1005564); }
    if app == "下载浏览器" || app == "com.downloader.browser" { return Some(1005565); }
    if app == "萌推" || app == "com.mengtuiapp.mall" { return Some(1005566); }
    if app == "象司机" || app == "com.yixc.xsj" { return Some(1005567); }
    if app == "快8小视频" || app == "com.yixia.quick8" { return Some(1005568); }
    if app == "译林小学英语" || app == "com.yixinjiang.goodbaba.app.presentation" { return Some(1005569); }
    if app == "好爸爸人教译林纳米盒" || app == "com.yixinjiang.goodbaba.app.presentation.pep" { return Some(1005570); }
    if app == "兔小贝" || app == "com.yixun.org" { return Some(1005571); }
    if app == "新速贷贷款" || app == "cn.xmfengrong.sudai" { return Some(1005572); }
    if app == "指尖钱包" || app == "com.yizhen.ZJQB" { return Some(1005573); }
    if app == "借钱宝分期贷款王" || app == "cn.xingyi.jieqianbao" { return Some(1005574); }
    if app == "爱车在线" || app == "com.jimi.tuqiang.ecar" { return Some(1005575); }
    if app == "轻颜相机" || app == "com.gorgeous.lite" { return Some(1005576); }
    if app == "儿歌多多HD" || app == "com.duoduo.child.storyhd" { return Some(1005577); }
    if app == "车智汇" || app == "com.ym.ecpark.obd" { return Some(1005578); }
    if app == "桃缘交友" || app == "com.ym.taoyuan.chat" { return Some(1005579); }
    if app == "叽里呱啦" || app == "com.jiliguala.niuwa" { return Some(1005580); }
    if app == "荣耀商城" || app == "com.hihonor.vmall" { return Some(1005581); }
    if app == "留影音乐相册制作" || app == "com.okmyapp.liuying" { return Some(1005582); }
    if app == "语音翻译" || app == "com.yo.voicetranslate" { return Some(1005583); }
    if app == "趣头条" || app == "com.jifen.qukan" { return Some(1005584); }
    if app == "wifi密码查看大师" || app == "com.jiesiwangluo.wifi" { return Some(1005585); }
    if app == "服饰美容" || app == "com.yoka.android.portal" { return Some(1005586); }
    if app == "皮皮搞笑" || app == "cn.xiaochuankeji.zuiyouLite" { return Some(1005587); }
    if app == "仓鼠阅读" || app == "com.yokong.bookfree" { return Some(1005588); }
    if app == "易到车主端" || app == "com.yongche" { return Some(1005589); }
    if app == "易到用车" || app == "com.yongche.android" { return Some(1005590); }
    if app == "永辉买菜" || app == "com.yonghui.freshdelivery" { return Some(1005591); }
    if app == "用钱宝" || app == "com.yongqianbao.credit" { return Some(1005592); }
    if app == "车生活" || app == "com.yongyou" { return Some(1005593); }
    if app == "花花钱包借钱贷款" || app == "com.jie.jieqiandaikuan" { return Some(1005594); }
    if app == "阅读神器" || app == "com.jie.bookreader" { return Some(1005595); }
    if app == "世纪佳缘" || app == "com.jiayuan" { return Some(1005596); }
    if app == "有车以后" || app == "com.youcheyihou.iyoursuv" { return Some(1005597); }
    if app == "最右" || app == "cn.xiaochuankeji.tieba" { return Some(1005598); }
    if app == "有道四六级" || app == "com.youdao.cet" { return Some(1005599); }
    if app == "有道精品课" || app == "com.youdao.course" { return Some(1005600); }
    if app == "有道口语" || app == "com.youdao.crackingenglish" { return Some(1005601); }
    if app == "网易有道词典" || app == "com.youdao.dict" { return Some(1005602); }
    if app == "有道少儿词典" || app == "com.youdao.kiddict" { return Some(1005603); }
    if app == "有道云笔记" || app == "com.youdao.note" { return Some(1005604); }
    if app == "有道翻译官" || app == "com.youdao.translator" { return Some(1005605); }
    if app == "旅行世界" || app == "com.jiayouya.travel" { return Some(1005606); }
    if app == "单词城堡" || app == "com.yougan233.wordjourney" { return Some(1005607); }
    if app == "理财计算器" || app == "com.youjin360.financialcalculator" { return Some(1005608); }
    if app == "来疯直播" || app == "com.youku.crazytogether" { return Some(1005609); }
    if app == "图样单词" || app == "wordremember.hxx.com.remember" { return Some(1005610); }
    if app == "优酷电视助手" || app == "com.youku.smart.assistant" { return Some(1005611); }
    if app == "优酷大屏幕 for Pad" || app == "com.youku.tv" { return Some(1005612); }
    if app == "优酷 VR Pro" || app == "com.youku.vr" { return Some(1005613); }
    if app == "优酷VR" || app == "com.youku.vr.lite" { return Some(1005614); }
    if app == "盟主直播" || app == "com.mengzhu.app" { return Some(1005615); }
    if app == "儿童音乐游戏乐园" || app == "cn.woosoft.kids.music" { return Some(1005616); }
    if app == "人人韩剧TV" || app == "cn.wkyo.rrhjtv.classic" { return Some(1005617); }
    if app == "微鲤免费小说" || app == "cn.weli.novel" { return Some(1005618); }
    if app == "证券从业资格准题库" || app == "cn.wangxiao.zqzhuntiku" { return Some(1005619); }
    if app == "悟空租车" || app == "com.jiaoyinbrother.monkeyking" { return Some(1005620); }
    if app == "荟萃浏览器" || app == "cn.uujian.browser" { return Some(1005621); }
    if app == "万年历" || app == "com.youloft.calendar" { return Some(1005622); }
    if app == "万年历黄历日历" || app == "com.youloft.calendar.almanac" { return Some(1005623); }
    if app == "竹兜育儿" || app == "com.youmei.zhudou" { return Some(1005624); }
    if app == "穿衣助手" || app == "com.yourdream.app.android" { return Some(1005625); }
    if app == "听书有声免费小说" || app == "com.yousheng.tingshushenqi" { return Some(1005626); }
    if app == "乐书小说" || app == "com.youshuge.happybook" { return Some(1005627); }
    if app == "巴啦啦魔法水晶鞋" || app == "cn.ultralisk.gameapp.game06" { return Some(1005628); }
    if app == "OppoAR" || app == "com.hiscene.app" { return Some(1005629); }
    if app == "方舟健客网上药店" || app == "com.jiankecom.jiankemall" { return Some(1005630); }
    if app == "人人捕鱼" || app == "com.youxila.renrenbuyu.aligames" { return Some(1005631); }
    if app == "育学园" || app == "com.drcuiyutao.babyhealth" { return Some(1005632); }
    if app == "有缘网" || app == "com.youyuan.yyhl" { return Some(1005633); }
    if app == "WiFi万能管理器" || app == "com.youzhi.wifimanage" { return Some(1005634); }
    if app == "疆小哥外卖" || app == "com.jiangxiaoge.waimai" { return Some(1005635); }
    if app == "一起学网校" || app == "com.yqxue.yqxue" { return Some(1005636); }
    if app == "千影浏览器" || app == "com.yr.browser" { return Some(1005637); }
    if app == "枕阅小说" || app == "com.yr.qmzs" { return Some(1005638); }
    if app == "趣新闻" || app == "com.ys.news" { return Some(1005639); }
    if app == "途虎商户" || app == "cn.tuhu.merchant" { return Some(1005640); }
    if app == "花梨阅读" || app == "cn.touchv.a3Zau3" { return Some(1005641); }
    if app == "齐家装修" || app == "com.jia.decoration" { return Some(1005642); }
    if app == "封面新闻" || app == "cn.thecover.www.covermedia" { return Some(1005643); }
    if app == "紫微大师星座算命" || app == "cn.taxen.ziwei" { return Some(1005644); }
    if app == "声吧语音聊天交友" || app == "cn.tantady.tt" { return Some(1005645); }
    if app == "WiFi Service" || app == "com.jhj.dev.wifi" { return Some(1005646); }
    if app == "借花花贷款" || app == "com.jhh" { return Some(1005647); }
    if app == "人教小学数学二下" || app == "com.nwoolf.xy.p522" { return Some(1005648); }
    if app == "订票助手" || app == "cn.suanya.train" { return Some(1005649); }
    if app == "石头理财" || app == "cn.stlc.app" { return Some(1005650); }
    if app == "良友学院" || app == "com.ytrain.liangyuan" { return Some(1005651); }
    if app == "掌通家园园丁" || app == "com.yuanding.seebaby" { return Some(1005652); }
    if app == "扫描宝" || app == "com.yuanhuize.smb" { return Some(1005653); }
    if app == "玖富万卡" || app == "com.jfbank.wanka" { return Some(1005654); }
    if app == "亿点连接万能WiFi钥匙" || app == "com.jf.wifihelper" { return Some(1005655); }
    if app == "享聊" || app == "cn.sharesmile.enjoychat" { return Some(1005656); }
    if app == "汇充电" || app == "com.hooenergy.hoocharge" { return Some(1005657); }
    if app == "莫愁花贷款" || app == "com.yuecai.mochouhua" { return Some(1005658); }
    if app == "悦动浏览器" || app == "com.yuedong.browser" { return Some(1005659); }
    if app == "PEP小学英语六下" || app == "com.nwoolf.xy.p362" { return Some(1005660); }
    if app == "旅游帮" || app == "com.gsy.video" { return Some(1005661); }
    if app == "全民交友" || app == "com.yuehui.jiaoyou" { return Some(1005662); }
    if app == "宝宝学数字加法" || app == "com.gtsoft.KidMath" { return Some(1005663); }
    if app == "车速拍" || app == "com.guazi.android.chesupai" { return Some(1005664); }
    if app == "毛豆新车" || app == "com.guazi.newcar" { return Some(1005665); }
    if app == "金斧子基金" || app == "com.gunxueqiu.activity" { return Some(1005666); }
    if app == "广东和教育" || app == "cn.qtone.xxt.guangdong" { return Some(1005667); }
    if app == "全本免费热门小说" || app == "cn.qbmfrmxs.reader" { return Some(1005668); }
    if app == "元气阅读" || app == "com.yuewen.yqacg" { return Some(1005669); }
    if app == "月芽阅读学生端" || app == "com.yueya.yuedu" { return Some(1005670); }
    if app == "PYRO音乐" || app == "cn.pyromusic.pyro" { return Some(1005671); }
    if app == "PEP小学英语五下" || app == "com.nwoolf.xy.p352" { return Some(1005672); }
    if app == "雨见浏览器" || app == "com.yujian.ResideMenuDemo" { return Some(1005673); }
    if app == "育镜家长" || app == "com.yujingparent.onetargetclient" { return Some(1005674); }
    if app == "微赞直播" || app == "cn.pengxun.vzanlive" { return Some(1005675); }
    if app == "PEP小学英语四下" || app == "com.nwoolf.xy.p342" { return Some(1005676); }
    if app == "云鸟司机" || app == "com.yunniaohuoyun.driver" { return Some(1005677); }
    if app == "国际妈咪" || app == "com.guojimami.app" { return Some(1005678); }
    if app == "麻雀浏览器" || app == "cn.pear.browser" { return Some(1005679); }
    if app == "PEP小学英语四上" || app == "com.nwoolf.xy.p341" { return Some(1005680); }
    if app == "企业浏览器" || app == "com.yunshipei.enterplorer" { return Some(1005681); }
    if app == "车行无忧查违章" || app == "cn.okek.chexingwuyou" { return Some(1005682); }
    if app == "优借" || app == "com.yuntu.jkzjpro" { return Some(1005683); }
    if app == "好分数" || app == "com.yunxiao.haofenshu" { return Some(1005684); }
    if app == "好分数家长版" || app == "com.yunxiao.hfs4p" { return Some(1005685); }
    if app == "虫虫钢琴" || app == "com.yusi.chongchong" { return Some(1005686); }
    if app == "nubia浏览器" || app == "cn.nubia.browser" { return Some(1005687); }
    if app == "国金宝理财" || app == "com.guojinbao.app" { return Some(1005688); }
    if app == "资讯宝" || app == "cn.ninetwoapp.news" { return Some(1005689); }
    if app == "爱车生活2" || app == "com.yw.acsh" { return Some(1005690); }
    if app == "Gofun出行" || app == "com.gvsoft.gofun" { return Some(1005691); }
    if app == "鱼丸捕鱼大作战" || app == "com.ywyx.ywyx.uc" { return Some(1005692); }
    if app == "一起来走路" || app == "com.yx.yqlzl" { return Some(1005693); }
    if app == "狮吼直播" || app == "com.yxd.live" { return Some(1005694); }
    if app == "兔小贝儿歌" || app == "com.yxeee.tuxiaobei" { return Some(1005695); }
    if app == "语文同步学" || app == "com.yxjy.chinesestudy" { return Some(1005696); }
    if app == "100教育" || app == "com.yy.android.tutor.student" { return Some(1005697); }
    if app == "格雷盒子家长端" || app == "com.gwchina.lssw.parent" { return Some(1005698); }
    if app == "捕鱼游戏厅" || app == "com.yy.dwyxt.uc" { return Some(1005699); }
    if app == "ME直播模拟器" || app == "com.yy.ourtimes.jcsj" { return Some(1005700); }
    if app == "小学英语课本同步点读" || app == "com.yy.xxyykbtbdd" { return Some(1005701); }
    if app == "YY信用" || app == "com.yycreditrn" { return Some(1005702); }
    if app == "创富CFD贵金属期货" || app == "com.gwtsz.gts2.cf" { return Some(1005703); }
    if app == "Uki" || app == "cn.neoclub.uki" { return Some(1005704); }
    if app == "音乐帮" || app == "com.yyekt" { return Some(1005705); }
    if app == "汽车违章查询" || app == "cn.mucang.kaka.android" { return Some(1005706); }
    if app == "小猪二手车" || app == "cn.mucang.drunkremind.android" { return Some(1005707); }
    if app == "车友头条" || app == "cn.mucang.android.qichetoutiao" { return Some(1005708); }
    if app == "平行进口车之家" || app == "cn.mucang.android.parallelvehicle" { return Some(1005709); }
    if app == "115浏览器" || app == "com.yyw.fastbrowser" { return Some(1005710); }
    if app == "汽车报价之家" || app == "cn.mucang.android.kaka.accountbook" { return Some(1005711); }
    if app == "56888一点通司机" || app == "com.gxt.ydt.driver" { return Some(1005712); }
    if app == "多融理财" || app == "com.yz.dr_app" { return Some(1005713); }
    if app == "神奇浏览器" || app == "com.z28j.feel" { return Some(1005714); }
    if app == "Firefox火狐浏览器简体中文版" || app == "cn.mozilla.firefox" { return Some(1005715); }
    if app == "知户型" || app == "com.zbj.zhouse" { return Some(1005716); }
    if app == "每日优鲜" || app == "cn.missfresh.application" { return Some(1005717); }
    if app == "麦芽贷" || app == "com.zd.myd" { return Some(1005718); }
    if app == "信用卡贷款借款" || app == "cn.mifengkong.twotypeloan" { return Some(1005719); }
    if app == "容易贷小额贷款" || app == "com.zelyy.recommend" { return Some(1005720); }
    if app == "起名取名" || app == "com.jason.measuringtools.naming" { return Some(1005721); }
    if app == "车秘" || app == "com.gzcarmi" { return Some(1005722); }
    if app == "儿童英语ABC" || app == "com.gzhl.kidsabc" { return Some(1005723); }
    if app == "买房记" || app == "com.zf.dsmfj" { return Some(1005724); }
    if app == "车质网" || app == "com.zgczw.chezhiwang" { return Some(1005725); }
    if app == "速贷贷款" || app == "cn.mifengkong.huaxiaapp" { return Some(1005726); }
    if app == "乐看影音播放器" || app == "com.zh_weir.videoplayer" { return Some(1005727); }
    if app == "雅思单词" || app == "com.zhan.ieltsword" { return Some(1005728); }
    if app == "Evermemo · 印象便签" || app == "com.zhan_dui.evermemo" { return Some(1005729); }
    if app == "95约吧直播" || app == "com.gzlok.gamemarket.yeyue.show" { return Some(1005730); }
    if app == "免费小说全本阅读" || app == "cn.mfxsqbyd.reader" { return Some(1005731); }
    if app == "95爱约直播" || app == "com.gzlok.gamemarket.yueai.show" { return Some(1005732); }
    if app == "掌心浏览器" || app == "com.zhangmen.browser" { return Some(1005733); }
    if app == "掌门好家长" || app == "com.zhangmen.parents.am" { return Some(1005734); }
    if app == "掌读看书小说阅读器" || app == "com.zhangyou.plamreading" { return Some(1005735); }
    if app == "翻书阅读" || app == "com.zhangyue.iReader" { return Some(1005736); }
    if app == "掌阅听书" || app == "com.zhangyue.tingreader" { return Some(1005737); }
    if app == "掌中小说书城" || app == "com.zhangzhongyun.inovel" { return Some(1005738); }
    if app == "掌中云小说" || app == "com.zhangzhongyun.store" { return Some(1005739); }
    if app == "博车网" || app == "com.menhoo.sellcars" { return Some(1005740); }
    if app == "快爽小说" || app == "com.dreamagic.read" { return Some(1005741); }
    if app == "睿教育家长版" || app == "com.gzlx.eduparent" { return Some(1005742); }
    if app == "浙江新闻" || app == "com.zhejiangdaily" { return Some(1005743); }
    if app == "珍爱" || app == "com.zhenai.android" { return Some(1005744); }
    if app == "广州妈妈网" || app == "com.gzmama.activity" { return Some(1005745); }
    if app == "桔子快贷" || app == "com.mesio.loan" { return Some(1005746); }
    if app == "小诺理财" || app == "com.nuoyuan.sp2p" { return Some(1005747); }
    if app == "爱小说" || app == "com.iyd.reader.ReadingJoy.aixiaoshuo" { return Some(1005748); }
    if app == "233乐园" || app == "com.meta.box" { return Some(1005749); }
    if app == "制服美女直播" || app == "com.zhifu.live" { return Some(1005750); }
    if app == "Boom音乐" || app == "cn.kuwo.boom" { return Some(1005751); }
    if app == "家长盒子" || app == "cn.knowbox.rc.parent" { return Some(1005752); }
    if app == "PEP小学英语三年级上册" || app == "com.zhihui.pep.primary.up" { return Some(1005753); }
    if app == "PEP人教版小学英语三年级下" || app == "com.zhihui.pep.three.down" { return Some(1005754); }
    if app == "指尖省钱xian" || app == "com.zhijiangsllq" { return Some(1005755); }
    if app == "鲨鱼浏览器" || app == "com.zhijianzhuoyue.sharkbrowser" { return Some(1005756); }
    if app == "知康掌上体检" || app == "com.zhikang.heathdetect" { return Some(1005757); }
    if app == "2Kids学拼音" || app == "cn.kidsapp.Pinyin" { return Some(1005758); }
    if app == "男人帮" || app == "com.haibao.forman" { return Some(1005759); }
    if app == "出国旅游英语" || app == "com.honghesoft.travelenglish" { return Some(1005760); }
    if app == "WiFi加速助手" || app == "com.zhishang.wifi" { return Some(1005761); }
    if app == "WiFi极速助手" || app == "com.zhishang.wifiSpeed" { return Some(1005762); }
    if app == "快乐疫苗" || app == "com.zhite.cvp" { return Some(1005763); }
    if app == "付啦信用卡管家" || app == "com.ixiye.fula.pro" { return Some(1005764); }
    if app == "女鞋货源" || app == "cn.k3.k3" { return Some(1005765); }
    if app == "音乐铃声制作" || app == "com.hfhengrui.jianji" { return Some(1005766); }
    if app == "ANT无线服务" || app == "com.dsi.ant.service.socket" { return Some(1005767); }
    if app == "233小游戏" || app == "com.meta.xyx" { return Some(1005768); }
    if app == "球鞋发售日历" || app == "cn.jugame.shoeking" { return Some(1005769); }
    if app == "军事TV" || app == "cn.js7tv.jstv" { return Some(1005770); }
    if app == "日日顺快线司机端" || app == "com.haier.rrs.driver" { return Some(1005771); }
    if app == "JJ捕鱼" || app == "cn.jj.fish" { return Some(1005772); }
    if app == "巴乐兔租房" || app == "com.haimai.baletu" { return Some(1005773); }
    if app == "周公解梦最新最全" || app == "com.zhouGongJieMeng" { return Some(1005774); }
    if app == "少儿趣配音" || app == "com.ishowedu.child.peiyin" { return Some(1005775); }
    if app == "华为AR地图" || app == "com.huawei.hereto" { return Some(1005776); }
    if app == "高考蜂背" || app == "com.zhouyue.Bee" { return Some(1005777); }
    if app == "音乐相册" || app == "com.zhouzining.yyxc" { return Some(1005778); }
    if app == "家居风水学" || app == "com.irobin" { return Some(1005779); }
    if app == "JJ斗地主" || app == "cn.jj" { return Some(1005780); }
    if app == "诸葛找房" || app == "com.zhugezhaofang" { return Some(1005781); }
    if app == "海风智学中心" || app == "com.hfjy.LearningCenter" { return Some(1005782); }
    if app == "优学派家长管理" || app == "com.noahph.pad_mng" { return Some(1005783); }
    if app == "问真八字" || app == "com.zhulu.zhulubazipaipan" { return Some(1005784); }
    if app == "e养车" || app == "com.zhuofu" { return Some(1005785); }
    if app == "小说免费大全" || app == "com.ireadercity.xsmfdq" { return Some(1005786); }
    if app == "红娘视频相亲" || app == "com.zhuyu.hongniang" { return Some(1005787); }
    if app == "贷款助手" || app == "cn.jiujiudai.H5291D269" { return Some(1005788); }
    if app == "酒店达人" || app == "cn.ikamobile.hotelfinder" { return Some(1005789); }
    if app == "书香小说大全" || app == "com.ireadercity" { return Some(1005790); }
    if app == "微胎心" || app == "cn.ihealthbaby.weitaixin" { return Some(1005791); }
    if app == "浙江预约挂号" || app == "com.zjrc.yygh" { return Some(1005792); }
    if app == "车行168" || app == "com.zjw.chehang168" { return Some(1005793); }
    if app == "3E口语" || app == "com.zjwocai.threeemobile" { return Some(1005794); }
    if app == "二手车估价" || app == "cn.iautos.android.app.bluerocktor" { return Some(1005795); }
    if app == "云课堂智慧职教" || app == "com.zjy.ykt" { return Some(1005796); }
    if app == "AA出行司机" || app == "cn.hznetroute.AAdriver" { return Some(1005797); }
    if app == "荣耀亲选" || app == "cn.honor.qinxuan" { return Some(1005798); }
    if app == "酷WiFi" || app == "com.zlianjie.coolwifi" { return Some(1005799); }
    if app == "现金贷贷款" || app == "com.zlkj.xianjindai" { return Some(1005800); }
    if app == "小额贷款" || app == "com.zlkj.xiaoedaikuan" { return Some(1005801); }
    if app == "信用白条贷款" || app == "com.zlkj.xinyongbaitiao" { return Some(1005802); }
    if app == "乐信阅读" || app == "cn.hanwenbook.lexin" { return Some(1005803); }
    if app == "啄米理财" || app == "com.zmlc.zm_app" { return Some(1005804); }
    if app == "掌门1对1HD" || app == "com.zmlearn.chat.apad" { return Some(1005805); }
    if app == "掌门1对1辅导" || app == "com.zmlearn.course.am" { return Some(1005806); }
    if app == "子腾园" || app == "com.zmsoft.forwatch" { return Some(1005807); }
    if app == "SCP基金会维基" || app == "com.nn5n.scp.foundation.db.online" { return Some(1005808); }
    if app == "口袋贵金属" || app == "org.sojex.finance" { return Some(1005809); }
    if app == "A佳教育" || app == "com.znxunzhi" { return Some(1005810); }
    if app == "55海淘" || app == "com.haitao" { return Some(1005811); }
    if app == "中国体育彩票" || app == "cn.gov.lottery" { return Some(1005812); }
    if app == "走路赚" || app == "com.zouluzhuan.bayue" { return Some(1005813); }
    if app == "走路赚钱" || app == "com.zouluzq.app" { return Some(1005814); }
    if app == "永辉超市" || app == "cn.fzfx.luop.yhcs" { return Some(1005815); }
    if app == "极速借钱贷款借款" || app == "com.zqxq.jisujieqian" { return Some(1005816); }
    if app == "作业答案" || app == "com.zqy.zuoyedaan" { return Some(1005817); }
    if app == "有声小说大全" || app == "cn.feisu1229.youshengxiaoshuodaquan" { return Some(1005818); }
    if app == "飞虎直播" || app == "cn.feihutv.zhibofeihu" { return Some(1005819); }
    if app == "海淘网" || app == "com.haitao.mapp" { return Some(1005820); }
    if app == "英语六级君" || app == "com.duia.cet6" { return Some(1005821); }
    if app == "证券从业资格对题库" || app == "com.duia.offline_zq_qbank" { return Some(1005822); }
    if app == "中华万年历日历" || app == "cn.etouch.ecalendar" { return Some(1005823); }
    if app == "钱路理财" || app == "com.hangzhou.qianlu" { return Some(1005824); }
    if app == "美周直播" || app == "com.dundun.live" { return Some(1005825); }
    if app == "借了吗" || app == "com.iot.glb" { return Some(1005826); }
    if app == "挖财宝" || app == "com.hangzhoucaimi.financial" { return Some(1005827); }
    if app == "阳光快药" || app == "com.ionicframework.DLSUNMedical" { return Some(1005828); }
    if app == "九秀语音" || app == "com.ninexiu.xjj" { return Some(1005829); }
    if app == "e代驾司机端" || app == "cn.edaijia.android.driverclient" { return Some(1005830); }
    if app == "九秀直播" || app == "com.ninexiu.sixninexiu" { return Some(1005831); }
    if app == "租租车" || app == "com.zuzuChe" { return Some(1005832); }
    if app == "海象理财" || app == "com.hanya.financing" { return Some(1005833); }
    if app == "在线直播课堂" || app == "com.zxkt.eduol" { return Some(1005834); }
    if app == "好停车" || app == "com.innotek.goodparking" { return Some(1005835); }
    if app == "球鞋指数" || app == "com.dunkhome.sindex" { return Some(1005836); }
    if app == "车轮" || app == "cn.eclicks.wzsearch" { return Some(1005837); }
    if app == "中国移动云南" || app == "com.haobo.huilife" { return Some(1005838); }
    if app == "作业帮口算" || app == "com.zybang.parent" { return Some(1005839); }
    if app == "中医药宝典" || app == "com.zybd.zdcpzydq" { return Some(1005840); }
    if app == "奥特曼系列OL" || app == "com.metek.ultraman.uc" { return Some(1005841); }
    if app == "好车无忧二手车" || app == "com.haoche51.buyerapp" { return Some(1005842); }
    if app == "校园家长版" || app == "com.inch.publicfamily" { return Some(1005843); }
    if app == "车轮社区" || app == "cn.eclicks.chelun" { return Some(1005844); }
    if app == "天王传奇-玛法世界" || app == "com.zzlywgl.h5.twcq" { return Some(1005845); }
    if app == "Nike? Run Club" || app == "com.nike.plusgps" { return Some(1005846); }
    if app == "卡惠信用卡优惠" || app == "com.imohoo.favorablecard" { return Some(1005847); }
    if app == "好贷-贷款APP借款现金借钱" || app == "com.haodai.quickloan" { return Some(1005848); }
    if app == "阿尔法贝蒂传奇" || app == "com.immortallord.game" { return Some(1005849); }
    if app == "Nike+" || app == "com.nike.omega" { return Some(1005850); }
    if app == "漫读小说" || app == "com.immomo.mread" { return Some(1005851); }
    if app == "重庆公租房" || app == "cq.apublic.info.rentalhouse" { return Some(1005852); }
    if app == "车车安" || app == "com.imecar" { return Some(1005853); }
    if app == "NikeTrainingClub" || app == "com.nike.ntc" { return Some(1005854); }
    if app == "玩咖直播" || app == "com.imay.live" { return Some(1005855); }
    if app == "wifi上网加速器" || app == "cn.dooone.wifihelper_cn" { return Some(1005856); }
    if app == "NikeConnect" || app == "com.nike.nikeconnect" { return Some(1005857); }
    if app == "水果音乐制造机" || app == "com.imageline.FLM" { return Some(1005858); }
    if app == "思维导图" || app == "czh.mindnode" { return Some(1005859); }
    if app == "十堰新闻" || app == "com.im.zhsy" { return Some(1005860); }
    if app == "爱空间家装" || app == "com.ikongjian" { return Some(1005861); }
    if app == "海风智学中心HD" || app == "com.hfjy.LearningCenter.apad" { return Some(1005862); }
    if app == "爱康体检宝" || app == "com.ikang.web" { return Some(1005863); }
    if app == "爱康约体检查报告" || app == "com.ikang.official" { return Some(1005864); }
    if app == "猎豹浏览器" || app == "com.ijinshan.browser_fast" { return Some(1005865); }
    if app == "学宝" || app == "com.haojiazhang.activity" { return Some(1005866); }
    if app == "周易六十四卦详解" || app == "com.haojuren.zy64gua" { return Some(1005867); }
    if app == "年糕妈妈育儿" || app == "com.nicomama.niangaomama" { return Some(1005868); }
    if app == "鲸鱼阅读pro" || app == "com.ihuayue.jingyu.huawei" { return Some(1005869); }
    if app == "鲸鱼阅读" || app == "com.ihuayue.jingyu" { return Some(1005870); }
    if app == "掌上体检" || app == "com.ihaozuo.plamexam" { return Some(1005871); }
    if app == "好看新闻" || app == "com.haokanhaokan.news" { return Some(1005872); }
    if app == "新牛津英汉双解大词典" || app == "cn.dictcn.android.digitize.oxford_necdict_17001" { return Some(1005873); }
    if app == "牛津外研英汉汉英词典" || app == "cn.dictcn.android.digitize.oxford_bilingdict_17200" { return Some(1005874); }
    if app == "速贷宝借款" || app == "cn.daoan.sudaibao" { return Some(1005875); }
    if app == "手机弹钢琴" || app == "evertone.Piano" { return Some(1005876); }
    if app == "讯飞翻译" || app == "com.iflytek.translatorapp" { return Some(1005877); }
    if app == "满贯捕鱼" || app == "fish.soonyo.uc" { return Some(1005878); }
    if app == "讯飞语音+" || app == "com.iflytek.speechcloud" { return Some(1005879); }
    if app == "得宝理财" || app == "www.gainbao.com" { return Some(1005880); }
    if app == "货车定位" || app == "com.hgj.truckposition" { return Some(1005881); }
    if app == "语音阅读器" || app == "com.iflytek.readassistant.voicereader" { return Some(1005882); }
    if app == "福建电信掌上OSS" || app == "fsti.android" { return Some(1005883); }
    if app == "富宝资讯" || app == "fubao.android" { return Some(1005884); }
    if app == "8684外卖" || app == "cn.com.tianqu.takeout.main" { return Some(1005885); }
    if app == "浦发手机银行" || app == "cn.com.spdb.mobilebank.per" { return Some(1005886); }
    if app == "小e小学英语" || app == "cn.com.primary" { return Some(1005887); }
    if app == "讯飞输入法" || app == "com.iflytek.inputmethod" { return Some(1005888); }
    if app == "智学网学生端" || app == "com.iflytek.elpmobile.student" { return Some(1005889); }
    if app == "智学网" || app == "com.iflytek.elpmobile.smartlearning" { return Some(1005890); }
    if app == "凤凰新闻极速版" || app == "com.ifext.news" { return Some(1005891); }
    if app == "凤凰视频HD" || app == "com.ifeng.video" { return Some(1005892); }
    if app == "太平洋汽车" || app == "cn.com.pcauto.android.browser" { return Some(1005893); }
    if app == "嘉实理财嘉" || app == "com.harvestfund2" { return Some(1005894); }
    if app == "大象新闻" || app == "com.hnr.dxxw" { return Some(1005895); }
    if app == "太平洋咖啡" || app == "cn.com.pacificcoffee" { return Some(1005896); }
    if app == "论八字" || app == "gz.aas.calc8words" { return Some(1005897); }
    if app == "凤凰新闻" || app == "com.ifeng.news2" { return Some(1005898); }
    if app == "翻阅小说" || app == "com.ifeng.android" { return Some(1005899); }
    if app == "Turbo浏览器" || app == "com.hawk.android.browser" { return Some(1005900); }
    if app == "网金社" || app == "com.ifaex_android" { return Some(1005901); }
    if app == "趣生财钱包" || app == "com.ielpm.wallet" { return Some(1005902); }
    if app == "叫了个车" || app == "com.hitaxi.passenger" { return Some(1005903); }
    if app == "小二租车" || app == "com.hna.urent" { return Some(1005904); }
    if app == "e车e站" || app == "com.iecez.ecez" { return Some(1005905); }
    if app == "每日经济新闻" || app == "cn.com.nbd.nbdmobile" { return Some(1005906); }
    if app == "WIFI精灵" || app == "cn.com.magicwifi" { return Some(1005907); }
    if app == "恒信车管家" || app == "hxqc.mall" { return Some(1005908); }
    if app == "NAVER词典" || app == "com.nhn.android.naverdic" { return Some(1005909); }
    if app == "BAND交友" || app == "com.nhn.android.band" { return Some(1005910); }
    if app == "小米锁屏画报" || app == "com.mfashiongallery.emag" { return Some(1005911); }
    if app == "对话翻译" || app == "com.hawsoft.mobile.speechtrans" { return Some(1005912); }
    if app == "轻文轻小说" || app == "in.iqing.app" { return Some(1005913); }
    if app == "首汽约车司机端" || app == "com.ichinait.gbdriver" { return Some(1005914); }
    if app == "罗森点点" || app == "cn.com.lawson" { return Some(1005915); }
    if app == "牵媒视频相亲交友" || app == "com.hm.blinddate" { return Some(1005916); }
    if app == "华林证券" || app == "com.hlsclc" { return Some(1005917); }
    if app == "酷听听书" || app == "cn.com.kuting.activity" { return Some(1005918); }
    if app == "徽常有财" || app == "cn.com.hsbank" { return Some(1005919); }
    if app == "AnkiDroid 单字卡" || app == "com.ichi2.anki" { return Some(1005920); }
    if app == "首都疫苗服务" || app == "io.dcloud.H5AC4580F" { return Some(1005921); }
    if app == "奇门遁甲排盘" || app == "io.dcloud.qimenshijia.paipan" { return Some(1005922); }
    if app == "玄空风水" || app == "com.nfbazi.xuankong" { return Some(1005923); }
    if app == "有道词典" || app == "io.dcloud.stream.H56022FE5" { return Some(1005924); }
    if app == "车学堂" || app == "cn.com.drivedu.chexuetang" { return Some(1005925); }
    if app == "奇门遁甲" || app == "com.nfbazi.qimen" { return Some(1005926); }
    if app == "外卖管家" || app == "io.dcloud.waimaiguanjia" { return Some(1005927); }
    if app == "全聚视频" || app == "io.github.trylovecatch.aplayer" { return Some(1005928); }
    if app == "i车保护神" || app == "com.icb.bhs" { return Some(1005929); }
    if app == "八字用神" || app == "com.nfbazi.baziysh" { return Some(1005930); }
    if app == "北京预约挂号" || app == "xinglin.com.health_assistant.beijing" { return Some(1005931); }
    if app == "激斗英雄奥特曼" || app == "xl.gba.djjdyx0000000" { return Some(1005932); }
    if app == "房产小蜜书" || app == "com.hj.littleSecretary" { return Some(1005933); }
    if app == "蝴蝶小说" || app == "com.ibczy.reader" { return Some(1005934); }
    if app == "惠龙易通车主版" || app == "com.hletong.jppt.vehicle" { return Some(1005935); }
    if app == "批八字算命" || app == "com.nfbazi.Pibazi" { return Some(1005936); }
    if app == "幼儿英语启蒙" || app == "com.ibbapp.childrenenglish" { return Some(1005937); }
    if app == "央视新闻" || app == "cn.cntvnews" { return Some(1005938); }
    if app == "Civa机器人" || app == "cn.civaonline.ccstudentsclient" { return Some(1005939); }
    if app == "中国移动河北" || app == "com.hbmcc.heshenghuo" { return Some(1005940); }
    if app == "爱上学家长版" || app == "com.hkyc.shouxinparent.ischool" { return Some(1005941); }
    if app == "苏州新闻" || app == "com.iCitySuzhou.suzhou001" { return Some(1005942); }
    if app == "菁优网" || app == "jyeoo.app.ystudy" { return Some(1005943); }
    if app == "273二手车" || app == "cn.car273" { return Some(1005944); }
    if app == "曹操加盟司机" || app == "cn.caocaokeji.dcdriver" { return Some(1005945); }
    if app == "阅读星HD" || app == "com.iBookStar.activityHd" { return Some(1005946); }
    if app == "乐车邦" || app == "lecar.android.view" { return Some(1005947); }
    if app == "汽车管家" || app == "cn.cafecar.android" { return Some(1005948); }
    if app == "单身交友" || app == "com.hzsj.dsjy" { return Some(1005949); }
    if app == "离线英语词典" || app == "livio.pack.lang.en_US" { return Some(1005950); }
    if app == "好好住" || app == "com.hzhu.m" { return Some(1005951); }
    if app == "爱上消消消" || app == "love.match.set" { return Some(1005952); }
    if app == "微车" || app == "cn.buding.martin" { return Some(1005953); }
    if app == "布丁优惠券" || app == "cn.buding.coupon" { return Some(1005954); }
    if app == "钱街贷款" || app == "com.hzdg.qianjie" { return Some(1005955); }
    if app == "春雨诊所" || app == "me.chunyu.ChunyuDoctorClient" { return Some(1005956); }
    if app == "春雨健康工具" || app == "me.chunyu.healthtool" { return Some(1005957); }
    if app == "全民钱包" || app == "com.hzcfapp.qmwallet" { return Some(1005958); }
    if app == "博雅小学堂" || app == "cn.boyakids.m" { return Some(1005959); }
    if app == "饿了么商家版" || app == "me.ele.napos" { return Some(1005960); }
    if app == "饿了么有菜" || app == "me.ele.youcai.restaurant" { return Some(1005961); }
    if app == "万车达" || app == "cn.bossche.wcd" { return Some(1005962); }
    if app == "新易贷微贷款" || app == "cn.boccfc.loan.wallet" { return Some(1005963); }
    if app == "够力解梦图版" || app == "com.hz.gaolatclassicdreamwithpic" { return Some(1005964); }
    if app == "万年历黄历" || app == "cn.bluecrane.calendar" { return Some(1005965); }
    if app == "智慧树园长版" || app == "com.hyww.wisdomtreebroomall" { return Some(1005966); }
    if app == "美丽修行" || app == "cn.bevol.p" { return Some(1005967); }
    if app == "伊对" || app == "me.yidui" { return Some(1005968); }
    if app == "智慧树园丁版" || app == "com.hyww.wisdomtree.gardener" { return Some(1005969); }
    if app == "米侠浏览器" || app == "mixiaba.com.Browser" { return Some(1005970); }
    if app == "方特旅游" || app == "com.hytch.ftthemepark" { return Some(1005971); }
    if app == "2345贷款王借款" || app == "com.hyron.b2b2p" { return Some(1005972); }
    if app == "药店小蜜" || app == "com.hydee.hdsec" { return Some(1005973); }
    if app == "激光捕鱼" || app == "com.hegssx.jgby.aligames" { return Some(1005974); }
    if app == "海豚浏览器国际版" || app == "mobi.mgeek.TunnyBrowser" { return Some(1005975); }
    if app == "和谐体检" || app == "com.hxtj.activity" { return Some(1005976); }
    if app == "六爻断卦" || app == "com.nfbazi.LiuyaoDuangua" { return Some(1005977); }
    if app == "手心家长" || app == "com.hkyc.shouxinparent" { return Some(1005978); }
    if app == "斑马司机端" || app == "cn.app.bm.driver.geo" { return Some(1005979); }
    if app == "移动行讯通" || app == "mobile.com.cn.ui" { return Some(1005980); }
    if app == "微贝浏览器" || app == "mobilebrowser.explore.webs" { return Some(1005981); }
    if app == "和祥行理财" || app == "com.hxh.hxh" { return Some(1005982); }
    if app == "家居风水" || app == "com.nfbazi.Jiajufsh" { return Some(1005983); }
    if app == "胎教音乐大全" || app == "cn.amanda.music_dq" { return Some(1005984); }
    if app == "摩范出行" || app == "com.hxcx.morefun" { return Some(1005985); }
    if app == "华夏二手车" || app == "com.hx.ui" { return Some(1005986); }
    if app == "虎扑" || app == "com.hupu.games" { return Some(1005987); }
    if app == "K12同步教育" || app == "mxb.educationnet" { return Some(1005988); }
    if app == "POCO相机" || app == "my.PCamera" { return Some(1005989); }
    if app == "美人相机" || app == "my.beautyCamera" { return Some(1005990); }
    if app == "多闪" || app == "my.maya.android" { return Some(1005991); }
    if app == "火猴浏览器" || app == "com.huohoubrowser" { return Some(1005992); }
    if app == "八字合婚" || app == "com.nfbazi.BaziHehun" { return Some(1005993); }
    if app == "喵聊交友" || app == "com.hj.cat.chat" { return Some(1005994); }
    if app == "货达司机" || app == "com.huoda.tms.driver" { return Some(1005995); }
    if app == "WiFi密码万能查看" || app == "nanjing.app.wificrack" { return Some(1005996); }
    if app == "二手货车" || app == "com.huoche.androids" { return Some(1005997); }
    if app == "视频美颜相机版" || app == "com.hello.wxspmy" { return Some(1005998); }
    if app == "依儿鸭家长版" || app == "net.babyduck.yierya.parent" { return Some(1005999); }
    if app == "贝多邦家长端" || app == "net.bhyf.parent" { return Some(1006000); }
    if app == "恒好用车司机" || app == "com.hengsheng.carpoolingdriver" { return Some(1006001); }
    if app == "赚钱闲鱼" || app == "com.newyork.phaeton" { return Some(1006002); }
    if app == "充电助手" || app == "net.canking.power" { return Some(1006003); }
    if app == "车缴查违章" || app == "com.hunan.weizhang" { return Some(1006004); }
    if app == "adidas" || app == "cn.adidas.app" { return Some(1006005); }
    if app == "沪江英语" || app == "com.hujiang.news" { return Some(1006006); }
    if app == "招商银行" || app == "cmb.pb" { return Some(1006007); }
    if app == "惠租车" || app == "com.huizuche.app" { return Some(1006008); }
    if app == "惠装装修" || app == "com.huizhuang.hz" { return Some(1006009); }
    if app == "作业大师" || app == "com.hengtiansoft.tgedu.checkanswer" { return Some(1006010); }
    if app == "洪铟八字算命" || app == "net.hybz" { return Some(1006011); }
    if app == "智慧树教师版" || app == "net.hyww.wisdomtree.teacher" { return Some(1006012); }
    if app == "智慧家校家长端" || app == "cc.zenking.edu.zhjx" { return Some(1006013); }
    if app == "ARSchool" || app == "com.newvision.arschool" { return Some(1006014); }
    if app == "优惠盒子" || app == "com.hengxin.rrh" { return Some(1006015); }
    if app == "惠借贷款" || app == "com.huijiekuan.huijie" { return Some(1006016); }
    if app == "惠借宝贷款" || app == "com.huijiebao" { return Some(1006017); }
    if app == "会找房" || app == "com.huifenqi.huizhaofangapp" { return Some(1006018); }
    if app == "皇包车旅行" || app == "com.hugboga.custom" { return Some(1006019); }
    if app == "语音翻译器" || app == "com.hudun.voicecovert" { return Some(1006020); }
    if app == "找房吧经纪人版" || app == "net.realtor.app.extranet.cmls" { return Some(1006021); }
    if app == "儿童涂颜色弹钢琴" || app == "com.here.afxdrawpiano" { return Some(1006022); }
    if app == "儿童宝宝学拼音" || app == "cc.flyblue.pinyin" { return Some(1006023); }
    if app == "儿童宝宝医院" || app == "cc.flyblue.hospital" { return Some(1006024); }
    if app == "中国汽车人才网" || app == "carjob.com.cn" { return Some(1006025); }
    if app == "免费听书神器" || app == "com.mfday.but.persist.hearfun" { return Some(1006026); }
    if app == "思维导图 Mindjet Maps" || app == "net.thinkingspace" { return Some(1006027); }
    if app == "花小猪司机端" || app == "com.huaxiaozhu.driver" { return Some(1006028); }
    if app == "旅程司机" || app == "net.untrip.app.lczcd" { return Some(1006029); }
    if app == "华为 VR 手柄" || app == "com.huawei.vrhandle" { return Some(1006030); }
    if app == "百万钱包" || app == "net.wecash.cidai" { return Some(1006031); }
    if app == "家校帮" || app == "net.whty.app.eyu" { return Some(1006032); }
    if app == "XMind思维导图" || app == "net.xmind.doughnut" { return Some(1006033); }
    if app == "中考必备" || app == "net.yyasp.middleschool" { return Some(1006034); }
    if app == "山西校讯通" || app == "net.zdsoft.szxy.android" { return Some(1006035); }
    if app == "华为技术支持" || app == "com.huawei.support.mobile" { return Some(1006036); }
    if app == "央广新闻" || app == "news.cnr.cn" { return Some(1006037); }
    if app == "华为图库" || app == "com.huawei.photos" { return Some(1006038); }
    if app == "懒人听书" || app == "bubei.tingshu" { return Some(1006039); }
    if app == "P2PWIFICAM" || app == "object.p2pwificam.client" { return Some(1006040); }
    if app == "会员中心" || app == "com.huawei.mycenter" { return Some(1006041); }
    if app == "顺历老黄历万年历日历" || app == "oms.mmc.app.almanac_inland" { return Some(1006042); }
    if app == "八字算命" || app == "oms.mmc.fortunetelling.fate.eightcharacters" { return Some(1006043); }
    if app == "八字排盘" || app == "oms.mmc.fortunetelling.gmpay.eightcharacters" { return Some(1006044); }
    if app == "起名解名宝宝取名" || app == "oms.mmc.fortunetelling.measuringtools.naming" { return Some(1006045); }
    if app == "HUAWEI Mobile WiFi 2" || app == "com.huawei.mw" { return Some(1006046); }
    if app == "易奇文化" || app == "yiqi.bazi" { return Some(1006047); }
    if app == "微软必应词典" || app == "bingdic.android.activity" { return Some(1006048); }
    if app == "365小时光" || app == "com.hesh.five" { return Some(1006049); }
    if app == "MX 浏览器" || app == "org.chromium.chrome" { return Some(1006050); }
    if app == "捕鱼达人3" || app == "org.cocos2d.fishingjoy3.uc" { return Some(1006051); }
    if app == "捕鱼达人" || app == "org.cocos2dx.FishGame" { return Some(1006052); }
    if app == "捕鱼达人2" || app == "org.cocos2dx.FishingJoy2" { return Some(1006053); }
    if app == "CAF浏览器" || app == "org.codeaurora.swe.browser.dev" { return Some(1006054); }
    if app == "cm浏览器" || app == "org.cyanogenmod.gello.browser" { return Some(1006055); }
    if app == "华为随心控" || app == "com.huawei.multiscreen" { return Some(1006056); }
    if app == "A8体育" || app == "org.fungo.a8sport" { return Some(1006057); }
    if app == "天天电视直播" || app == "org.fungo.fungolive" { return Some(1006058); }
    if app == "全民电视直播" || app == "org.fungo.fungoliveallstar" { return Some(1006059); }
    if app == "天天手机电视直播" || app == "org.fungo.vest3" { return Some(1006060); }
    if app == "Flybook阅读器" || app == "org.geometerplus.zlibrary.ui.androidfly" { return Some(1006061); }
    if app == "北斗卫星地图" || app == "bdbd.wiex.ditu" { return Some(1006062); }
    if app == "华为智能摄像机" || app == "com.huawei.ipc" { return Some(1006063); }
    if app == "12306买火车票" || app == "battymole.trainticket" { return Some(1006064); }
    if app == "宝宝取名" || app == "babyname.babyname" { return Some(1006065); }
    if app == "WiFi密码查看钥匙" || app == "augustwf.app.wificrackys" { return Some(1006066); }
    if app == "蚂蚁浏览器" || app == "org.mozilla.fennec_mylinux" { return Some(1006067); }
    if app == "Firefox" || app == "org.mozilla.firefox" { return Some(1006068); }
    if app == "Firefox Focus：隐私浏览器" || app == "org.mozilla.focus" { return Some(1006069); }
    if app == "国学三字经和弟子规" || app == "com.hewei.sinologyhd" { return Some(1006070); }
    if app == "艺龙酒店" || app == "com.elong.hotel.ui" { return Some(1006071); }
    if app == "万年黄历" || app == "com.mm.calendar" { return Some(1006072); }
    if app == "陪聊视频交友" || app == "com.mm.peiliao" { return Some(1006073); }
    if app == "X浏览器" || app == "com.mmbox.xbrowser" { return Some(1006074); }
    if app == "美剧星球" || app == "com.mjxq.app" { return Some(1006075); }
    if app == "蜜约交友" || app == "com.miyue.friend" { return Some(1006076); }
    if app == "蜜蜂出行" || app == "com.mmuu.travel.client" { return Some(1006077); }
    if app == "mo9信用钱包" || app == "com.mo9.app.view" { return Some(1006078); }
    if app == "充电桩" || app == "com.electric.chargingpile" { return Some(1006079); }
    if app == "美女一起来" || app == "com.miui.whetstone" { return Some(1006080); }
    if app == "全能浏览器" || app == "com.moban.wnbrowser" { return Some(1006081); }
    if app == "MIUI天气" || app == "com.miui.weather2" { return Some(1006082); }
    if app == "摩拜单车" || app == "com.mobike.mobikeapp" { return Some(1006083); }
    if app == "小米百变锁屏(MiLocker)" || app == "com.miui.home" { return Some(1006084); }
    if app == "小米计算器" || app == "com.miui.calculator" { return Some(1006085); }
    if app == "牛津英语词典" || app == "com.mobisystems.msdict.embedded.wireless.oxford.dictionaryofenglish" { return Some(1006086); }
    if app == "畅读听书" || app == "com.mobo.changduvoice" { return Some(1006087); }
    if app == "蘑菇租房" || app == "com.mogoroom.renter" { return Some(1006088); }
    if app == "用户反馈" || app == "com.miui.bugreport" { return Some(1006089); }
    if app == "灯塔家长" || app == "com.miteng.lighthouse.family.android" { return Some(1006090); }
    if app == "一点金库理财" || app == "com.mirror.easyclient" { return Some(1006091); }
    if app == "墨迹天气" || app == "com.moji.mjweather" { return Some(1006092); }
    if app == "墨迹天气极速版" || app == "com.moji.mjweather.light" { return Some(1006093); }
    if app == "千炮捕鱼大赛" || app == "com.moling.buyu10002" { return Some(1006094); }
    if app == "彩金捕鱼" || app == "org.shenshi.myfish" { return Some(1006095); }
    if app == "财富" || app == "com.hexin.plat.android.CaiFuSecurity" { return Some(1006096); }
    if app == "翼课学生" || app == "com.ekwing.students" { return Some(1006097); }
    if app == "翼课家长" || app == "com.ekwing.flyparents" { return Some(1006098); }
    if app == "儿童学英文字母" || app == "com.moon.babykowns.englishalphabet" { return Some(1006099); }
    if app == "儿童学习教育游戏" || app == "com.moon.ertong2.yizhi2.game2" { return Some(1006100); }
    if app == "吉利GNetLink" || app == "com.ericsson.geely" { return Some(1006101); }
    if app == "直播地球" || app == "com.moonshow" { return Some(1006102); }
    if app == "边走边听背单词" || app == "com.ejetsoft.efs.wordsend4android" { return Some(1006103); }
    if app == "一嗨租车" || app == "com.ehai" { return Some(1006104); }
    if app == "233电影" || app == "com.ersansandianying" { return Some(1006105); }
    if app == "蓝墨云班课" || app == "com.mosoink.mosoteach" { return Some(1006106); }
    if app == "磨铁阅读" || app == "com.motie.motiereader" { return Some(1006107); }
    if app == "联播服务" || app == "com.milink.service" { return Some(1006108); }
    if app == "恒房通" || app == "com.movitech.grandehb" { return Some(1006109); }
    if app == "浏览器plus" || app == "com.moying.browserplus" { return Some(1006110); }
    if app == "直播绵阳" || app == "com.moyun.zbmy.main" { return Some(1006111); }
    if app == "英孚英语" || app == "com.ef.english24_7" { return Some(1006112); }
    if app == "知命八字算命占卜" || app == "com.mrkj.sm" { return Some(1006113); }
    if app == "衣联网" || app == "com.eelly.buyer" { return Some(1006114); }
    if app == "任意门日淘" || app == "com.mrnew.door" { return Some(1006115); }
    if app == "家长管理" || app == "com.eebbk.parentalcontrol" { return Some(1006116); }
    if app == "体育疯-NBA直播" || app == "com.msports.tyf" { return Some(1006117); }
    if app == "单词听写" || app == "com.eebbk.dictation" { return Some(1006118); }
    if app == "家长帮" || app == "com.eduu.bang" { return Some(1006119); }
    if app == "美图秀秀" || app == "com.mt.mtxx.mtxx" { return Some(1006120); }
    if app == "夜夜直播" || app == "com.mt.yyzb" { return Some(1006121); }
    if app == "华为手机文件管理器" || app == "com.huawei.hidisk" { return Some(1006122); }
    if app == "司机讲堂" || app == "com.mtzj.xbljkhw" { return Some(1006123); }
    if app == "陪育家长版" || app == "com.edutao.xxztc.android.parents" { return Some(1006124); }
    if app == "我找车" || app == "com.es.freight" { return Some(1006125); }
    if app == "哎呀音乐" || app == "com.edusoho.iyamusic" { return Some(1006126); }
    if app == "连枝家长版" || app == "com.edugateapp.client.family" { return Some(1006127); }
    if app == "一点音乐" || app == "com.muscdp.rchtcorigation" { return Some(1006128); }
    if app == "音乐裁剪大师" || app == "com.musicropku" { return Some(1006129); }
    if app == "歌词音乐播放器" || app == "com.musixmatch.android.lyrify" { return Some(1006130); }
    if app == "聚多多优惠" || app == "com.miguo.ui" { return Some(1006131); }
    if app == "ES文件浏览器" || app == "com.estrongs.android.pop" { return Some(1006132); }
    if app == "电子音乐板" || app == "com.mvtrail.electrodrumpad" { return Some(1006133); }
    if app == "MindMaster思维导图" || app == "com.edrawsoft.mindmaster" { return Some(1006134); }
    if app == "傲游浏览器" || app == "com.mx.browser" { return Some(1006135); }
    if app == "傲游浏览器for?Pad" || app == "com.mx.browser.tablet" { return Some(1006136); }
    if app == "上汽同行" || app == "com.ecology.view" { return Some(1006137); }
    if app == "4D书城" || app == "com.mxr.dreambook" { return Some(1006138); }
    if app == "音乐剪辑铃声制作" || app == "com.ecloud.musiceditor" { return Some(1006139); }
    if app == "易直播极速版" || app == "com.easylive.lite" { return Some(1006140); }
    if app == "浏览器加" || app == "com.my.browserplus" { return Some(1006141); }
    if app == "英语单词速查" || app == "com.my.english" { return Some(1006142); }
    if app == "ES文件浏览器专业版" || app == "com.estrongs.android.pop.pro" { return Some(1006143); }
    if app == "听车汽修版" || app == "com.microtalk.tingche" { return Some(1006144); }
    if app == "微软翻译" || app == "com.microsoft.translator" { return Some(1006145); }
    if app == "Microsoft Word" || app == "com.microsoft.office.word" { return Some(1006146); }
    if app == "WiFi伴侣" || app == "com.mydream.wifi" { return Some(1006147); }
    if app == "Microsoft Excel" || app == "com.microsoft.office.excel" { return Some(1006148); }
    if app == "麦田认字" || app == "com.mytian.appstore.rz" { return Some(1006149); }
    if app == "麦田拼音" || app == "com.mytian.pinyin" { return Some(1006150); }
    if app == "哔哔小说" || app == "com.myyh.mkyd" { return Some(1006151); }
    if app == "ZAKER新闻" || app == "com.myzaker.ZAKER_Phone" { return Some(1006152); }
    if app == "舟道网司机专版" || app == "com.myzhoudao.EnterpriseZhoudaoDriver" { return Some(1006153); }
    if app == "花花直播" || app == "com.nacai.bocai" { return Some(1006154); }
    if app == "深夜直播" || app == "com.nacai.taose" { return Some(1006155); }
    if app == "顺风车司机版" || app == "com.nade.ownerride" { return Some(1006156); }
    if app == "天天拼货团女装批发服装批发" || app == "com.nahuo.quicksale" { return Some(1006157); }
    if app == "乐培家长" || app == "com.earen.lps_client_patriarch" { return Some(1006158); }
    if app == "男衣邦穿衣搭配" || app == "com.nanyibang.nomi" { return Some(1006159); }
    if app == "NET-A-PORTER" || app == "com.nap" { return Some(1006160); }
    if app == "极品飞车：最高通缉" || app == "com.ea.games.nfs13_row" { return Some(1006161); }
    if app == "司机伙伴" || app == "com.etl.puhaibo.activity" { return Some(1006162); }
    if app == "海报新闻" || app == "com.dzwww.sd.android" { return Some(1006163); }
    if app == "丁香妈妈" || app == "com.dxy.gaia" { return Some(1006164); }
    if app == "支付中心（系统软件）" || app == "com.nearme.atlas" { return Some(1006165); }
    if app == "皮皮虾传奇" || app == "com.miaoju.ppxcq.jrtt" { return Some(1006166); }
    if app == "NearMe云笔记" || app == "com.nearme.note" { return Some(1006167); }
    if app == "备份还原" || app == "com.nearme.sync" { return Some(1006168); }
    if app == "儿歌多多" || app == "com.duoduo.child.story" { return Some(1006169); }
    if app == "二手车直卖网" || app == "com.evaluate.activity" { return Some(1006170); }
    if app == "免费小说" || app == "com.mianfei.book" { return Some(1006171); }
    if app == "CC直播" || app == "com.netease.cc" { return Some(1006172); }
    if app == "免费WiFi万能破解器" || app == "com.mfwifi.wnpjq" { return Some(1006173); }
    if app == "月亮听书" || app == "ylts.listen.host.com" { return Some(1006174); }
    if app == "亲宝宝" || app == "com.dw.btime" { return Some(1006175); }
    if app == "网易云课堂" || app == "com.netease.edu.study" { return Some(1006176); }
    if app == "荒野行动" || app == "com.netease.hyxd" { return Some(1006177); }
    if app == "火星小说" || app == "com.duyao.poisonnovel" { return Some(1006178); }
    if app == "安卓听书" || app == "com.mht.mkl.tingshu" { return Some(1006179); }
    if app == "专业滤镜相机" || app == "com.netease.loftercam.activity" { return Some(1006180); }
    if app == "伴鱼绘本" || app == "com.duwo.reading" { return Some(1006181); }
    if app == "肯德基麦当劳优惠券" || app == "com.mhealth37.coupons" { return Some(1006182); }
    if app == "贷款荚借款贷款" || app == "com.duozhejinrong.daikuanjia" { return Some(1006183); }
    if app == "阴阳师" || app == "com.netease.onmyoji" { return Some(1006184); }
    if app == "YY交友" || app == "com.duowan.yylove" { return Some(1006185); }
    if app == "虎牙直播" || app == "com.duowan.kiwi" { return Some(1006186); }
    if app == "多米音乐" || app == "com.duomi.android" { return Some(1006187); }
    if app == "长隆旅游" || app == "com.eventmosh.changlong" { return Some(1006188); }
    if app == "小米遥控器" || app == "com.duokan.phone.remotecontroller" { return Some(1006189); }
    if app == "多多团长" || app == "com.duoduo.tuanzhang" { return Some(1006190); }
    if app == "糖果缤纷乐" || app == "com.neteasepublishing.king.ccf" { return Some(1006191); }
    if app == "多多小说" || app == "com.duoduo.novel.read" { return Some(1006192); }
    if app == "多多超市" || app == "com.duoduo.games.market" { return Some(1006193); }
    if app == "WiFi防蹭网" || app == "com.netsacnmanager" { return Some(1006194); }
    if app == "奇瑞助手" || app == "com.neusoft.ssp.chery.assistant" { return Some(1006195); }
    if app == "阅读器" || app == "com.neverland.alreader" { return Some(1006196); }
    if app == "快乐街机捕鱼" || app == "com.pubu.fishingoffline" { return Some(1006197); }
    if app == "朴朴超市" || app == "com.pupumall.customer" { return Some(1006198); }
    if app == "嘀嗒出租司机" || app == "com.didapinche.taxidriver" { return Some(1006199); }
    if app == "中国白银" || app == "com.zhongguobaiyin.morgan" { return Some(1006200); }
    if app == "崩坏：星穹铁道" || app == "com.miHoYo.hkrpg" { return Some(1006201); }
    if app == "河马剧场" || app == "com.dz.hmjc" { return Some(1006202); }
    if app == "书旗小说" || app == "com.shuqi.controller" { return Some(1006203); }
    if app == "华通白银" || app == "com.huatongbaiyin.morgan" { return Some(1006204); }
    if app == "鲸鱼体育" || app == "com.dq17.app" { return Some(1006205); }
    if app == "鲸鱼体育" || app == "com.dq17.app01" { return Some(1006206); }
    if app == "红狮智富" || app == "com.hsunny8899.app" { return Some(1006207); }
    if app == "红狮" || app == "com.longwayhs.app" { return Some(1006208); }
    if app == "红狮智富" || app == "com.wisdom.lion" { return Some(1006209); }
    if app == "Soul" || app == "cn.soulapp.android" { return Some(1006210); }
    if app == "这城有良田-心动良田，如梦随行" || app == "com.zcylt.eworld" { return Some(1006211); }
    if app == "钜丰贵金属" || app == "com.jfjinye.jfjinye" { return Some(1006212); }
    if app == "三国吧兄弟" || app == "com.wzhx.mobile.sgbxdhx" { return Some(1006213); }
    if app == "腾讯视频" || app == "com.tencent.qqlive" { return Some(1006214); }
    if app == "天天白银" || app == "com.tiantianbaiyin.morgan" { return Some(1006215); }
    if app == "红狮智富" || app == "com.hszfapp24.app" { return Some(1006216); }
    if app == "惠牛订购" || app == "com.hn.dinggou" { return Some(1006217); }
    if app == "摩耶上门按摩" || app == "com.moyeuser.ws" { return Some(1006218); }
    if app == "Kuaishou" || app == "com.smile.gifmaker" { return Some(1006219); }
    if app == "捕鱼炸翻天" || app == "com.byzft.uctf" { return Some(1006220); }
    if app == "自如-品质租房选自如" || app == "com.ziroom.ZiroomProject" { return Some(1006221); }
    if app == "七猫小说-看小说电子书的阅读神器" || app == "com.yueyou.cyreader" { return Some(1006222); }
    if app == "看球体育" || app == "com.htty.kanqiu" { return Some(1006223); }
    if app == "全民江湖" || app == "com.wzsc.mobile.rxjhqmjh2" { return Some(1006224); }
    if app == "神火大陆 - 暗黑魔域战神觉醒魔幻动作手游!" || app == "com.shenhuomeituan" { return Some(1006225); }
    if app == "一元手游" || app == "com.yuewan.yiyuangbdyz" { return Some(1006226); }
    if app == "叮咚买菜——想吃什么 就上叮咚" || app == "com.mmbang.neighborhood" { return Some(1006227); }
    if app == "魔界军团" || app == "com.uc.tmgp.lmjh.mjjt" { return Some(1006228); }
    if app == "九九订购" || app == "com.xrthinkive.jiujiu" { return Some(1006229); }
    if app == "新仙魔九界" || app == "com.shiyi.xxmjj" { return Some(1006230); }
    if app == "斗罗大陆：史莱克学院" || app == "com.tanwan.dldlslkxy9" { return Some(1006231); }
    if app == "末日远征" || app == "com.mryz.tt_20241" { return Some(1006232); }
    if app == "秦皇汉武：权谋之战" || app == "com.sswl.qhhwqmzz" { return Some(1006233); }
    if app == "挚爱婚恋" || app == "com.yueyuan.youlove" { return Some(1006234); }
    if app == "锦礼订购" || app == "com.jinli.dinggou" { return Some(1006235); }
    if app == "我爱拼方块" || app == "com.xc.wapfk" { return Some(1006236); }
    if app == "布阵小军师" || app == "com.youyi262.yy" { return Some(1006237); }
    if app == "解压高手" || app == "com.hainanyyqj.jygs" { return Some(1006238); }
    if app == "荒野迷城" || app == "com.uc.cm.hymc" { return Some(1006239); }
    if app == "剑魂世界" || app == "com.jhsjwsuc.ggws" { return Some(1006240); }
    if app == "上古诛神录" || app == "com.sgzslwsucsdk2.ggws" { return Some(1006241); }
    if app == "一淘" || app == "com.taobao.etao" { return Some(1006242); }
    if app == "冒险大作战" || app == "com.g3.mxdzzuc1" { return Some(1006243); }
    if app == "荒野迷城" || app == "com.caohua.hymc.andriod" { return Some(1006244); }
    if app == "大师斗地主游戏软件" || app == "com.aiqi.dsddz" { return Some(1006245); }
    if app == "天弘订购" || app == "uni.tianhong" { return Some(1006246); }
    if app == "指尖订购" || app == "com.zhijian.dinggou" { return Some(1006247); }
    if app == "这城有良田" || app == "com.zcylt.eworld" { return Some(1006248); }
    if app == "牡丹订购" || app == "com.mudan.dinggou" { return Some(1006249); }
    if app == "腾龙订购" || app == "com.tenglong.dinggou" { return Some(1006250); }
    if app == "传世霸业" || app == "com.csbyaf.uc0318" { return Some(1006251); }
    if app == "小小蚁国" || app == "com.xxygwsucsdk4.xxyg" { return Some(1006252); }
    if app == "剑魂online" || app == "com.jhonlinewsuc.ggws" { return Some(1006253); }
    if app == "珍牛订购" || app == "com.zhenniu.dinggou" { return Some(1006254); }
    if app == "传奇1.76怀旧版-永恒屠龙" || app == "com.think.peak" { return Some(1006255); }
    if app == "约个摩" || app == "uni.UNI81AEE71" { return Some(1006256); }
    if app == "仙剑至尊-正版单机仙剑奇侠传游戏" || app == "com.xjzzxjqxzsy" { return Some(1006257); }
    if app == "最强祖师" || app == "com.zqzs.ma.android" { return Some(1006258); }
    if app == "乱世群英传" || app == "com.cjbd.gcsg2.sgwljqb" { return Some(1006259); }
    if app == "三国怀旧版" || app == "com.xqi.gcsg2.sghjbzs" { return Some(1006260); }
    if app == "库课网校" || app == "com.kuke" { return Some(1006261); }
    if app == "航海王：梦想指针-动画正版授权" || app == "com.qianxunshe.dp.alpha" { return Some(1006262); }
    if app == "合出大西瓜" || app == "com.xc.hcdxg.vivo" { return Some(1006263); }
    if app == "keke语音" || app == "com.kekeyuyin.guoguo" { return Some(1006264); }
    if app == "按摩到家" || app == "com.yangchong.user" { return Some(1006265); }
    if app == "斗战神佛" || app == "com.dzsfwsuc2.ggws" { return Some(1006266); }
    if app == "云上城之歌" || app == "com.ysczgucwsucsdk2.sy929" { return Some(1006267); }
    if app == "觅趣" || app == "com.jiayan.sunshine" { return Some(1006268); }
    if app == "洪恩识字-儿童识字启蒙必备" || app == "com.ihuman.words" { return Some(1006269); }
    if app == "荒野迷城-废土求生" || app == "com.caohua.hymc" { return Some(1006270); }
    if app == "传奇官方版-怀旧服 2024正版三职业复刻热血回归:遮天斩" || app == "ztcqztz.darecord.com" { return Some(1006271); }
    if app == "趣看看短剧" || app == "com.shortplay" { return Some(1006272); }
    if app == "黄金奇迹:MU官方新版本-魔法大陆" || app == "com.qjmf.qjios" { return Some(1006273); }
    if app == "红果免费短剧" || app == "com.phoenix.read" { return Some(1006274); }
    if app == "星野" || app == "com.xingye.app" { return Some(1006275); }
    None
}
