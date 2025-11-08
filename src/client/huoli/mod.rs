use std::time::Duration;

use urlencoding::encode;

use crate::{protocol::*, Assets, Cache, Client, Connection, HttpPool, Identifiers, Price, ResultMessage};

pub mod material;
pub mod ad;
pub mod app;
pub mod caid;
pub mod device;
pub mod geo;
pub mod imp;
pub mod request;
pub mod response;
pub mod seat;
pub mod user;

pub use material::HuoliMaterial;
pub use ad::HuoliAd;
pub use app::HuoliApp;
pub use caid::HuoliCaid;
pub use device::HuoliDevice;
pub use geo::HuoliGeo;
pub use imp::HuoliImp;
pub use request::HuoliRequest;
pub use response::HuoliResponse;
pub use seat::HuoliSeat;
pub use user::HuoliUser;

pub struct Huoli {

}

impl Client for Huoli {

    async fn request(request: &Request, connection: &Connection, pool: &HttpPool, cache: &Cache) -> Result<Response, ResultMessage> {
        let pid = connection.client_tag_id.split("|").nth(0).unwrap();
        let appid = connection.client_tag_id.split("|").nth(1).unwrap();

        let request_id = cache.get_sequence();

        let mut assets = Assets::new(request);
        let identifiers = Identifiers::new(request);

        let request_huoli = HuoliRequest {
            id: {
                request_id.to_string()
            },
            test: {
                Some(connection.test)
            },
            imp: [HuoliImp {
                id: {
                    0
                },
                pid: {
                    pid.to_string()
                },
                width: {
                    match request.item[0].spec.display.w {
                        Some(w) => w,
                        None => 0,
                    }
                },
                height: {
                    match request.item[0].spec.display.h {
                        Some(h) => h,
                        None => 0,
                    }
                },
                slot_num: {
                    Some(1)
                },
                dealid: {
                    None
                },
                bid_floor: {
                    Some(Price::to_client(connection, request.item[0].flr.map(f64::from)) as i32)
                },
            }].to_vec(),
            app: {
                match &request.context.app {
                    Some(app) => HuoliApp {
                        app_id: {
                            appid.to_string()
                        },
                        name: {
                            match &connection.client_media_appname {
                                Some(client_media_appname) => client_media_appname.clone(),
                                None => app.name.clone(),
                            }
                        },
                        bundle: {
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
                        keywords: {
                            None
                        },
                        version: {
                            app.ver.clone()
                        },
                    },
                    None => HuoliApp {
                        app_id: {
                            appid.to_string()
                        },
                        name: {
                            match &connection.client_media_appname {
                                Some(client_media_appname) => client_media_appname.clone(),
                                None => "".to_string(),
                            }
                        },
                        bundle: {
                            match &connection.client_media_apppackage {
                                Some(client_media_apppackage) => client_media_apppackage.clone(),
                                None => "".to_string(),
                            }
                        },
                        keywords: {
                            None
                        },
                        version: {
                            None
                        },
                    },
                }
            },
            device: {
                HuoliDevice {
                    ip: {
                        match &request.context.device.ip {
                            Some(ip) => ip.clone(),
                            None => "".to_string(),
                        }
                    },
                    ipv6: {
                        request.context.device.ipv6.clone()
                    },
                    geo: {
                        match &request.context.device.geo {
                            Some(geo) => Some(HuoliGeo {
                                lat: {
                                    match geo.lat {
                                        Some(lat) => {
                                            if lat > 90.0 || lat < -90.0 {
                                                0.0
                                            } else {
                                                lat
                                            }
                                        },
                                        None => 0.0,
                                    }
                                },
                                lon: {
                                    match geo.lon {
                                        Some(lon) => {
                                            if lon > 180.0 || lon < -180.0 {
                                                0.0
                                            } else {
                                                lon
                                            }
                                        },
                                        None => 0.0,
                                    }
                                },
                            }),
                            None => None,
                        }
                    },
                    user_agent: {
                        request.context.device.ua.clone()
                    },
                    device_type: {
                        match request.context.device.devicetype {
                            Some(2) => Some(3),
                            Some(3) => Some(4),
                            Some(4) => Some(1),
                            Some(5) => Some(2),
                            _ => Some(0),
                        }
                    },
                    make: {
                        match &request.context.device.make {
                            Some(make) => make.clone(),
                            None => "".to_string(),
                        }
                    },
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
                    os: {
                        match &request.context.device.os {
                            Some(2) => "Android".to_string(),
                            Some(13) => "iOS".to_string(),
                            Some(501) => "Hmos".to_string(),
                            _ => "Others".to_string(),
                        }
                    },
                    osv: {
                        match &request.context.device.osv {
                            Some(osv) => osv.clone(),
                            None => "".to_string(),
                        }
                    },
                    operator: {
                        match &request.context.device.carrier {
                            Some(carrier) => {
                                match carrier.as_str() {
                                    "cmcc" => Some(1),
                                    "unicom" => Some(3),
                                    "telecom" => Some(2),
                                    _ => Some(0),
                                }
                            },
                            None => Some(0),
                        }
                    },
                    network: {
                        match &request.context.device.contype {
                            Some(2) => 1,
                            Some(4) => 2,
                            Some(5) => 3,
                            Some(6) => 4,
                            Some(7) => 5,
                            _ => 0,
                        }
                    },
                    idfa: {
                        match identifiers.get_id(507, 0) {
                            Some(uid) => Some(uid.id.clone()),
                            None => None,
                        }
                    },
                    idfamd5: {
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
                    oaid: {
                        match identifiers.get_id(505, 0) {
                            Some(uid) => Some(uid.id.clone()),
                            None => None,
                        }
                    },
                    oaidmd5: {
                        match identifiers.get_id(506, 0) {
                            Some(uid) => Some(uid.id.clone()),
                            None => None,
                        }
                    },
                    aidplain: {
                        match identifiers.get_id(509, 0) {
                            Some(uid) => Some(uid.id.clone()),
                            None => None,
                        }
                    },
                    aidmd5: {
                        match identifiers.get_id(510, 0) {
                            Some(uid) => Some(uid.id.clone()),
                            None => None,
                        }
                    },
                    orientation: {
                        match &request.context.device.orientation {
                            Some(501) => Some(1),
                            Some(502) => Some(3),
                            _ => Some(0),
                        }
                    },
                    width: {
                        match request.context.device.w {
                            Some(w) => w,
                            None => 320,
                        }
                    },
                    height: {
                        match request.context.device.h {
                            Some(h) => h,
                            None => 480,
                        }
                    },
                    pixel_ratio: {
                        request.context.device.pxratio.clone()
                    },
                    ppi: {
                        request.context.device.ppi.clone()
                    },
                    mac: {
                        match identifiers.get_id(511, 0) {
                            Some(uid) => Some(uid.id.clone()),
                            None => None,
                        }
                    },
                    hmsv: {
                        request.context.device.hmsv.clone()
                    },
                    mosn: {
                        request.context.device.romname.clone()
                    },
                    mosv: {
                        request.context.device.romv.clone()
                    },
                    mappv: {
                        request.context.device.storev.clone()
                    },
                    boot_mark: {
                        request.context.device.bootmark.clone()
                    },
                    update_mark: {
                        request.context.device.updatemark.clone()
                    },
                    birth_time: {
                        request.context.device.inittime.clone()
                    },
                    boot_time: {
                        request.context.device.boottime.clone()
                    },
                    update_time: {
                        request.context.device.updatetime.clone()
                    },
                    system_mem: {
                        request.context.device.sysmemory.clone()
                    },
                    system_disk: {
                        request.context.device.sysdisksize.clone()
                    },
                    country_code: {
                        request.context.device.country.clone()
                    },
                    language: {
                        request.context.device.lang.clone()
                    },
                    phone_name: {
                        match identifiers.get_id(517, 0) {
                            Some(uid) => Some(uid.id.clone()),
                            None => None,
                        }
                    },
                    installs: {
                        match &request.context.device.app {
                            Some(app) => Some(app.split(",").map(|s| s.to_string()).collect()),
                            None => None,
                        }
                    },
                    caids: {
                        let mut caids = vec![];

                        match identifiers.get_id(513, 0) {
                            Some(uid) => {
                                caids.push(HuoliCaid {
                                    id: {
                                        uid.id.clone()
                                    },
                                    version: {
                                        match &uid.ver {
                                            Some(ver) => ver.clone(),
                                            None => "".to_string(),
                                        }
                                    },
                                    vendor: {
                                        match &uid.vendor {
                                            Some(vendor) => {
                                                match vendor.parse() {
                                                    Ok(vendor) => Some(vendor),
                                                    Err(_) => Some(0),
                                                }
                                            },
                                            None => Some(0),
                                        }
                                    },
                                    time: {
                                        uid.time.clone()
                                    },
                                });
                            },
                            None => (),
                        };

                        match identifiers.get_id(513, 1) {
                            Some(uid) => {
                                caids.push(HuoliCaid {
                                    id: {
                                        uid.id.clone()
                                    },
                                    version: {
                                        match &uid.ver {
                                            Some(ver) => ver.clone(),
                                            None => "".to_string(),
                                        }
                                    },
                                    vendor: {
                                        match &uid.vendor {
                                            Some(vendor) => {
                                                match vendor.parse() {
                                                    Ok(vendor) => Some(vendor),
                                                    Err(_) => Some(0),
                                                }
                                            },
                                            None => Some(0),
                                        }
                                    },
                                    time: {
                                        uid.time.clone()
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
                    aaid: {
                        match identifiers.get_id(514, 0) {
                            Some(uid) => Some(uid.id.clone()),
                            None => None,
                        }
                    },
                    openudid: {
                        None
                    },
                }
            },
            user: {
                Some(HuoliUser {
                    yob: {
                        match &request.context.user.yob {
                            Some(yob) => Some(yob.to_string()),
                            None => None,
                        }
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
                })
            },
            time: {
                Some(connection.timeout as i32)
            },
        };

        let response_huoli: HuoliResponse;

        let client = {
            let pool_huoli_lock = pool.pool_huoli.clone();
            let pool_huoli = pool_huoli_lock.read().unwrap();
            pool_huoli.clone()
        };
        let response_huoli_raw = client.post(format!("{}", "https://api.huoli.com/ad/xy/").as_str())
            .json(&request_huoli)
            .header("Accept-Encoding", "gzip")
            .header("Connection", "keep-alive")
            .header("Content-Type", "application/json")
            .timeout(Duration::from_millis(connection.timeout))
            .send().await;

        match response_huoli_raw {
            Ok(response_huoli_raw) => {
                let status = response_huoli_raw.status();
                if status != 200 {
                    return Err(ResultMessage {
                        code: 992,
                        message: {
                            match response_huoli_raw.text().await {
                                Ok(text) => format!("upstream error {}: {}", status, text),
                                Err(_) => format!("upstream error {}", status),
                            }
                        },
                    });
                } else {
                    match response_huoli_raw.text().await {
                        Ok(text) => {
                            match serde_json::from_str::<HuoliResponse>(&text) {
                                Ok(json) => {
                                    match json.code {
                                        0 => {
                                            match &json.seat {
                                                Some(seat) => {
                                                    if seat.len() == 0 {
                                                        return Err(ResultMessage {
                                                            code: 993,
                                                            message: "upstream error: no data".to_string(),
                                                        });
                                                    }
                                                    match &seat[0].ad {
                                                        Some(ad) => {
                                                            if ad.len() == 0 {
                                                                return Err(ResultMessage {
                                                                    code: 993,
                                                                    message: "upstream error: no data".to_string(),
                                                                });
                                                            }
                                                            response_huoli = json;
                                                        },
                                                        None => {
                                                            return Err(ResultMessage {
                                                                code: 993,
                                                                message: "upstream error: no data".to_string(),
                                                            });
                                                        }
                                                    }
                                                },
                                                None => {
                                                    return Err(ResultMessage {
                                                        code: 993,
                                                        message: "upstream error: no data".to_string(),
                                                    });
                                                },
                                            }
                                        },
                                        204 => {
                                            return Err(ResultMessage {
                                                code: 993,
                                                message: "".to_string(),
                                            });
                                        },
                                        _ => {
                                            return Err(ResultMessage {
                                                code: 994,
                                                message: format!("upstream error: {}", json.code),
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
                let mut seats = vec![];

                for seat in &response_huoli.seat.unwrap() {
                    let mut bids = vec![];

                    for ad_huoli in &seat.clone().ad.unwrap() {
                        let link_asset = LinkAsset {
                            linktype: {
                                match &ad_huoli.landing_type {
                                    2 => 2,
                                    _ => 1,
                                }
                            },
                            universallink: {
                                ad_huoli.ulk.clone()
                            },
                            storeid: None,
                            deeplink: {
                                match &ad_huoli.deeplink_url {
                                    Some(deeplink_url) => Some(replace_macro(deeplink_url)),
                                    None => None,
                                }
                            },
                            quickapplink: None,
                            wechatmppath: ad_huoli.wxopath.clone(),
                            wechatmpid: ad_huoli.wxoid.clone(),
                            marketurl: None,
                            downloadurl: None,
                            url: replace_macro(&ad_huoli.click_url),
                            urlfb: None,
                        };

                        let bid = Bid {
                            id: Some(request_id.to_string()),
                            item: request.item[0].id.clone(),
                            price: { // update later
                                match ad_huoli.price {
                                    Some(price) => price as i32,
                                    None => connection.default_price,
                                }
                            },
                            burl: {
                                let mut burl = Vec::<String>::new();
                                match &ad_huoli.nurl {
                                    Some(nurl) => {
                                        for nurl in nurl {
                                            burl.push(replace_macro(&nurl));
                                        }
                                    },
                                    None => (),
                                }
                                Some(burl)
                            },
                            lurl: {
                                let mut lurl = Vec::<String>::new();
                                match &ad_huoli.fnurl {
                                    Some(fnurl) => {
                                        for nurl in fnurl {
                                            lurl.push(replace_macro(&nurl));
                                        }
                                    },
                                    None => (),
                                }
                                Some(lurl)
                            },
                            media: Ad {
                                id: ad_huoli.id.to_string(),
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
                                        match &ad_huoli.images {
                                            Some(images) => {
                                                if images.len() > 0 {
                                                    display.w = Some(images[0].w);
                                                    display.h = Some(images[0].h);
                                                    display.banner = Some(Banner {
                                                        img: images[0].url.clone(),
                                                        link: Some(link_asset.clone()),
                                                    });
                                                }
                                            },
                                            None => (),
                                        }
                                    }
                                    if assets.get_asset_total_size() > 0 {
                                        match &ad_huoli.title {
                                            Some(title) => {
                                                asset_vec.push(Asset {
                                                    id: assets.consume_asset("title"),
                                                    req: 1,
                                                    title: Some(TitleAsset {
                                                        text: title.clone(),
                                                        subtitle: None,
                                                        desc: ad_huoli.description.clone(),
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
                                        match &ad_huoli.images {
                                            Some(images) => {
                                                if assets.get_asset_size("img") > 0 {
                                                    for image in images {
                                                        asset_vec.push(Asset {
                                                            id: assets.consume_asset("img"),
                                                            req: 1,
                                                            title: None,
                                                            img: Some(ImageAsset {
                                                                url: image.url.clone(),
                                                                mime: None,
                                                                w: Some(image.w),
                                                                h: Some(image.h),
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
                                                    for image in images {
                                                        asset_vec.push(Asset {
                                                            id: assets.consume_asset("thumb"),
                                                            req: 1,
                                                            title: None,
                                                            img: Some(ImageAsset {
                                                                url: image.url.clone(),
                                                                mime: None,
                                                                w: Some(image.w),
                                                                h: Some(image.h),
                                                                imagetype: Some(501),
                                                            }),
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
                                        match &ad_huoli.video {
                                            Some(video) => {
                                                asset_vec.push(Asset {
                                                    id: assets.consume_asset("video"),
                                                    req: 1,
                                                    title: None,
                                                    img: None,
                                                    video: Some(VideoAsset {
                                                        url: video.url.clone(),
                                                        mime: None,
                                                        w: Some(video.w),
                                                        h: Some(video.h),
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

                                                match &video.cover_url {
                                                    Some(cover_url) => {
                                                        asset_vec.push(Asset {
                                                            id: assets.consume_asset("video#cover"),
                                                            req: 1,
                                                            title: None,
                                                            img: Some(ImageAsset {
                                                                url: cover_url.clone(),
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
                                            },
                                            None => (),
                                        }
                                    }

                                    match &ad_huoli.app_name {
                                        Some(app_name) => {
                                            let asset = Asset {
                                                id: assets.consume_asset("app"),
                                                req: 0,
                                                title: None,
                                                img: None,
                                                video: None,
                                                data: None,
                                                html: None,
                                                app: Some(AppAsset {
                                                    name: app_name.clone(),
                                                    desc: ad_huoli.app_descriptioin.clone(),
                                                    descurl: ad_huoli.app_descriptioin_url.clone(),
                                                    domain: None,
                                                    bundle: ad_huoli.app_package.clone(),
                                                    ver: ad_huoli.app_version.clone(),
                                                    developer: ad_huoli.app_developer.clone(),
                                                    icon: ad_huoli.app_icon.clone(),
                                                    storeid: None,
                                                    storeurl: None,
                                                    paid: 0,
                                                    size: ad_huoli.app_size.clone(),
                                                    md5: None,
                                                    registration: None,
                                                    privacy: None,
                                                    privacyurl: ad_huoli.app_privacy_url.clone(),
                                                    permission: None,
                                                    permissionurl: ad_huoli.app_permission_url.clone(),
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

                                    match &ad_huoli.expose_tracking_url {
                                        Some(expose_tracking_url) => {
                                            for event in expose_tracking_url {
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
                                    match &ad_huoli.click_tracking_url {
                                        Some(click_tracking_url) => {
                                            for event in click_tracking_url {
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
                                    match &ad_huoli.dp_try_url {
                                        Some(dp_try_url) => {
                                            for event in dp_try_url {
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
                                    match &ad_huoli.dp_succ_url {
                                        Some(dp_succ_url) => {
                                            for event in dp_succ_url {
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
                                    match &ad_huoli.dp_fail_url {
                                        Some(dp_fail_url) => {
                                            for event in dp_fail_url {
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
                                    match &ad_huoli.download_start {
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
                                    match &ad_huoli.download_end {
                                        Some(download_end) => {
                                            for event in download_end {
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
                                    match &ad_huoli.install_start {
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
                                    match &ad_huoli.install_end {
                                        Some(install_end) => {
                                            for event in install_end {
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
                                    match &ad_huoli.activate_app {
                                        Some(activate_app) => {
                                            for event in activate_app {
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

                                    display.event = event_vec;

                                    display
                                },
                                advertiser: None,
                                advertisericon: None,
                            },
                        };

                        bids.push(bid);
                    }

                    seats.push(Seatbid {
                        bid: bids,
                    });
                }

                Some(seats)
            }
        };

        Ok(response)
    }

    async fn bidding_notify_win(url: String, win_price: i32, _next_price: i32, iv: &String, connection: &Connection, pool: &HttpPool) {
        let encrypt_price = Self::encrypt_price(win_price, iv, connection);
        let replaced_url = url
            .replace("__WIN_PRICE__", &encode(encrypt_price.as_str()));

        let client = {
            let pool_huoli_lock = pool.pool_huoli.clone();
            let pool_huoli = pool_huoli_lock.read().unwrap();
            pool_huoli.clone()
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

    replaced = replaced.replace("__TS__", "__TS_S__");
    replaced = replaced.replace("__TMS__", "__TS__");
    replaced = replaced.replace("__P_W__", "__WIDTH__");
    replaced = replaced.replace("__P_H__", "__HEIGHT__");
    replaced = replaced.replace("__CLICK_DOWN_X__", "__DOWN_X__");
    replaced = replaced.replace("__CLICK_DOWN_Y__", "__DOWN_Y__");
    replaced = replaced.replace("__CLICK_UP_X__", "__UP_X__");
    replaced = replaced.replace("__CLICK_UP_Y__", "__UP_Y__");

    replaced
}
