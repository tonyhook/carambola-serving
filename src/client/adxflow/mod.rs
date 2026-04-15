use std::{collections::HashMap, time::Duration};

use reqwest::Url;

use crate::{protocol::*, Assets, Cache, Client, Connection, HttpPool, Identifiers, Price, ResultMessage};

pub mod adm;
pub mod app;
pub mod bid;
pub mod bidinfo;
pub mod caid;
pub mod device;
pub mod device_ext;
pub mod ext;
pub mod geo;
pub mod img;
pub mod imp;
pub mod request;
pub mod response;
pub mod seatbid;
pub mod user;
pub mod video;
pub mod wx;

pub use adm::AdxflowAdm;
pub use app::{AdxflowApp, AdxflowRespApp};
pub use bid::AdxflowBid;
pub use bidinfo::AdxflowBidinfo;
pub use caid::AdxflowCaid;
pub use device::AdxflowDevice;
pub use device_ext::AdxflowDeviceExt;
pub use ext::AdxflowExt;
pub use geo::AdxflowGeo;
pub use img::AdxflowImg;
pub use imp::AdxflowImp;
pub use request::AdxflowRequest;
pub use response::AdxflowResponse;
pub use seatbid::AdxflowSeatbid;
pub use user::AdxflowUser;
pub use video::AdxflowVideo;
pub use wx::AdxflowWx;

pub struct Adxflow {
}

impl Client for Adxflow {
    async fn request(request: &Request, connection: &Connection, pool: &HttpPool, cache: &Cache) -> Result<Response, ResultMessage> {
        let adid = connection.client_tag_id.split('|').nth(0).unwrap();
        let token = connection.client_tag_id.split('|').nth(1).unwrap();
        let madid = connection.client_tag_id.split('|').nth(2).unwrap();

        let request_id = cache.get_sequence();
        let mut assets = Assets::new(request);
        let identifiers = Identifiers::new(request);
        let title_size = assets.get_asset_size("title");
        let desc_size = assets.get_asset_size("data#desc") + assets.get_asset_size("data#desc2");
        let img_size = assets.get_asset_size("img") + assets.get_asset_size("thumb");
        let icon_size = assets.get_asset_size("icon");
        let video_size = assets.get_asset_size("video");
        let video_cover_size = assets.get_asset_size("video#cover");
        let video_end_img_size = assets.get_asset_size("video#end#img");
        let html_size = assets.get_asset_size("html") + assets.get_asset_size("video#end#html");

        let request_adxflow = AdxflowRequest {
            id: {
                Some(request_id.to_string())
            },
            token: {
                token.to_string()
            },
            app: {
                match &request.context.app {
                    Some(app) => AdxflowApp {
                        id: {
                            Some(match connection.client_media_apppackage.clone().or(app.bundle.clone()) {
                                Some(bundle) => bundle,
                                None => "".to_string(),
                            })
                        },
                        name: {
                            connection.client_media_appname.clone().unwrap_or_else(|| app.name.clone())
                        },
                        bundle: {
                            Some(match connection.client_media_apppackage.clone().or(app.bundle.clone()) {
                                Some(bundle) => bundle,
                                None => "".to_string(),
                            })
                        },
                        ver: {
                            app.ver.clone()
                        },
                        domain: {
                            app.domain.clone()
                        },
                        storeurl: {
                            app.storeurl.clone()
                        },
                        cat: {
                            app.domain.as_ref().map(split_csv)
                        },
                        keywords: {
                            request.context.user.keywords.clone()
                        },
                    },
                    None => AdxflowApp {
                        id: None,
                        name: connection.client_media_appname.clone().unwrap_or_else(|| "".to_string()),
                        bundle: None,
                        ver: None,
                        domain: None,
                        storeurl: None,
                        cat: None,
                        keywords: None,
                    },
                }
            },
            device: {
                AdxflowDevice {
                    imei: identifiers.get_id(501, 0).map(|uid| uid.id.clone()),
                    imeimd5: identifiers.get_id(502, 0).map(|uid| uid.id.clone()),
                    oaid: identifiers.get_id(505, 0).map(|uid| uid.id.clone()),
                    oaidmd5: identifiers.get_id(506, 0).map(|uid| uid.id.clone()),
                    aid: identifiers.get_id(509, 0).map(|uid| uid.id.clone()),
                    aidmd5: identifiers.get_id(510, 0).map(|uid| uid.id.clone()),
                    idfa: identifiers.get_id(507, 0).map(|uid| uid.id.clone()),
                    idfamd5: identifiers.get_id(508, 0).map(|uid| uid.id.clone()),
                    idfv: identifiers.get_id(515, 0).map(|uid| uid.id.clone()),
                    openuuid: request.context.device.mntid.clone(),
                    caid: identifiers.get_id(513, 0).map(|uid| uid.id.clone()),
                    caidver: identifiers.get_id(513, 0).and_then(|uid| uid.ver.clone()),
                    caidmd5: None,
                    caid_infos: identifiers.get_ids(513).map(|uids| {
                        uids.into_iter().map(|uid| AdxflowCaid {
                            caid: Some(uid.id.clone()),
                            version: uid.ver.clone(),
                            caid_md5: None,
                        }).collect::<Vec<AdxflowCaid>>()
                    }).filter(|caids| !caids.is_empty()),
                    os: match request.context.device.os {
                        Some(2) => 1,
                        Some(13) => 2,
                        _ => 0,
                    },
                    osv: request.context.device.osv.clone(),
                    make: request.context.device.make.clone().or(request.context.device.brand.clone()),
                    model: request.context.device.model.clone(),
                    device_type: Some(match request.context.device.devicetype {
                        Some(1) | Some(4) => 1,
                        Some(5) => 2,
                        Some(2) => 3,
                        _ => 0,
                    }),
                    ip: request.context.device.ip.clone(),
                    ipv6: request.context.device.ipv6.clone(),
                    mac: identifiers.get_id(511, 0).map(|uid| uid.id.clone()),
                    macmd5: identifiers.get_id(512, 0).map(|uid| uid.id.clone()),
                    ua: request.context.device.ua.clone(),
                    carrier: Some({
                        if let Some(carrier) = request.context.device.carrier.as_ref() {
                            match carrier.as_str() {
                                "cmcc" => 1,
                                "unicom" => 2,
                                "telecom" => 3,
                                _ => 0,
                            }
                        } else if let Some(mccmnc) = request.context.device.mccmnc.as_ref() {
                            if mccmnc.starts_with("46000") || mccmnc.starts_with("46002") || mccmnc.starts_with("46004") || mccmnc.starts_with("46007") || mccmnc.starts_with("46008") {
                                1
                            } else if mccmnc.starts_with("46001") || mccmnc.starts_with("46006") || mccmnc.starts_with("46009") {
                                2
                            } else if mccmnc.starts_with("46003") || mccmnc.starts_with("46005") || mccmnc.starts_with("46011") {
                                3
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    }),
                    connection_type: Some(match request.context.device.contype {
                        Some(2) => 1,
                        Some(4) => 2,
                        Some(5) => 3,
                        Some(6) => 4,
                        Some(7) => 5,
                        _ => 0,
                    }),
                    w: request.context.device.w,
                    h: request.context.device.h,
                    ppid: request.context.device.ppi,
                    density: request.context.device.pxratio,
                    orientation: request.context.device.orientation.map(|orientation| match orientation {
                        501 => 1,
                        502 => 2,
                        _ => 0,
                    }),
                    geo: request.context.device.geo.as_ref().map(|geo| AdxflowGeo {
                        lat: geo.lat,
                        lon: geo.lon,
                        r#type: Some(1),
                        city: geo.city.clone(),
                        prov: geo.province.clone(),
                        contry: geo.country.clone(),
                    }),
                    ext: Some(AdxflowDeviceExt {
                        hwv: request.context.device.hwv.clone(),
                        android_ad_id: identifiers.get_id(509, 0).map(|uid| uid.id.clone()),
                        serial_number: request.context.device.serial.clone(),
                        app_store_version: request.context.device.storev.clone(),
                        hms_ver_code: request.context.device.hmsv.clone(),
                        os_com_time: {
                            request.context.device.romtime.as_ref().and_then(|romtime| {
                                romtime.split(".").next().and_then(|romtime| romtime.parse::<i64>().ok())
                            })
                        },
                        phone_name: identifiers.get_id(527, 0).map(|uid| uid.id.clone()),
                        memory: {
                            request.context.device.sysmemory.map(|v| (v / 1024 / 1024 / 1024) as i32)
                        },
                        disk: {
                            request.context.device.sysdisksize.map(|v| (v / 1024 / 1024 / 1024) as i32)
                        },
                        cpu_number: request.context.device.syscpu,
                        cpu_frequency: request.context.device.syscpufreq,
                        battery_status: {
                            request.context.device.sysbatterystatus.map(|v| match v {
                                0 => "Unkown".to_string(),
                                1 => "Unplugged".to_string(),
                                2 => "Charging".to_string(),
                                3 => "Full".to_string(),
                                _ => "Unkown".to_string(),
                            })
                        },
                        battery_power: request.context.device.sysbatterypower,
                        ssid: identifiers.get_id(524, 0).map(|uid| uid.id.clone()),
                        wifi_mac: identifiers.get_id(522, 0).map(|uid| uid.id.clone()),
                        lmt: request.context.device.lmt,
                        country: request.context.device.country.clone(),
                        language: request.context.device.lang.clone(),
                        installs: request.context.device.app.as_ref().map(|app| split_csv(app)),
                        ali_aaid: identifiers.get_id(514, 0).map(|uid| uid.id.clone()),
                        birth_time: request.context.device.inittime.clone(),
                        boot_time: {
                            request.context.device.boottime.as_ref().map(|boottime| boottime.split(".").next().unwrap_or(boottime.as_str()).to_string())
                        },
                        update_time: {
                            request.context.device.updatetime.as_ref().map(|updatetime| updatetime.split(".").next().unwrap_or(updatetime.as_str()).to_string())
                        },
                        boot_mark: request.context.device.bootmark.clone(),
                        update_mark: request.context.device.updatemark.clone(),
                        miui_ver: request.context.device.uiv.clone(),
                        rom_ver: request.context.device.romv.clone().or(request.context.device.uiv.clone()),
                        skadnetwork_versions: request.context.device.skan.clone(),
                    }),
                }
            },
            imp: {
                vec![AdxflowImp {
                    id: {
                        Some(request.item[0].id.clone())
                    },
                    adid: {
                        adid.to_string()
                    },
                    madid: {
                        madid.to_string()
                    },
                    adtype: {
                        Some(if video_size > 0 {
                            3
                        } else if img_size == 0
                            && icon_size == 0
                            && title_size + desc_size + html_size > 0 {
                            4
                        } else if assets.get_asset_total_size() > 0 {
                            2
                        } else {
                            1
                        })
                    },
                    ftype: {
                        Some(if video_size > 0 {
                            if request.item[0].spec.reward > 0 {
                                vec![11]
                            } else if video_cover_size > 0 && video_end_img_size > 0 {
                                vec![9]
                            } else if video_cover_size > 0 && video_end_img_size == 0 {
                                vec![8]
                            } else {
                                vec![10]
                            }
                        } else if img_size == 0
                            && icon_size == 0
                            && title_size + desc_size + html_size > 0 {
                            if title_size + desc_size + html_size > 1 {
                                vec![13]
                            } else {
                                vec![12]
                            }
                        } else if img_size <= 1 && title_size + desc_size <= 1 {
                            vec![2]
                        } else if img_size >= 3 && title_size + desc_size <= 1 {
                            vec![3]
                        } else if img_size == 2 && title_size + desc_size <= 1 {
                            vec![4]
                        } else if img_size <= 1 && title_size + desc_size > 1 {
                            vec![5]
                        } else if img_size >= 3 && title_size + desc_size > 1 {
                            vec![6]
                        } else if img_size == 2 && title_size + desc_size > 1 {
                            vec![7]
                        } else {
                            vec![1]
                        })
                    },
                    interact: {
                        Some(vec![1, 2, 3, 4, 5])
                    },
                    bidinfo: {
                        Some(AdxflowBidinfo {
                            bidfloor: {
                                Some(Price::to_client(connection, request.item[0].flr.map(f64::from)) as i32)
                            },
                            bidtype: {
                                Some(1)
                            },
                        })
                    },
                }]
            },
            user: {
                if request.context.user.id.is_none()
                    && request.context.user.gender.is_none()
                    && request.context.user.yob.is_none()
                    && request.context.user.keywords.is_none()
                {
                    None
                } else {
                    Some(AdxflowUser {
                        id: request.context.user.id.clone(),
                        gender: request.context.user.gender.as_ref().and_then(|gender| match gender.as_str() {
                            "M" | "m" => Some("M".to_string()),
                            "F" | "f" => Some("F".to_string()),
                            _ => None,
                        }),
                        yob: request.context.user.yob,
                        keywords: request.context.user.keywords.as_ref().map(split_csv),
                    })
                }
            },
            bcat: {
                None
            },
            badv: {
                None
            },
            tmax: {
                Some(connection.timeout as i32)
            },
            test: {
                Some(connection.test)
            },
            secure: {
                Some(if connection.client_port == 443 { 1 } else { 0 })
            },
        };

        let client = {
            let pool_adxflow_lock = pool.pool_adxflow.clone();
            let pool_adxflow = pool_adxflow_lock.read().unwrap();
            pool_adxflow.clone()
        };

        let response_adxflow_raw = client
            .post(if connection.test {
                "http://test.ex.adxflow.com/api/v1/app"
            } else {
                "http://ex.adxflow.com/api/v1/app"
            })
            .json(&request_adxflow)
            .header("Accept-Encoding", "gzip")
            .header("Connection", "keep-alive")
            .header("Content-Type", "application/json;charset=UTF-8")
            .timeout(Duration::from_millis(connection.timeout))
            .send()
            .await;

        let response_adxflow: AdxflowResponse = match response_adxflow_raw {
            Ok(response_adxflow_raw) => {
                let status = response_adxflow_raw.status();
                if status == 204 {
                    return Err(ResultMessage {
                        code: 993,
                        message: "".to_string()
                    });
                } else if status != 200 {
                    return Err(ResultMessage {
                        code: 992,
                        message: match response_adxflow_raw.text().await {
                            Ok(text) => format!("upstream error {}: {}", status, text),
                            Err(_) => format!("upstream error {}", status),
                        },
                    });
                }

                match response_adxflow_raw.text().await {
                    Ok(text) => match serde_json::from_str::<AdxflowResponse>(&text) {
                        Ok(json) => json,
                        Err(error) => return Err(ResultMessage {
                            code: 997,
                            message: error.to_string()
                        }),
                    },
                    Err(error) => return Err(ResultMessage {
                        code: 992,
                        message: error.to_string()
                     }),
                }
            }
            Err(error) => {
                if error.is_timeout() {
                    return Err(ResultMessage {
                        code: 991,
                        message: "upstream request timeout".to_string()
                    });
                }
                return Err(ResultMessage {
                    code: 992,
                    message: format!("upstream request failed: {:?}", error)
                });
            }
        };

        Ok(Response {
            id: {
                request.id.clone()
            },
            nbr: {
                None
            },
            seatbid: {
                match &response_adxflow.seatbid {
                    Some(seatbids) => {
                        match seatbids.iter().find_map(|seatbid| seatbid.bid.as_ref().and_then(|bids| bids.first())) {
                            Some(bid) => {
                                match &bid.adm {
                                    Some(adm) => {
                                        Some(vec![Seatbid {
                                            bid: {
                                                let mut bids = vec![];
                                                let link_asset = LinkAsset {
                                                    linktype: {
                                                        if adm.wx.as_ref().and_then(|wx| wx.program_id.clone()).is_some()
                                                            || adm.wx.as_ref().and_then(|wx| wx.path.clone()).is_some() {
                                                            1
                                                        } else {
                                                            match adm.interaction_type.unwrap_or_default() {
                                                                2 => 2,
                                                                5 => 3,
                                                                _ => 1,
                                                            }
                                                        }
                                                    },
                                                    universallink: None,
                                                    storeid: adm.app.as_ref().and_then(|app| app.app_id.clone()),
                                                    deeplink: adm.deeplink.clone(),
                                                    quickapplink: None,
                                                    wechatmppath: adm.wx.as_ref().and_then(|wx| wx.path.clone()),
                                                    wechatmpid: adm.wx.as_ref().and_then(|wx| wx.program_id.clone()),
                                                    marketurl: None,
                                                    downloadurl: if adm.wx.as_ref().and_then(|wx| wx.program_id.clone()).is_some()
                                                        || adm.wx.as_ref().and_then(|wx| wx.path.clone()).is_some() {
                                                        None
                                                    } else if adm.interaction_type.unwrap_or_default() == 2 || adm.interaction_type.unwrap_or_default() == 5 {
                                                        adm.land.clone()
                                                    } else {
                                                        None
                                                    },
                                                    url: adm.land.clone().unwrap_or_default(),
                                                    urlfb: None,
                                                };

                                                bids.push(Bid {
                                                    id: {
                                                        Some(request_id.to_string())
                                                    },
                                                    item: {
                                                        request.item[0].id.clone()
                                                    },
                                                    price: {
                                                        bid.price.unwrap_or(connection.default_price)
                                                    },
                                                    burl: {
                                                        match &bid.nurl {
                                                            Some(nurl) => {
                                                                Some(vec![replace_macro(request, &identifiers, nurl)])
                                                            },
                                                            None => None,
                                                        }
                                                    },
                                                    lurl: {
                                                        None
                                                    },
                                                    media: Ad {
                                                        id: {
                                                            response_adxflow.id.clone().unwrap_or_else(|| request_id.to_string())
                                                        },
                                                        display: Display {
                                                            w: {
                                                                None
                                                            },
                                                            h: {
                                                                None
                                                            },
                                                            banner: {
                                                                if assets.get_banner_size() > 0 {
                                                                    adm.img.as_ref().and_then(|imgs| imgs.first().map(|img| Banner {
                                                                        img: img.url.clone(),
                                                                        link: Some(link_asset.clone()),
                                                                    }))
                                                                } else {
                                                                    None
                                                                }
                                                            },
                                                            native: {
                                                                if assets.get_asset_total_size() == 0 {
                                                                    None
                                                                } else {
                                                                    let mut asset_vec = vec![];

                                                                    if let Some(video) = &adm.video {
                                                                        asset_vec.push(Asset {
                                                                            id: assets.consume_asset("video"),
                                                                            req: 1,
                                                                            title: None,
                                                                            img: None,
                                                                            video: Some(VideoAsset {
                                                                                url: video.url.clone(),
                                                                                mime: None,
                                                                                w: video.w,
                                                                                h: video.h,
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
                                                                    }

                                                                    if let Some(title) = &adm.title {
                                                                        asset_vec.push(Asset {
                                                                            id: assets.consume_asset("title"),
                                                                            req: 1,
                                                                            title: Some(TitleAsset {
                                                                                text: title.clone(),
                                                                                subtitle: adm.ext.as_ref().and_then(|ext| ext.subtitle.clone()),
                                                                                desc: adm.desc.clone(),
                                                                                len: Some(title.len() as i32),
                                                                            }),
                                                                            img: None,
                                                                            video: None,
                                                                            data: None,
                                                                            html: None,
                                                                            app: None,
                                                                        });
                                                                    }

                                                                    if let Some(desc) = &adm.desc {
                                                                        asset_vec.push(Asset {
                                                                            id: assets.consume_asset("data#desc"),
                                                                            req: 0,
                                                                            title: None,
                                                                            img: None,
                                                                            video: None,
                                                                            data: Some(DataAsset {
                                                                                value: desc.clone(),
                                                                                len: Some(desc.len() as i32),
                                                                                datatype: Some(2),
                                                                            }),
                                                                            html: None,
                                                                            app: None,
                                                                        });
                                                                    }

                                                                    if let Some(btn_text) = adm.ext.as_ref().and_then(|ext| ext.btn_text.clone()) {
                                                                        asset_vec.push(Asset {
                                                                            id: assets.consume_asset("data#ctatext"),
                                                                            req: 0,
                                                                            title: None,
                                                                            img: None,
                                                                            video: None,
                                                                            data: Some(DataAsset {
                                                                                value: btn_text.clone(),
                                                                                len: Some(btn_text.len() as i32),
                                                                                datatype: Some(12),
                                                                            }),
                                                                            html: None,
                                                                            app: None,
                                                                        });
                                                                    }

                                                                    if let Some(icon) = adm.ext.as_ref().and_then(|ext| ext.icon.clone()).or(adm.app.as_ref().and_then(|app| app.icon.clone())) {
                                                                        asset_vec.push(Asset {
                                                                            id: assets.consume_asset("icon"),
                                                                            req: 0,
                                                                            title: None,
                                                                            img: Some(ImageAsset {
                                                                                url: icon,
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
                                                                    }

                                                                    if let Some(imgs) = &adm.img {
                                                                        for img in imgs {
                                                                            asset_vec.push(Asset {
                                                                                id: assets.consume_asset("img"),
                                                                                req: 1,
                                                                                title: None,
                                                                                img: Some(ImageAsset {
                                                                                    url: img.url.clone(),
                                                                                    mime: None,
                                                                                    w: img.w,
                                                                                    h: img.h,
                                                                                    imagetype: Some(3),
                                                                                }),
                                                                                video: None,
                                                                                data: None,
                                                                                html: None,
                                                                                app: None,
                                                                            });
                                                                        }
                                                                    }

                                                                    if let Some(app) = &adm.app {
                                                                        asset_vec.push(Asset {
                                                                            id: assets.consume_asset("app"),
                                                                            req: 0,
                                                                            title: None,
                                                                            img: None,
                                                                            video: None,
                                                                            data: None,
                                                                            html: None,
                                                                            app: Some(AppAsset {
                                                                                name: app.name.clone().unwrap_or_default(),
                                                                                desc: adm.desc.clone(),
                                                                                descurl: None,
                                                                                domain: None,
                                                                                bundle: app.package_name.clone(),
                                                                                ver: None,
                                                                                developer: adm.ext.as_ref().and_then(|ext| ext.ader_name.clone()),
                                                                                icon: app.icon.clone(),
                                                                                storeid: app.app_id.clone(),
                                                                                storeurl: None,
                                                                                paid: 0,
                                                                                size: None,
                                                                                md5: None,
                                                                                registration: None,
                                                                                privacy: None,
                                                                                privacyurl: None,
                                                                                permission: None,
                                                                                permissionurl: None,
                                                                            }),
                                                                        });
                                                                    }

                                                                    Some(Native { asset: asset_vec, link: Some(link_asset.clone()) })
                                                                }
                                                            },
                                                            event: {
                                                                let mut event_vec = vec![];

                                                                for (urls, eventtype) in [
                                                                    (&bid.imp_trackers, 501),
                                                                    (&bid.click_trackers, 502),
                                                                    (&bid.deeplink_start_trackers, 503),
                                                                    (&bid.deeplink_comp_trackers, 504),
                                                                    (&bid.deeplink_fail_trackers, 505),
                                                                    (&bid.down_start_trackers, 601),
                                                                    (&bid.down_comp_trackers, 602),
                                                                    (&bid.install_start_trackers, 603),
                                                                    (&bid.install_comp_trackers, 604),
                                                                    (&bid.video_start_trackers, 701),
                                                                    (&bid.video_quarter_trackers, 702),
                                                                    (&bid.video_half_trackers, 703),
                                                                    (&bid.video_threefourhs_trackers, 704),
                                                                    (&bid.video_comp_trackers, 705),
                                                                    (&bid.video_skip_trackers, 710),
                                                                    (&bid.video_close_trackers, 711),
                                                                    (&bid.video_pause_trackers, 708),
                                                                    (&bid.video_resume_trackers, 709),
                                                                    (&bid.video_full_trackers, 715),
                                                                    (&bid.video_ext_full_trackers, 716),
                                                                    (&bid.video_mute_trackers, 713),
                                                                    (&bid.video_unmute_trackers, 714),
                                                                ] {
                                                                    if let Some(urls) = urls {
                                                                        for url in urls {
                                                                            event_vec.push(Event {
                                                                                eventtype,
                                                                                method: 1,
                                                                                url: replace_macro(request, &identifiers, url),
                                                                                header: None,
                                                                                content: None,
                                                                            });
                                                                        }
                                                                    }
                                                                }

                                                                match &bid.click_area_report_url {
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
                                                                                        url: replace_macro(request, &identifiers, click_area_report_url),
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
                                                            },
                                                        },
                                                        advertiser: {
                                                            adm.ext.as_ref().and_then(|ext| ext.ader_name.clone())
                                                        },
                                                        advertisericon: {
                                                            adm.ext.as_ref().and_then(|ext| ext.logo.clone().or(ext.icon.clone())).or(adm.app.as_ref().and_then(|app| app.icon.clone()))
                                                        },
                                                    },
                                                });

                                                bids
                                            },
                                        }])
                                    },
                                    None => return Err(ResultMessage {
                                        code: 993,
                                        message: "upstream response missing adm".to_string(),
                                    }),
                                }
                            },
                            None => return Err(ResultMessage {
                                code: 993,
                                message: "".to_string(),
                            }),
                        }
                    },
                    None => return Err(ResultMessage {
                        code: 993,
                        message: "".to_string(),
                    }),
                }
            },
        })
    }

    async fn bidding_notify_win(url: String, win_price: i32, next_price: i32, _iv: &String, _connection: &Connection, pool: &HttpPool) {
        let replaced_url = url
            .replace("__WIN_PRICE__", &win_price.to_string())
            .replace("${AUCTION_PRICE}", &win_price.to_string())
            .replace("__PRICE__", &win_price.to_string())
            .replace("__2ND_PRICE__", &next_price.to_string())
            .replace("__LOSS_PR__", &next_price.to_string());

        let client = {
            let pool_adxflow_lock = pool.pool_adxflow.clone();
            let pool_adxflow = pool_adxflow_lock.read().unwrap();
            pool_adxflow.clone()
        };
        let _ = client.get(replaced_url).send().await;
    }

    async fn bidding_notify_lose(url: String, lose_price: i32, lose_reason: i32, lose_adn_name: &String, _iv: &String, _connection: &Connection, pool: &HttpPool) {
        let replaced_url = url
            .replace("__LOSE_PRICE__", &lose_price.to_string())
            .replace("__BID_ECPM__", &lose_price.to_string())
            .replace("__AD_ECPM__", &lose_price.to_string())
            .replace("__BID_FAIL_REASON__", &lose_reason.to_string())
            .replace("__LOSE_ADN_NAME__", lose_adn_name)
            .replace("__ADN_NAME__", lose_adn_name);

        let client = {
            let pool_adxflow_lock = pool.pool_adxflow.clone();
            let pool_adxflow = pool_adxflow_lock.read().unwrap();
            pool_adxflow.clone()
        };
        let _ = client.get(replaced_url).send().await;
    }

    fn encrypt_price(price: i32, _iv: &String, _connection: &Connection) -> String {
        price.to_string()
    }
}

fn replace_macro(request: &Request, identifiers: &Identifiers, orig: &String) -> String {
    let mut replaced = orig.clone();

    replaced = replaced.replace("${AUCTION_PRICE}", "__WIN_PRICE__");
    replaced = replaced.replace("__DOWN_X__", "__R_DOWN_X__");
    replaced = replaced.replace("__DOWN_Y__", "__R_DOWN_Y__");
    replaced = replaced.replace("__UP_X__", "__R_UP_X__");
    replaced = replaced.replace("__UP_Y__", "__R_UP_Y__");
    replaced = replaced.replace("__ACTION_ID__", "__CLICK_ID__");
    replaced = replaced.replace("__OFFSET_X__", "__R_DOWN_X__");
    replaced = replaced.replace("__OFFSET_Y__", "__R_DOWN_Y__");
    replaced = replaced.replace("__AB_DOWN_X__", "__ABS_DOWN_X__");
    replaced = replaced.replace("__AB_DOWN_Y__", "__ABS_DOWN_Y__");
    replaced = replaced.replace("__AB_UP_X__", "__ABS_UP_X__");
    replaced = replaced.replace("__AB_UP_Y__", "__ABS_UP_Y__");
    replaced = replaced.replace("__DP_DOWN_X__", "__R_DOWN_DP_X__");
    replaced = replaced.replace("__DP_DOWN_Y__", "__R_DOWN_DP_Y__");
    replaced = replaced.replace("__DP_UP_X__", "__R_UP_DP_X__");
    replaced = replaced.replace("__DP_UP_Y__", "__R_UP_DP_Y__");
    replaced = replace_macro_device(request, identifiers, &replaced);

    replaced
}

fn replace_macro_device(request: &Request, identifiers: &Identifiers, orig: &String) -> String {
    let mut replaced = orig.clone();

    if let Some(uid) = identifiers.get_id(511, 0) {
        replaced = replaced.replace("__MAC__", &uid.id);
        replaced = replaced.replace("__MAC1__", &uid.id.replace(":", ""));
    }
    if let Some(uid) = identifiers.get_id(509, 0) {
        replaced = replaced.replace("__ADID__", &uid.id);
    }
    if let Some(uid) = identifiers.get_id(510, 0) {
        replaced = replaced.replace("__ADID_MD5__", &uid.id);
    }
    if let Some(uid) = identifiers.get_id(501, 0) {
        replaced = replaced.replace("__IMEI__", &uid.id);
    }
    if let Some(uid) = identifiers.get_id(502, 0) {
        replaced = replaced.replace("__IMEI_MD5__", &uid.id);
    }
    if let Some(uid) = identifiers.get_id(505, 0) {
        replaced = replaced.replace("__OAID__", &uid.id);
    }
    if let Some(uid) = identifiers.get_id(507, 0) {
        replaced = replaced.replace("__IDFA__", &uid.id);
    }
    if let Some(uid) = identifiers.get_id(508, 0) {
        replaced = replaced.replace("__IDFA_MD5__", &uid.id);
    }
    if let Some(ip) = &request.context.device.ip {
        replaced = replaced.replace("__IP__", ip);
    }
    replaced = replaced.replace("__UA__", &request.context.device.ua);
    if let Some(uid) = identifiers.get_id(514, 0) {
        replaced = replaced.replace("__ALL_AAID__", &uid.id);
    }
    if let Some(uid) = identifiers.get_id(513, 0) {
        replaced = replaced.replace("__CAID__", &uid.id);
    }

    replaced
}

fn split_csv(value: &String) -> Vec<String> {
    value
        .split(',')
        .map(|item| item.trim().to_string())
        .filter(|item| !item.is_empty())
        .collect::<Vec<String>>()
}
