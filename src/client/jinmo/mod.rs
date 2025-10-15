use std::time::Duration;

use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyInit};
use base64::prelude::*;
use chrono::{Datelike, Local, TimeZone, Utc};
use chrono_tz::Tz;
use prost::Message;
use urlencoding::encode;

use crate::{protocol::*, Assets, Cache, Client, Connection, HttpPool, Identifiers, Price, ResultMessage};

type Aes128EcbEnc = ecb::Encryptor<aes::Aes128>;

pub mod ad;
pub mod ad_app;
pub mod btn;
pub mod caid;
pub mod device;
pub mod device_id;
pub mod geo;
pub mod image;
pub mod imp;
pub mod interaction;
pub mod logo;
pub mod material;
pub mod media_app;
pub mod network;
pub mod request;
pub mod response;
pub mod track;
pub mod user;
pub mod video;

pub use ad::JinmoAd;
pub use ad_app::JinmoAdApp;
pub use btn::JinmoBtn;
pub use caid::JinmoCaid;
pub use device::JinmoDevice;
pub use device_id::JinmoDeviceId;
pub use geo::JinmoGeo;
pub use image::JinmoImage;
pub use imp::JinmoImp;
pub use interaction::JinmoInteraction;
pub use logo::JinmoLogo;
pub use material::JinmoMaterial;
pub use media_app::JinmoMediaApp;
pub use network::JinmoNetwork;
pub use request::JinmoRequest;
pub use response::JinmoResponse;
pub use track::JinmoTrack;
pub use user::JinmoUser;
pub use video::JinmoVideo;

pub struct Jinmo {

}

impl Client for Jinmo {

    async fn request(request: &Request, connection: &Connection, pool: &HttpPool, cache: &Cache) -> Result<Response, ResultMessage> {
        let space_id = connection.client_tag_id.split("|").nth(0).unwrap();
        let media_id = connection.client_tag_id.split("|").nth(1).unwrap();

        let request_id = cache.get_sequence();

        let mut assets = Assets::new(request);
        let identifiers = Identifiers::new(request);

        let request_jinmo = JinmoRequest {
            request_id: {
                request_id.to_string()
            },
            media_id: {
                match media_id.parse::<i64>() {
                    Ok(id) => id,
                    Err(_) => 0,
                }
            },
            imp_list: [JinmoImp {
                imp_id: {
                    "1".to_string()
                },
                space_id: {
                    space_id.to_string()
                },
                keyword: {
                    None
                },
                tags: {
                    vec![]
                },
                space_width: {
                    match request.item[0].spec.display.w {
                        Some(w) => Some(w),
                        None => None,
                    }
                },
                space_height: {
                    match request.item[0].spec.display.h {
                        Some(h) => Some(h),
                        None => None,
                    }
                },
                material_type: {
                    vec![1, 2]
                },
                ad_slot_type: {
                    let mut ad_slot_type = 0;
                    let reward = request.item[0].spec.reward;
                    let instl = request.item[0].spec.display.instl;

                    if assets.get_banner_size() > 0 {
                        if instl == 1 {
                            ad_slot_type = 2;
                        } else {
                            if request.item[0].spec.display.w > request.item[0].spec.display.h {
                                ad_slot_type = 4;
                            } else {
                                ad_slot_type = 1;
                            }
                        }
                    }
                    if assets.get_asset_size("img") > 0 {
                        ad_slot_type = 3;
                    }
                    if assets.get_asset_size("video") > 0 {
                        if instl == 1 {
                            ad_slot_type = 2;
                        } else {
                            if reward == 1 {
                                ad_slot_type = 5;
                            } else {
                                ad_slot_type = 1;
                            }
                        }
                    }

                    Some(ad_slot_type)
                },
                interaction_type: {
                    vec![1, 2, 3, 4, 5]
                },
                bid_type: {
                    1
                },
                cpm_bid_floor: {
                    (Price::to_client(connection, request.item[0].flr.map(f64::from)) as i64).into()
                },
                cpc_bid_floor: {
                    None
                },
            }].to_vec(),
            device: {
                Some(JinmoDevice {
                    device_type: {
                        match request.context.device.devicetype {
                            Some(devicetype) => {
                                match devicetype {
                                    2 => 3,
                                    3 => 4,
                                    4 => 1,
                                    5 => 2,
                                    _ => 0,
                                }
                            },
                            None => 0,
                        }
                    },
                    os_type: {
                        match request.context.device.os {
                            Some(2) => 1,
                            Some(13) => 2,
                            _ => 0,
                        }
                    },
                    os_version: {
                        match &request.context.device.osv {
                            Some(osv) => osv.clone(),
                            None => "".to_string(),
                        }
                    },
                    model: {
                        match &request.context.device.model {
                            Some(model) => model.clone(),
                            None => "".to_string(),
                        }
                    },
                    brand: {
                        match &request.context.device.make {
                            Some(brand) => brand.clone(),
                            None => "".to_string(),
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
                    device_ids: {
                        Some(JinmoDeviceId {
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
                            caid_list: {
                                let mut caids = vec![];

                                match identifiers.get_id(513, 0) {
                                    Some(uid) => {
                                        caids.push(JinmoCaid {
                                            caid: {
                                                uid.id.clone()
                                            },
                                            version: {
                                                match &uid.ver {
                                                    Some(ver) => ver.clone(),
                                                    None => "0".to_string(),
                                                }
                                            },
                                        });
                                    },
                                    None => (),
                                };

                                match identifiers.get_id(513, 1) {
                                    Some(uid) => {
                                        caids.push(JinmoCaid {
                                            caid: {
                                                uid.id.clone()
                                            },
                                            version: {
                                                match &uid.ver {
                                                    Some(ver) => ver.clone(),
                                                    None => "0".to_string(),
                                                }
                                            },
                                        });
                                    },
                                    None => (),
                                };

                                if caids.len() > 0 {
                                    caids
                                } else {
                                    vec![]
                                }
                            },
                            idfv: {
                                match identifiers.get_id(515, 0) {
                                    Some(uid) => Some(uid.id.clone()),
                                    None => None,
                                }
                            },
                            idfv_md5: {
                                match identifiers.get_id(516, 0) {
                                    Some(uid) => Some(uid.id.clone()),
                                    None => None,
                                }
                            },
                            paid: {
                                let mut paid = None;
                                match identifiers.get_id(519, 0) {
                                    Some(uid) => {
                                        match &uid.ver {
                                            Some(ver) => {
                                                if ver != "1.4" {
                                                    paid = Some(uid.id.clone());
                                                }
                                            },
                                            None => {
                                                paid = Some(uid.id.clone());
                                            },
                                        }
                                    },
                                    None => (),
                                }
                                match identifiers.get_id(519, 1) {
                                    Some(uid) => {
                                        match &uid.ver {
                                            Some(ver) => {
                                                if ver != "1.4" {
                                                    paid = Some(uid.id.clone());
                                                }
                                            },
                                            None => {
                                                paid = Some(uid.id.clone());
                                            },
                                        }
                                    },
                                    None => (),
                                }

                                paid
                            },
                            paid_1_4: {
                                let mut paid = None;
                                match identifiers.get_id(519, 0) {
                                    Some(uid) => {
                                        match &uid.ver {
                                            Some(ver) => {
                                                if ver == "1.4" {
                                                    paid = Some(uid.id.clone());
                                                }
                                            },
                                            None => (),
                                        }
                                    },
                                    None => (),
                                }
                                match identifiers.get_id(519, 1) {
                                    Some(uid) => {
                                        match &uid.ver {
                                            Some(ver) => {
                                                if ver == "1.4" {
                                                    paid = Some(uid.id.clone());
                                                }
                                            },
                                            None => (),
                                        }
                                    },
                                    None => (),
                                }

                                paid
                            },
                        })
                    },
                    geo: {
                        match &request.context.device.geo {
                            Some(geo) => Some(JinmoGeo {
                                latitude: {
                                    match geo.lat {
                                        Some(lat) => lat,
                                        None => 0.0,
                                    }
                                },
                                longitude: {
                                    match geo.lon {
                                        Some(lon) => lon,
                                        None => 0.0,
                                    }
                                },
                                coordinate_type: {
                                    match geo.coordinate {
                                        Some(1) => Some(1),
                                        Some(2) => Some(2),
                                        Some(3) => Some(3),
                                        _ => Some(0),
                                    }
                                },
                                laccu: {
                                    // geo.laccu.clone()
                                    None
                                },
                                accuracy_m: {
                                    match geo.accur {
                                        Some(accur) => Some(accur as f64),
                                        None => None,
                                    }
                                },
                            }),
                            None => None,
                        }
                    },
                    user_agent: {
                        request.context.device.ua.clone()
                    },
                    network: {
                        Some(JinmoNetwork {
                            connect_type: {
                                match request.context.device.contype {
                                    Some(1) => 1,
                                    Some(2) => 2,
                                    Some(3) => 3,
                                    Some(4) => 4,
                                    Some(5) => 5,
                                    Some(6) => 6,
                                    _ => 0,
                                }
                            },
                            ipv4: {
                                match &request.context.device.ip {
                                    Some(ip) => {
                                        ip.clone()
                                    },
                                    None => "0.0.0.0".to_string(),
                                }
                            },
                            ipv6: {
                                request.context.device.ipv6.clone()
                            },
                            carrier: {
                                match &request.context.device.carrier {
                                    Some(carrier) => {
                                        match carrier.as_str() {
                                            "cmcc" => Some(1),
                                            "unicom" => Some(2),
                                            "telecom" => Some(3),
                                            _ => Some(0),
                                        }
                                    },
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
                                match identifiers.get_id(529, 0) {
                                    Some(uid) => Some(uid.id.clone()),
                                    None => None,
                                }
                            },
                        })
                    },
                    country: {
                        request.context.device.country.clone()
                    },
                    language: {
                        request.context.device.lang.clone()
                    },
                    time_zone: {
                        match &request.context.device.timezone {
                            Some(timezone) => {
                                let tz: Result<Tz, chrono_tz::ParseError> = timezone.parse();
                                match tz {
                                    Ok(tz) => {
                                        let t = tz.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
                                        let utc = chrono_tz::UTC.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
                                        Some((utc.timestamp() - t.timestamp()).to_string())
                                    },
                                    Err(_) => None,
                                }
                            },
                            None => None,
                        }
                    },
                    cpu_num: {
                        request.context.device.syscpu.clone()
                    },
                    system_disk_size: {
                        request.context.device.sysdisksize.clone()
                    },
                    system_available_size: {
                        // request.context.device.sysavailabledisksize.clone()
                        None
                    },
                    system_memory_size: {
                        request.context.device.sysmemory.clone()
                    },
                    hwv: {
                        request.context.device.hwv.clone()
                    },
                    ppi: {
                        request.context.device.ppi.clone()
                    },
                    rom_version: {
                        request.context.device.romv.clone()
                    },
                    hms_ver: {
                        request.context.device.hmsv.clone()
                    },
                    hwag_ver: {
                        request.context.device.storev.clone()
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
                    sys_compiling_time: {
                        request.context.device.romtime.clone()
                    },
                    os_birth_time: {
                        request.context.device.inittime.clone()
                    },
                    os_boot_time: {
                        request.context.device.boottime.clone()
                    },
                    os_boot_mark: {
                        request.context.device.bootmark.clone()
                    },
                    os_update_time: {
                        request.context.device.updatetime.clone()
                    },
                    os_update_mark: {
                        request.context.device.updatemark.clone()
                    },
                    client_time: {
                        format!("{}", Utc::now().format("%Y-%m-%d %H:%M:%S.3fZ")).into()
                    },
                    installed_packages: {
                        let mut installed_apps = vec![];

                        match &request.context.device.app {
                            Some(app) => {
                                for app1 in app.split(",") {
                                    installed_apps.push(app1.to_string());
                                }
                            },
                            None => (),
                        }

                        installed_apps
                    },
                    app_store_version: {
                        request.context.device.storev.clone()
                    },
                    imsi: {
                        match identifiers.get_id(503, 0) {
                            Some(uid) => Some(uid.id.clone()),
                            None => None,
                        }
                    },
                    battery_status: {
                        request.context.device.sysbatterystatus.clone()
                    },
                    battery_power: {
                        request.context.device.sysbatterypower.clone()
                    },
                    cpu_freq: {
                        request.context.device.syscpufreq.clone()
                    },
                    orientation: {
                        match request.context.device.orientation {
                            Some(501) => Some(1),
                            Some(502) => Some(2),
                            _ => Some(0),
                        }
                    },
                    logical_width: {
                        None
                    },
                    logical_height: {
                        None
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
                    osl: {
                        match request.context.device.oslevel {
                            Some(oslevel) => Some(oslevel.to_string()),
                            None => None,
                        }
                    },
                    serial_no: {
                        request.context.device.serial.clone()
                    },
                    hardware_model: {
                        request.context.device.hwmodel.clone()
                    },
                    hardware_machine: {
                        request.context.device.hwmachine.clone()
                    },
                    auth_status: {
                        request.context.device.lmt.clone()
                    },
                })
            },
            media_app: {
                match &request.context.app {
                    Some(app) => Some(JinmoMediaApp {
                        package_name: {
                            match &connection.client_media_apppackage {
                                Some(client_media_apppackage) => client_media_apppackage.clone(),
                                None => {
                                    match &app.bundle {
                                        Some(bundle) => bundle.clone(),
                                        None => "".to_string(),
                                    }
                                },
                            }
                        },
                        app_name: {
                            match &connection.client_media_appname {
                                Some(client_media_appname) => client_media_appname.clone(),
                                None => app.name.clone(),
                            }
                        },
                        app_version: {
                            app.ver.clone()
                        },
                        network_protocol: {
                            1
                        },
                    }),
                    None => Some(JinmoMediaApp {
                        package_name: {
                            match &connection.client_media_apppackage {
                                Some(client_media_apppackage) => client_media_apppackage.clone(),
                                None => "".to_string(),
                            }
                        },
                        app_name: {
                            match &connection.client_media_appname {
                                Some(client_media_appname) => client_media_appname.clone(),
                                None => "".to_string(),
                            }
                        },
                        app_version: {
                            None
                        },
                        network_protocol: {
                            1
                        },
                    }),
                }
            },
            user: {
                Some(JinmoUser {
                    user_id: {
                        request.context.user.id.clone()
                    },
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
                                Some((year - yob).to_string())
                            },
                            None => None,
                        }
                    },
                    keywords: {
                        request.context.user.keywords.clone()
                    },
                })
            },
            api_version: {
                "1.0.4".to_string()
            },
            timeout: {
                Some(connection.timeout as i64)
            },
        };

        let bytes = request_jinmo.encode_to_vec();

        let response_jinmo: JinmoResponse;

        let client = {
            let pool_jinmo_lock = pool.pool_jinmo.clone();
            let pool_jinmo = pool_jinmo_lock.read().unwrap();
            pool_jinmo.clone()
        };
        let response_jinmo_raw = client.post(format!("{}{}", "http://adx-api.jinmo.tech/dsp/rtb/common?m=", media_id).as_str())
            .body(bytes)
            .header("Accept-Encoding", "gzip")
            .header("Connection", "keep-alive")
            .header("Content-Type", "application/x-protobuf")
            .timeout(Duration::from_millis(connection.timeout))
            .send().await;

        match response_jinmo_raw {
            Ok(response_jinmo_raw) => {
                let status = response_jinmo_raw.status();
                if status == 204 {
                    return Err(ResultMessage {
                        code: 993,
                        message: "".to_string(),
                    });
                } else if status == 200 {
                    match response_jinmo_raw.bytes().await {
                        Ok(text) => {
                            match JinmoResponse::decode(text) {
                                Ok(json) => {
                                    response_jinmo = json;
                                    match response_jinmo.code {
                                        0 => {
                                            if response_jinmo.ad_list.len() == 0 {
                                                return Err(ResultMessage {
                                                    code: 993,
                                                    message: "".to_string(),
                                                });
                                            }
                                        },
                                        _ => {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: {
                                                    format!("upstream error {}: {}", response_jinmo.code, response_jinmo.msg)
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
                } else {
                    return Err(ResultMessage {
                        code: 992,
                        message: format!("upstream request failed: {:?}", status),
                    });
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

                for ad in &response_jinmo.ad_list {
                    let link_asset = LinkAsset {
                        linktype: {
                            match &ad.interaction {
                                Some(interaction) => {
                                    match interaction.interaction_type {
                                        Some(1) => 2,
                                        _ => 1,
                                    }
                                },
                                None => 1,
                            }
                        },
                        universallink: {
                             match &ad.interaction {
                                Some(interaction) => {
                                    interaction.universal_link_url.clone()
                                },
                                None => None,
                            }
                        },
                        storeid: None,
                        deeplink: {
                            match &ad.interaction {
                                Some(interaction) => {
                                    interaction.deeplink_url.clone()
                                },
                                None => None,
                            }
                        },
                        quickapplink: None,
                        wechatmppath: {
                            match &ad.interaction {
                                Some(interaction) => {
                                    interaction.wx_mini_program_path.clone()
                                },
                                None => None,
                            }
                        },
                        wechatmpid: {
                            match &ad.interaction {
                                Some(interaction) => {
                                    interaction.wx_mini_program_id.clone()
                                },
                                None => None,
                            }
                        },
                        marketurl: {
                            match &ad.interaction {
                                Some(interaction) => {
                                    interaction.market_deeplink_url.clone()
                                },
                                None => None,
                            }
                        },
                        downloadurl: {
                            match &ad.interaction {
                                Some(interaction) => {
                                    interaction.download_url.clone()
                                },
                                None => None,
                            }
                        },
                        url: {
                            match &ad.interaction {
                                Some(interaction) => {
                                    match &interaction.landing_url {
                                        Some(landing_url) => landing_url.clone(),
                                        None => "".to_string(),
                                    }
                                },
                                None => "".to_string(),
                            }
                        },
                        urlfb: None,
                    };

                    let track_info = ad.track_info.as_ref().unwrap();

                    let bid = Bid {
                        id: Some(request_id.to_string()),
                        item: request.item[0].id.clone(),
                        price: { // update later
                            ad.bid_price as i32
                        },
                        burl: {
                            let mut burl = Vec::<String>::new();
                            for win_notice_url in &track_info.win_notice_urls {
                                let nurl = win_notice_url.clone();
                                let nurl = nurl.replace("__PRICE__", "__WIN_PRICE__");
                                burl.push(replace_macro(&nurl));
                            }
                            Some(burl)
                        },
                        lurl: None,
                        media: Ad {
                            id: {
                                ad.track_id.clone()
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

                                let material = ad.material.as_ref().unwrap();
                                if assets.get_banner_size() > 0 {
                                    let image_list = &material.image_list;
                                    if image_list.len() > 0 {
                                        display.w = image_list[0].width.clone();
                                        display.h = image_list[0].height.clone();
                                        display.banner = Some(Banner {
                                            img: image_list[0].url.clone(),
                                            link: Some(link_asset.clone()),
                                        });
                                    }
                                }
                                if assets.get_asset_total_size() > 0 {
                                    match &material.title {
                                        Some(title) => {
                                            asset_vec.push(Asset {
                                                id: assets.consume_asset("title"),
                                                req: 1,
                                                title: Some(TitleAsset {
                                                    text: title.clone(),
                                                    subtitle: material.sub_title.clone(),
                                                    desc: material.desc.clone(),
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
                                    if assets.get_asset_size("img") > 0 {
                                        let image_list = &material.image_list;
                                        for image in image_list {
                                            asset_vec.push(Asset {
                                                id: assets.consume_asset("img"),
                                                req: 1,
                                                title: None,
                                                img: Some(ImageAsset {
                                                    url: image.url.clone(),
                                                    mime: None,
                                                    w: image.width.clone(),
                                                    h: image.height.clone(),
                                                    imagetype: Some(3),
                                                }),
                                                video: None,
                                                data: None,
                                                html: None,
                                                app: None,
                                            });
                                        }
                                    }
                                    if assets.get_asset_size("thumb") > 0 {
                                        let image_list = &material.image_list;
                                        for image in image_list {
                                            asset_vec.push(Asset {
                                                id: assets.consume_asset("thumb"),
                                                req: 1,
                                                title: None,
                                                img: Some(ImageAsset {
                                                    url: image.url.clone(),
                                                    mime: None,
                                                    w: image.width.clone(),
                                                    h: image.height.clone(),
                                                    imagetype: Some(501),
                                                }),
                                                video: None,
                                                data: None,
                                                html: None,
                                                app: None,
                                            });
                                        }
                                    }
                                    if assets.get_asset_size("video") > 0 {
                                        let video_list = &material.video_list;
                                        for video in video_list {
                                            asset_vec.push(Asset {
                                                id: assets.consume_asset("video"),
                                                req: 1,
                                                title: None,
                                                img: None,
                                                video: Some(VideoAsset {
                                                    url: video.video_url.clone(),
                                                    mime: None,
                                                    w: video.width.clone(),
                                                    h: video.height.clone(),
                                                    dur: video.duration.clone(),
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

                                            match &video.cover_image {
                                                Some(cover_image) => {
                                                    asset_vec.push(Asset {
                                                        id: assets.consume_asset("video#cover"),
                                                        req: 1,
                                                        title: None,
                                                        img: Some(ImageAsset {
                                                            url: cover_image.clone(),
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
                                    }
                                    match &material.icon_image {
                                        Some(icon_image) => {
                                            asset_vec.push(Asset {
                                                id: assets.consume_asset("icon"),
                                                req: 1,
                                                title: None,
                                                img: Some(ImageAsset {
                                                    url: icon_image.url.clone(),
                                                    mime: None,
                                                    w: icon_image.width.clone(),
                                                    h: icon_image.height.clone(),
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
                                }

                                match &ad.btn_info {
                                    Some(btn_info) => {
                                        match &btn_info.btn_name {
                                            Some(btn_name) => {
                                                asset_vec.push(Asset {
                                                    id: assets.consume_asset("data#ctatext"),
                                                    req: 1,
                                                    title: None,
                                                    img: None,
                                                    video: None,
                                                    data: Some(DataAsset {
                                                        value: btn_name.clone(),
                                                        len: None,
                                                        datatype: Some(12),
                                                    }),
                                                    html: None,
                                                    app: None,
                                                });
                                            },
                                            None => (),
                                        }
                                    },
                                    None => (),
                                }
                                match &ad.ad_app {
                                    Some(ad_app) => {
                                        let asset = Asset {
                                            id: assets.consume_asset("app"),
                                            req: 0,
                                            title: None,
                                            img: None,
                                            video: None,
                                            data: None,
                                            html: None,
                                            app: Some(AppAsset {
                                                name: ad_app.app_name.clone(),
                                                desc: ad_app.app_desc.clone(),
                                                descurl: ad_app.app_desc_url.clone(),
                                                domain: None,
                                                bundle: Some(ad_app.package_name.clone()),
                                                ver: Some(ad_app.app_version.clone()),
                                                developer: Some(ad_app.app_developer.clone()),
                                                icon: {
                                                    match &ad_app.app_icon {
                                                        Some(app_icon) => Some(app_icon.url.clone()),
                                                        None => None,
                                                    }
                                                },
                                                storeid: {
                                                    match ad_app.itunes_id {
                                                        Some(itunes_id) => Some(itunes_id.to_string()),
                                                        None => None,
                                                    }
                                                },
                                                storeurl: ad_app.download_url.clone(),
                                                paid: 0,
                                                size: {
                                                    match ad_app.app_size {
                                                        Some(app_size) => Some(app_size as i32),
                                                        None => None,
                                                    }
                                                },
                                                md5: None,
                                                registration: None,
                                                privacy: None,
                                                privacyurl: Some(ad_app.privacy_policy_link.clone()),
                                                permission: None,
                                                permissionurl: Some(ad_app.permissions_link.clone()),
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

                                for event in &track_info.view_monitor_urls {
                                    event_vec.push(Event {
                                        eventtype: 501,
                                        method: 1,
                                        url: {
                                            let url = replace_macro(event);
                                            let mut price = ad.bid_price as i32;
                                            if price <= 0 {
                                                price = connection.default_price;
                                            }
                                            let encrypt_price = Self::encrypt_price(price, &"".to_string(), connection);
                                            url.replace("__PRICE__", &encode(encrypt_price.as_str()))
                                        },
                                        header: None,
                                        content: None,
                                    });
                                }
                                for event in &track_info.click_monitor_urls {
                                    event_vec.push(Event {
                                        eventtype: 502,
                                        method: 1,
                                        url: replace_macro(event),
                                        header: None,
                                        content: None,
                                    });
                                }
                                for event in &track_info.download_start_monitor_urls {
                                    event_vec.push(Event {
                                        eventtype: 601,
                                        method: 1,
                                        url: replace_macro(event),
                                        header: None,
                                        content: None,
                                    });
                                }
                                for event in &track_info.download_finish_monitor_urls {
                                    event_vec.push(Event {
                                        eventtype: 602,
                                        method: 1,
                                        url: replace_macro(event),
                                        header: None,
                                        content: None,
                                    });
                                }
                                for event in &track_info.install_start_monitor_urls {
                                    event_vec.push(Event {
                                        eventtype: 603,
                                        method: 1,
                                        url: replace_macro(event),
                                        header: None,
                                        content: None,
                                    });
                                }
                                for event in &track_info.installed_monitor_urls {
                                    event_vec.push(Event {
                                        eventtype: 604,
                                        method: 1,
                                        url: replace_macro(event),
                                        header: None,
                                        content: None,
                                    });
                                }
                                for event in &track_info.dp_click_monitor_urls {
                                    event_vec.push(Event {
                                        eventtype: 503,
                                        method: 1,
                                        url: replace_macro(event),
                                        header: None,
                                        content: None,
                                    });
                                }
                                for event in &track_info.dp_success_monitor_urls {
                                    event_vec.push(Event {
                                        eventtype: 504,
                                        method: 1,
                                        url: replace_macro(event),
                                        header: None,
                                        content: None,
                                    });
                                }
                                for event in &track_info.dp_failed_monitor_urls {
                                    event_vec.push(Event {
                                        eventtype: 505,
                                        method: 1,
                                        url: replace_macro(event),
                                        header: None,
                                        content: None,
                                    });
                                }
                                for event in &track_info.video_play_start_monitor_urls {
                                    event_vec.push(Event {
                                        eventtype: 701,
                                        method: 1,
                                        url: replace_macro(event),
                                        header: None,
                                        content: None,
                                    });
                                }
                                for event in &track_info.video_play_end_monitor_urls {
                                    event_vec.push(Event {
                                        eventtype: 705,
                                        method: 1,
                                        url: replace_macro(event),
                                        header: None,
                                        content: None,
                                    });
                                }
                                for event in &track_info.video_play_25_monitor_urls {
                                    event_vec.push(Event {
                                        eventtype: 702,
                                        method: 1,
                                        url: replace_macro(event),
                                        header: None,
                                        content: None,
                                    });
                                }
                                for event in &track_info.video_play_50_monitor_urls {
                                    event_vec.push(Event {
                                        eventtype: 703,
                                        method: 1,
                                        url: replace_macro(event),
                                        header: None,
                                        content: None,
                                    });
                                }
                                for event in &track_info.video_play_75_monitor_urls {
                                    event_vec.push(Event {
                                        eventtype: 704,
                                        method: 1,
                                        url: replace_macro(event),
                                        header: None,
                                        content: None,
                                    });
                                }
                                for event in &track_info.video_pause_monitor_urls {
                                    event_vec.push(Event {
                                        eventtype: 708,
                                        method: 1,
                                        url: replace_macro(event),
                                        header: None,
                                        content: None,
                                    });
                                }
                                for event in &track_info.video_continue_monitor_urls {
                                    event_vec.push(Event {
                                        eventtype: 709,
                                        method: 1,
                                        url: replace_macro(event),
                                        header: None,
                                        content: None,
                                    });
                                }
                                for event in &track_info.video_skip_monitor_urls {
                                    event_vec.push(Event {
                                        eventtype: 710,
                                        method: 1,
                                        url: replace_macro(event),
                                        header: None,
                                        content: None,
                                    });
                                }

                                display.event = event_vec;

                                display
                            },
                            advertiser: {
                                match &ad.logo_info {
                                    Some(logo_info) => {
                                        logo_info.title.clone()
                                    },
                                    None => None,
                                }
                            },
                            advertisericon: {
                                match &ad.logo_info {
                                    Some(logo_info) => {
                                        match &logo_info.image {
                                            Some(image) => Some(image.url.clone()),
                                            None => None,
                                        }
                                    },
                                    None => None,
                                }
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
            let pool_jinmo_lock = pool.pool_jinmo.clone();
            let pool_jinmo = pool_jinmo_lock.read().unwrap();
            pool_jinmo.clone()
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

    replaced = replaced.replace("__REQ_WIDTH__", "__WIDTH__");
    replaced = replaced.replace("__REQ_HEIGHT__", "__HEIGHT__");

    replaced = replaced.replace("__DBZ_DX__", "__ABS_DOWN_X__");
    replaced = replaced.replace("__DBZ_DY__", "__ABS_DOWN_Y__");
    replaced = replaced.replace("__DBZ_UX__", "__ABS_UP_X__");
    replaced = replaced.replace("__DBZ_UY__", "__ABS_UP_Y__");
    replaced = replaced.replace("__SBZ_DX__", "__R_DOWN_X__");
    replaced = replaced.replace("__SBZ_DY__", "__R_DOWN_Y__");
    replaced = replaced.replace("__SBZ_UX__", "__R_UP_X__");
    replaced = replaced.replace("__SBZ_UY__", "__R_UP_Y__");
    replaced = replaced.replace("__MBZ_DX__", "__DOWN_X__");
    replaced = replaced.replace("__MBZ_DY__", "__DOWN_Y__");
    replaced = replaced.replace("__MBZ_UX__", "__UP_X__");
    replaced = replaced.replace("__MBZ_UY__", "__UP_Y__");

    replaced = replaced.replace("__TS_SEC__", "__TS_S__");
    replaced = replaced.replace("__LATITUDE__", "__LAT__");
    replaced = replaced.replace("__LONGITUDE__", "__LON__");

    replaced = replaced.replace("__SOD1__", "__SLD__");
    replaced = replaced.replace("__SOD2__", "__SLD__");
    replaced = replaced.replace("__SOD3__", "__SLD__");

    replaced
}
