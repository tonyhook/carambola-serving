use std::{io::Write, time::Duration};

use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyInit};
use base64::prelude::*;
use flate2::{Compression, write::GzEncoder};
use urlencoding::encode;

use crate::{protocol::*, Assets, Cache, Client, Connection, HttpPool, Identifiers, Price, ResultMessage};

type Aes256EcbEnc = ecb::Encryptor<aes::Aes256>;

pub mod ad;
pub mod app;
pub mod caid;
pub mod request;
pub mod response;
pub mod user;
pub mod video;

pub use ad::SpinviewAd;
pub use app::SpinviewApp;
pub use caid::SpinviewCaid;
pub use request::SpinviewRequest;
pub use response::SpinviewResponse;
pub use user::SpinviewUser;
pub use video::SpinviewVideo;

pub struct Spinview {

}

impl Client for Spinview {

    async fn request(request: &Request, connection: &Connection, pool: &HttpPool, cache: &Cache) -> Result<Response, ResultMessage> {
        let slot_id = &connection.client_tag_id;
        let token = &connection.client_ekey;

        let request_id = cache.get_sequence();

        let mut assets = Assets::new(request);
        let identifiers = Identifiers::new(request);

        let request_spinview = SpinviewRequest {
            id: {
                request_id.to_string()
            },
            slot_id: {
                slot_id.parse::<i32>().unwrap()
            },
            slot_width: {
                match request.item[0].spec.display.w {
                    Some(w) => {
                        w
                    },
                    None => 0,
                }
            },
            slot_height: {
                match request.item[0].spec.display.h {
                    Some(h) => {
                        h
                    },
                    None => 0,
                }
            },
            bid_floor: {
                Some(Price::to_client(connection, request.item[0].flr.map(f64::from)))
            },
            bid_floor_cur: {
                None
            },
            app_name: {
                match &request.context.app {
                    Some(app) => app.name.clone(),
                    None => "".to_string(),
                }
            },
            app_package: {
                match &request.context.app {
                    Some(app) => {
                        match &app.bundle {
                            Some(bundle) => Some(bundle.clone()),
                            None => None,
                        }
                    },
                    None => None,
                }
            },
            app_ver: {
                match &request.context.app {
                    Some(app) => {
                        match &app.ver {
                            Some(ver) => Some(ver.clone()),
                            None => None,
                        }
                    },
                    None => None,
                }
            },
            app_store_url: {
                match &request.context.app {
                    Some(app) => {
                        match &app.storeurl {
                            Some(storeurl) => Some(storeurl.clone()),
                            None => None,
                        }
                    },
                    None => None,
                }
            },
            ip: {
                match &request.context.device.ip {
                    Some(ip) => ip.clone(),
                    None => "".to_string(),
                }
            },
            ipv6: {
                match &request.context.device.ipv6 {
                    Some(ipv6) => Some(ipv6.clone()),
                    None => None,
                }
            },
            user_agent: {
                request.context.device.ua.clone()
            },
            os_type: {
                match request.context.device.os {
                    Some(2) => 1,
                    Some(13) => 2,
                    Some(501) => 3,
                    _ => 0,
                }
            },
            os_name: {
                None
            },
            os_version: {
                request.context.device.osv.clone()
            },
            os_level: {
                match &request.context.device.oslevel {
                    Some(oslevel) => {
                        Some(oslevel.to_string())
                    },
                    None => None,
                }
            },
            imei: {
                match identifiers.get_id(501, 0) {
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
            android_id: {
                match identifiers.get_id(509, 0) {
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
            openudid: {
                None
            },
            caids: {
                let mut caids = vec![];

                match identifiers.get_id(513, 0) {
                    Some(uid) => {
                        caids.push(SpinviewCaid {
                            caid: {
                                Some(uid.id.clone())
                            },
                            version: {
                                match &uid.ver {
                                    Some(ver) => Some(ver.clone()),
                                    None => None,
                                }
                            },
                        });
                    },
                    None => (),
                };

                match identifiers.get_id(513, 1) {
                    Some(uid) => {
                        caids.push(SpinviewCaid {
                            caid: {
                                Some(uid.id.clone())
                            },
                            version: {
                                match &uid.ver {
                                    Some(ver) => Some(ver.clone()),
                                    None => None,
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
            paid: {
                match identifiers.get_id(519, 0) {
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
            imei_md5: {
                match identifiers.get_id(502, 0) {
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
            mac_md5: {
                match identifiers.get_id(512, 0) {
                    Some(uid) => Some(uid.id.clone()),
                    None => None,
                }
            },
            vendor: {
                match &request.context.device.make {
                    Some(make) => Some(make.clone()),
                    None => None,
                }
            },
            brand: {
                match &request.context.device.brand {
                    Some(brand) => Some(brand.clone()),
                    None => None,
                }
            },
            model: {
                match &request.context.device.model {
                    Some(model) => Some(model.clone()),
                    None => None,
                }
            },
            serialno: {
                None
                // match request.context.device.serial {
                //     Some(serial) => Some(serial.clone()),
                //     None => None,
                // }
            },
            screen_orientation: {
                match request.context.device.orientation {
                    Some(orientation) => {
                        match orientation {
                            501 => 1,
                            502 => 2,
                            _ => 3,
                        }
                    },
                    None => 3,
                }
            },
            screen_width: {
                match request.context.device.w {
                    Some(w) => w,
                    None => 0,
                }
            },
            screen_height: {
                match request.context.device.h {
                    Some(h) => h,
                    None => 0,
                }
            },
            screen_dpi: {
                match request.context.device.ppi {
                    Some(ppi) => ppi,
                    None => 0,
                }
            },
            screen_ppi: {
                match request.context.device.ppi {
                    Some(ppi) => ppi,
                    None => 0,
                }
            },
            screen_density: {
                match request.context.device.pxratio {
                    Some(pxratio) => pxratio,
                    None => 0.0,
                }
            },
            screen_size: {
                match request.context.device.size {
                    Some(size) => Some(size.to_string()),
                    None => None,
                }
            },
            connection_type: {
                match request.context.device.contype {
                    Some(1) => 0,
                    Some(2) => 1,
                    Some(4) => 2,
                    Some(5) => 3,
                    Some(6) => 4,
                    Some(7) => 5,
                    _ => 0,
                }
            },
            device_type: {
                match request.context.device.devicetype {
                    Some(devicetype) => {
                        match devicetype {
                            2 => 4,
                            3 => 3,
                            4 => 1,
                            5 => 2,
                            _ => 0,
                        }
                    },
                    None => 0,
                }
            },
            carrier: {
                match &request.context.device.carrier {
                    Some(carrier) => {
                        match carrier.as_str() {
                            "cmcc" => 1,
                            "unicom" => 2,
                            "telecom" => 3,
                            _ => 0,
                        }
                    },
                    None => 0,
                }
            },
            mccmnc: {
                None
                // match &request.context.device.mccmnc {
                //     Some(mccmnc) => {
                //         mccmnc.replace("-", "")
                //     },
                //     None => None,
                // }
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
            rom_version: {
                request.context.device.romv.clone()
            },
            sys_compiling_time: {
                request.context.device.romtime.clone()
            },
            app_store_version: {
                request.context.device.storev.clone()
            },
            hms: {
                request.context.device.hmsv.clone()
            },
            hag: {
                request.context.device.storev.clone()
            },
            hardware_machine: {
                request.context.device.hwmachine.clone()
            },
            hardware_model: {
                request.context.device.hwmodel.clone()
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
            init_time: {
                match &request.context.device.inittime {
                    Some(inittime) => Some(inittime.clone()),
                    None => None,
                }
            },
            startup_time: {
                request.context.device.boottime.clone()
            },
            upgrade_time: {
                request.context.device.updatetime.clone()
            },
            timezone: {
                request.context.device.timezone.clone()
            },
            country: {
                request.context.device.country.clone()
            },
            language: {
                request.context.device.lang.clone()
            },
            memory: {
                request.context.device.sysmemory.clone()
            },
            hard_disk: {
                request.context.device.sysdisksize.clone()
            },
            cpu_num: {
                request.context.device.syscpu.clone()
            },
            cpu_freq: {
                request.context.device.syscpufreq.clone()
            },
            idfa_policy: {
                request.context.device.lmt.clone()
            },
            battery_status: {
                request.context.device.sysbatterystatus.clone()
            },
            battery_power: {
                request.context.device.sysbatterypower.clone()
            },
            boot_mark: {
                request.context.device.bootmark.clone()
            },
            update_mark: {
                request.context.device.updatemark.clone()
            },
            packages: {
                let mut installed_apps = vec![];

                match &request.context.device.app {
                    Some(app) => {
                        for app1 in app.split(",") {
                            installed_apps.push(app1.to_string());
                        }
                    },
                    None => (),
                }

                Some(installed_apps)
            },
            ista: {
                None
            },
            user: {
                Some(SpinviewUser {
                    id: {
                        match &request.context.user.id {
                            Some(id) => Some(id.clone()),
                            None => None,
                        }
                    },
                    keywords: {
                        match &request.context.user.keywords {
                            Some(keywords) => Some(keywords.clone()),
                            None => None,
                        }
                    },
                    gender: {
                        match &request.context.user.gender {
                            Some(gender) => {
                                match gender.as_str() {
                                    "M" => Some(1),
                                    "F" => Some(0),
                                    _ => Some(-1),
                                }
                            },
                            None => None,
                        }
                    },
                    yob: {
                        match &request.context.user.yob {
                            Some(yob) => Some(yob.to_string()),
                            None => None,
                        }
                    },
                    jd: None,
                    jid: None,
                    taobao: None,
                })
            },
            elapse_time: {
                None
            },
            secure: {
                Some(0)
            },
        };

        let json_string = serde_json::to_vec(&request_spinview).unwrap();
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&json_string).unwrap();
        let compressed_bytes = encoder.finish().unwrap();

        let response_spinview: SpinviewResponse;

        let client = {
            let pool_spinview_lock = pool.pool_spinview.clone();
            let pool_spinview = pool_spinview_lock.read().unwrap();
            pool_spinview.clone()
        };
        let response_spinview_raw = client.post(format!("{}{}", if connection.test { "http://adx-test.spin-view.com/api/bid/v6/" } else { "http://adx.spin-view.com/api/bid/v6/" }, token))
            .body(compressed_bytes)
            .header("Accept-Encoding", "gzip")
            .header("Connection", "keep-alive")
            .header("Content-Encoding", "gzip")
            .header("Content-Type", "application/json;charset=utf-8")
            .timeout(Duration::from_millis(connection.timeout))
            .send().await;

        match response_spinview_raw {
            Ok(response_spinview_raw) => {
                let status = response_spinview_raw.status();
                if status == 204 {
                    return Err(ResultMessage {
                        code: 993,
                        message: "".to_string(),
                    });
                } else if status != 200 {
                    return Err(ResultMessage {
                        code: 992,
                        message: {
                            match response_spinview_raw.text().await {
                                Ok(text) => format!("upstream error {}: {}", status, text),
                                Err(_) => format!("upstream error {}", status),
                            }
                        },
                    });
                } else {
                    match response_spinview_raw.text().await {
                        Ok(text) => {
                            match serde_json::from_str::<SpinviewResponse>(&text) {
                                Ok(json) => {
                                    response_spinview = json;

                                    match response_spinview.code {
                                        1 => {
                                            return Err(ResultMessage {
                                                code: 993,
                                                message: "".to_string(),
                                            });
                                        },
                                        2 => {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: format!("upstream error : {:?}", response_spinview.message),
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

                        match &response_spinview.ads {
                            Some(ads) => {
                                for ad in ads {
                                    let mut link_asset = LinkAsset {
                                        linktype: {
                                            match ad.interaction_type {
                                                1 => 1,
                                                2 => 2,
                                                3 => 1,
                                                4 => 3,
                                                5 => 1,
                                                _ => 1,
                                            }
                                        },
                                        universallink: {
                                            match &ad.ulk {
                                                Some(ulk) => Some(ulk.clone()),
                                                None => None,
                                            }
                                        },
                                        storeid: {
                                            None
                                        },
                                        deeplink: {
                                            match &ad.deeplink_url {
                                                Some(deeplink_url) => Some(deeplink_url.clone()),
                                                None => None,
                                            }
                                        },
                                        quickapplink: None,
                                        wechatmppath: {
                                            match &ad.applet_path {
                                                Some(applet_path) => Some(applet_path.clone()),
                                                None => None,
                                            }
                                        },
                                        wechatmpid: {
                                            match &ad.applet_id {
                                                Some(applet_id) => Some(applet_id.clone()),
                                                None => None,
                                            }
                                        },
                                        marketurl: {
                                            None
                                        },
                                        downloadurl: {
                                            None
                                        },
                                        url: {
                                            ad.click_url.clone()
                                        },
                                        urlfb: None,
                                    };

                                    let bid = Bid {
                                        id: Some(request_id.to_string()),
                                        item: request.item[0].id.clone(),
                                        price: { // update later
                                            match ad.price {
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
                                        burl: None,
                                        lurl: None,
                                        media: Ad {
                                            id: response_spinview.id.clone(),
                                            display: Display {
                                                w: {
                                                    Some(ad.width)
                                                },
                                                h: {
                                                    Some(ad.height)
                                                },
                                                banner: {
                                                    if assets.get_banner_size() > 0 {
                                                        Some(Banner {
                                                            img: {
                                                                match &ad.images {
                                                                    Some(images) => {
                                                                        if images.len() > 0 {
                                                                            images[0].clone()
                                                                        } else {
                                                                            "".to_string()
                                                                        }
                                                                    },
                                                                    None => "".to_string(),
                                                                }
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

                                                        match &ad.video {
                                                            Some(video) => {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("video"),
                                                                    req: 1,
                                                                    video: Some(VideoAsset {
                                                                        url: video.url.clone(),
                                                                        mime: None,
                                                                        w: video.width,
                                                                        h: video.height,
                                                                        dur: Some(video.duration),
                                                                        skipoffset: video.skip,
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

                                                                match &video.cover_url {
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
                                                                match &video.endcard_url {
                                                                    Some(endcard_url) => {
                                                                        asset_vec.push(Asset {
                                                                            id: assets.consume_asset("video#end#html"),
                                                                            req: 0,
                                                                            img: None,
                                                                            title: None,
                                                                            video: None,
                                                                            data: None,
                                                                            html: {
                                                                                Some(HtmlAsset {
                                                                                    html: None,
                                                                                    link: Some(endcard_url.clone()),
                                                                                    len: None,
                                                                                })
                                                                            },
                                                                            app: None,
                                                                        });
                                                                    },
                                                                    None => (),
                                                                }
                                                                match &video.endcard_html {
                                                                    Some(endcard_html) => {
                                                                        asset_vec.push(Asset {
                                                                            id: assets.consume_asset("video#end#html"),
                                                                            req: 0,
                                                                            img: None,
                                                                            title: None,
                                                                            video: None,
                                                                            data: None,
                                                                            html: {
                                                                                Some(HtmlAsset {
                                                                                    html: Some(endcard_html.clone()),
                                                                                    link: None,
                                                                                    len: Some(endcard_html.len() as i32),
                                                                                })
                                                                            },
                                                                            app: None,
                                                                        });
                                                                    },
                                                                    None => (),
                                                                }
                                                                match &video.end_title {
                                                                    Some(end_title) => {
                                                                        asset_vec.push(Asset {
                                                                            id: assets.consume_asset("video#end#title"),
                                                                            req: 0,
                                                                            img: None,
                                                                            title: {
                                                                                Some(TitleAsset {
                                                                                    text: end_title.clone(),
                                                                                    subtitle: None,
                                                                                    desc: video.end_description.clone(),
                                                                                    len: Some(end_title.len() as i32),
                                                                                })
                                                                            },
                                                                            video: None,
                                                                            data: None,
                                                                            html: None,
                                                                            app: None,
                                                                        });
                                                                    },
                                                                    None => (),
                                                                }
                                                                match &video.end_button_text {
                                                                    Some(end_button_text) => {
                                                                        asset_vec.push(Asset {
                                                                            id: assets.consume_asset("video#end#button#text"),
                                                                            req: 0,
                                                                            img: None,
                                                                            title: None,
                                                                            video: None,
                                                                            data: {
                                                                                Some(DataAsset {
                                                                                    value: end_button_text.clone(),
                                                                                    len: Some(end_button_text.clone().len() as i32),
                                                                                    datatype: Some(12),
                                                                                })
                                                                            },
                                                                            html: None,
                                                                            app: None,
                                                                        });
                                                                    },
                                                                    None => (),
                                                                }
                                                                match &video.end_button_url {
                                                                    Some(end_button_url) => {
                                                                        link_asset.url = end_button_url.clone();
                                                                    },
                                                                    None => (),
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                        match &ad.icon_url {
                                                            Some(icon_url) => {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("icon"),
                                                                    req: 0,
                                                                    img: Some(ImageAsset {
                                                                        url: icon_url.clone(),
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
                                                        match &ad.title {
                                                            Some(title) => {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("title"),
                                                                    req: 1,
                                                                    title: Some(TitleAsset {
                                                                        text: title.clone(),
                                                                        subtitle: None,
                                                                        desc: ad.description.clone(),
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
                                                        match &ad.images {
                                                            Some(images) => {
                                                                if assets.get_asset_size("img") > 0 {
                                                                    for image in images.iter() {
                                                                        asset_vec.push(Asset {
                                                                            id: assets.consume_asset("img"),
                                                                            req: 1,
                                                                            img: {
                                                                                Some(ImageAsset {
                                                                                    url: image.clone(),
                                                                                    mime: None,
                                                                                    w: Some(ad.width),
                                                                                    h: Some(ad.height),
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
                                                                }
                                                                if assets.get_asset_size("thumb") > 0 {
                                                                    for image in images.iter() {
                                                                        asset_vec.push(Asset {
                                                                            id: assets.consume_asset("thumb"),
                                                                            req: 1,
                                                                            img: {
                                                                                Some(ImageAsset {
                                                                                    url: image.clone(),
                                                                                    mime: None,
                                                                                    w: Some(ad.width),
                                                                                    h: Some(ad.height),
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

                                                        match &ad.app {
                                                            Some(app) => {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("app"),
                                                                    req: 0,
                                                                    app: Some(AppAsset {
                                                                        name: {
                                                                            match &app.name {
                                                                                Some(name) => name.clone(),
                                                                                None => {
                                                                                    match &ad.app_name {
                                                                                        Some(app_name) => app_name.clone(),
                                                                                        None => "".to_string(),
                                                                                    }
                                                                                },
                                                                            }
                                                                        },
                                                                        desc: app.info.clone(),
                                                                        descurl: None,
                                                                        domain: None,
                                                                        bundle: app.pack.clone(),
                                                                        ver: app.ver.clone(),
                                                                        developer: app.devs.clone(),
                                                                        icon: app.icon.clone(),
                                                                        storeid: None,
                                                                        storeurl: None,
                                                                        paid: 0,
                                                                        size: None,
                                                                        md5: app.md5.clone(),
                                                                        registration: None,
                                                                        privacy: None,
                                                                        privacyurl: app.pcyurl.clone(),
                                                                        permission: app.pmsdesc.clone(),
                                                                        permissionurl: app.pmsurl.clone(),
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

                                                    for event in &ad.impression_urls {
                                                        event_vec.push(Event {
                                                            eventtype: 501,
                                                            method: 1,
                                                            url: replace_macro(event),
                                                            header: None,
                                                            content: None,
                                                        });
                                                    }
                                                    for event in &ad.click_urls {
                                                        event_vec.push(Event {
                                                            eventtype: 502,
                                                            method: 1,
                                                            url: replace_macro(event),
                                                            header: None,
                                                            content: None,
                                                        });
                                                    }
                                                    match &ad.deeplink_try_urls {
                                                        Some(deeplink_try_urls) => {
                                                            for event in deeplink_try_urls {
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
                                                    match &ad.deeplink_success_urls {
                                                        Some(deeplink_success_urls) => {
                                                            for event in deeplink_success_urls {
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
                                                    match &ad.deeplink_fail_urls {
                                                        Some(deeplink_fail_urls) => {
                                                            for event in deeplink_fail_urls {
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
                                                    match &ad.download_begin_urls {
                                                        Some(download_begin_urls) => {
                                                            for event in download_begin_urls {
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
                                                    match &ad.download_end_urls {
                                                        Some(download_end_urls) => {
                                                            for event in download_end_urls {
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
                                                    match &ad.install_begin_urls {
                                                        Some(install_begin_urls) => {
                                                            for event in install_begin_urls {
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
                                                    match &ad.install_end_urls {
                                                        Some(install_end_urls) => {
                                                            for event in install_end_urls {
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
                                                    match &ad.active_urls {
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

                                                    match &ad.video {
                                                        Some(video) => {
                                                            match &video.video_begin_urls {
                                                                Some(video_begin_urls) => {
                                                                    for event in video_begin_urls {
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
                                                            match &video.video_first_quartile_urls {
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
                                                            match &video.video_midpoint_urls {
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
                                                            match &video.video_third_quartile_urls {
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
                                                            match &video.video_end_urls {
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
                                                            match &video.video_mute_urls {
                                                                Some(video_mute_urls) => {
                                                                    for event in video_mute_urls {
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
                                                            match &video.video_unmute_urls {
                                                                Some(video_unmute_urls) => {
                                                                    for event in video_unmute_urls {
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
                                                            match &video.video_skip_urls {
                                                                Some(video_skip_urls) => {
                                                                    for event in video_skip_urls {
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
                                                            match &video.video_close_urls {
                                                                Some(video_close_urls) => {
                                                                    for event in video_close_urls {
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
                                                            match &video.video_pause_urls {
                                                                Some(video_pause_urls) => {
                                                                    for event in video_pause_urls {
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
                                                            match &video.video_resume_urls {
                                                                Some(video_resume_urls) => {
                                                                    for event in video_resume_urls {
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
                                                            match &video.video_replay_urls {
                                                                Some(video_replay_urls) => {
                                                                    for event in video_replay_urls {
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
                                                            match &video.video_fullscreen_urls {
                                                                Some(video_fullscreen_urls) => {
                                                                    for event in video_fullscreen_urls {
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
                                                            match &video.video_exit_fullscreen_urls {
                                                                Some(video_exit_fullscreen_urls) => {
                                                                    for event in video_exit_fullscreen_urls {
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
                                                            match &video.video_up_scroll_urls {
                                                                Some(video_up_scroll_urls) => {
                                                                    for event in video_up_scroll_urls {
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
                                                            match &video.video_down_scroll_urls {
                                                                Some(video_down_scroll_urls) => {
                                                                    for event in video_down_scroll_urls {
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

                                                    event_vec
                                                }
                                            },
                                            advertiser: None,
                                            advertisericon: None,
                                        },
                                    };

                                    bids.push(bid);
                                }
                            },
                            None => (),
                        }
                        bids
                    }
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
            let pool_spinview_lock = pool.pool_spinview.clone();
            let pool_spinview = pool_spinview_lock.read().unwrap();
            pool_spinview.clone()
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

        let cipher = Aes256EcbEnc::new(key[0..32].into())
            .encrypt_padded_mut::<Pkcs7>(&mut buffer, pos)
            .unwrap();

        let message_base64 = BASE64_STANDARD.encode(cipher);

        message_base64.replace("+", "-").replace("/", "_").replace("=", "")
    }

}

fn replace_macro(orig: &String) -> String {
    let mut replaced = orig.clone();

    replaced = replaced.replace("__REQ_WIDTH__", "__WIDTH__");
    replaced = replaced.replace("__REQ_HEIGHT__", "__HEIGHT__");

    replaced = replaced.replace("__AB_DOWN_X__", "__ABS_DOWN_X__");
    replaced = replaced.replace("__AB_DOWN_Y__", "__ABS_DOWN_Y__");
    replaced = replaced.replace("__AB_UP_X__", "__ABS_UP_X__");
    replaced = replaced.replace("__AB_UP_Y__", "__ABS_UP_Y__");
    replaced = replaced.replace("__DP_DOWN_X__", "__DOWN_DP_X__");
    replaced = replaced.replace("__DP_DOWN_Y__", "__DOWN_DP_Y__");
    replaced = replaced.replace("__DP_UP_X__", "__UP_DP_X__");
    replaced = replaced.replace("__DP_UP_Y__", "__UP_DP_Y__");

    replaced = replaced.replace("__EVENT_TIME__", "__TS_S__");
    replaced = replaced.replace("__EVENT_TIME_MS__", "__TS__");
    replaced = replaced.replace("__EVENT_TIME_START__", "__EVENT_TIME_START__");
    replaced = replaced.replace("__EVENT_TIME_END__", "__EVENT_TIME_END__");

    replaced = replaced.replace("__DPLINK__", "__DP_STATUS__");

    replaced = replaced.replace("__DURATION__", "__VIDEO_TIME__");
    replaced = replaced.replace("__PLAY_BEGIN_TIME__", "__VIDEO_BEGIN_TIME__");
    replaced = replaced.replace("__PLAY_END_TIME__", "__VIDEO_END_TIME__");
    replaced = replaced.replace("__PLAY_FIRST_FRAME__", "__VIDEO_FIRST_FRAME__");
    replaced = replaced.replace("__PLAY_LAST_FRAME__", "__VIDEO_LAST_FRAME__");
    replaced = replaced.replace("__SCENE__", "__VIDEO_PLAY_SCENE__");
    replaced = replaced.replace("__TYPE__", "__VIDEO_PLAY_TYPE__");
    replaced = replaced.replace("__BEHAIVOR__", "__VIDEO_PLAY_TRIGGER__");
    replaced = replaced.replace("__STATUS__", "__VIDEO_PLAY_STATUS__");
    replaced = replaced.replace("__PLAY_TIME__", "__VIDEO_PLAY_PROGRESS_S__");
    replaced = replaced.replace("__PLAY_TIME_MS__", "__VIDEO_PLAY_PROGRESS__");

    replaced
}
