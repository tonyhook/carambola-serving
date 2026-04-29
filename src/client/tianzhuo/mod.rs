use std::{collections::HashMap, time::Duration};

use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyInit};
use base64::prelude::*;
use reqwest::Url;
use urlencoding::encode;

use crate::{protocol::*, Assets, Cache, Client, Connection, HttpPool, Identifiers, Price, ResultMessage};

pub mod advertiser;
pub mod app;
pub mod attach_detail;
pub mod banner;
pub mod bid;
pub mod bid_app;
pub mod check_video_urls;
pub mod content;
pub mod data;
pub mod deal;
pub mod device;
pub mod ext;
pub mod geo;
pub mod icon;
pub mod image;
pub mod imp;
pub mod logo;
pub mod macros;
pub mod native_request;
pub mod native_response;
pub mod pmp;
pub mod producer;
pub mod publisher;
pub mod request;
pub mod response;
pub mod seat;
pub mod segment;
pub mod site;
pub mod user;
pub mod video;
pub mod video_request;

pub use advertiser::TianzhuoAdvertiser;
pub use app::TianzhuoApp;
pub use attach_detail::TianzhuoAttachDetail;
pub use banner::TianzhuoBanner;
pub use bid::TianzhuoBid;
pub use bid_app::TianzhuoBidApp;
pub use check_video_urls::TianzhuoCheckVideoUrls;
pub use content::TianzhuoContent;
pub use data::TianzhuoData;
pub use deal::TianzhuoDeal;
pub use device::TianzhuoDevice;
pub use ext::TianzhuoExt;
pub use geo::TianzhuoGeo;
pub use icon::TianzhuoIcon;
pub use image::TianzhuoImage;
pub use imp::TianzhuoImp;
pub use logo::TianzhuoLogo;
pub use macros::TianzhuoMacros;
pub use native_request::TianzhuoNativeRequest;
pub use native_response::TianzhuoNativeResponse;
pub use pmp::TianzhuoPmp;
pub use producer::TianzhuoProducer;
pub use publisher::TianzhuoPublisher;
pub use request::TianzhuoRequest;
pub use response::TianzhuoResponse;
pub use seat::TianzhuoSeat;
pub use segment::TianzhuoSegment;
pub use site::TianzhuoSite;
pub use user::TianzhuoUser;
pub use video::TianzhuoVideo;
pub use video_request::TianzhuoVideoRequest;

type Aes128EcbEnc = ecb::Encryptor<aes::Aes128>;

pub struct Tianzhuo {
}

impl Client for Tianzhuo {
    async fn request(request: &Request, connection: &Connection, pool: &HttpPool, cache: &Cache) -> Result<Response, ResultMessage> {
        let tag_id = connection.client_tag_id.split("|").nth(0).unwrap();
        let app_id = connection.client_tag_id.split("|").nth(1).unwrap();

        let request_id = cache.get_sequence();
        let mut assets = Assets::new(request);
        let identifiers = Identifiers::new(request);

        let request_tianzhuo = TianzhuoRequest {
            id: {
                request_id.to_string()
            },
            imp: {
                vec![TianzhuoImp {
                    id: {
                        request_id.to_string()
                    },
                    tagid: {
                        tag_id.to_string()
                    },
                    allowstyle: {
                        None
                    },
                    ad_slot_type: {
                        if assets.get_banner_size() > 0 {
                            if request.item[0].spec.display.instl > 0 {
                                "6".to_string()
                            } else if request.item[0].spec.display.pos == Some(7) {
                                "3".to_string()
                            } else {
                                "2".to_string()
                            }
                        } else {
                            if assets.get_asset_size("video") > 0 {
                                "4".to_string()
                            } else {
                                "1".to_string()
                            }
                        }
                    },
                    banner: {
                        if assets.get_banner_size() > 0 {
                            Some(TianzhuoBanner {
                                w: {
                                    assets.get_banner().unwrap().w.clone()
                                },
                                h: {
                                    assets.get_banner().unwrap().h.clone()
                                },
                                btype: {
                                    None
                                },
                                battr: {
                                    None
                                },
                                api: {
                                    None
                                },
                                id: {
                                    None
                                },
                                pos: {
                                    None
                                },
                                mimes: {
                                    None
                                },
                                wmax: {
                                    None
                                },
                                hmax: {
                                    None
                                },
                                wmin: {
                                    None
                                },
                                hmin: {
                                    None
                                },
                            })
                        } else {
                            None
                        }
                    },
                    video: {
                        if assets.get_asset_size("video") > 0 {
                            let video = assets.get_current_asset("video").unwrap().video.clone().unwrap();
                            Some(TianzhuoVideoRequest {
                                mimes: {
                                    video.mime
                                },
                                minduration: {
                                    video.mindur
                                },
                                maxduration: {
                                    video.maxdur
                                },
                                w: {
                                    video.w
                                },
                                h: {
                                    video.h
                                },
                                startdelay: {
                                    Some(video.skipmin)
                                },
                                protocols: {
                                    None
                                },
                                battr: {
                                    None
                                },
                                minbitrate: {
                                    video.minbitr
                                },
                                maxbitrate: {
                                    video.maxbitr
                                },
                                pos: {
                                    request.item[0].spec.display.pos
                                },
                                linearity: {
                                    None
                                },
                            })
                        } else {
                            None
                        }
                    },
                    native: {
                        if assets.get_asset_total_size() > 0 && assets.get_asset_size("video") == 0 {
                            Some(TianzhuoNativeRequest {
                                w: {
                                    match request.item[0].spec.display.w {
                                        Some(w) => Some(w),
                                        None => None,
                                    }
                                },
                                h: {
                                    match request.item[0].spec.display.h {
                                        Some(h) => Some(h),
                                        None => None,
                                    }
                                },
                                request: {
                                    None
                                },
                                api: {
                                    None
                                },
                                battr: {
                                    None
                                },
                                image_nums: {
                                    Some(3)
                                },
                                ver: {
                                    None
                                },
                                native_field: {
                                    None
                                },
                                title_max: {
                                    None
                                },
                                desc_max: {
                                    None
                                },
                                iw: {
                                    None
                                },
                                ih: {
                                    None
                                },
                            })
                        } else {
                            None
                        }
                    },
                    bidfloor: {
                        Some(Price::to_client(connection, request.item[0].flr.map(f64::from)))
                    },
                    bidfloorcur: {
                        None
                    },
                    pmp: {
                        None
                    },
                    refresh_time: {
                        None
                    },
                    ad_type: {
                        None
                    },
                    req_num: {
                        Some(1)
                    },
                }]
            },
            app: {
                match &request.context.app {
                    Some(app) => Some(TianzhuoApp {
                        id: {
                            app_id.to_string()
                        },
                        bundle: {
                            match &connection.client_media_apppackage {
                                Some(client_media_apppackage) => Some(client_media_apppackage.clone()),
                                None => app.bundle.clone(),
                            }
                        },
                        ver: {
                            app.ver.clone()
                        },
                        cat: {
                            None
                        },
                        sectioncat: {
                            None
                        },
                        pagecat: {
                            None
                        },
                        name: {
                            match &connection.client_media_appname {
                                Some(client_media_appname) => Some(client_media_appname.clone()),
                                None => Some(app.name.clone()),
                            }
                        },
                        domain: {
                            app.domain.clone()
                        },
                        storeurl: {
                            app.storeurl.clone()
                        },
                        privacypolicy: {
                            Some(0)
                        },
                        paid: {
                            Some(app.paid)
                        },
                        publisher: {
                            None
                        },
                        content: {
                            None
                        },
                        keywords: {
                            None
                        },
                        applist: {
                            None
                        },
                        hc_applist: {
                            None
                        },
                    }),
                    None => None,
                }

            },
            site: {
                match &request.context.site {
                    Some(site) => Some(TianzhuoSite {
                        id: {
                            None
                        },
                        name: {
                            Some(site.name.clone())
                        },
                        domain: {
                            site.domain.clone().map(|domain| vec![domain])
                        },
                        sectioncat: {
                            None
                        },
                        pagecat: {
                            None
                        },
                        page: {
                            site.page.clone()
                        },
                        referrer: {
                            site.referrer.clone()
                        },
                        search: {
                            None
                        },
                        mobile: {
                            site.mobile
                        },
                        privacypolicy: {
                            None
                        },
                        publisher: {
                            None
                        },
                        content: {
                            None
                        },
                        keywords: {
                            None
                        },
                    }),
                    None => None,
                }
            },
            device: {
                let device = &request.context.device;

                TianzhuoDevice {
                    ua: {
                        device.ua.clone()
                    },
                    geo: {
                        match &device.geo {
                            Some(geo) => Some(TianzhuoGeo {
                                lat: {
                                    geo.lat
                                },
                                lon: {
                                    geo.lon
                                },
                                llt: {
                                    Some("0".to_string())
                                },
                                llp: {
                                    match geo.geotype {
                                        Some(1) => Some("g".to_string()),
                                        Some(2) => Some("n".to_string()),
                                        _ => None,
                                    }
                                },
                                wifi: {
                                    Some("".to_string())
                                },
                                geo_type: {
                                    None
                                },
                                utcoffset: {
                                    Some(480)
                                },
                                country: {
                                    device.country.clone()
                                },
                                region: {
                                    None
                                },
                                metro: {
                                    None
                                },
                                city: {
                                    geo.city.clone()
                                },
                                zip: {
                                    None
                                },
                                lalo_type: {
                                    geo.coordinate
                                },
                            }),
                            None => None,
                        }
                    },
                    ip: {
                        device.ip.clone()
                    },
                    ipv6: {
                        device.ipv6.clone()
                    },
                    devicetype: {
                        match device.devicetype {
                            Some(1) => Some("phone".to_string()),
                            Some(2) => Some("pc".to_string()),
                            Some(3) => Some("tv".to_string()),
                            Some(4) => Some("phone".to_string()),
                            Some(5) => Some("ipad".to_string()),
                            _ => Some("phone".to_string()),
                        }
                    },
                    make: {
                        device.make.clone()
                    },
                    model: {
                        device.model.clone()
                    },
                    os: {
                        match device.os {
                            Some(2) => Some("0".to_string()),
                            Some(13) => Some("1".to_string()),
                            _ => Some("0".to_string()),
                        }
                    },
                    osv: {
                        device.osv.clone()
                    },
                    av: {
                        device.oslevel.map(|oslevel| oslevel.to_string())
                    },
                    mccmnc: {
                        device.mccmnc.clone()
                    },
                    geofetch: {
                        None
                    },
                    hwv: {
                        device.hwv.clone()
                    },
                    h: {
                        device.h
                    },
                    w: {
                        device.w
                    },
                    ppi: {
                        device.ppi
                    },
                    pxratio: {
                        device.pxratio
                    },
                    dpr: {
                        device.pxratio
                    },
                    deny: {
                        device.pxratio
                    },
                    js: {
                        Some(1)
                    },
                    carrier: {
                        match device.carrier.as_deref() {
                            Some("cmcc") => "70120",
                            Some("unicom") => "70123",
                            Some("telecom") => "70121",
                            Some("cbn") => "70122",
                            _ => "70124",
                        }.to_string()
                    },
                    connectiontype: {
                        match device.contype {
                            Some(2) => 2,
                            Some(3) => 4,
                            Some(4) => 5,
                            Some(5) => 6,
                            Some(6) => 7,
                            _ => 0,
                        }
                    },
                    gaid: {
                        None
                    },
                    imei: {
                        identifiers.get_id(501, 0).map(|uid| uid.id.clone())
                    },
                    imei_md5: {
                        identifiers.get_id(502, 0).map(|uid| uid.id.clone())
                    },
                    imei_sha1: {
                        None
                    },
                    android_id: {
                        identifiers.get_id(509, 0).map(|uid| uid.id.clone())
                    },
                    android_id_md5: {
                        identifiers.get_id(510, 0).map(|uid| uid.id.clone())
                    },
                    android_id_sha1: {
                        None
                    },
                    oaid: {
                        identifiers.get_id(505, 0).map(|uid| uid.id.clone())
                    },
                    oaid_md5: {
                        identifiers.get_id(506, 0).map(|uid| uid.id.clone())
                    },
                    idfa: {
                        identifiers.get_id(507, 0).map(|uid| uid.id.clone())
                    },
                    idfa_md5: {
                        identifiers.get_id(508, 0).map(|uid| uid.id.clone())
                    },
                    idfa_sha1: {
                        None
                    },
                    idfv: {
                        identifiers.get_id(515, 0).map(|uid| uid.id.clone())
                    },
                    caid: {
                        identifiers.get_id(513, 0).map(|uid| uid.id.clone())
                    },
                    caid_version: {
                        identifiers.get_id(513, 0).and_then(|uid| uid.ver.clone())
                    },
                    caid2: {
                        identifiers.get_id(513, 1).map(|uid| uid.id.clone())
                    },
                    caid_version2: {
                        identifiers.get_id(513, 1).and_then(|uid| uid.ver.clone())
                    },
                    mac: {
                        identifiers.get_id(511, 0).map(|uid| uid.id.clone())
                    },
                    mac_md5: {
                        identifiers.get_id(512, 0).map(|uid| uid.id.clone())
                    },
                    mac_sha1: {
                        None
                    },
                    country: {
                        device.country.clone()
                    },
                    language: {
                        device.lang.clone()
                    },
                    orientation: {
                        device.orientation.map(|orientation| match orientation {
                            501 => 1,
                            502 => 2,
                            _ => 0,
                        })
                    },
                    open_udid: {
                        None
                    },
                    flashver: {
                        None
                    },
                    phone_name: {
                        identifiers.get_id(527, 0).map(|uid| uid.id.clone())
                    },
                    init_time_sec: {
                        device.inittime.clone()
                    },
                    init_time_file_sec: {
                        device.inittime.clone()
                    },
                    boot_time_sec: {
                        request.context.device.boottime.as_ref().map(|boottime| boottime.split(".").next().unwrap_or(boottime.as_str()).to_string())
                    },
                    boot_time_milli_sec: {
                        device.boottime.clone()
                    },
                    os_update_time_sec: {
                        request.context.device.updatetime.as_ref().map(|updatetime| updatetime.split(".").next().unwrap_or(updatetime.as_str()).to_string())
                    },
                    os_update_time_nano_sec: {
                        device.updatetime.clone()
                    },
                    update_mark: {
                        device.updatemark.clone()
                    },
                    boot_mark: {
                        device.bootmark.clone()
                    },
                    disk_size: {
                        device.sysdisksize
                    },
                    memory_size: {
                        device.sysmemory
                    },
                    cpu_number: {
                        device.syscpu
                    },
                    model_code: {
                        device.hwmodel.clone()
                    },
                    time_zone: {
                        device.timezone.clone()
                    },
                    local_name: {
                        device.timezone.clone()
                    },
                    hardware_machine: {
                        device.hwmachine.clone()
                    },
                    appstore_ver: {
                        device.storev.clone()
                    },
                    vercodeofhms: {
                        device.hmsv.clone()
                    },
                    paid: {
                        identifiers.get_id(519, 0).map(|uid| uid.id.clone())
                    },
                }
            },
            media_version: {
                "1.0.7".to_string()
            },
            wseat: {
                None
            },
            user: {
                Some(TianzhuoUser {
                    id: {
                        request.context.user.id.clone()
                    },
                    buyeruid: {
                        None
                    },
                    yob: {
                        request.context.user.yob
                    },
                    gender: {
                        request.context.user.gender.clone()
                    },
                    keywords: {
                        request.context.user.keywords.clone()
                    },
                    geo: {
                        None
                    },
                    age: {
                        None
                    },
                    ip: {
                        request.context.device.ip.clone()
                    },
                    data: {
                        None
                    },
                    app_list: {
                        None
                    },
                })
            },
            content: {
                None
            },
            test: {
                if connection.test { 1 } else { 0 }
            },
            at: {
                None
            },
            is_https: {
                false
            },
        };

        let client = {
            let pool_tianzhuo_lock = pool.pool_tianzhuo.clone();
            let pool_tianzhuo = pool_tianzhuo_lock.read().unwrap();
            pool_tianzhuo.clone()
        };

        let response_tianzhuo_raw = client.post(format!("http://adx-api.tianzhuobj.com/ad-core-master/api/json/tzRequest{}", if connection.test { "?test=1" } else { "" }))
            .json(&request_tianzhuo)
            .header("Accept-Encoding", "gzip")
            .header("Connection", "keep-alive")
            .header("Content-Type", "application/json;charset=UTF-8")
            .header("p-version", "3.0")
            .timeout(Duration::from_millis(connection.timeout))
            .send().await;

        let response_tianzhuo: TianzhuoResponse = match response_tianzhuo_raw {
            Ok(response_tianzhuo_raw) => {
                let status = response_tianzhuo_raw.status();
                if status != 200 {
                    return Err(ResultMessage {
                        code: 992,
                        message: {
                            match response_tianzhuo_raw.text().await {
                                Ok(text) => format!("upstream error {}: {}", status, text),
                                Err(_) => format!("upstream error {}", status),
                            }
                        },
                    });
                }
                match response_tianzhuo_raw.text().await {
                    Ok(text) => match serde_json::from_str::<TianzhuoResponse>(&text) {
                        Ok(json) => json,
                        Err(error) => return Err(ResultMessage {
                            code: 997,
                            message: error.to_string(),
                        }),
                    },
                    Err(error) => return Err(ResultMessage {
                        code: 992,
                        message: error.to_string(),
                    }),
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
            }
        };

        if response_tianzhuo.code == 204 {
            return Err(ResultMessage {
                code: 993,
                message: "".to_string(),
            });
        }
        if response_tianzhuo.code != 200 {
            return Err(ResultMessage {
                code: 994,
                message: format!("upstream error {}: {}", response_tianzhuo.code, response_tianzhuo.msg.unwrap_or_default()),
            });
        }

        let response_data = response_tianzhuo.data.ok_or_else(|| ResultMessage {
            code: 993, // defending
            message: "".to_string(),
        })?;
        let seatbid = response_data.seatbid.as_ref()
            .and_then(|seats| seats.first())
            .and_then(|seat| seat.bid.as_ref())
            .and_then(|bids| bids.first())
            .ok_or_else(|| ResultMessage {
                code: 993,
                message: "".to_string(),
            })?;

        Ok(Response {
            id: {
                request.id.clone()
            },
            nbr: {
                response_data.nbr
            },
            seatbid: {
                Some([Seatbid {
                    bid: {
                        let mut bids = vec![];

                        let link_asset = LinkAsset {
                            linktype: {
                                match seatbid.clicktype.as_deref() {
                                    Some("4") => 2,
                                    Some("3") => 3,
                                    _ => 1,
                                }
                            },
                            universallink: {
                                seatbid.apt_ul.clone()
                            },
                            storeid: {
                                None
                            },
                            deeplink: {
                                seatbid.deeplink_url.clone()
                            },
                            quickapplink: {
                                None
                            },
                            wechatmppath: {
                                seatbid.apt_path.clone()
                            },
                            wechatmpid: {
                                seatbid.apt_org_id.clone().or(seatbid.apt_app_id.clone())
                            },
                            marketurl: {
                                seatbid.market_url.clone()
                            },
                            downloadurl: {
                                seatbid.download_url.clone()
                            },
                            url: {
                                match &seatbid.click_url {
                                    Some(click_url) => click_url.clone(),
                                    None => "".to_string(),
                                }
                            },
                            urlfb: {
                                seatbid.fallback.clone()
                            },
                        };

                        let bid = Bid {
                            id: Some(request_id.to_string()),
                            item: request.item[0].id.clone(),
                            price: { // update later
                                match seatbid.price {
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
                            burl: {
                                match &seatbid.nurl {
                                    Some(nurl) => {
                                        let mut burl = Vec::<String>::new();
                                        for win_url in nurl {
                                            let mut nurl = win_url.clone();
                                            nurl = nurl.replace("__WIN_PRICE__", "__WIN_PRICE__");
                                            nurl = nurl.replace("__LOSS_PR__", "__2ND_PRICE__");
                                            burl.push(replace_macro(&identifiers, &nurl));
                                        }
                                        Some(burl)
                                    },
                                    None => None,
                                }
                            },
                            lurl: {
                                None
                            },
                            media: Ad {
                                id: request.id.clone(),
                                display: Display {
                                    w: None,
                                    h: None,
                                    banner: {
                                        if assets.get_banner_size() > 0 {
                                            match &seatbid.images {
                                                Some(images) => {
                                                    if images.len() > 0 && images[0].url.is_some() {
                                                        Some(Banner {
                                                            img: images[0].url.clone().unwrap(),
                                                            link: Some(link_asset.clone()),
                                                        })
                                                    } else {
                                                        None
                                                    }
                                                },
                                                None => {
                                                    None
                                                },
                                            }
                                        } else {
                                            None
                                        }
                                    },
                                    native: {
                                        if assets.get_asset_total_size() > 0 {
                                            let mut asset_vec = vec![];

                                            match &seatbid.video {
                                                Some(video) => {
                                                    if video.url.is_some() && assets.get_asset_size("video") > 0 {
                                                        asset_vec.push(Asset {
                                                            id: assets.consume_asset("video"),
                                                            req: 1,
                                                            video: Some(VideoAsset {
                                                                url: video.url.clone().unwrap(),
                                                                mime: None,
                                                                w: video.w.clone(),
                                                                h: video.h.clone(),
                                                                dur: video.duration,
                                                                skipoffset: None,
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

                                                        match &video.conver_image {
                                                            Some(conver_image) => {
                                                                if conver_image.url.is_some() && assets.get_asset_size("video#cover") > 0 {
                                                                    asset_vec.push(Asset {
                                                                        id: assets.consume_asset("video#cover"),
                                                                        req: 1,
                                                                        img: Some(ImageAsset {
                                                                            url: conver_image.url.clone().unwrap(),
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
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                    }
                                                },
                                                None => (),
                                            }

                                            match &seatbid.images {
                                                Some(images) => {
                                                    for image in images {
                                                        if image.url.is_some() && assets.get_asset_size("img") > 0 {
                                                            asset_vec.push(Asset {
                                                                id: assets.consume_asset("img"),
                                                                req: 1,
                                                                img: Some(ImageAsset {
                                                                    url: image.url.clone().unwrap(),
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
                                                        }
                                                    }
                                                },
                                                None => (),
                                            }

                                            match &seatbid.app {
                                                Some(app) => {
                                                    asset_vec.push(Asset {
                                                        id: assets.consume_asset("app"),
                                                        req: 0,
                                                        app: Some(AppAsset {
                                                            name: {
                                                                match app.app_name.clone() {
                                                                    Some(name) => name,
                                                                    None => "".to_string(),
                                                                }
                                                            },
                                                            desc: None,
                                                            descurl: None,
                                                            domain: None,
                                                            bundle: app.bundle.clone(),
                                                            ver: None,
                                                            developer: None,
                                                            icon: app.app_icon.clone(),
                                                            storeid: None,
                                                            storeurl: None,
                                                            paid: 0,
                                                            size: app.app_size.map(|size| size as i32),
                                                            md5: None,
                                                            registration: None,
                                                            privacy: None,
                                                            privacyurl: None,
                                                            permission: None,
                                                            permissionurl: None,
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
                                        for (urls, eventtype) in [
                                            (&seatbid.check_views, 501),
                                            (&seatbid.check_clicks, 502),
                                            (&seatbid.deeplink_installed, 503),
                                            (&seatbid.check_success_deeplinks, 504),
                                            (&seatbid.check_fail_deeplinks, 505),
                                            (&seatbid.deeplink_not_installed, 506),
                                            (&seatbid.check_start_downloads, 601),
                                            (&seatbid.check_end_downloads, 602),
                                            (&seatbid.check_start_installs, 603),
                                            (&seatbid.check_end_installs, 604),
                                            (&seatbid.check_activations, 605),
                                            (&seatbid.check_video_start, 701),
                                            (&seatbid.check_video_25, 702),
                                            (&seatbid.check_video_middle, 703),
                                            (&seatbid.check_video_75, 704),
                                            (&seatbid.check_video_end, 705),
                                        ] {
                                            if let Some(urls) = urls {
                                                for url in urls {
                                                    event_vec.push(Event {
                                                        eventtype: eventtype,
                                                        method: 1,
                                                        url: {
                                                            if eventtype == 501 {
                                                                let url = replace_macro(&identifiers, url);
                                                                let price = match seatbid.price {
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
                                                                url.replace("__WIN_PRICE__", &encode(encrypt_price.as_str()))
                                                            } else {
                                                                replace_macro(&identifiers, url)
                                                            }
                                                        },
                                                        header: None,
                                                        content: None,
                                                    });
                                                }
                                            }
                                        }

                                        if let Some(iurl) = &seatbid.iurl {
                                            event_vec.push(Event {
                                                eventtype: 501,
                                                method: 1,
                                                url: {
                                                    replace_macro(&identifiers, iurl)
                                                },
                                                header: None,
                                                content: None,
                                            });
                                        }

                                        match &seatbid.click_area_report_url {
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
                                                                url: replace_macro(&identifiers, click_area_report_url),
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
                                    None
                                },
                                advertisericon: {
                                    None
                                },
                            },
                        };

                        bids.push(bid);

                        bids
                    }
                }].to_vec())
            },
        })
    }

    async fn bidding_notify_win(url: String, win_price: i32, next_price: i32, iv: &String, connection: &Connection, pool: &HttpPool) {
        let encrypt_price = Self::encrypt_price(win_price, iv, connection);
        let replaced_url = url
            .replace("__WIN_PRICE__", &encode(encrypt_price.as_str()))
            .replace("__PRICE__", &encode(encrypt_price.as_str()))
            .replace("__2ND_PRICE__", &next_price.to_string());

        let client = {
            let pool_tianzhuo_lock = pool.pool_tianzhuo.clone();
            let pool_tianzhuo = pool_tianzhuo_lock.read().unwrap();
            pool_tianzhuo.clone()
        };
        let _ = client.get(replaced_url).send().await;
    }

    async fn bidding_notify_lose(url: String, lose_price: i32, lose_reason: i32, lose_adn_name: &String, iv: &String, connection: &Connection, pool: &HttpPool) {
        let encrypt_price = Self::encrypt_price(lose_price, iv, connection);
        let replaced_url = url
            .replace("__LOSE_PRICE__", &encode(encrypt_price.as_str()))
            .replace("__BID_ECPM__", &encode(encrypt_price.as_str()))
            .replace("__BID_FAIL_REASON__", &lose_reason.to_string())
            .replace("__LOSE_ADN_NAME__", lose_adn_name);

        let client = {
            let pool_tianzhuo_lock = pool.pool_tianzhuo.clone();
            let pool_tianzhuo = pool_tianzhuo_lock.read().unwrap();
            pool_tianzhuo.clone()
        };
        let _ = client.get(replaced_url).send().await;
    }

    fn encrypt_price(price: i32, _iv: &String, connection: &Connection) -> String {
        let key = connection.client_tag_id.split("|").nth(2).unwrap_or("");
        let plaintext = price.to_string();
        let pos = plaintext.len();
        let mut buffer = [0u8; 32];
        buffer[..pos].copy_from_slice(plaintext.as_bytes());

        let mut aes_key = [0u8; 16];
        aes_key.copy_from_slice(&key.as_bytes()[..16]);

        let cipher = Aes128EcbEnc::new(aes_key[0..16].into())
            .encrypt_padded_mut::<Pkcs7>(&mut buffer, pos)
            .unwrap();

        BASE64_STANDARD.encode(cipher).replace("+", "-").replace("/", "_").replace("=", "")
    }
}

fn replace_macro(identifiers: &Identifiers, orig: &String) -> String {
    let mut replaced = orig.clone();

    replaced = replaced.replace("__REQ_WIDTH__", "__WIDTH__");
    replaced = replaced.replace("__REQ_HEIGHT__", "__HEIGHT__");

    replaced = replaced.replace("__DOWN_X__", "__R_DOWN_X__");
    replaced = replaced.replace("__DOWN_Y__", "__R_DOWN_Y__");
    replaced = replaced.replace("__UP_X__", "__R_UP_X__");
    replaced = replaced.replace("__UP_Y__", "__R_UP_Y__");
    replaced = replaced.replace("__DISPLAY_LUX__", "__LT_X__");
    replaced = replaced.replace("__DISPLAY_LUY__", "__LT_Y__");
    replaced = replaced.replace("__DISPLAY_RDX__", "__RB_X__");
    replaced = replaced.replace("__DISPLAY_RDY__", "__RB_Y__");
    replaced = replaced.replace("__BUTTON_LUX__", "__BUTTON_LT_X__");
    replaced = replaced.replace("__BUTTON_LUY__", "__BUTTON_LT_Y__");
    replaced = replaced.replace("__BUTTON_RDX__", "__BUTTON_RB_X__");
    replaced = replaced.replace("__BUTTON_RDY__", "__BUTTON_RB_Y__");

    replaced = replaced.replace("__TS__", "__EVENT_TIME_START__");
    replaced = replaced.replace("__TS_S__", "__EVENT_TIME_START_S__");
    replaced = replaced.replace("__END__TS__", "__EVENT_TIME_END__");
    replaced = replaced.replace("__SHOW_TIME__", "__SHOW_DURATION__");

    replaced = replaced.replace("__BEGIN_TIME__", "__VIDEO_BEGIN_TIME__");
    replaced = replaced.replace("__END_TIME__", "__VIDEO_END_TIME__");
    replaced = replaced.replace("__PLAY_FIRST_FRAME__", "__VIDEO_FIRST_FRAME__");
    replaced = replaced.replace("__PLAY_LAST_FRAME__", "__VIDEO_LAST_FRAME__");
    replaced = replaced.replace("__SCENE__", "__VIDEO_PLAY_SCENE__");
    replaced = replaced.replace("__TYPE__", "__VIDEO_PLAY_TYPE__");
    replaced = replaced.replace("__BEHAVIOR__", "__VIDEO_PLAY_TRIGGER__");
    replaced = replaced.replace("__STATUS__", "__VIDEO_PLAY_STATUS__");

    replaced = replaced.replace("__IMEI__", identifiers.get_id(501, 0).map(|uid| uid.id.as_str()).unwrap_or(""));
    replaced = replaced.replace("__IMEI_M__", identifiers.get_id(502, 0).map(|uid| uid.id.as_str()).unwrap_or(""));
    replaced = replaced.replace("__OAID__", identifiers.get_id(505, 0).map(|uid| uid.id.as_str()).unwrap_or(""));
    replaced = replaced.replace("__ OAID __", identifiers.get_id(505, 0).map(|uid| uid.id.as_str()).unwrap_or(""));
    replaced = replaced.replace("__ANDROIDID__", identifiers.get_id(509, 0).map(|uid| uid.id.as_str()).unwrap_or(""));
    replaced = replaced.replace("__ ANDROIDID __", identifiers.get_id(509, 0).map(|uid| uid.id.as_str()).unwrap_or(""));
    replaced = replaced.replace("__ANDROIDID_M__", identifiers.get_id(510, 0).map(|uid| uid.id.as_str()).unwrap_or(""));
    replaced = replaced.replace("__ ANDROIDID_M __", identifiers.get_id(510, 0).map(|uid| uid.id.as_str()).unwrap_or(""));
    replaced = replaced.replace("__IDFA__", identifiers.get_id(507, 0).map(|uid| uid.id.as_str()).unwrap_or(""));
    replaced = replaced.replace("__ IDFA __", identifiers.get_id(507, 0).map(|uid| uid.id.as_str()).unwrap_or(""));
    replaced = replaced.replace("__MAC__", identifiers.get_id(511, 0).map(|uid| uid.id.as_str()).unwrap_or(""));
    replaced = replaced.replace("__ MAC __", identifiers.get_id(511, 0).map(|uid| uid.id.as_str()).unwrap_or(""));

    replace_macro_device(identifiers, &replaced)
}

fn replace_macro_device(identifiers: &Identifiers, orig: &String) -> String {
    let mut replaced = orig.clone();

    replaced = replaced.replace("__IMEI__", identifiers.get_id(501, 0).map(|uid| uid.id.as_str()).unwrap_or(""));
    replaced = replaced.replace("__IMEI_M__", identifiers.get_id(502, 0).map(|uid| uid.id.as_str()).unwrap_or(""));
    replaced = replaced.replace("__OAID__", identifiers.get_id(505, 0).map(|uid| uid.id.as_str()).unwrap_or(""));
    replaced = replaced.replace("__ OAID __", identifiers.get_id(505, 0).map(|uid| uid.id.as_str()).unwrap_or(""));
    replaced = replaced.replace("__ANDROIDID__", identifiers.get_id(509, 0).map(|uid| uid.id.as_str()).unwrap_or(""));
    replaced = replaced.replace("__ ANDROIDID __", identifiers.get_id(509, 0).map(|uid| uid.id.as_str()).unwrap_or(""));
    replaced = replaced.replace("__ANDROIDID_M__", identifiers.get_id(510, 0).map(|uid| uid.id.as_str()).unwrap_or(""));
    replaced = replaced.replace("__ ANDROIDID_M __", identifiers.get_id(510, 0).map(|uid| uid.id.as_str()).unwrap_or(""));
    replaced = replaced.replace("__IDFA__", identifiers.get_id(507, 0).map(|uid| uid.id.as_str()).unwrap_or(""));
    replaced = replaced.replace("__ IDFA __", identifiers.get_id(507, 0).map(|uid| uid.id.as_str()).unwrap_or(""));
    replaced = replaced.replace("__MAC__", identifiers.get_id(511, 0).map(|uid| uid.id.as_str()).unwrap_or(""));
    replaced = replaced.replace("__ MAC __", identifiers.get_id(511, 0).map(|uid| uid.id.as_str()).unwrap_or(""));

    replaced
}
