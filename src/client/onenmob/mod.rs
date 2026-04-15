use std::time::Duration;

use chrono::{Datelike, TimeZone};
use chrono_tz::Tz;

use crate::{protocol::*, Assets, Cache, Client, Connection, HttpPool, Identifiers, ResultMessage};

pub mod ad;
pub mod adslot;
pub mod app;
pub mod caid;
pub mod device;
pub mod request;
pub mod request_header;
pub mod response;
pub mod track;
pub mod user;
pub mod video;

pub use ad::OnenmobAd;
pub use adslot::OnenmobAdslot;
pub use app::OnenmobApp;
pub use caid::OnenmobCaid;
pub use device::OnenmobDevice;
pub use request::OnenmobRequest;
pub use request_header::OnenmobRequestHeader;
pub use response::OnenmobResponse;
pub use track::OnenmobTrack;
pub use user::OnenmobUser;
pub use video::OnenmobVideo;

pub struct Onenmob {
}

impl Client for Onenmob {

    async fn request(request: &Request, connection: &Connection, pool: &HttpPool, cache: &Cache) -> Result<Response, ResultMessage> {
        let request_id = cache.get_sequence();

        let mut assets = Assets::new(request);
        let identifiers = Identifiers::new(request);

        let request_onenmob = OnenmobRequest {
            request_id: {
                request_id.to_string()
            },
            api_version: {
                "2.7.8".to_string()
            },
            source_type: {
                Some(if request.context.app.is_some() { "app".to_string() } else { "wap".to_string() })
            },
            user_agent: {
                request.context.device.ua.clone()
            },
            ip: {
                request.context.device.ip.clone()
            },
            user_info_param: {
                Some(OnenmobUser {
                    gender: {
                        request.context.user.gender.as_ref().map(|gender| match gender.as_str() {
                            "M" | "m" => 1,
                            "F" | "f" => 2,
                            _ => 0,
                        })
                    },
                    age: {
                        request.context.user.yob.map(|yob| chrono::Local::now().year() - yob)
                    },
                    keywords: {
                        request.context.user.keywords.clone()
                    },
                })
            },
            app_info_param: {
                match &request.context.app {
                    Some(app) => OnenmobApp {
                        app_id: {
                            connection.client_tag_id.clone()
                        },
                        app_name: {
                            match &connection.client_media_appname {
                                Some(app_name) => app_name.clone(),
                                None => app.name.clone(),
                            }
                        },
                        package_name: {
                            match &connection.client_media_apppackage {
                                Some(bundle) => bundle.clone(),
                                None => match &app.bundle {
                                    Some(bundle) => bundle.clone(),
                                    None => "".to_string(),
                                },
                            }
                        },
                        app_category: {
                            None
                        },
                        version: {
                            app.ver.clone()
                        },
                        latitude: {
                            request.context.device.geo.as_ref().and_then(|geo| geo.lat.map(|lat| lat.to_string()))
                        },
                        longitude: {
                            request.context.device.geo.as_ref().and_then(|geo| geo.lon.map(|lon| lon.to_string()))
                        },
                        store_url: {
                            app.storeurl.clone()
                        },
                    },
                    None => OnenmobApp {
                        app_id: connection.client_tag_id.clone(),
                        app_name: {
                            match &connection.client_media_appname {
                                Some(app_name) => app_name.clone(),
                                None => "".to_string(),
                            }
                        },
                        package_name: {
                            match &connection.client_media_apppackage {
                                Some(bundle) => bundle.clone(),
                                None => "".to_string(),
                            }
                        },
                        app_category: None,
                        version: None,
                        latitude: None,
                        longitude: None,
                        store_url: None,
                    },
                }
            },
            device_info_param: {
                OnenmobDevice {
                    device_id: {
                        match request.context.device.os {
                            Some(2) => identifiers.get_id(509, 0).map(|uid| uid.id.clone()).unwrap_or_default(),
                            Some(13) => identifiers.get_id(507, 0).map(|uid| uid.id.clone()).unwrap_or_default(),
                            _ => "".to_string(),
                        }
                    },
                    imei: {
                        identifiers.get_id(501, 0).map(|uid| uid.id.clone())
                    },
                    imei_md5: {
                        identifiers.get_id(502, 0).map(|uid| uid.id.clone())
                    },
                    oaid: {
                        identifiers.get_id(505, 0).map(|uid| uid.id.clone())
                    },
                    open_udid: {
                        None
                    },
                    ssid: {
                        identifiers.get_id(524, 0).map(|uid| uid.id.clone())
                    },
                    wifi_mac: {
                        identifiers.get_id(522, 0).map(|uid| uid.id.clone())
                    },
                    dpi: {
                        request.context.device.ppi
                    },
                    ppi: {
                        request.context.device.ppi
                    },
                    density: {
                        request.context.device.pxratio
                    },
                    phone_name: {
                        identifiers.get_id(527, 0).map(|uid| uid.id.clone())
                    },
                    power_on_time: {
                        None
                    },
                    mac: {
                        identifiers.get_id(511, 0).map(|uid| uid.id.clone())
                    },
                    imsi: {
                        identifiers.get_id(503, 0).map(|uid| uid.id.clone())
                    },
                    device_type: {
                        match request.context.device.devicetype {
                            Some(1) => 1,
                            Some(4) => 1,
                            Some(5) => 2,
                            Some(3) => 3,
                            _ => 0,
                        }
                    },
                    os: {
                        match request.context.device.os {
                            Some(2) => "Android".to_string(),
                            Some(13) => "IOS".to_string(),
                            _ => "".to_string(),
                        }
                    },
                    os_version: {
                        request.context.device.osv.clone()
                    },
                    vendor: {
                        request.context.device.brand.clone()
                    },
                    model: {
                        request.context.device.model.clone()
                    },
                    language: {
                        request.context.device.lang.clone()
                    },
                    conn_type: {
                        match request.context.device.contype {
                            Some(2) => 1,
                            Some(4) => 2,
                            Some(5) => 3,
                            Some(6) => 4,
                            Some(7) => 5,
                            _ => 0,
                        }
                    },
                    operator_type: {
                        if let Some(carrier) = request.context.device.carrier.as_ref() {
                            match carrier.as_str() {
                                "cmcc" => 1,
                                "telecom" => 2,
                                "unicom" => 3,
                                _ => 99,
                            }
                        } else if let Some(mccmnc) = request.context.device.mccmnc.as_ref() {
                            if mccmnc.starts_with("46000") || mccmnc.starts_with("46002") || mccmnc.starts_with("46004") || mccmnc.starts_with("46007") || mccmnc.starts_with("46008") {
                                1
                            } else if mccmnc.starts_with("46003") || mccmnc.starts_with("46005") || mccmnc.starts_with("46011") {
                                2
                            } else if mccmnc.starts_with("46001") || mccmnc.starts_with("46006") || mccmnc.starts_with("46009") {
                                3
                            } else {
                                99
                            }
                        } else {
                            0
                        }
                    },
                    screen_width: {
                        match request.context.device.w {
                            Some(width) => width,
                            None => 0,
                        }
                    },
                    screen_height: {
                        match request.context.device.h {
                            Some(height) => height,
                            None => 0,
                        }
                    },
                    orientation: {
                        request.context.device.orientation.map(|orientation| match orientation {
                            501 => 1,
                            502 => 2,
                            _ => 0,
                        })
                    },
                    rom_version: {
                        request.context.device.romv.clone().or(request.context.device.uiv.clone())
                    },
                    sys_compling_time: {
                        request.context.device.romtime.clone()
                    },
                    boot_time_sec: {
                        match &request.context.device.boottime {
                            Some(boottime) => Some(boottime.split('.').next().unwrap_or_default().to_string()),
                            None => None,
                        }
                    },
                    os_update_time_sec: {
                        match &request.context.device.updatetime {
                            Some(updatetime) => Some(updatetime.split('.').next().unwrap_or_default().to_string()),
                            None => None,
                        }
                    },
                    battery_status: {
                        request.context.device.sysbatterystatus
                    },
                    battery_power: {
                        request.context.device.sysbatterypower
                    },
                    cpu_number: {
                        request.context.device.syscpu
                    },
                    cpu_frequency: {
                        request.context.device.syscpufreq
                    },
                    model_code: {
                        request.context.device.hwmodel.clone()
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
                                    Err(_) => None,
                                }
                            },
                            None => None,
                        }
                    },
                    lmt: {
                        request.context.device.lmt
                    },
                    laccu: {
                        None
                    },
                    caid: {
                        identifiers.get_id(513, 0).map(|uid| uid.id.clone())
                    },
                    caid_version: {
                        identifiers.get_id(513, 0).and_then(|uid| uid.ver.clone())
                    },
                    caids: {
                        identifiers.get_ids(513).map(|uids| {
                            uids.iter()
                                .filter_map(|uid| uid.ver.as_ref().map(|version| OnenmobCaid {
                                    id: uid.id.clone(),
                                    version: version.clone(),
                                }))
                                .collect::<Vec<OnenmobCaid>>()
                        }).filter(|caids| !caids.is_empty())
                    },
                    boot_mark: {
                        request.context.device.bootmark.clone()
                    },
                    update_mark: {
                        request.context.device.updatemark.clone()
                    },
                    app_store_version: {
                        request.context.device.storev.clone()
                    },
                    hms_version: {
                        request.context.device.hmsv.clone()
                    },
                    api_level: {
                        request.context.device.oslevel.map(|api_level| api_level.to_string())
                    },
                    hardware_machine: {
                        request.context.device.hwmachine.clone()
                    },
                    serialno: {
                        request.context.device.serial.clone()
                    },
                    birth_time: {
                        request.context.device.inittime.clone()
                    },
                    elapse_time: {
                        None
                    },
                    country: {
                        request.context.device.country.clone()
                    },
                    physical_memory_byte: {
                        request.context.device.sysmemory.map(|size| size.to_string())
                    },
                    harddisk_size_byte: {
                        request.context.device.sysdisksize.map(|size| size.to_string())
                    },
                    paid: {
                        match &request.context.app {
                            Some(app) => {
                                if app.paid > 0 { Some(app.paid.to_string()) } else { None }
                            },
                            None => None,
                        }
                    },
                    app_list: {
                        request.context.device.app.as_ref().map(|app_list| {
                            app_list.split(',').map(|item| item.trim().to_string()).filter(|item| !item.is_empty()).collect::<Vec<String>>()
                        }).filter(|app_list| !app_list.is_empty())
                    },
                }
            },
            ad_slot_info_param: {
                OnenmobAdslot {
                    ad_type: {
                        // if assets.get_asset_size("video") > 0 {
                        //     if request.item[0].spec.reward > 0 {
                        //         6
                        //     } else {
                        //         5
                        //     }
                        // } else if assets.get_asset_total_size() > 0 {
                        //     2
                        // } else if request.item[0].spec.display.instl > 0 {
                        //     4
                        // } else if request.item[0].spec.display.pos == Some(7) {
                        //     3
                        // } else {
                        //     1
                        // }

                        // patch for 1nmob ad types
                        match connection.client_format.as_str() {
                            "banner" => {
                                1
                            },
                            "interstitial" => {
                                4
                            },
                            "splash" => {
                                3
                            },
                            "feeds" => {
                                2
                            },
                            "video" => {
                                5
                            },
                            _ => 2,
                        }
                    },
                    position: {
                        match request.item[0].spec.display.pos {
                            Some(1) => 1,
                            Some(2) => 4,
                            Some(3) => 2,
                            Some(4) => 1,
                            Some(5) => 2,
                            Some(7) => 5,
                            Some(501) => 3,
                            _ => 5,
                        }
                    },
                    accepted_creative_types: None,
                    accepted_interaction_type: None,
                    width: {
                        match request.item[0].spec.display.w {
                            Some(width) => width,
                            None => 0,
                        }
                    },
                    height: {
                        match request.item[0].spec.display.h {
                            Some(height) => height,
                            None => 0,
                        }
                    },
                }
            },
            is_support_dp: {
                true
            },
        };

        let client = {
            let pool_onenmob_lock = pool.pool_onenmob.clone();
            let pool_onenmob = pool_onenmob_lock.read().unwrap();
            pool_onenmob.clone()
        };
        let response_onenmob_raw = client.post(if connection.test {
                "http://ssp.1nmob.com/ad_api/media_test"
            } else {
                "http://ssp.1nmob.com/ad_api/media_ad"
            })
            .json(&request_onenmob)
            .header("Accept-Encoding", "gzip")
            .header("Connection", "keep-alive")
            .header("Content-Type", "application/json;charset=UTF-8")
            .timeout(Duration::from_millis(connection.timeout))
            .send().await;

        let response_onenmob: OnenmobResponse = match response_onenmob_raw {
            Ok(response_onenmob_raw) => {
                let status = response_onenmob_raw.status();
                if status != 200 {
                    return Err(ResultMessage {
                        code: 992,
                        message: match response_onenmob_raw.text().await {
                            Ok(text) => format!("upstream error {}: {}", status, text),
                            Err(_) => format!("upstream error {}", status),
                        },
                    });
                }

                match response_onenmob_raw.text().await {
                    Ok(text) => match serde_json::from_str::<OnenmobResponse>(&text) {
                        Ok(json) => {
                            json
                        },
                        Err(error) => {
                            return Err(ResultMessage {
                                code: 997,
                                message: error.to_string(),
                            });
                        },
                    },
                    Err(error) => {
                        return Err(ResultMessage {
                            code: 992,
                            message: error.to_string(),
                        });
                    },
                }
            },
            Err(error) => {
                if error.is_timeout() {
                    return Err(ResultMessage {
                        code: 991,
                        message: "upstream request timeout".to_string(),
                    });
                }

                return Err(ResultMessage {
                    code: 992,
                    message: format!("upstream request failed: {:?}", error),
                });
            },
        };

        if response_onenmob.code != 0 && response_onenmob.code != 200 {
            if response_onenmob.code == 13 {
                return Err(ResultMessage {
                    code: 993,
                    message: "".to_string(),
                });
            } else {
                return Err(ResultMessage {
                    code: 994,
                    message: format!("upstream error {}: {}", response_onenmob.code, response_onenmob.msg.unwrap_or_default()),
                });
            }
        }

        let response = Response {
            id: request.id.clone(),
            nbr: None,
            seatbid: {
                match &response_onenmob.data {
                    Some(data) => {
                        if data.status_code != 200 {
                            return Err(ResultMessage {
                                code: 994,
                                message: format!("upstream status error {}", data.status_code),
                            });
                        }

                        match &data.ads {
                            Some(ad) => {
                                Some(vec![Seatbid {
                                    bid: {
                                        let mut bids = vec![];
                                        let link_asset = LinkAsset {
                                            linktype: match ad.interaction_type {
                                                Some(3) => 2,
                                                Some(4) => 3,
                                                _ => 1,
                                            },
                                            universallink: None,
                                            storeid: None,
                                            deeplink: ad.deeplink.as_ref().filter(|deeplink| !deeplink.is_empty()).cloned(),
                                            quickapplink: None,
                                            wechatmppath: None,
                                            wechatmpid: None,
                                            marketurl: None,
                                            downloadurl: match ad.interaction_type {
                                                Some(3) | Some(4) => ad.app_download_url.clone().or(ad.download_url.clone()).or(ad.click_ad_url.clone()),
                                                _ => None,
                                            },
                                            url: ad.click_ad_url.clone().unwrap_or_default(),
                                            urlfb: None,
                                        };

                                        bids.push(Bid {
                                            id: Some(request_id.to_string()),
                                            item: request.item[0].id.clone(),
                                            price: {
                                                match data.bid_price {
                                                    Some(price) if price > 0 => price,
                                                    _ => connection.default_price,
                                                }
                                            },
                                            burl: None,
                                            lurl: None,
                                            media: Ad {
                                                id: ad.ad_id.clone(),
                                                display: Display {
                                                    w: None,
                                                    h: None,
                                                    banner: {
                                                        if assets.get_banner_size() > 0 {
                                                            ad.image_srcs.as_ref().and_then(|images| images.first()).map(|image| Banner {
                                                                img: image.clone(),
                                                                link: Some(link_asset.clone()),
                                                            })
                                                        } else {
                                                            None
                                                        }
                                                    },
                                                    native: {
                                                        let mut asset_vec = vec![];

                                                        if let Some(video) = &ad.video {
                                                            if video.video_url.is_some() {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("video"),
                                                                    req: 1,
                                                                    title: None,
                                                                    img: None,
                                                                    video: Some(VideoAsset {
                                                                        url: video.video_url.clone().unwrap_or_default(),
                                                                        mime: None,
                                                                        w: video.video_width,
                                                                        h: video.video_height,
                                                                        dur: video.video_duration,
                                                                        size: video.size,
                                                                        skipoffset: video.skip_seconds,
                                                                        delivery: video.prefetch.map(|prefetch| if prefetch { 2 } else { 1 }),
                                                                        orientation: None,
                                                                        autolanding: video.auto_landing.map(|auto_landing| if auto_landing { 1 } else { 0 }).unwrap_or(0),
                                                                        clickable: video.click_able.map(|click_able| if click_able { 1 } else { 0 }).unwrap_or(0),
                                                                    }),
                                                                    data: None,
                                                                    html: None,
                                                                    app: None,
                                                                });

                                                                if let Some(cover) = video.cover_img_url.as_ref().and_then(|covers| covers.first()) {
                                                                    asset_vec.push(Asset {
                                                                        id: assets.consume_asset("video#cover"),
                                                                        req: 0,
                                                                        title: None,
                                                                        img: Some(ImageAsset {
                                                                            url: cover.clone(),
                                                                            mime: None,
                                                                            w: video.video_width,
                                                                            h: video.video_height,
                                                                            imagetype: Some(3),
                                                                        }),
                                                                        video: None,
                                                                        data: None,
                                                                        html: None,
                                                                        app: None,
                                                                    });
                                                                }
                                                            }
                                                        }

                                                        if let Some(title) = &ad.title {
                                                            asset_vec.push(Asset {
                                                                id: assets.consume_asset("title"),
                                                                req: 1,
                                                                title: Some(TitleAsset {
                                                                    text: title.clone(),
                                                                    subtitle: None,
                                                                    desc: ad.descriptions.clone(),
                                                                    len: Some(title.len() as i32),
                                                                }),
                                                                img: None,
                                                                video: None,
                                                                data: None,
                                                                html: None,
                                                                app: None,
                                                            });
                                                        }

                                                        if let Some(description) = &ad.descriptions {
                                                            asset_vec.push(Asset {
                                                                id: assets.consume_asset("data#desc"),
                                                                req: 0,
                                                                title: None,
                                                                img: None,
                                                                video: None,
                                                                data: Some(DataAsset {
                                                                    value: description.clone(),
                                                                    len: Some(description.len() as i32),
                                                                    datatype: Some(2),
                                                                }),
                                                                html: None,
                                                                app: None,
                                                            });
                                                        }

                                                        if let Some(icon) = &ad.icon_srcs {
                                                            asset_vec.push(Asset {
                                                                id: assets.consume_asset("icon"),
                                                                req: 0,
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
                                                        }

                                                        if let Some(images) = &ad.image_srcs {
                                                            for image in images {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("img"),
                                                                    req: 1,
                                                                    title: None,
                                                                    img: Some(ImageAsset {
                                                                        url: image.clone(),
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
                                                            }
                                                        }

                                                        if ad.app_name.is_some() || ad.package_name.is_some() {
                                                            asset_vec.push(Asset {
                                                                id: assets.consume_asset("app"),
                                                                req: 0,
                                                                title: None,
                                                                img: None,
                                                                video: None,
                                                                data: None,
                                                                html: None,
                                                                app: Some(AppAsset {
                                                                    name: ad.app_name.clone().unwrap_or_default(),
                                                                    desc: ad.descriptions.clone(),
                                                                    descurl: ad.intro_url.clone(),
                                                                    domain: ad.source.clone(),
                                                                    bundle: ad.package_name.clone(),
                                                                    ver: ad.app_version.clone(),
                                                                    developer: ad.developer_name.clone(),
                                                                    icon: ad.icon_srcs.clone(),
                                                                    storeid: None,
                                                                    storeurl: None,
                                                                    paid: 0,
                                                                    size: None,
                                                                    md5: None,
                                                                    registration: None,
                                                                    privacy: None,
                                                                    privacyurl: ad.privacy_policy_url.clone(),
                                                                    permission: None,
                                                                    permissionurl: ad.permission_url.clone(),
                                                                }),
                                                            });
                                                        }

                                                        Some(Native {
                                                            asset: asset_vec,
                                                            link: Some(link_asset.clone()),
                                                        })
                                                    },
                                                    event: {
                                                        let mut event_vec = vec![];

                                                        if let Some(show_urls) = &ad.show_url {
                                                            for url in show_urls {
                                                                event_vec.push(Event {
                                                                    eventtype: 501,
                                                                    method: 1,
                                                                    url: replace_macro(url),
                                                                    header: None,
                                                                    content: None,
                                                                });
                                                            }
                                                        }

                                                        if let Some(click_urls) = &ad.click_url {
                                                            for url in click_urls {
                                                                event_vec.push(Event {
                                                                    eventtype: 502,
                                                                    method: 1,
                                                                    url: replace_macro(url),
                                                                    header: None,
                                                                    content: None,
                                                                });
                                                            }
                                                        }

                                                        for track in &ad.tracks {
                                                            if let Some(eventtype) = match track.track_type {
                                                                1 => Some(501),
                                                                2 => Some(502),
                                                                3 => Some(601),
                                                                4 => Some(602),
                                                                5 => Some(603),
                                                                6 => Some(604),
                                                                7 => Some(605),
                                                                8 => Some(509),
                                                                9 => Some(502),
                                                                10 => Some(607),
                                                                11 => Some(608),
                                                                12 => Some(609),
                                                                16 => Some(505),
                                                                17 => Some(504),
                                                                18 => Some(507),
                                                                19 => Some(506),
                                                                20 => Some(503),
                                                                31 => Some(701),
                                                                32 => Some(702),
                                                                33 => Some(703),
                                                                34 => Some(704),
                                                                35 => Some(705),
                                                                36 => Some(710),
                                                                37 => Some(715),
                                                                38 => Some(716),
                                                                39 => Some(719),
                                                                40 => Some(720),
                                                                41 => Some(713),
                                                                42 => Some(714),
                                                                43 => Some(708),
                                                                44 => Some(709),
                                                                45 => Some(724),
                                                                46 => Some(712),
                                                                47 => Some(717),
                                                                48 => Some(718),
                                                                49 => Some(711),
                                                                50 => Some(502),
                                                                60 => Some(508),
                                                                61 => Some(509),
                                                                62 => None,
                                                                _ => None,
                                                            } {
                                                                for url in &track.urls {
                                                                    let header = {
                                                                        let mut header_vec = vec![];

                                                                        if let Some(request_headers) = &track.request_header {
                                                                            for request_header in request_headers {
                                                                                if let Some(key) = &request_header.key {
                                                                                    header_vec.push(Header {
                                                                                        key: key.clone(),
                                                                                        value: request_header.value.as_ref().map(replace_macro),
                                                                                    });
                                                                                }
                                                                            }
                                                                        }

                                                                        if ad.report_add_header_ua.unwrap_or(false) && !header_vec.iter().any(|header| header.key.eq_ignore_ascii_case("User-Agent")) {
                                                                            header_vec.push(Header {
                                                                                key: "User-Agent".to_string(),
                                                                                value: Some(replace_macro(&"__UA__".to_string())),
                                                                            });
                                                                        }

                                                                        if header_vec.is_empty() {
                                                                            None
                                                                        } else {
                                                                            Some(header_vec)
                                                                        }
                                                                    };
                                                                    event_vec.push(Event {
                                                                        eventtype,
                                                                        method: match track.method.as_deref() {
                                                                            Some("POST") | Some("post") => 2,
                                                                            _ => 1,
                                                                        },
                                                                        url: replace_macro(url),
                                                                        header,
                                                                        content: track.content.as_ref().map(replace_macro),
                                                                    });
                                                                }
                                                            }
                                                        }

                                                        event_vec
                                                    },
                                                },
                                                advertiser: ad.source.clone(),
                                                advertisericon: ad.icon_srcs.clone(),
                                            },
                                        });

                                        bids
                                    },
                                }])
                            },
                            None => return Err(ResultMessage {
                                code: 993, // defending
                                message: "".to_string(),
                            }),
                        }
                    },
                    None => return Err(ResultMessage {
                        code: 993, // defending
                        message: "".to_string(),
                    }),
                }
            },
        };

        Ok(response)
    }

    async fn bidding_notify_win(_url: String, _win_price: i32, _next_price: i32, _iv: &String, _connection: &Connection, _pool: &HttpPool) {
    }

    async fn bidding_notify_lose(_url: String, _lose_price: i32, _lose_reason: i32, _lose_adn_name: &String, _iv: &String, _connection: &Connection, _pool: &HttpPool) {
    }

    fn encrypt_price(price: i32, _iv: &String, _connection: &Connection) -> String {
        price.to_string()
    }

}

fn replace_macro(orig: &String) -> String {
    let mut replaced = orig.clone();

    replaced = replaced.replace("__ADOWN_X__", "__ABS_DOWN_X__");
    replaced = replaced.replace("__ADOWN_Y__", "__ABS_DOWN_Y__");
    replaced = replaced.replace("__AUP_X__", "__ABS_UP_X__");
    replaced = replaced.replace("__AUP_Y__", "__ABS_UP_Y__");
    replaced = replaced.replace("__TIME_START__", "__EVENT_TIME_START__");
    replaced = replaced.replace("__TIME_END__", "__EVENT_TIME_END__");
    replaced = replaced.replace("__EVENT_TIME_SECOND__", "__EVENT_TIME_START_S__");
    replaced = replaced.replace("__EVENT_TIME_END_SECOND__", "__EVENT_TIME_END_S__");
    replaced = replaced.replace("__DP_DOWN_X__", "__DOWN_DP_X__");
    replaced = replaced.replace("__DP_DOWN_Y__", "__DOWN_DP_Y__");
    replaced = replaced.replace("__DP_UP_X__", "__UP_DP_X__");
    replaced = replaced.replace("__DP_UP_Y__", "__UP_DP_Y__");
    replaced = replaced.replace("__TARGET_APP_INSTALL__", "__DP_TARGET__");
    replaced = replaced.replace("__DISPLAY_LUX__", "__LT_X__");
    replaced = replaced.replace("__DISPLAY_LUY__", "__LT_Y__");
    replaced = replaced.replace("__DISPLAY_RDX__", "__RB_X__");
    replaced = replaced.replace("__DISPLAY_RDY__", "__RB_Y__");
    replaced = replaced.replace("__BUTTON_LUX__", "__BUTTON_LT_X__");
    replaced = replaced.replace("__BUTTON_LUY__", "__BUTTON_LT_Y__");
    replaced = replaced.replace("__BUTTON_RDX__", "__BUTTON_RB_X__");
    replaced = replaced.replace("__BUTTON_RDY__", "__BUTTON_RB_Y__");
    replaced = replaced.replace("__VIDEO_BEHAVIOR__", "__VIDEO_PLAY_TRIGGER_0__");
    replaced = replaced.replace("__VIDEO__PLAY_CUR__", "__VIDEO_PLAY_PROGRESS_S__");
    replaced = replaced.replace("__VIDEO_START_TIME__", "__VIDEO_BEGIN_TIME__");
    replaced = replaced.replace("__VIDEO_PLAY_FIRSR_FRAME__", "__VIDEO_FIRST_FRAME__");
    replaced = replaced.replace("__VIDEO_PLAY_LAST_FRAME__", "__VIDEO_LAST_FRAME__");
    replaced = replaced.replace("__VIDEO_SCENE__", "__VIDEO_PLAY_SCENE__");
    replaced = replaced.replace("__VIDEO_STATUS__", "__VIDEO_PLAY_STATUS__");

    replaced
}
