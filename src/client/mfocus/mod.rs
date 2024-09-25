use std::{collections::HashMap, time::Duration};

use crate::{protocol::*, Assets, Cache, Client, Connection, ResultMessage};

pub mod ad_content;
pub mod request;
pub mod response;
pub mod result;
pub mod wechat_mini_program;

pub use ad_content::MfocusAdContent;
pub use request::MfocusRequest;
pub use response::MfocusResponse;
pub use result::MfocusResult;
pub use wechat_mini_program::MfocusWechatMiniProgram;

pub struct Mfocus {

}

impl Client for Mfocus {

    async fn request(request: &Request, connection: &Connection, cache: &Cache) -> Result<Response, ResultMessage> {
        let id = connection.client_tag_id.split("|").nth(0).unwrap();
        let vdid = connection.client_tag_id.split("|").nth(1).unwrap();
        let dealid = connection.client_tag_id.split("|").nth(2).unwrap();

        let eids = &request.context.user.eids;
        let mut id_map = HashMap::new();
        for eid in eids {
            let uids = &eid.uids;
            for uid in uids {
                id_map.insert(uid.atype, uid.id.clone());
            }
        }

        let request_id = cache.get_sequence();

        let assets = Assets::new(request);

        let request_mfocus = MfocusRequest {
            id: {
                id.to_string()
            },
            vdid: {
                vdid.to_string()
            },
            dealid: {
                dealid.to_string()
            },
            os: {
                match request.context.device.os {
                    Some(os) => {
                        match os {
                            2 => 0,
                            13 => 1,
                            28 => 2,
                            _ => 3,
                        }
                    },
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.device.os is required for upstream".to_string(),
                    }),
                }
            },
            ip: {
                match &request.context.device.ip {
                    Some(ip) => ip.clone(),
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.device.ip is required for upstream".to_string(),
                    }),
                }
            },
            av: {
                "v3.0.1".to_string()
            },
            idfa: {
                match id_map.get(&507) {
                    Some(id) => id.clone(),
                    None => "".to_string(),
                }
            },
            idfa_md5: {
                match id_map.get(&508) {
                    Some(id) => id.clone(),
                    None => "".to_string(),
                }
            },
            caid: {
                match id_map.get(&513) {
                    Some(id) => id.clone(),
                    None => "".to_string(),
                }
            },
            caid_md5: {
                match id_map.get(&513) {
                    Some(id) => {
                        format!("{:x}", md5::compute(format!("{}", id.clone()).as_bytes()))
                    }
                    None => "".to_string(),
                }
            },
            imei: {
                match id_map.get(&501) {
                    Some(id) => id.clone(),
                    None => "".to_string(),
                }
            },
            imei_md5: {
                match id_map.get(&502) {
                    Some(id) => id.clone(),
                    None => "".to_string(),
                }
            },
            oaid: {
                match id_map.get(&505) {
                    Some(id) => id.clone(),
                    None => "".to_string(),
                }
            },
            oaid_md5: {
                match id_map.get(&506) {
                    Some(id) => id.clone(),
                    None => "".to_string(),
                }
            },
            androidid_md5: {
                match id_map.get(&510) {
                    Some(id) => id.clone(),
                    None => "".to_string(),
                }
            },
            ua: {
                request.context.device.ua.clone()
            },
            mac: {
                match id_map.get(&511) {
                    Some(id) => id.clone(),
                    None => "".to_string(),
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
            sc_w: {
                match request.context.device.w {
                    Some(w) => w.to_string(),
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.device.w is required for upstream".to_string(),
                    }),
                }
            },
            sc_h: {
                match request.context.device.h {
                    Some(h) => h.to_string(),
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.device.h is required for upstream".to_string(),
                    }),
                }
            },
            mid: {
                match &request.context.device.osv {
                    Some(osv) => osv.clone(),
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.device.osv is required for upstream".to_string(),
                    }),
                }
            },
            imsi: {
                match &request.context.device.carrier {
                    Some(carrier) => {
                        match carrier.as_str() {
                            "cmcc" => "46000".to_string(),
                            "unicom" => "46001".to_string(),
                            "telecom" => "46003".to_string(),
                            _ => "-1".to_string(),
                        }
                    },
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.device.carrier is required for upstream".to_string(),
                    }),
                }
            },
            network: {
                match &request.context.device.contype {
                    Some(contype) => {
                        match contype {
                            1 => "-1".to_string(),
                            2 => "1".to_string(),
                            3 => "-1".to_string(),
                            4 => "4".to_string(),
                            5 => "3".to_string(),
                            6 => "2".to_string(),
                            7 => "6".to_string(),
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
            brand: {
                match &request.context.device.brand {
                    Some(brand) => brand.clone(),
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.device.brand is required for upstream".to_string(),
                    }),
                }
            },
        };

        let response_mfocus: MfocusResponse;

        let client = reqwest::ClientBuilder::new()
            .gzip(true)
            .no_brotli()
            .no_deflate()
            .build().unwrap();
        let response_mfocus_raw = client.post(if connection.test { "http://test.m-focus.cn/v1/api/vender/ad" } else { "https://yjdsp.m-focus.cn/v1/api/vender/ad" })
            .json(&request_mfocus)
            .header("Accept-Encoding", "gzip")
            .header("Connection", "keep-alive")
            .header("Content-Type", "application/json")
            .timeout(Duration::from_millis(connection.timeout))
            .send().await;

        match response_mfocus_raw {
            Ok(response_mfocus_raw) => {
                let status = response_mfocus_raw.status();
                if status == 204 {
                    return Err(ResultMessage {
                        code: 993,
                        message: "".to_string(),
                    });
                } else if status != 200 {
                    return Err(ResultMessage {
                        code: 992,
                        message: {
                            match response_mfocus_raw.text().await {
                                Ok(text) => format!("upstream error {}: {}", status, text),
                                Err(_) => format!("upstream error {}", status),
                            }
                        },
                    });
                } else {
                    match response_mfocus_raw.text().await {
                        Ok(text) => {
                            match serde_json::from_str::<MfocusResponse>(&text) {
                                Ok(json) => {
                                    response_mfocus = json;

                                    match response_mfocus.code.as_str() {
                                        "200" => (),
                                        "50010" => {
                                            return Err(ResultMessage {
                                                code: 992,
                                                message: format!("upstream error: signature expired: {}", response_mfocus.msg.to_string()),
                                            });
                                        },
                                        "50011" => {
                                            return Err(ResultMessage {
                                                code: 992,
                                                message: format!("upstream error: invalid appkey: {}", response_mfocus.msg.to_string()),
                                            });
                                        },
                                        "50012" => {
                                            return Err(ResultMessage {
                                                code: 992,
                                                message: format!("upstream error: invalid vender: {}", response_mfocus.msg.to_string()),
                                            });
                                        },
                                        "50013" => {
                                            return Err(ResultMessage {
                                                code: 992,
                                                message: format!("upstream error: wrong signature: {}", response_mfocus.msg.to_string()),
                                            });
                                        },
                                        "50020" => {
                                            return Err(ResultMessage {
                                                code: 992,
                                                message: format!("upstream error: invalid field: {}", response_mfocus.msg.to_string()),
                                            });
                                        },
                                        "50041" => {
                                            return Err(ResultMessage {
                                                code: 992,
                                                message: format!("upstream error: wrong deadid: {}", response_mfocus.msg.to_string()),
                                            });
                                        },
                                        "50042" => {
                                            return Err(ResultMessage {
                                                code: 992,
                                                message: format!("upstream error: deal is not active: {}", response_mfocus.msg.to_string()),
                                            });
                                        },
                                        "50043" => {
                                            return Err(ResultMessage {
                                                code: 992,
                                                message: format!("upstream error: deal is suspended: {}", response_mfocus.msg.to_string()),
                                            });
                                        },
                                        "50044" => {
                                            return Err(ResultMessage {
                                                code: 992,
                                                message: format!("upstream error: wrong ad id: {}", response_mfocus.msg.to_string()),
                                            });
                                        },
                                        _ => {
                                            return Err(ResultMessage {
                                                code: 992,
                                                message: format!("upstream error: {} {}", response_mfocus.code, response_mfocus.msg.to_string()),
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
                        message: "upstream request timeout".to_string(),
                    });
                } else {
                    return Err(ResultMessage {
                        code: 992,
                        message: format!("upstream request failed: {}", error.to_string()),
                    });
                }
            }
        }

        let response = Response {
            id: request.id.clone(),
            nbr: None,
            seatbid: {
                match response_mfocus.result {
                    Some(result) => {
                        Some([Seatbid {
                            bid: {
                                let mut bids = vec![];

                                let link_asset = LinkAsset {
                                    linktype: {
                                        1
                                    },
                                    universallink: {
                                        None
                                    },
                                    storeid: {
                                        None
                                    },
                                    deeplink: {
                                        match &result.deeplink {
                                            Some(deeplink) => Some(deeplink.clone()),
                                            None => None,
                                        }
                                    },
                                    quickapplink: None,
                                    wechatmppath: {
                                        match &result.wx_mini_program {
                                            Some(wx_mini_program) => {
                                                match &wx_mini_program.path {
                                                    Some(path) => Some(path.clone()),
                                                    None => None,
                                                }
                                            },
                                            None => None,
                                        }
                                    },
                                    wechatmpid: {
                                        match &result.wx_mini_program {
                                            Some(wx_mini_program) => {
                                                match &wx_mini_program.appid {
                                                    Some(appid) => Some(appid.clone()),
                                                    None => None,
                                                }
                                            },
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
                                        result.clk.clone()
                                    },
                                    urlfb: None,
                                };

                                match &result.ad_content {
                                    Some(ad_content) => {
                                        let bid = Bid {
                                            id: Some(request_id.to_string()),
                                            item: request.item[0].id.clone(),
                                            price: { // update later
                                                match result.cpm {
                                                    Some(cpm) => {
                                                        if cpm > 0 {
                                                            cpm
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
                                                id: result.adid,
                                                display: Display {
                                                    w: {
                                                        None
                                                    },
                                                    h: {
                                                        None
                                                    },
                                                    banner: {
                                                        if request.item[0].spec.display.displayfmt.is_some() {
                                                            Some(Banner {
                                                                img: {
                                                                    match &ad_content.mainimage {
                                                                        Some(mainimage) => {
                                                                            mainimage.clone()
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
                                                        if request.item[0].spec.display.nativefmt.is_some() {
                                                            let mut asset_vec = vec![];

                                                            if assets.video_asset.len() > 0 && ad_content.videourl.is_some() {
                                                                asset_vec.push(Asset {
                                                                    id: assets.video_asset.get(0).unwrap().id,
                                                                    req: 1,
                                                                    video: Some(VideoAsset {
                                                                        url: {
                                                                            match &ad_content.videourl {
                                                                                Some(videourl) => {
                                                                                    videourl.clone()
                                                                                },
                                                                                None => "".to_string(),
                                                                            }
                                                                        },
                                                                        mime: None,
                                                                        w: None,
                                                                        h: None,
                                                                        dur: None,
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
                                                            }
                                                            if assets.video_cover_asset.len() > 0 && ad_content.coverimage.is_some() {
                                                                asset_vec.push(Asset {
                                                                    id: assets.video_cover_asset.get(0).unwrap().id,
                                                                    req: 0,
                                                                    img: Some(ImageAsset {
                                                                        url: {
                                                                            match &ad_content.coverimage {
                                                                                Some(coverimage) => {
                                                                                    coverimage.clone()
                                                                                },
                                                                                None => "".to_string(),
                                                                            }
                                                                        },
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
                                                            if assets.title_asset.len() > 0 && ad_content.title.is_some() {
                                                                asset_vec.push(Asset {
                                                                    id: assets.title_asset.get(0).unwrap().id,
                                                                    req: 1,
                                                                    title: Some(TitleAsset {
                                                                        text: ad_content.title.clone().unwrap(),
                                                                        desc: ad_content.text.clone(),
                                                                        len: Some(ad_content.title.clone().unwrap().len() as i32),
                                                                    }),
                                                                    img: None,
                                                                    video: None,
                                                                    data: None,
                                                                    html: None,
                                                                    app: None,
                                                                });
                                                            }
                                                            for (i, asset) in assets.img_asset.iter().enumerate() {
                                                                if i == 0 && ad_content.image1.is_some() {
                                                                    asset_vec.push(Asset {
                                                                        id: asset.id,
                                                                        req: 1,
                                                                        img: {
                                                                            Some(ImageAsset {
                                                                                url: ad_content.image1.clone().unwrap(),
                                                                                mime: None,
                                                                                w: None,
                                                                                h: None,
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
                                                                if i == 1 && ad_content.image2.is_some() {
                                                                    asset_vec.push(Asset {
                                                                        id: asset.id,
                                                                        req: 1,
                                                                        img: {
                                                                            Some(ImageAsset {
                                                                                url: ad_content.image2.clone().unwrap(),
                                                                                mime: None,
                                                                                w: None,
                                                                                h: None,
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
                                                                if i == 2 && ad_content.image3.is_some() {
                                                                    asset_vec.push(Asset {
                                                                        id: asset.id,
                                                                        req: 1,
                                                                        img: {
                                                                            Some(ImageAsset {
                                                                                url: ad_content.image3.clone().unwrap(),
                                                                                mime: None,
                                                                                w: None,
                                                                                h: None,
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
                                                                if i == 3 && ad_content.image4.is_some() {
                                                                    asset_vec.push(Asset {
                                                                        id: asset.id,
                                                                        req: 1,
                                                                        img: {
                                                                            Some(ImageAsset {
                                                                                url: ad_content.image4.clone().unwrap(),
                                                                                mime: None,
                                                                                w: None,
                                                                                h: None,
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

                                                        match &result.impr_trackers {
                                                            Some(impr_trackers) => {
                                                                for event in impr_trackers {
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
                                                        match &result.clk_trackers {
                                                            Some(clk_trackers) => {
                                                                for event in clk_trackers {
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

                                                        event_vec
                                                    }
                                                },
                                            },
                                        };

                                        bids.push(bid);
                                    },
                                    None => {
                                        return Err(ResultMessage {
                                            code: 993,
                                            message: "".to_string(),
                                        });
                                    },
                                }

                                bids
                            }
                        }].to_vec())
                    },
                    None => None,
                }
            }
        };

        Ok(response)
    }

    async fn bidding_notify_win(_url: String, _win_price: i32, _next_price: i32, _iv: &String, _connection: &Connection) {

    }

    async fn bidding_notify_lose(_url: String, _lose_price: i32, _lose_reason: i32, _lose_adn_name: &String, _iv: &String, _connection: &Connection) {

    }

    fn encrypt_price(price: i32, _iv: &String, _connection: &Connection) -> String {
        price.to_string()
    }

}

fn replace_macro(orig: &String) -> String {
    let replaced = orig.clone();

    replaced
}
