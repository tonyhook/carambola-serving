use std::{io::Write, time::Duration};

use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyInit};
use base64::prelude::*;
use chrono::{DateTime, Datelike, Local, TimeZone, Utc};
use chrono_tz::Tz;
use flate2::{Compression, write::GzEncoder};
use urlencoding::encode;

use crate::{protocol::*, Assets, Cache, Client, Connection, HttpPool, Identifiers, Price, ResultMessage};

type Aes128EcbEnc = ecb::Encryptor<aes::Aes128>;

pub mod ad;
pub mod app;
pub mod app_info;
pub mod caid;
pub mod creative;
pub mod device_id;
pub mod device;
pub mod ext;
pub mod geo;
pub mod image;
pub mod imp;
pub mod mini_program;
pub mod network;
pub mod permission;
pub mod request;
pub mod response;
pub mod reward;
pub mod tracker;
pub mod user;
pub mod video;
pub mod web_site;

pub use ad::YiweiAd;
pub use app::YiweiApp;
pub use app_info::YiweiAppInfo;
pub use caid::YiweiCaid;
pub use creative::YiweiCreative;
pub use device_id::YiweiDeviceId;
pub use device::YiweiDevice;
pub use ext::YiweiExt;
pub use geo::YiweiGeo;
pub use image::YiweiImage;
pub use imp::YiweiImp;
pub use mini_program::YiweiMiniProgram;
pub use network::YiweiNetwork;
pub use permission::YiweiPermission;
pub use request::YiweiRequest;
pub use response::YiweiResponse;
pub use reward::YiweiReward;
pub use tracker::YiweiTracker;
pub use user::YiweiUser;
pub use video::YiweiVideo;
pub use web_site::YiweiWebSite;

pub struct Yiwei {

}

impl Client for Yiwei {

    async fn request(request: &Request, connection: &Connection, pool: &HttpPool, cache: &Cache) -> Result<Response, ResultMessage> {
        let time = Utc::now();

        let slot_id = connection.client_tag_id.split("|").nth(0).unwrap();
        let pub_id = connection.client_tag_id.split("|").nth(1).unwrap();
        let app_id = connection.client_tag_id.split("|").nth(2).unwrap();

        let request_id = cache.get_sequence();

        let mut assets = Assets::new(request);
        let identifiers = Identifiers::new(request);

        let request_yiwei = YiweiRequest {
            request_id: {
                request_id.to_string()
            },
            protocol_version: {
                "1.0.3".to_string()
            },
            app: {
                match &request.context.app {
                    Some(app) => {
                        YiweiApp {
                            app_id: Some(app_id.to_string()),
                            app_name: Some(app.name.clone()),
                            pkg_name: app.bundle.clone(),
                            itunes_id: app.storeid.clone(),
                            app_version: {
                                match &app.ver {
                                    Some(ver) => ver.clone(),
                                    None => return Err(ResultMessage {
                                        code: 998,
                                        message: "request.context.app.ver is required for upstream".to_string(),
                                    }),
                                }
                            },
                            store_url: app.storeurl.clone(),
                            pub_id: pub_id.to_string(),
                            developer_domain: None,
                            is_paid: Some(app.paid),
                            keywords: None,
                            categories: None,
                            language: None,
                            country: None,
                        }
                    },
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.app is required for upstream".to_string(),
                    }),
                }
            },
            web_site: {
                match &request.context.site {
                    Some(site) => {
                        Some(YiweiWebSite {
                            site_id: Some(app_id.to_string()),
                            site_name: Some(site.name.clone()),
                            domain: {
                                match &site.domain {
                                    Some(domain) => domain.clone(),
                                    None => return Err(ResultMessage {
                                        code: 998,
                                        message: "request.context.site.domain is required for upstream".to_string(),
                                    }),
                                }
                            },
                            url: {
                                match &site.page {
                                    Some(page) => page.clone(),
                                    None => return Err(ResultMessage {
                                        code: 998,
                                        message: "request.context.site.page is required for upstream".to_string(),
                                    }),
                                }
                            },
                            referrer: site.referrer.clone(),
                            keywords: None,
                        })
                    },
                    None => None,
                }
            },
            device: {
                YiweiDevice {
                    device_id: {
                        YiweiDeviceId {
                            advance_id: None,
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
                            imsi: {
                                match identifiers.get_id(503, 0) {
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
                            idfv_md5: {
                                match identifiers.get_id(516, 0) {
                                    Some(uid) => Some(uid.id.clone()),
                                    None => None,
                                }
                            },
                            open_udid: {
                                None
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
                            wifi_mac: {
                                match identifiers.get_id(522, 0) {
                                    Some(uid) => Some(uid.id.clone()),
                                    None => None,
                                }
                            },
                            wifi_mac_md5: {
                                None
                            },
                            ssid: {
                                match identifiers.get_id(524, 0) {
                                    Some(uid) => Some(uid.id.clone()),
                                    None => None,
                                }
                            },
                            caids: {
                                match identifiers.get_ids(504) {
                                    Some(uids) => {
                                        let mut caids = vec![];
                                        for uid in uids {
                                            match &uid.ver {
                                                Some(ver) => caids.push(YiweiCaid {
                                                    caid: Some(uid.id.clone()),
                                                    caid_md5: None,
                                                    version: Some(ver.clone()),
                                                }),
                                                _ => (),
                                            }
                                        }
                                        Some(caids)
                                    },
                                    None => None,
                                }
                            },
                            paid: {
                                match identifiers.get_id(519, 0) {
                                    Some(uid) => Some(uid.id.clone()),
                                    None => None,
                                }
                            },
                            aaid: {
                                match identifiers.get_id(514, 0) {
                                    Some(uid) => Some(uid.id.clone()),
                                    None => None,
                                }
                            }
                        }
                    },
                    device_type: {
                        match request.context.device.devicetype {
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
                            None => return Err(ResultMessage {
                                code: 998,
                                message: "request.context.device.ip is required for upstream".to_string(),
                            }),
                        }
                    },
                    ipv6: {
                        request.context.device.ipv6.clone()
                    },
                    ua: {
                        request.context.device.ua.clone()
                    },
                    os: {
                        match &request.context.device.os {
                            Some(2) => 1,
                            Some(13) => 2,
                            Some(28) => 3,
                            _ => 0,
                        }
                    },
                    osv: {
                        match &request.context.device.osv {
                            Some(osv) => osv.clone(),
                            None => return Err(ResultMessage {
                                code: 998,
                                message: "request.context.device.osv is required for upstream".to_string(),
                            }),
                        }
                    },
                    manufacturer: {
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
                    language: {
                        match &request.context.device.lang {
                            Some(lang) => lang.clone(),
                            None => return Err(ResultMessage {
                                code: 998,
                                message: "request.context.device.lang is required for upstream".to_string(),
                            }),
                        }
                    },
                    ro_locale: None,
                    locale_country: None,
                    sim_country: None,
                    belong_country: {
                        "CN".to_string()
                    },
                    vendor_country: None,
                    ro_locale_country: None,
                    store_country: None,
                    router_country: None,
                    orientation: {
                        match request.context.device.orientation {
                            Some(501) => 1,
                            Some(502) => 2,
                            _ => 0,
                        }
                    },
                    width: {
                        match request.context.device.w {
                            Some(w) => w,
                            None => return Err(ResultMessage {
                                code: 998,
                                message: "request.context.device.w is required for upstream".to_string(),
                            }),
                        }
                    },
                    height: {
                        match request.context.device.h {
                            Some(h) => h,
                            None => return Err(ResultMessage {
                                code: 998,
                                message: "request.context.device.h is required for upstream".to_string(),
                            }),
                        }
                    },
                    density: {
                        request.context.device.pxratio.clone()
                    },
                    dpi: {
                        request.context.device.ppi.clone()
                    },
                    ppi: {
                        request.context.device.ppi.clone()
                    },
                    dpr: {
                        request.context.device.pxratio.clone()
                    },
                    sys_ui_version: {
                        request.context.device.uiv.clone()
                    },
                    build_version: None,
                    store_version: {
                        match &request.context.device.storev {
                            Some(storev) => storev.clone(),
                            None => return Err(ResultMessage {
                                code: 998,
                                message: "request.context.device.storev is required for upstream".to_string(),
                            }),
                        }
                    },
                    hms_version: {
                        request.context.device.hmsv.clone()
                    },
                    is_tracking_enabled: {
                        match identifiers.get_id(505, 0) {
                            Some(_) => true,
                            None => false,
                        }
                    },
                    is_gaid_tracking_enabled: {
                        false
                    },
                    idfa_auth_status: {
                        match request.context.device.lmt {
                            Some(lmt) => lmt,
                            None => 0,
                        }
                    },
                    client_time: {
                        format!("{}", time.format("%Y-%m-%d %H:%M:%S.3fZ"))
                    },
                    serial_no: {
                        request.context.device.serial.clone()
                    },
                    meid: {
                        match identifiers.get_id(520, 0) {
                            Some(uid) => Some(uid.id.clone()),
                            None => None,
                        }
                    },
                    rom_version: {
                        request.context.device.romv.clone()
                    },
                    android_api_level: {
                        request.context.device.oslevel.clone()
                    },
                    boot_mark: {
                        request.context.device.bootmark.clone()
                    },
                    update_mark: {
                        request.context.device.updatemark.clone()
                    },
                    boot_sec: {
                        match &request.context.device.boottime {
                            Some(boottime) => {
                                match boottime.parse::<f64>() {
                                    Ok(time) => Some(time),
                                    Err(_) => None,
                                }
                            },
                            None => None,
                        }
                    },
                    update_sec: {
                        match &request.context.device.updatetime {
                            Some(updatetime) => {
                                match updatetime.parse::<f64>() {
                                    Ok(time) => Some(time),
                                    Err(_) => None,
                                }
                            },
                            None => None,
                        }
                    },
                    up_time: None,
                    cpu_freq: {
                        match request.context.device.syscpufreq {
                            Some(syscpufreq) => Some(syscpufreq.to_string()),
                            None => None,
                        }
                    },
                    cpu_num: {
                        request.context.device.syscpu.clone()
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
                    memory_size: {
                        request.context.device.sysmemory.clone()
                    },
                    disk_size: {
                        request.context.device.sysdisksize.clone()
                    },
                    disk_space_left: None,
                    hardware_machine: {
                        request.context.device.hwmachine.clone()
                    },
                    hardware_model: {
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
                                        Some(t.timestamp() - utc.timestamp())
                                    },
                                    Err(_) => None,
                                }
                            },
                            None => None,
                        }
                    },
                    time_zone_name: {
                        request.context.device.timezone.clone()
                    },
                    birth_time: {
                        request.context.device.inittime.clone()
                    },
                    compiling_time: {
                        request.context.device.romtime.clone()
                    },
                    battery_power: {
                        request.context.device.sysbatterypower.clone()
                    },
                    battery_status: {
                        request.context.device.sysbatterystatus.clone()
                    },
                    network: {
                        YiweiNetwork {
                            carrier: {
                                match &request.context.device.carrier {
                                    Some(carrier) => {
                                        match carrier.as_str() {
                                            "cmcc" => 1,
                                            "unicom" => 3,
                                            "telecom" => 2,
                                            _ => 0,
                                        }
                                    },
                                    None => 0,
                                }
                            },
                            mcc: {
                                match &request.context.device.mccmnc {
                                    Some(mccmnc) => {
                                        if mccmnc.len() >= 3 {
                                            Some(mccmnc[0..3].to_string().parse().unwrap())
                                        } else {
                                            Some(460)
                                        }
                                    },
                                    None => None,
                                }
                            },
                            mnc: {
                                match &request.context.device.mccmnc {
                                    Some(mccmnc) => {
                                        if mccmnc.len() >= 6 {
                                            Some(mccmnc[4..6].to_string().parse().unwrap())
                                        } else {
                                            match &request.context.device.carrier {
                                                Some(carrier) => {
                                                    match carrier.as_str() {
                                                        "cmcc" => Some(0),
                                                        "unicom" => Some(1),
                                                        "telecom" => Some(3),
                                                        "cbn" => Some(15),
                                                        _ => None,
                                                    }
                                                },
                                                None => None,
                                            }
                                        }
                                    },
                                    None => None,
                                }
                            },
                            conn_type: {
                                match &request.context.device.contype {
                                    Some(contype) => {
                                        match contype {
                                            1 => 100,
                                            2 => 1,
                                            3 => 0,
                                            4 => 2,
                                            5 => 3,
                                            6 => 4,
                                            7 => 5,
                                            _ => return Err(ResultMessage {
                                                code: 998,
                                                message: "request.context.device.contype should be 1-7 for upstream".to_string(),
                                            }),
                                        }
                                    },
                                    None => return Err(ResultMessage {
                                        code: 998,
                                        message: "request.context.device.contype is required for upstream".to_string(),
                                    }),
                                }
                            },
                        }
                    },
                    geo: {
                        match &request.context.device.geo {
                            Some(geo) => {
                                Some(YiweiGeo {
                                    latitude: {
                                        geo.lat.clone()
                                    },
                                    longitude: {
                                        geo.lon.clone()
                                    },
                                    accuracy: {
                                        match geo.accur {
                                            Some(accur) => Some(accur as i32),
                                            None => None,
                                        }
                                    },
                                    source: {
                                        geo.geotype.clone()
                                    },
                                    last_fix: {
                                        None
                                    },
                                    city_code: {
                                        geo.city.clone()
                                    },
                                    province_code: {
                                        geo.province.clone()
                                    },
                                    district_code: {
                                        geo.district.clone()
                                    },
                                    country_code: {
                                        geo.country.clone()
                                    },
                                })
                            },
                            None => None,
                        }
                    },
                    boot_milli_sec: {
                        match &request.context.device.boottime {
                            Some(boottime) => {
                                match boottime.parse::<f64>() {
                                    Ok(time) => Some(time * 1000.0),
                                    Err(_) => None,
                                }
                            },
                            None => None,
                        }
                    },
                    update_nano_sec: {
                        match &request.context.device.updatetime {
                            Some(updatetime) => {
                                match updatetime.parse::<f64>() {
                                    Ok(time) => Some(time * 1000.0),
                                    Err(_) => None,
                                }
                            },
                            None => None,
                        }
                    },
                    boot_time: {
                        match &request.context.device.boottime {
                            Some(boottime) => {
                                match boottime.parse::<f64>() {
                                    Ok(time) => Some(time.to_string()),
                                    Err(_) => None,
                                }
                            },
                            None => None,
                        }
                    },
                    update_time: {
                        match &request.context.device.updatetime {
                            Some(updatetime) => {
                                match updatetime.parse::<f64>() {
                                    Ok(time) => Some(time.to_string()),
                                    Err(_) => None,
                                }
                            },
                            None => None,
                        }
                    },
                }
            },
            imps: {
                [YiweiImp {
                    imp_id: {
                        request_id.to_string()
                    },
                    slot_id: {
                        slot_id.to_string()
                    },
                    ad_type: {
                        let mut ad_type = 0;
                        let reward = request.item[0].spec.reward;
                        let instl = request.item[0].spec.display.instl;

                        if assets.get_banner_size() > 0 {
                            if instl == 1 {
                                ad_type = 5;
                            } else {
                                if request.item[0].spec.display.w > request.item[0].spec.display.h {
                                    ad_type = 3;
                                } else {
                                    ad_type = 1;
                                }
                            }
                        }
                        if assets.get_asset_size("img") > 0 {
                            ad_type = 2;
                        }
                        if assets.get_asset_size("thumb") > 0 {
                            ad_type = 2;
                        }
                        if assets.get_asset_size("video") > 0 {
                            if instl == 1 {
                                ad_type = 7;
                            } else {
                                if reward == 1 {
                                    ad_type = 4;
                                } else {
                                    ad_type = 6;
                                }
                            }
                        }

                        ad_type
                    },
                    ad_count: {
                        1
                    },
                    is_test: {
                        connection.test
                    },
                    bid_floor: {
                        Price::to_client(connection, request.item[0].flr.map(f64::from)) as i64
                    },
                    bid_floor_currency: {
                        Some("CNY".to_string())
                    },
                    video_min_duration: {
                        if assets.get_asset_size("video") == 1 && assets.get_asset_size("video") == assets.get_asset_total_size() {
                            let video = assets.get_current_asset("video").unwrap().video.clone().unwrap();
                            match video.mindur {
                                Some(mindur) => {
                                    Some(mindur)
                                },
                                None => None,
                            }
                        } else {
                            None
                        }
                    },
                    video_max_duration: {
                        if assets.get_asset_size("video") == 1 && assets.get_asset_size("video") == assets.get_asset_total_size() {
                            let video = assets.get_current_asset("video").unwrap().video.clone().unwrap();
                            match video.maxdur {
                                Some(maxdur) => {
                                    Some(maxdur)
                                },
                                None => None,
                            }
                        } else {
                            None
                        }
                    },
                    creative_types: {
                        let mut creative_types = [].to_vec();

                        let mut has_image = false;
                        let mut has_video = false;
                        let mut has_text = false;

                        if assets.get_banner_size() > 0 {
                            has_image = true;
                        }
                        if assets.get_asset_size("img") > 0 {
                            has_image = true;
                        }
                        if assets.get_asset_size("thumb") > 0 {
                            has_image = true;
                        }
                        if assets.get_asset_size("video") > 0 {
                            has_video = true;
                        }
                        if assets.get_asset_size("title") > 0 || assets.get_asset_size("html") > 0 {
                            has_text = true;
                        }

                        if has_image {
                            creative_types.push(1);
                        }
                        if has_video {
                            creative_types.push(2);
                        }
                        if has_text {
                            creative_types.push(3);
                        }

                        creative_types
                    },
                    width: {
                        match request.item[0].spec.display.w {
                            Some(w) => w,
                            None => return Err(ResultMessage {
                                code: 998,
                                message: "request.item[0].spec.display.w is required for upstream".to_string(),
                            })
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
                    interaction_types: {
                        [2, 3, 4, 5, 6].to_vec()
                    },
                    accept_price_type: {
                        1
                    },
                    is_supports_302: {
                        Some(false)
                    },
                    is_supports_ctr_agent: {
                        Some(false)
                    },
                    secure_support: {
                        Some(0)
                    },
                    block_bundles: {
                        None
                    },
                    block_domains: {
                        None
                    },
                    block_categories: {
                        None
                    },
                }].to_vec()
            },
            timestamp: {
                let utc: DateTime<Utc> = Utc::now();
                utc.timestamp_millis()
            },
            timeout: {
                Some(connection.timeout as i32)
            },
            user: {
                Some(YiweiUser {
                    uid: {
                        request.context.user.id.clone()
                    },
                    gender: {
                        match &request.context.user.gender {
                            Some(gender) => {
                                match gender.as_str() {
                                    "M" => Some(1),
                                    "F" => Some(2),
                                    _ => Some(0),
                                }
                            },
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
                        match &request.context.user.keywords {
                            Some(keywords) => Some(keywords.split(",").map(|s| s.to_string()).collect()),
                            None => None,
                        }
                    },
                    tags: {
                        None
                    },
                    installed_apps: {
                        request.context.device.app.clone()
                    },
                    is_ias_base64: {
                        None
                    },
                    categories: {
                        None
                    },
                })
            },
            currencies: None,
            ext: None,
            is_test: {
                match connection.test {
                    true => Some(1),
                    false => None,
                }
            },
        };

        let json_string = serde_json::to_vec(&request_yiwei).unwrap();
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&json_string).unwrap();
        let compressed_bytes = encoder.finish().unwrap();

        let response_yiwei: YiweiResponse;

        let client = {
            let pool_yiwei_lock = pool.pool_yiwei.clone();
            let pool_yiwei = pool_yiwei_lock.read().unwrap();
            pool_yiwei.clone()
        };
        let response_yiwei_raw = client.post(format!("{}", if connection.test { "http://rtb.yiweiads.com/getAd/advance_pb_test" } else { "http://rtb.yiweiads.com/getAd/advance_pb_test" }))
            .body(compressed_bytes)
            .header("Accept-Encoding", "gzip")
            .header("Connection", "keep-alive")
            .header("Content-Encoding", "gzip")
            .header("Content-Type", "application/json")
            .timeout(Duration::from_millis(connection.timeout))
            .send().await;

        match response_yiwei_raw {
            Ok(response_yiwei_raw) => {
                let status = response_yiwei_raw.status();
                if status != 200 {
                    return Err(ResultMessage {
                        code: 992,
                        message: {
                            match response_yiwei_raw.text().await {
                                Ok(text) => format!("upstream error {}: {}", status, text),
                                Err(_) => format!("upstream error {}", status),
                            }
                        },
                    });
                } else {
                    match response_yiwei_raw.text().await {
                        Ok(text) => {
                            match serde_json::from_str::<YiweiResponse>(&text) {
                                Ok(json) => {
                                    if json.code == 0 {
                                        response_yiwei = json;
                                    } else {
                                        if json.code == 10001 {
                                            return Err(ResultMessage {
                                                code: 993,
                                                message: "".to_string(),
                                            });
                                        } else {
                                            match json.msg {
                                                Some(msg) => {
                                                    return Err(ResultMessage {
                                                        code: 994,
                                                        message: format!("upstream error: {}", msg),
                                                    });
                                                },
                                                None => {
                                                    return Err(ResultMessage {
                                                        code: 994,
                                                        message: "upstream error".to_string(),
                                                    });
                                                },
                                            }
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

                        for ad in response_yiwei.ads.unwrap() {
                            let link_asset = LinkAsset {
                                linktype: {
                                    match ad.interaction_type {
                                        3 => 2,
                                        6 => 3,
                                        _ => 1,
                                    }
                                },
                                universallink: None,
                                storeid: None,
                                deeplink: {
                                    ad.creative.deeplink_url.clone()
                                },
                                quickapplink: None,
                                wechatmppath: {
                                    match &ad.creative.mini_program {
                                        Some(mini_program) => {
                                            if mini_program.miniprogramtype == Some(1) {
                                                Some(mini_program.jump_url.clone())
                                            } else {
                                                None
                                            }
                                        },
                                        None => None,
                                    }
                                },
                                wechatmpid: {
                                    match &ad.creative.mini_program {
                                        Some(mini_program) => {
                                            if mini_program.miniprogramtype == Some(1) {
                                                mini_program.app_id.clone()
                                            } else {
                                                None
                                            }
                                        },
                                        None => None,
                                    }
                                },
                                marketurl: {
                                    ad.creative.store_url.clone()
                                },
                                downloadurl: {
                                    ad.creative.download_url.clone()
                                },
                                url: {
                                    match ad.creative.target_url {
                                        Some(target_url) => {
                                            target_url
                                        },
                                        None => {
                                            "".to_string()
                                        },
                                    }
                                },
                                urlfb: None,
                            };

                            let bid = Bid {
                                id: Some(request_id.to_string()),
                                item: request.item[0].id.clone(),
                                price: { // update later
                                    if ad.price > 0 {
                                        ad.price as i32
                                    } else {
                                        connection.default_price
                                    }
                                },
                                burl: {
                                    match &ad.creative.nurl {
                                        Some(nurl) => {
                                            let mut burl = Vec::<String>::new();
                                            let mut nurl = nurl.clone();
                                            nurl = nurl.replace("${bid_price}", "__WIN_PRICE__");
                                            burl.push(replace_macro(&nurl));
                                            Some(burl)
                                        },
                                        None => None,
                                    }
                                },
                                lurl: {
                                    match &ad.creative.lurl {
                                        Some(bidlurl) => {
                                            let mut lurl = Vec::<String>::new();
                                            let mut nurl = bidlurl.clone();
                                            nurl = nurl.replace("${bid_price}", "__WIN_PRICE__");
                                            lurl.push(replace_macro(&nurl));
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
                                                match &ad.creative.imgs {
                                                    Some(imgs) => {
                                                        Some(Banner {
                                                            img: imgs[0].url.clone(),
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

                                                match &ad.creative.imgs {
                                                    Some(imgs) => {
                                                        if assets.get_asset_size("img") > 0 {
                                                            for img in imgs {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("img"),
                                                                    req: 1,
                                                                    img: Some(ImageAsset {
                                                                        url: img.url.clone(),
                                                                        mime: img.mime_type.clone(),
                                                                        w: Some(img.width),
                                                                        h: Some(img.height),
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
                                                                        url: img.url.clone(),
                                                                        mime: img.mime_type.clone(),
                                                                        w: Some(img.width),
                                                                        h: Some(img.height),
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

                                                match &ad.creative.icon {
                                                    Some(icon) => {
                                                        asset_vec.push(Asset {
                                                            id: assets.consume_asset("icon"),
                                                            req: 1,
                                                            title: None,
                                                            img: Some(ImageAsset {
                                                                url: icon.url.clone(),
                                                                mime: icon.mime_type.clone(),
                                                                w: Some(icon.width),
                                                                h: Some(icon.height),
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

                                                match &ad.creative.title {
                                                    Some(title) => {
                                                        asset_vec.push(Asset {
                                                            id: assets.consume_asset("title"),
                                                            req: 1,
                                                            title: Some(TitleAsset {
                                                                text: title.clone(),
                                                                subtitle: None,
                                                                desc: ad.creative.desc.clone(),
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

                                                match &ad.creative.video {
                                                    Some(video) => {
                                                        asset_vec.push(Asset {
                                                            id: assets.consume_asset("video"),
                                                            req: 1,
                                                            video: Some(VideoAsset {
                                                                url: video.url.clone(),
                                                                mime: video.mime_type.clone(),
                                                                w: Some(video.width),
                                                                h: Some(video.height),
                                                                dur: video.duration.clone(),
                                                                skipoffset: ad.creative.min_duration.clone(),
                                                                size: {
                                                                    match video.file_size {
                                                                        Some(size) => Some(size as i32),
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

                                                        asset_vec.push(Asset {
                                                            id: assets.consume_asset("video#cover"),
                                                                req: 0,
                                                                img: Some(ImageAsset {
                                                                    url: video.cover_url.clone(),
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
                                                        match &video.end_card_url {
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
                                                    },
                                                    None => (),
                                                }

                                                match &ad.creative.btn_text {
                                                    Some(btn_text) => {
                                                        asset_vec.push(Asset {
                                                            id: assets.consume_asset("data#ctatext"),
                                                            req: 1,
                                                            title: None,
                                                            img: None,
                                                            video: None,
                                                            data: Some(DataAsset {
                                                                value: btn_text.clone(),
                                                                len: None,
                                                                datatype: Some(12),
                                                            }),
                                                            html: None,
                                                            app: None,
                                                        });
                                                    },
                                                    None => (),
                                                }

                                                match &ad.creative.app {
                                                    Some(app) => {
                                                        asset_vec.push(Asset {
                                                            id: assets.consume_asset("app"),
                                                            req: 0,
                                                            app: Some(AppAsset {
                                                                name: app.app_name.clone(),
                                                                desc: app.desc.clone(),
                                                                descurl: app.introduce_url.clone(),
                                                                domain: app.developer_domain.clone(),
                                                                bundle: app.pkg_name.clone(),
                                                                ver: app.app_version.clone(),
                                                                developer: app.developer_name.clone(),
                                                                icon: app.icon_url.clone(),
                                                                storeid: app.itunes_id.clone(),
                                                                storeurl: None,
                                                                paid: 0,
                                                                size: {
                                                                    match app.pkg_size {
                                                                        Some(size) => Some(size as i32),
                                                                        None => None,
                                                                    }
                                                                },
                                                                md5: None,
                                                                registration: app.icp_number.clone(),
                                                                privacy: None,
                                                                privacyurl: app.privacy_url.clone(),
                                                                permission: None,
                                                                permissionurl: app.permission_url.clone(),
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

                                            for tracker in &ad.creative.trackers {
                                                let eventtype = match tracker.event_type.as_str() {
                                                    "imp" => 501,
                                                    "click" => 502,
                                                    "video_play_start" => 701,
                                                    "video_play_q1" => 702,
                                                    "video_play_half" => 703,
                                                    "video_play_q3" => 704,
                                                    "video_play_finish" => 705,
                                                    "video_play_pause" => 708,
                                                    "video_play_resume" => 709,
                                                    "video_close" => 711,
                                                    "video_full_screen" => 715,
                                                    "video_float_window" => 725,
                                                    "video_restore" => 716,
                                                    "video_mute" => 713,
                                                    "video_unmute" => 714,
                                                    "video_skip" => 710,
                                                    "deeplink_attempt" => 503,
                                                    "deeplink_success" => 504,
                                                    "deeplink_fail" => 505,
                                                    "download_start" => 601,
                                                    "download_finish" => 602,
                                                    "install_start" => 603,
                                                    "install_finish" => 604,
                                                    "activated" => 605,
                                                    "reward_video_loaded" => 719,
                                                    "reward_video_error" => 720,
                                                    "reward_video_success" => 726,
                                                    "ad_close" => 509,
                                                    _ => continue,
                                                };

                                                for url in &tracker.urls {
                                                    let mut replaced_url = replace_macro(url);

                                                    if eventtype == 501 || eventtype == 502 {
                                                        let encrypt_price = Self::encrypt_price(ad.price as i32, &"".to_string(), connection);
                                                        replaced_url = replaced_url.replace("${bid_price}", &encode(encrypt_price.as_str()));
                                                    }

                                                    event_vec.push(Event {
                                                        eventtype,
                                                        method: 1,
                                                        url: replaced_url,
                                                        header: None,
                                                        content: None,
                                                    });
                                                }
                                            }

                                            event_vec
                                        }
                                    },
                                    advertiser: {
                                        response_yiwei.adv.clone()
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
            let pool_yiwei_lock = pool.pool_yiwei.clone();
            let pool_yiwei = pool_yiwei_lock.read().unwrap();
            pool_yiwei.clone()
        };
        let _ = client.get(replaced_url).send().await;
    }

    async fn bidding_notify_lose(url: String, lose_price: i32, _lose_reason: i32, _lose_adn_name: &String, iv: &String, connection: &Connection, pool: &HttpPool) {
        let encrypt_price = Self::encrypt_price(lose_price, iv, connection);
        let replaced_url = url
            .replace("__LOSE_PRICE__", &encode(encrypt_price.as_str()));

        let client = {
            let pool_sweet_lock = pool.pool_sweet.clone();
            let pool_sweet = pool_sweet_lock.read().unwrap();
            pool_sweet.clone()
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

        let key = &connection.client_ekey.as_bytes();

        let cipher = Aes128EcbEnc::new(key[0..16].into())
            .encrypt_padded_mut::<Pkcs7>(&mut buffer, pos)
            .unwrap();

        let message_base64 = BASE64_STANDARD.encode(cipher);

        message_base64.replace("+", "-").replace("/", "_")
    }

}

fn replace_macro(orig: &String) -> String {
    let mut replaced = orig.clone();

    replaced = replaced.replace("__AD_LT_X__", "__LT_X__");
    replaced = replaced.replace("__AD_LT_Y__", "__LT_Y__");
    replaced = replaced.replace("__AD_RB_X__", "__RB_X_");
    replaced = replaced.replace("__AD_RB_Y__", "__RB_Y__");
    replaced = replaced.replace("__WIDTH__", "__DP_WIDTH__");
    replaced = replaced.replace("__HEIGHT__", "__DP_HEIGHT__");
    replaced = replaced.replace("__CLICK_DOWN_X__", "__DOWN_X__");
    replaced = replaced.replace("__CLICK_DOWN_Y__", "__DOWN_U__");
    replaced = replaced.replace("__CLICK_UP_X__", "__UP_X__");
    replaced = replaced.replace("__CLICK_UP_Y__", "__UP_Y__");
    replaced = replaced.replace("__DP_FAIL_REASON__", "__DP_REASON__");
    replaced = replaced.replace("__VIDEO_START_TIME__", "__VIDEO_BEGIN_TIME__");
    replaced = replaced.replace("__VIDEO_CURR_TIME__", "__VIDEO_PLAY_PROGRESS_S__");
    replaced = replaced.replace("__VIDEO_SCENE__", "__VIDEO_PLAY_SCENE__");
    replaced = replaced.replace("__VIDEO_BEHAVIOR__", "__VIDEO_PLAY_TRIGGER__");
    replaced = replaced.replace("__VIDEO_STATUS__", "__VIDEO_PLAY_STATUS__");

    replaced
}
