use std::{collections::HashMap, time::Duration};

use reqwest::Url;
use urlencoding::encode;

use crate::{protocol::*, Assets, Cache, Client, Connection, HttpPool, Identifiers, Price, ResultMessage};

pub mod ad;
pub mod app;
pub mod caid;
pub mod request;
pub mod response;
pub mod track;
pub mod video;

pub use ad::AdxworkAd;
pub use app::AdxworkApp;
pub use caid::AdxworkCaid;
pub use request::AdxworkRequest;
pub use response::AdxworkResponse;
pub use track::AdxworkTrack;
pub use video::AdxworkVideo;

pub struct Adxwork {
}

impl Client for Adxwork {
    async fn request(request: &Request, connection: &Connection, pool: &HttpPool, cache: &Cache) -> Result<Response, ResultMessage> {
        let slot_id = connection.client_tag_id.split("|").nth(0).unwrap();

        let request_id = cache.get_sequence();
        let mut assets = Assets::new(request);
        let identifiers = Identifiers::new(request);

        let request_adxwork = AdxworkRequest {
            id: {
                slot_id.to_string()
            },
            ad_type: {
                if assets.get_asset_size("video") > 0 {
                    if request.item[0].spec.reward > 0 {
                        5.to_string()
                    } else {
                        6.to_string()
                    }
                } else if assets.get_asset_total_size() > 0 {
                    3.to_string()
                } else if request.item[0].spec.display.instl > 0 {
                    4.to_string()
                } else if request.item[0].spec.display.pos == Some(7) {
                    2.to_string()
                } else {
                    match (request.item[0].spec.display.w, request.item[0].spec.display.h) {
                        (Some(w), Some(h)) => {
                            if w > h * 3 {
                                1.to_string()
                            } else {
                                2.to_string()
                            }
                        },
                        _ => 1.to_string(),
                    }
                }
            },
            adw: {
                match request.item[0].spec.display.w {
                    Some(w) => Some(w.to_string()),
                    None => None,
                }
            },
            adh: {
                match request.item[0].spec.display.h {
                    Some(h) => Some(h.to_string()),
                    None => None,
                }
            },
            pkg: {
                match &connection.client_media_apppackage {
                    Some(client_media_apppackage) => Some(client_media_apppackage.clone()),
                    None => {
                        match &request.context.app {
                            Some(app) => app.bundle.clone(),
                            None => None,
                        }
                    },
                }
            },
            appv: {
                match &request.context.app {
                    Some(app) => app.ver.clone(),
                    None => None,
                }
            },
            aname: {
                match &connection.client_media_appname {
                    Some(client_media_appname) => Some(client_media_appname.clone()),
                    None => {
                        match &request.context.app {
                            Some(app) => Some(app.name.clone()),
                            None => None,
                        }
                    },
                }
            },
            ua: {
                encode(&request.context.device.ua).to_string()
            },
            os: {
                match request.context.device.os {
                    Some(2) => 0.to_string(),
                    Some(13) => 1.to_string(),
                    Some(15) => 2.to_string(),
                    _ => 3.to_string(),
                }
            },
            osv: {
                request.context.device.osv.clone()
            },
            osl: {
                match request.context.device.oslevel {
                    Some(oslevel) => Some(oslevel.to_string()),
                    _ => None,
                }
            },
            mnc: {
                match &request.context.device.carrier {
                    Some(carrier) => {
                        match carrier.as_str() {
                            "cmcc" => 1.to_string(),
                            "unicom" => 2.to_string(),
                            "telecom" => 3.to_string(),
                            _ => 4.to_string(),
                        }
                    },
                    _ => 4.to_string(),
                }
            },
            conn: {
                match &request.context.device.contype {
                    Some(7) => 0.to_string(),
                    Some(4) => 1.to_string(),
                    Some(5) => 2.to_string(),
                    Some(6) => 3.to_string(),
                    Some(2) => 4.to_string(),
                    _ => 5.to_string(),
                }
            },
            ip: {
                request.context.device.ip.clone()
            },
            ipv6: {
                request.context.device.ipv6.clone()
            },
            maker: {
                request.context.device.make.clone()
            },
            brand: {
                request.context.device.brand.clone()
            },
            model: {
                request.context.device.model.clone()
            },
            uuid: {
                match request.context.device.os {
                    Some(2) => identifiers.get_id(501, 0).map(|uid| uid.id.clone()),
                    Some(13) => identifiers.get_id(507, 0).map(|uid| uid.id.clone()),
                    _ => None,
                }
            },
            umd5: {
                match request.context.device.os {
                    Some(2) => identifiers.get_id(502, 0).map(|uid| uid.id.clone()),
                    Some(13) => identifiers.get_id(508, 0).map(|uid| uid.id.clone()),
                    _ => None,
                }
            },
            anid: {
                identifiers.get_id(509, 0).map(|uid| uid.id.clone())
            },
            amd5: {
                identifiers.get_id(510, 0).map(|uid| uid.id.clone())
            },
            oaid: {
                identifiers.get_id(505, 0).map(|uid| uid.id.clone())
            },
            omd5: {
                identifiers.get_id(506, 0).map(|uid| uid.id.clone())
            },
            mac: {
                None
                // TODO: mac address validation
                // identifiers.get_id(511, 0).map(|uid| uid.id.clone())
            },
            mmd5: {
                identifiers.get_id(512, 0).map(|uid| uid.id.clone())
            },
            idfv: {
                identifiers.get_id(515, 0).map(|uid| uid.id.clone())
            },
            caid: {
                identifiers.get_id(513, 0).map(|uid| uid.id.clone())
            },
            caidv: {
                identifiers.get_id(513, 0).and_then(|uid| uid.ver.clone())
            },
            caidvd: {
                identifiers.get_id(513, 0).and_then(|uid| uid.vendor.clone())
            },
            caids: {
                identifiers.get_ids(513).map(|uids| {
                    uids.iter().map(|uid| AdxworkCaid {
                        caid: Some(uid.id.clone()),
                        ver: uid.ver.clone(),
                    }).collect::<Vec<AdxworkCaid>>()
                }).filter(|uids| !uids.is_empty())
            },
            oudid: {
                request.context.device.mntid.clone()
            },
            imsi: {
                identifiers.get_id(503, 0).map(|uid| uid.id.clone())
            },
            ssl: {
                Some(1.to_string())
            },
            isdp: {
                Some(1.to_string())
            },
            pw: {
                request.context.device.w.map(|w| w.to_string())
            },
            ph: {
                request.context.device.h.map(|h| h.to_string())
            },
            den: {
                request.context.device.pxratio.map(|pxratio| pxratio.to_string())
            },
            ppi: {
                request.context.device.ppi.map(|ppi| ppi.to_string())
            },
            dpi: {
                request.context.device.ppi.map(|ppi| ppi.to_string())
            },
            ori: {
                request.context.device.orientation.map(|orientation| match orientation {
                    501 => 1,
                    502 => 2,
                    _ => 0,
                }).map(|v| v.to_string())
            },
            dtype: {
                match request.context.device.devicetype {
                    Some(1) | Some(4) => 1.to_string(),
                    Some(5) => 2.to_string(),
                    _ => 0.to_string(),
                }
            },
            isurl: {
                Some(if identifiers.get_id(507, 0).is_some() { "1" } else { "0" }.to_string())
            },
            wmac: {
                identifiers.get_id(522, 0).map(|uid| uid.id.clone())
            },
            ssid: {
                identifiers.get_id(524, 0).map(|uid| uid.id.clone())
            },
            rver: {
                request.context.device.romv.clone().or(request.context.device.uiv.clone())
            },
            sysc: {
                request.context.device.romtime.clone()
            },
            miuiver: {
                request.context.device.uiv.clone()
            },
            hmsver: {
                request.context.device.hmsv.clone()
            },
            hwagver: {
                request.context.device.storev.clone()
            },
            appsver: {
                request.context.device.storename.clone()
            },
            appsvercode: {
                request.context.device.storev.clone()
            },
            bootmark: {
                request.context.device.bootmark.clone()
            },
            updatemark: {
                request.context.device.updatemark.clone()
            },
            lat: {
                request.context.device.geo.as_ref().and_then(|geo| geo.lat.map(|v| v.to_string()))
            },
            lon: {
                request.context.device.geo.as_ref().and_then(|geo| geo.lon.map(|v| v.to_string()))
            },
            btime: {
                request.context.device.boottime.clone()
            },
            uptime: {
                request.context.device.updatetime.clone()
            },
            dftime: {
                request.context.device.inittime.clone()
            },
            hwmodel: {
                request.context.device.hwmodel.clone()
            },
            country: {
                request.context.device.country.clone()
            },
            language: {
                request.context.device.lang.clone()
            },
            hwm: {
                request.context.device.hwmachine.clone()
            },
            tzone: {
                request.context.device.timezone.clone()
            },
            localname: {
                request.context.device.timezone.clone()
            },
            memory: {
                request.context.device.sysmemory.map(|v| v.to_string())
            },
            hdisk: {
                request.context.device.sysdisksize.map(|v| v.to_string())
            },
            dname: {
                identifiers.get_id(527, 0).map(|uid| uid.id.clone())
            },
            dnmd5: {
                identifiers.get_id(528, 0).map(|uid| uid.id.clone())
            },
            cpunum: {
                request.context.device.syscpu.map(|v| v.to_string())
            },
            cpufreq: {
                request.context.device.syscpufreq.map(|v| v.to_string())
            },
            idfapolicy: {
                request.context.device.lmt.map(|v| match v {
                    0 => "3".to_string(),
                    1 => "1".to_string(),
                    2 => "2".to_string(),
                    _ => "0".to_string(),
                })
            },
            bstatus: {
                request.context.device.sysbatterystatus.map(|v| match v {
                    0 => "1".to_string(),
                    1 => "2".to_string(),
                    2 => "3".to_string(),
                    3 => "4".to_string(),
                    _ => "1".to_string(),
                })
            },
            bpower: {
                request.context.device.sysbatterypower.map(|v| v.to_string())
            },
            aaid: {
                identifiers.get_id(514, 0).map(|uid| uid.id.clone())
            },
            paid: {
                identifiers.get_id(519, 0).map(|uid| uid.id.clone())
            },
            applist: {
                request.context.device.app.clone().map(|v| encode(&v).to_string())
            },
            bidfloor: {
                Some((Price::to_client(connection, request.item[0].flr.map(f64::from)) as i32).to_string())
            },
            elapsetime: {
                None
            },
        };

        let client = {
            let pool_adxwork_lock = pool.pool_adxwork.clone();
            let pool_adxwork = pool_adxwork_lock.read().unwrap();
            pool_adxwork.clone()
        };
        let response_adxwork_raw = client.post("http://ssp.adxwork.com/q")
            .json(&request_adxwork)
            .header("Accept-Encoding", "gzip")
            .header("Connection", "keep-alive")
            .header("Content-Type", "application/json;charset=UTF-8")
            .timeout(Duration::from_millis(connection.timeout))
            .send().await;

        let response_adxwork: AdxworkResponse = match response_adxwork_raw {
            Ok(response_adxwork_raw) => {
                let status = response_adxwork_raw.status();
                if status == 204 {
                    return Err(ResultMessage {
                        code: 993, // defending
                        message: "".to_string()
                    });
                } else if status != 200 {
                    return Err(ResultMessage {
                        code: 992,
                        message: match response_adxwork_raw.text().await {
                            Ok(text) => format!("upstream error {}: {}", status, text),
                            Err(_) => format!("upstream error {}", status),
                        },
                    });
                }
                match response_adxwork_raw.bytes().await {
                    Ok(bytes) => {
                        let text = match std::str::from_utf8(&bytes) {
                            Ok(text) => text,
                            Err(error) => {
                                return Err(ResultMessage {
                                    code: 992,
                                    message: error.to_string(),
                                });
                            }
                        };
                        match serde_json::from_str::<AdxworkResponse>(text) {
                            Ok(json) => json,
                            Err(error) => return Err(ResultMessage {
                                code: 997,
                                message: error.to_string(),
                            }),
                        }
                    }
                    Err(error) => {
                        return Err(ResultMessage {
                            code: 992,
                            message: error.to_string(),
                        })
                    }
                }
            },
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

        if response_adxwork.code == "204" {
            return Err(ResultMessage {
                code: 993,
                message: format!("upstream error {}: {}", response_adxwork.code, response_adxwork.msg.unwrap_or_default()),
            });
        }
        if response_adxwork.code != "200" {
            return Err(ResultMessage {
                code: 994,
                message: format!("upstream error {}: {}", response_adxwork.code, response_adxwork.msg.unwrap_or_default()),
            });
        }

        Ok(Response {
            id: request.id.clone(),
            nbr: None,
            seatbid: {
                match &response_adxwork.ads {
                    Some(ads) => {
                        match ads.first() {
                            Some(ad) => {
                                Some(vec![Seatbid {
                                    bid: {
                                        let mut bids = vec![];
                                        let link_asset = LinkAsset {
                                            linktype: match ad.adct {
                                                Some(2) => 2,
                                                Some(4) => 3,
                                                _ => 1,
                                            },
                                            universallink: ad.uslink.clone().or(ad.unlink.clone()),
                                            storeid: None,
                                            deeplink: ad.dplink.clone(),
                                            quickapplink: None,
                                            wechatmppath: ad.wxpath.clone(),
                                            wechatmpid: ad.wxid.clone(),
                                            marketurl: ad.mkurl.clone(),
                                            downloadurl: match ad.adct {
                                                Some(2) | Some(4) => ad.curl.clone(),
                                                _ => None,
                                            },
                                            url: ad.curl.clone().unwrap_or_default(),
                                            urlfb: None,
                                        };

                                        bids.push(Bid {
                                            id: Some(request_id.to_string()),
                                            item: request.item[0].id.clone(),
                                            price: ad.price.unwrap_or(connection.default_price),
                                            burl: {
                                                match &ad.kswurls {
                                                    Some(urls) => {
                                                        let mut burl = vec![];
                                                        for url in urls {
                                                            let nurl = url.replace("__LOSS_PR__", "__2ND_PRICE__");
                                                            burl.push(replace_macro(request, &identifiers, &nurl));
                                                        }
                                                        if burl.is_empty() { None } else { Some(burl) }
                                                    },
                                                    None => None,
                                                }
                                            },
                                            lurl: {
                                                let mut lurl = vec![];
                                                for urls in [&ad.kslurls, &ad.lurls] {
                                                    if let Some(urls) = urls {
                                                        for url in urls {
                                                            let mut nurl = url.clone();
                                                            nurl = nurl.replace("__BID_ECPM__", "__LOSE_PRICE__");
                                                            nurl = nurl.replace("__AD_ECPM__", "__LOSE_PRICE__");
                                                            nurl = nurl.replace("__ADN_NAME__", "__LOSE_ADN_NAME__");
                                                            nurl = nurl.replace("__AD_REQID__", request_id.to_string().as_str());
                                                            lurl.push(replace_macro(request, &identifiers, &nurl));
                                                        }
                                                    }
                                                }
                                                if lurl.is_empty() { None } else { Some(lurl) }
                                            },
                                            media: Ad {
                                                id: response_adxwork.id.clone().unwrap_or_else(|| request_id.to_string()),
                                                display: Display {
                                                    w: None,
                                                    h: None,
                                                    banner: {
                                                        if assets.get_banner_size() > 0 {
                                                            if let Some(imgs) = &ad.imgs {
                                                                imgs.first().map(|img| Banner {
                                                                    img: img.clone(),
                                                                    link: Some(link_asset.clone()),
                                                                })
                                                            } else {
                                                                ad.imgurl.as_ref().map(|img| Banner {
                                                                    img: img.clone(),
                                                                    link: Some(link_asset.clone()),
                                                                })
                                                            }
                                                        } else {
                                                            None
                                                        }
                                                    },
                                                    native: {
                                                        if assets.get_asset_total_size() == 0 {
                                                            None
                                                        } else {
                                                            let mut asset_vec = vec![];

                                                            if let Some(video) = &ad.video {
                                                                if let Some(url) = &video.url {
                                                                    asset_vec.push(Asset {
                                                                        id: assets.consume_asset("video"),
                                                                        req: 1,
                                                                        title: None,
                                                                        img: None,
                                                                        video: Some(VideoAsset {
                                                                            url: url.clone(),
                                                                            mime: None,
                                                                            w: video.width,
                                                                            h: video.height,
                                                                            dur: video.dur,
                                                                            size: video.length.map(|v| v * 1024),
                                                                            skipoffset: video.keepdur,
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
                                                                if let Some(coverurl) = &video.coverurl {
                                                                    asset_vec.push(Asset {
                                                                        id: assets.consume_asset("video#cover"),
                                                                        req: 0,
                                                                        title: None,
                                                                        img: Some(ImageAsset {
                                                                            url: coverurl.clone(),
                                                                            mime: None,
                                                                            w: video.width,
                                                                            h: video.height,
                                                                            imagetype: Some(3),
                                                                        }),
                                                                        video: None,
                                                                        data: None,
                                                                        html: None,
                                                                        app: None,
                                                                    });
                                                                }
                                                                if let Some(endcard) = &video.endcard {
                                                                    asset_vec.push(Asset {
                                                                        id: assets.consume_asset("video#end#img"),
                                                                        req: 0,
                                                                        title: None,
                                                                        img: Some(ImageAsset {
                                                                            url: endcard.clone(),
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
                                                                if let Some(endhtml) = &video.endhtml {
                                                                    asset_vec.push(Asset {
                                                                        id: assets.consume_asset("video#end#html"),
                                                                        req: 0,
                                                                        title: None,
                                                                        img: None,
                                                                        video: None,
                                                                        data: None,
                                                                        html: Some(HtmlAsset {
                                                                            html: Some(endhtml.clone()),
                                                                            link: None,
                                                                            len: Some(endhtml.len() as i32),
                                                                        }),
                                                                        app: None,
                                                                    });
                                                                }
                                                            }

                                                            if let Some(title) = ad.title.clone().or(ad.video.as_ref().and_then(|video| video.title.clone())) {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("title"),
                                                                    req: 1,
                                                                    title: Some(TitleAsset {
                                                                        text: title.clone(),
                                                                        subtitle: None,
                                                                        desc: ad.text.clone().or(ad.video.as_ref().and_then(|video| video.desc.clone())),
                                                                        len: Some(title.len() as i32),
                                                                    }),
                                                                    img: None,
                                                                    video: None,
                                                                    data: None,
                                                                    html: None,
                                                                    app: None,
                                                                });
                                                            }

                                                            if let Some(desc) = ad.text.clone().or(ad.video.as_ref().and_then(|video| video.desc.clone())) {
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

                                                            if let Some(icon) = ad.icon.clone().or(ad.app.as_ref().and_then(|app| app.icon.clone())).or(ad.video.as_ref().and_then(|video| video.icon.clone())) {
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

                                                            if let Some(imgs) = &ad.imgs {
                                                                for img in imgs {
                                                                    asset_vec.push(Asset {
                                                                        id: assets.consume_asset("img"),
                                                                        req: 1,
                                                                        title: None,
                                                                        img: Some(ImageAsset {
                                                                            url: img.clone(),
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
                                                            } else if let Some(imgurl) = &ad.imgurl {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("img"),
                                                                    req: 1,
                                                                    title: None,
                                                                    img: Some(ImageAsset {
                                                                        url: imgurl.clone(),
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

                                                            if let Some(adm) = &ad.adm {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("html"),
                                                                    req: 0,
                                                                    title: None,
                                                                    img: None,
                                                                    video: None,
                                                                    data: None,
                                                                    html: Some(HtmlAsset {
                                                                        html: Some(adm.clone()),
                                                                        link: None,
                                                                        len: Some(adm.len() as i32),
                                                                    }),
                                                                    app: None,
                                                                });
                                                            }

                                                            if let Some(app) = &ad.app {
                                                                asset_vec.push(Asset {
                                                                    id: assets.consume_asset("app"),
                                                                    req: 0,
                                                                    title: None,
                                                                    img: None,
                                                                    video: None,
                                                                    data: None,
                                                                    html: None,
                                                                    app: Some(AppAsset {
                                                                        name: app.aname.clone().unwrap_or_default(),
                                                                        desc: ad.text.clone(),
                                                                        descurl: None,
                                                                        domain: None,
                                                                        bundle: app.adpkg.clone(),
                                                                        ver: app.appv.clone(),
                                                                        developer: app.developer.clone(),
                                                                        icon: app.icon.clone(),
                                                                        storeid: None,
                                                                        storeurl: None,
                                                                        paid: 0,
                                                                        size: app.size,
                                                                        md5: None,
                                                                        registration: None,
                                                                        privacy: None,
                                                                        privacyurl: app.privacy.clone(),
                                                                        permission: app.authority.clone(),
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
                                                            (&ad.imps, 501),
                                                            (&ad.clicks, 502),
                                                            (&ad.sdpurls, 503),
                                                            (&ad.fdpurls, 505),
                                                            (&ad.dpurls, 504),
                                                            (&ad.durls, 601),
                                                            (&ad.ddurls, 602),
                                                            (&ad.iurls, 603),
                                                            (&ad.idurls, 604),
                                                            (&ad.surls, 605),
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

                                                        if let Some(video) = &ad.video {
                                                            for (urls, eventtype) in [(&video.impurls, 501), (&video.clkurls, 502)] {
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
                                                            if let Some(plays) = &video.plays {
                                                                for play in plays {
                                                                    let eventtype = match play.ttype {
                                                                        Some(1) => Some(701),
                                                                        Some(2) => Some(702),
                                                                        Some(3) => Some(703),
                                                                        Some(4) => Some(704),
                                                                        Some(5) => Some(705),
                                                                        Some(6) => Some(713),
                                                                        Some(7) => Some(715),
                                                                        Some(8) => Some(710),
                                                                        Some(9) => Some(711),
                                                                        _ => None,
                                                                    };

                                                                    if eventtype.is_some() && play.urls.is_some() {
                                                                        let eventtype = eventtype.unwrap();
                                                                        let urls = play.urls.clone().unwrap();

                                                                        for url in &urls {
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
                                                            }
                                                        }

                                                        match &ad.click_area_report_url {
                                                            Some(click_area_report_urls) => {
                                                                for click_area_report_url in click_area_report_urls {
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
                                                                        }
                                                                        Err(_) => (),
                                                                    }
                                                                }
                                                            },
                                                            None => (),
                                                        }

                                                        event_vec
                                                    },
                                                },
                                                advertiser: ad.app.as_ref().and_then(|app| app.developer.clone()),
                                                advertisericon: ad.icon.clone(),
                                            },
                                        });

                                        bids
                                    },
                                }])
                            },
                            None => {
                                return Err(ResultMessage {
                                    code: 993, // defending
                                    message: "".to_string(),
                                })
                            },
                        }
                    },
                    None =>  {
                        return Err(ResultMessage {
                            code: 993, // defending
                            message: "".to_string(),
                        })
                    },
                }
            },
        })
    }

    async fn bidding_notify_win(url: String, win_price: i32, next_price: i32, _iv: &String, _connection: &Connection, pool: &HttpPool) {
        let replaced_url = url
            .replace("__WIN_PRICE__", &win_price.to_string())
            .replace("__PRICE__", &win_price.to_string())
            .replace("__2ND_PRICE__", &next_price.to_string())
            .replace("__LOSS_PR__", &next_price.to_string());

        let client = {
            let pool_adxwork_lock = pool.pool_adxwork.clone();
            let pool_adxwork = pool_adxwork_lock.read().unwrap();
            pool_adxwork.clone()
        };
        let _ = client.get(replaced_url).send().await;
    }

    async fn bidding_notify_lose(url: String, lose_price: i32, lose_reason: i32, lose_adn_name: &String, _iv: &String, _connection: &Connection, pool: &HttpPool) {
        let replaced_url = url
            .replace("__LOSE_PRICE__", &lose_price.to_string())
            .replace("__BID_ECPM__", &lose_price.to_string())
            .replace("__BID_FAIL_REASON__", &lose_reason.to_string())
            .replace("__LOSE_ADN_NAME__", lose_adn_name);

        let client = {
            let pool_adxwork_lock = pool.pool_adxwork.clone();
            let pool_adxwork = pool_adxwork_lock.read().unwrap();
            pool_adxwork.clone()
        };
        let _ = client.get(replaced_url).send().await;
    }

    fn encrypt_price(price: i32, _iv: &String, _connection: &Connection) -> String {
        price.to_string()
    }
}

fn replace_macro(request: &Request, identifiers: &Identifiers, orig: &String) -> String {
    let mut replaced = orig.clone();

    replaced = replaced.replace("_WIDTH_", "__WIDTH__");
    replaced = replaced.replace("_HEIGHT_", "__HEIGHT__");
    replaced = replaced.replace("_ABS_DOWN_X_", "__ABS_DOWN_X__");
    replaced = replaced.replace("_ABS_DOWN_Y_", "__ABS_DOWN_Y__");
    replaced = replaced.replace("_ABS_UP_X_", "__ABS_UP_X__");
    replaced = replaced.replace("_ABS_UP_Y_", "__ABS_UP_Y__");
    replaced = replaced.replace("_TS_S_", "__EVENT_TIME_START_S__");
    replaced = replaced.replace("_START_TS_", "__EVENT_TIME_START__");
    replaced = replaced.replace("_END_TS_", "__EVENT_TIME_END__");
    replaced = replaced.replace("_END_SEC_", "__EVENT_TIME_END_S__");
    replaced = replaced.replace("_CLICK_ID_", "__CLICK_ID__");
    replaced = replaced.replace("_RLT_DOWN_X_", "__R_DOWN_X__");
    replaced = replaced.replace("_RLT_DOWN_Y_", "__R_DOWN_Y__");
    replaced = replaced.replace("_RLT_UP_X_", "__R_UP_X__");
    replaced = replaced.replace("_RLT_UP_Y_", "__R_UP_Y__");
    replaced = replaced.replace("_DP_W_", "__DP_WIDTH__");
    replaced = replaced.replace("_DP_H_", "__DP_HEIGHT__");
    replaced = replaced.replace("_DP_DOWN_X_", "__DOWN_DP_X__");
    replaced = replaced.replace("_DP_DOWN_Y_", "__DOWN_DP_Y__");
    replaced = replaced.replace("_DP_UP_X_", "__UP_DP_X__");
    replaced = replaced.replace("_DP_UP_Y_", "__UP_DP_Y__");

    replaced = replaced.replace("__TARGET_APP_INSTALL__", "__DP_TARGET__");
    replaced = replaced.replace("__DISPLAY_LUX__", "__LT_X__");
    replaced = replaced.replace("__DISPLAY_LUY__", "__LT_Y__");
    replaced = replaced.replace("__DISPLAY_RDX__", "__RB_X__");
    replaced = replaced.replace("__DISPLAY_RDY__", "__RB_Y__");
    replaced = replaced.replace("__BUTTON_LUX__", "__BUTTON_LT_X__");
    replaced = replaced.replace("__BUTTON_LUY__", "__BUTTON_LT_Y__");
    replaced = replaced.replace("__BUTTON_RDX__", "__BUTTON_RB_X__");
    replaced = replaced.replace("__BUTTON_RDY__", "__BUTTON_RB_Y__");
    replaced = replace_macro_device(request, identifiers, &replaced);
    replaced = replaced.replace("__ADN_TYPE__", "");
    replaced = replaced.replace("__ADN_NAME__", "__LOSE_ADN_NAME__");
    replaced = replaced.replace("__AD_N__", "");
    replaced = replaced.replace("__AD_TI__", "");
    replaced = replaced.replace("__IS_S__", "");
    replaced = replaced.replace("__IS_C__", "");

    replaced
}

fn replace_macro_device(request: &Request, identifiers: &Identifiers, orig: &String) -> String {
    let mut replaced = orig.clone();

    if let Some(uid) = identifiers.get_id(507, 0) {
        replaced = replaced.replace("__IDFA__", &uid.id);
    }
    if let Some(uid) = identifiers.get_id(508, 0) {
        replaced = replaced.replace("__IDFAMD5__", &uid.id);
    }
    if let Some(uid) = identifiers.get_id(501, 0) {
        replaced = replaced.replace("__IMEI__", &uid.id);
    }
    if let Some(uid) = identifiers.get_id(502, 0) {
        replaced = replaced.replace("__IMEIMD5__", &uid.id);
    }
    if let Some(ip) = &request.context.device.ip {
        replaced = replaced.replace("__IP__", ip);
    }
    replaced = replaced.replace("__UA__", &request.context.device.ua);
    if let Some(uid) = identifiers.get_id(505, 0) {
        replaced = replaced.replace("__OAID__", &uid.id);
    }
    if let Some(uid) = identifiers.get_id(511, 0) {
        replaced = replaced.replace("__MAC__", &uid.id.replace(":", ""));
        replaced = replaced.replace("__MAC1__", &uid.id);
    }
    if let Some(uid) = identifiers.get_id(509, 0) {
        replaced = replaced.replace("__ANDROIDID__", &uid.id);
    }
    if let Some(uid) = identifiers.get_id(510, 0) {
        replaced = replaced.replace("__ANDROIDIDMD5__", &uid.id);
    }
    if let Some(uid) = identifiers.get_id(514, 0) {
        replaced = replaced.replace("__ALL_AAID__", &uid.id);
    }
    if let Some(uid) = identifiers.get_id(513, 0) {
        replaced = replaced.replace("__CAID__", &uid.id);
    }

    replaced
}
