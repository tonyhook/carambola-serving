use std::{collections::HashMap, time::Duration};

use chrono::{Datelike, Local, TimeZone};
use chrono_tz::Tz;
use regex::Regex;
use reqwest::Url;
use urlencoding::encode;

use crate::{protocol::*, Assets, Cache, Client, Connection, HttpPool, Identifiers, Price, ResultMessage};

pub mod ad;
pub mod app_list;
pub mod app_request;
pub mod app_response;
pub mod creative_specs;
pub mod device;
pub mod geo;
pub mod icon;
pub mod image;
pub mod imp_request;
pub mod imp_response;
pub mod interaction;
pub mod native;
pub mod request;
pub mod response;
pub mod site;
pub mod size;
pub mod text;
pub mod tracker;
pub mod user;
pub mod video;

pub use ad::MvpmobAd;
pub use app_list::MvpmobAppList;
pub use app_request::MvpmobAppRequest;
pub use app_response::MvpmobAppResponse;
pub use creative_specs::MvpmobCreativeSpecs;
pub use device::MvpmobDevice;
pub use geo::MvpmobGeo;
pub use icon::MvpmobIcon;
pub use image::MvpmobImage;
pub use imp_request::MvpmobImpRequest;
pub use imp_response::MvpmobImpResponse;
pub use interaction::MvpmobInteraction;
pub use native::MvpmobNative;
pub use request::MvpmobRequest;
pub use response::MvpmobResponse;
pub use site::MvpmobSite;
pub use size::MvpmobSize;
pub use text::MvpmobText;
pub use tracker::MvpmobTracker;
pub use user::MvpmobUser;
pub use video::MvpmobVideo;

pub struct Mvpmob {

}

impl Client for Mvpmob {

    async fn request(request: &Request, connection: &Connection, pool: &HttpPool, cache: &Cache) -> Result<Response, ResultMessage> {
        let request_id = cache.get_sequence();

        let mut assets = Assets::new(request);
        let identifiers = Identifiers::new(request);

        let request_mvpmob = MvpmobRequest {
            request_id: {
                Some(request_id.to_string())
            },
            app: {
                MvpmobAppRequest {
                    bundle_id: {
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
                    store_url: {
                        match &request.context.app {
                            Some(app) => {
                                app.storeurl.clone()
                            },
                            None => None,
                        }
                    },
                    app_name: {
                        match &request.context.app {
                            Some(app) => {
                                Some(app.name.clone())
                            },
                            None => None,
                        }
                    },
                    domain: {
                        match &request.context.app {
                            Some(app) => {
                                app.domain.clone()
                            },
                            None => None,
                        }
                    },
                    publisher_name: {
                        None
                    },
                    version: {
                        match &request.context.app {
                            Some(app) => {
                                app.ver.clone()
                            },
                            None => None,
                        }
                    },
                    privacy_link: {
                        None
                    },
                    permission_link: {
                        None
                    },
                }
            },
            site: {
                MvpmobSite {
                    id: {
                        connection.client_tag_id.clone()
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
                                ad_type = 5;
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
                    creative_specs: {
                        let mut creative_specs = Vec::new();

                        if assets.get_banner_size() > 0 {
                            creative_specs.push(MvpmobCreativeSpecs {
                                support_creative_type: 1,
                                support_sizes: MvpmobSize {
                                    w: {
                                        match assets.get_banner().unwrap().w {
                                            Some(w) => w as i32,
                                            None => 0,
                                        }
                                    },
                                    h: {
                                        match assets.get_banner().unwrap().h {
                                            Some(h) => h as i32,
                                            None => 0,
                                        }
                                    },
                                },
                            });
                        }
                        if assets.get_asset_size("img") == 1 {
                            creative_specs.push(MvpmobCreativeSpecs {
                                support_creative_type: 1,
                                support_sizes: MvpmobSize {
                                    w: {
                                        match assets.get_current_asset("img").unwrap().img.clone().unwrap().w {
                                            Some(w) => w as i32,
                                            None => 0,
                                        }
                                    },
                                    h: {
                                        match assets.get_current_asset("img").unwrap().img.clone().unwrap().h {
                                            Some(h) => h as i32,
                                            None => 0,
                                        }
                                    },
                                },
                            });
                        }
                        if assets.get_asset_size("thumb") == 1 {
                            creative_specs.push(MvpmobCreativeSpecs {
                                support_creative_type: 1,
                                support_sizes: MvpmobSize {
                                    w: {
                                        match assets.get_current_asset("thumb").unwrap().img.clone().unwrap().w {
                                            Some(w) => w as i32,
                                            None => 0,
                                        }
                                    },
                                    h: {
                                        match assets.get_current_asset("thumb").unwrap().img.clone().unwrap().h {
                                            Some(h) => h as i32,
                                            None => 0,
                                        }
                                    },
                                },
                            });
                        }
                        if assets.get_asset_size("img") > 1 {
                            creative_specs.push(MvpmobCreativeSpecs {
                                support_creative_type: 4,
                                support_sizes: MvpmobSize {
                                    w: {
                                        match assets.get_current_asset("img").unwrap().img.clone().unwrap().w {
                                            Some(w) => w as i32,
                                            None => 0,
                                        }
                                    },
                                    h: {
                                        match assets.get_current_asset("img").unwrap().img.clone().unwrap().h {
                                            Some(h) => h as i32,
                                            None => 0,
                                        }
                                    },
                                },
                            });
                        }
                        if assets.get_asset_size("thumb") > 1 {
                            creative_specs.push(MvpmobCreativeSpecs {
                                support_creative_type: 4,
                                support_sizes: MvpmobSize {
                                    w: {
                                        match assets.get_current_asset("thumb").unwrap().img.clone().unwrap().w {
                                            Some(w) => w as i32,
                                            None => 0,
                                        }
                                    },
                                    h: {
                                        match assets.get_current_asset("thumb").unwrap().img.clone().unwrap().h {
                                            Some(h) => h as i32,
                                            None => 0,
                                        }
                                    },
                                },
                            });
                        }
                        if assets.get_asset_size("video") > 0 {
                            creative_specs.push(MvpmobCreativeSpecs {
                                support_creative_type: 4,
                                support_sizes: MvpmobSize {
                                    w: {
                                        match assets.get_current_asset("video").unwrap().video.clone().unwrap().w {
                                            Some(w) => w as i32,
                                            None => 0,
                                        }
                                    },
                                    h: {
                                        match assets.get_current_asset("video").unwrap().video.clone().unwrap().h {
                                            Some(h) => h as i32,
                                            None => 0,
                                        }
                                    },
                                },
                            });
                        }

                        creative_specs
                    },
                    support_interaction_type: {
                        [1, 2, 3, 4].to_vec()
                    },
                    support302: {
                        true
                    },
                }
            },
            imp: {
                MvpmobImpRequest {
                    floor_price: {
                        Some(Price::to_client(connection, request.item[0].flr.map(f64::from)))
                    },
                    secure: Some(0),
                }
            },
            device: {
                MvpmobDevice {
                    ua: {
                        request.context.device.ua.clone()
                    },
                    ipv4: {
                        match &request.context.device.ip {
                            Some(ip) => {
                                Some(ip.clone())
                            },
                            None => None,
                        }
                    },
                    ipv6: {
                        match &request.context.device.ipv6 {
                            Some(ipv6) => {
                                Some(ipv6.clone())
                            },
                            None => {
                                Some("::".to_string())
                            },
                        }
                    },
                    lmt: {
                        match &request.context.device.lmt {
                            Some(lmt) => {
                                Some(lmt.clone())
                            },
                            None => {
                                match identifiers.get_id(507, 0) {
                                    Some(_) => Some(3),
                                    None => Some(1),
                                }
                            },
                        }
                    },
                    idfa: {
                        match identifiers.get_id(507, 0) {
                            Some(uid) => {
                                let regex = Regex::new("[0-9A-F]{8}-[0-9A-F]{4}-[1-5][0-9A-F]{3}-[89AB][0-9A-F]{3}-[0-9A-F]{12}").unwrap();
                                if regex.is_match(&uid.id) {
                                    Some(uid.id.clone())
                                } else {
                                    return Err(ResultMessage {
                                        code: 998,
                                        message: format!("idfa format error: {}", uid.id),
                                    });
                                }
                            },
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
                    android_id: {
                        match identifiers.get_id(509, 0) {
                            Some(uid) => uid.id.clone(),
                            None => "".to_string(),
                        }
                    },
                    android_id_md5: {
                        match identifiers.get_id(510, 0) {
                            Some(uid) => uid.id.clone(),
                            None => "".to_string(),
                        }
                    },
                    device_type: {
                        match request.context.device.devicetype {
                            Some(devicetype) => {
                                match devicetype {
                                    4 => 1,
                                    5 => 2,
                                    _ => 1,
                                }
                            },
                            None => 1,
                        }
                    },
                    h: {
                        match request.context.device.h {
                            Some(h) => h,
                            None => 0,
                        }
                    },
                    w: {
                        match request.context.device.w {
                            Some(w) => w,
                            None => 0,
                        }
                    },
                    os: {
                        match request.context.device.os {
                            Some(os) => {
                                match os {
                                    2 => 1,
                                    13 => 2,
                                    _ => 0,
                                }
                            },
                            None => 0,
                        }
                    },
                    osv: {
                        match &request.context.device.osv {
                            Some(osv) => Some(osv.clone()),
                            None => None,
                        }
                    },
                    make: {
                        match &request.context.device.make {
                            Some(make) => Some(make.clone()),
                            None => None,
                        }
                    },
                    model: {
                        match &request.context.device.model {
                            Some(model) => Some(model.clone()),
                            None => None,
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
                    conn_type: {
                        match &request.context.device.contype {
                            Some(contype) => {
                                match contype {
                                    2 => 4,
                                    5 => 1,
                                    6 => 2,
                                    7 => 3,
                                    _ => 99,
                                }
                            },
                            None => 0,
                        }
                    },
                    boot_second: {
                        request.context.device.boottime.clone()
                    },
                    update_second: {
                        request.context.device.updatetime.clone()
                    },
                    cpu_frequency: {
                        request.context.device.syscpufreq.clone()
                    },
                    cpu_number: {
                        request.context.device.syscpu.clone()
                    },
                    battery_power: {
                        request.context.device.sysbatterypower.clone()
                    },
                    battery_status: {
                        match request.context.device.sysbatterystatus {
                            Some(sysbatterystatus) => {
                                match sysbatterystatus {
                                    0 => Some("Unkown".to_string()),
                                    1 => Some("Unplugged".to_string()),
                                    2 => Some("Charging".to_string()),
                                    3 => Some("Full".to_string()),
                                    _ => None,
                                }
                            },
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
                    pre_caid: {
                        match identifiers.get_id(513, 1) {
                            Some(uid) => Some(uid.id.clone()),
                            None => None,
                        }
                    },
                    pre_caid_version: {
                        match identifiers.get_id(513, 1) {
                            Some(uid) => uid.ver.clone(),
                            None => None,
                        }
                    },
                    old_caid_version: {
                        None
                    },
                    country_code: {
                        request.context.device.country.clone()
                    },
                    language: {
                        request.context.device.lang.clone()
                    },
                    phone_name_md5: {
                        match identifiers.get_id(518, 0) {
                            Some(uid) => Some(uid.id.clone()),
                            None => None,
                        }
                    },
                    memory_size: {
                        match request.context.device.sysmemory {
                            Some(sysmemory) => Some(sysmemory as f64),
                            None => Some(0.0),
                        }
                    },
                    disk_size: {
                        match request.context.device.sysdisksize {
                            Some(sysdisksize) => Some(sysdisksize as f64),
                            None => Some(0.0),
                        }
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
                    geo: {
                        match &request.context.device.geo {
                            Some(geo) => {
                                Some(MvpmobGeo {
                                    lat: {
                                        match geo.lat {
                                            Some(lat) => lat,
                                            None => 0.0,
                                        }
                                    },
                                    lon: {
                                        match geo.lon {
                                            Some(lon) => lon,
                                            None => 0.0,
                                        }
                                    },
                                    source: {
                                        1
                                    },
                                })
                            },
                            None => None,
                        }
                    },
                    birth_time: {
                        request.context.device.inittime.clone()
                    },
                    compling_time: {
                        request.context.device.romtime.clone()
                    },
                    boot_mark: {
                        request.context.device.bootmark.clone()
                    },
                    update_mark: {
                        request.context.device.updatemark.clone()
                    },
                    pxratio: {
                        request.context.device.pxratio.clone()
                    },
                    dpi: {
                        request.context.device.ppi.clone()
                    },
                    ppi: {
                        request.context.device.ppi.clone()
                    },
                    brand: {
                        request.context.device.brand.clone()
                    },
                    orientation: {
                        match request.context.device.orientation {
                            Some(orientation) => {
                                match orientation {
                                    501 => Some(0),
                                    502 => Some(1),
                                    _ => Some(9),
                                }
                            },
                            None => Some(9),
                        }
                    },
                    device_start_sec: {
                        request.context.device.boottime.clone()
                    },
                    system_update_sec: {
                        request.context.device.updatetime.clone()
                    },
                    hardware_machine: {
                        request.context.device.hwmachine.clone()
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
                    },
                    mac: {
                        match identifiers.get_id(511, 0) {
                            Some(uid) => Some(uid.id.clone()),
                            None => None,
                        }
                    },
                    api_level: {
                        match request.context.device.oslevel {
                            Some(oslevel) => {
                                Some(oslevel.to_string())
                            },
                            None => None,
                        }
                    },
                    mac_md5: {
                        match identifiers.get_id(512, 0) {
                            Some(uid) => Some(uid.id.clone()),
                            None => None,
                        }
                    },
                    hms_ver: {
                        request.context.device.hmsv.clone()
                    },
                    ag: {
                        request.context.device.storev.clone()
                    },
                    maker: {
                        request.context.device.make.clone()
                    },
                    open_udid: {
                        None
                    },
                }
            },
            user: {
                MvpmobUser {
                    gender: {
                        match &request.context.user.gender {
                            Some(gender) => {
                                match gender.as_str() {
                                    "M" => Some(1),
                                    "F" => Some(2),
                                    _ => None,
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
                    tag_id: {
                        None
                    },
                    app_list: {
                        None
                    },
                    user_category_list: {
                        None
                    },
                }
            },
            tmax: {
                connection.timeout as i32
            },
        };

        let response_mvpmob: MvpmobResponse;

        let client = {
            let pool_mvpmob = pool.pool_mvpmob.clone();
            let pool_mvpmob = pool_mvpmob.read().unwrap();
            pool_mvpmob.clone()
        };
        let response_mvpmob_raw = client.post(if connection.test { "http://test.api.mvpmob.com/api/bid/v2.0" } else { "http://api.mvpmob.com/api/bid/v2.0" })
            .json(&request_mvpmob)
            .header("Accept-Encoding", "gzip")
            .header("Connection", "keep-alive")
            .header("Content-Type", "application/json")
            .timeout(Duration::from_millis(connection.timeout))
            .send().await;

        match response_mvpmob_raw {
            Ok(response_mvpmob_raw) => {
                let status = response_mvpmob_raw.status();
                if status != 200 {
                    return Err(ResultMessage {
                        code: 992,
                        message: {
                            match response_mvpmob_raw.text().await {
                                Ok(text) => format!("upstream error {}: {}", status, text),
                                Err(_) => format!("upstream error {}", status),
                            }
                        },
                    });
                } else {
                    match response_mvpmob_raw.text().await {
                        Ok(text) => {
                            match serde_json::from_str::<MvpmobResponse>(&text) {
                                Ok(json) => {
                                    if json.ad.is_none() {
                                        return Err(ResultMessage {
                                            code: 993,
                                            message: "".to_string(),
                                        });
                                    };

                                    response_mvpmob = json;
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
                let mut seatbids = vec![];

                let mut bid = vec![];
                let ad = response_mvpmob.ad.unwrap();

                if assets.get_asset_size("video") > 0 {
                    if ad.native.video.is_none() {
                        return Err(ResultMessage {
                            code: 993,
                            message: "".to_string(),
                        });
                    }
                }

                let link_asset = LinkAsset {
                    linktype: {
                        match ad.interaction.interaction_type.as_str() {
                            "H5_LINK" => 1,
                            "DOWNLOAD" => {
                                let mut gdt = false;
                                for click_tracker in &ad.native.click_trackers {
                                    if click_tracker.contains("c.gdt.qq.com") {
                                        gdt = true;
                                        break;
                                    }
                                }
                                if gdt {
                                    2
                                } else {
                                    1
                                }
                            },
                            "DEEPLINK" => 1,
                            "WECHAT_MINI_PROG" => 1,
                            _ => 1,
                        }
                    },
                    universallink: None,
                    storeid: None,
                    deeplink: {
                        match ad.interaction.interaction_type.as_str() {
                            "DEEPLINK" => ad.interaction.deeplink_url.clone(),
                            _ => None,
                        }
                    },
                    quickapplink: None,
                    wechatmppath: None,
                    wechatmpid: {
                        ad.interaction.origin_id.clone()
                    },
                    marketurl: None,
                    downloadurl: {
                        match ad.interaction.interaction_type.as_str() {
                            "DOWNLOAD" => Some(ad.interaction.target_url.clone()),
                            _ => None,
                        }
                    },
                    url: {
                        ad.interaction.target_url.clone()
                    },
                    urlfb: None,
                };

                bid.push(Bid {
                    id: Some(request_id.to_string()),
                    item: request.item[0].id.clone(),
                    price: { // update later
                        match response_mvpmob.imp {
                            Some(imp) => {
                                match imp.price {
                                    Some(price) => price as i32,
                                    None => connection.default_price,
                                }
                            },
                            None => connection.default_price,
                        }
                    },
                    burl: {
                        let mut burl = Vec::<String>::new();

                        let nurl = ad.native.winnotice_tracker.clone();
                        burl.push(replace_macro(&nurl));

                        match ad.native.wurls {
                            Some(wurls) => {
                                for wurl in wurls {
                                    let nurl = wurl.clone();
                                    burl.push(replace_macro(&nurl));
                                }
                            },
                            None => (),
                        }

                        Some(burl)
                    },
                    lurl: {
                        let mut lurl = Vec::<String>::new();

                        match ad.native.lurls {
                            Some(lurls) => {
                                for lnurl in lurls {
                                    let nurl = lnurl.clone();
                                    lurl.push(replace_macro(&nurl));
                                }
                            },
                            None => (),
                        }

                        Some(lurl)
                    },
                    media: Ad {
                        id: ad.adid.clone(),
                        display: Display {
                            w: None,
                            h: None,
                            banner: {
                                if assets.get_banner_size() > 0 {
                                    match &ad.native.image {
                                        Some(image) => {
                                            Some(Banner {
                                                img: image.url.clone(),
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

                                    match &ad.native.text {
                                        Some(text) => {
                                            asset_vec.push(Asset {
                                                id: assets.consume_asset("title"),
                                                req: 1,
                                                title: Some(TitleAsset {
                                                    text: text.title.clone(),
                                                    subtitle: None,
                                                    desc: Some(text.desc.clone()),
                                                    len: None,
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
                                    match &ad.native.image {
                                        Some(image) => {
                                            if assets.get_asset_size("img") > 0 {
                                                asset_vec.push(Asset {
                                                    id: assets.consume_asset("img"),
                                                    req: 1,
                                                    img: {
                                                        Some(ImageAsset {
                                                            url: image.url.clone(),
                                                            mime: image.image_mime.clone(),
                                                            w: image.w.clone(),
                                                            h: image.h.clone(),
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
                                            if assets.get_asset_size("thumb") > 0 {
                                                asset_vec.push(Asset {
                                                    id: assets.consume_asset("thumb"),
                                                    req: 1,
                                                    img: {
                                                        Some(ImageAsset {
                                                            url: image.url.clone(),
                                                            mime: image.image_mime.clone(),
                                                            w: image.w.clone(),
                                                            h: image.h.clone(),
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
                                        },
                                        None => (),
                                    }
                                    match &ad.native.video {
                                        Some(video) => {
                                            asset_vec.push(Asset {
                                                id: assets.consume_asset("video"),
                                                req: 1,
                                                title: None,
                                                img: None,
                                                video: Some(VideoAsset {
                                                    url: video.url.clone(),
                                                    mime: video.video_mime.clone(),
                                                    w: video.w.clone(),
                                                    h: video.h.clone(),
                                                    dur: Some(video.duration),
                                                    skipoffset: None,
                                                    size: None,
                                                    delivery: None,
                                                    orientation: None,
                                                    autolanding: 0,
                                                    clickable: 0,
                                                }),
                                                data: None,
                                                html: None,
                                                app: None,
                                            });

                                            match &video.cover_url {
                                                Some(cover_url) => {
                                                    asset_vec.push(Asset {
                                                        id: assets.consume_asset("video#cover"),
                                                        req: 1,
                                                        img: {
                                                            Some(ImageAsset {
                                                                url: cover_url.clone(),
                                                                mime: video.cover_mime.clone(),
                                                                w: None,
                                                                h: None,
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
                                        },
                                        None => (),
                                    }
                                    match &ad.native.icon {
                                        Some(icon) => {
                                            if assets.get_asset_size("icon") > 0 {
                                                match &icon.url {
                                                    Some(url) => {
                                                        asset_vec.push(Asset {
                                                            id: assets.consume_asset("icon"),
                                                            req: 1,
                                                            img: {
                                                                Some(ImageAsset {
                                                                    url: url.clone(),
                                                                    mime: None,
                                                                    w: Some(icon.width),
                                                                    h: Some(icon.height),
                                                                    imagetype: Some(1),
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
                                        }
                                        None => (),
                                    }

                                    match &ad.interaction.app {
                                        Some(app) => {
                                            asset_vec.push(Asset {
                                                id: assets.consume_asset("app"),
                                                req: 0,
                                                app: Some(AppAsset {
                                                    name: {
                                                        match &app.app_name {
                                                            Some(app_name) => app_name.clone(),
                                                            None => "".to_string(),
                                                        }
                                                    },
                                                    desc: None,
                                                    descurl: None,
                                                    domain: app.domain.clone(),
                                                    bundle: app.bundle_id.clone(),
                                                    ver: app.version.clone(),
                                                    developer: app.publisher_name.clone(),
                                                    icon: None,
                                                    storeid: None,
                                                    storeurl: app.store_url.clone(),
                                                    paid: 0,
                                                    size: None,
                                                    md5: None,
                                                    registration: None,
                                                    privacy: None,
                                                    privacyurl: app.privacy_link.clone(),
                                                    permission: None,
                                                    permissionurl: app.permission_link.clone(),
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

                                for impression_tracker in &ad.native.impression_trackers {
                                    event_vec.push(Event {
                                        eventtype: 501,
                                        method: 1,
                                        url: replace_macro(impression_tracker),
                                        header: None,
                                        content: None,
                                    });
                                }
                                for click_tracker in &ad.native.click_trackers {
                                    event_vec.push(Event {
                                        eventtype: 502,
                                        method: 1,
                                        url: replace_macro(click_tracker),
                                        header: None,
                                        content: None,
                                    });
                                }
                                match &ad.native.conversion_trackers {
                                    Some(conversion_trackers) => {
                                        for conversion_tracker in conversion_trackers {
                                            for tracker_url in &conversion_tracker.tracker_urls {
                                                event_vec.push(Event {
                                                    eventtype: {
                                                        match conversion_tracker.event_type.as_str() {
                                                            "VIDEO_START" => 701,
                                                            "VIDEO_FIRST_QUARTILE" => 702,
                                                            "VIDEO_MID_POINT" => 703,
                                                            "VIDEO_THIRD_QUARTILE" => 704,
                                                            "VIDEO_COMPLETE" => 705,
                                                            "VIDEO_AUTO_PLAY" => 5,
                                                            "VIDEO_RESUME" => 708,
                                                            "VIDEO_PAUSE" => 709,
                                                            "DOWNLOAD_START" => 601,
                                                            "DOWNLOAD_FINISH" => 602,
                                                            "INSTALL_FINISH" => 603,
                                                            "APP_INSTALLED" => 604,
                                                            "DEEPLINK_SUCCESS" => 504,
                                                            "DEEPLINK_FALLBACK" => 505,
                                                            "DEEPLINK_ATTEMPT" => 503,
                                                            _ => 501,
                                                        }
                                                    },
                                                    method: 1,
                                                    url: replace_macro(tracker_url),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                        }
                                    },
                                    None => (),
                                }
                                match &ad.native.click_area_report_url {
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
                        advertiser: None,
                        advertisericon: None,
                    },
                });

                seatbids.push(Seatbid {
                    bid,
                });

                Some(seatbids)
            },
        };

        Ok(response)
    }

    async fn bidding_notify_win(url: String, win_price: i32, _next_price: i32, iv: &String, connection: &Connection, pool: &HttpPool) {
        let encrypt_price = Self::encrypt_price(win_price, iv, connection);
        let replaced_url = url
            .replace("__WIN_PRICE__", &encode(encrypt_price.as_str()));

        let client = {
            let pool_mvpmob_lock = pool.pool_mvpmob.clone();
            let pool_mvpmob = pool_mvpmob_lock.read().unwrap();
            pool_mvpmob.clone()
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

    fn encrypt_price(price: i32, _iv: &String, _connection: &Connection) -> String {
        price.to_string()
    }

}

fn replace_macro(orig: &String) -> String {
    let mut replaced = orig.clone();

    replaced = replaced.replace("__ADN_TYPE__", "");
    replaced = replaced.replace("__ADN_NAME__", "__LOSE_ADN_NAME__");
    replaced = replaced.replace("__AD_N__", "");
    replaced = replaced.replace("__AD_TI__", "");
    replaced = replaced.replace("__IS_S__", "");
    replaced = replaced.replace("__IS_C__", "");

    replaced = replaced.replace("__RED_WIDTH__", "__WIDTH__");
    replaced = replaced.replace("__RED_HEIGHT__", "__HEIGHT__");

    replaced = replaced.replace("__DURATION__", "__VIDEO_TIME__");
    replaced = replaced.replace("__BEGIN_TIME__", "__VIDEO_BEGIN_TIME__");
    replaced = replaced.replace("__END_TIME__", "__VIDEO_END_TIME__");
    replaced = replaced.replace("__PLAY_FIRST_FRAME__", "__VIDEO_FIRST_FRAME__");
    replaced = replaced.replace("__PLAY_LAST_FRAME__", "__VIDEO_LAST_FRAME__");
    replaced = replaced.replace("__SCENE__", "__VIDEO_PLAY_SCENE__");
    replaced = replaced.replace("__TYPE__", "__VIDEO_PLAY_TYPE__");
    replaced = replaced.replace("__BEHAIVOR__", "__VIDEO_PLAY_TRIGGER__");
    replaced = replaced.replace("__STATUS__", "__VIDEO_PLAY_STATUS__");

    replaced
}
