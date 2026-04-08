use base64::prelude::*;
use hmac::{Hmac, Mac};
use sha1::Sha1;

use crate::{protocol::*, Cache, Client, Connection, HttpPool, Request, Response, ResultMessage};

use super::Assets;

type HmacSha1 = Hmac<Sha1>;

pub struct Dummy {

}

impl Client for Dummy {

    async fn request(request: &Request, connection: &Connection, _pool: &HttpPool, cache: &Cache) -> Result<Response, ResultMessage> {
        let request_id = cache.get_sequence();

        let assets = Assets::new(request);

        let response = Response {
            id: request.id.clone(),
            nbr: None,
            seatbid: Some([Seatbid {
                bid: {
                    let mut bids = vec![];
                    let link_asset = LinkAsset {
                        linktype: 1,
                        universallink: None,
                        storeid: None,
                        deeplink: None,
                        quickapplink: None,
                        wechatmppath: None,
                        wechatmpid: None,
                        marketurl: None,
                        downloadurl: None,
                        url: "https://www.baidu.com/".to_string(),
                        urlfb: None,
                    };
                    let bid = Bid {
                        id: Some(request_id.to_string()),
                        item: request.item[0].id.clone(),
                        price: { // update later
                            connection.default_price
                        },
                        burl: Some(["".to_string()].to_vec()),
                        lurl: Some(["".to_string()].to_vec()),
                        media: Ad {
                            id: request_id.to_string(),
                            display: Display {
                                w: request.item[0].spec.display.w,
                                h: request.item[0].spec.display.h,
                                banner: {
                                    if assets.get_banner_size() > 0 {
                                        Some(Banner {
                                            img: "https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png".to_string(),
                                            link: Some(link_asset.clone()),
                                        })
                                    } else {
                                        None
                                    }
                                },
                                native: {
                                    if assets.get_asset_total_size() > 0 {
                                        let mut asset_vec = vec![];

                                        match assets.get_current_asset("title") {
                                            Some(asset) => {
                                                asset_vec.push(Asset {
                                                    id: asset.id,
                                                    req: 1,
                                                    title: Some(TitleAsset {
                                                        text: "test".to_string(),
                                                        subtitle: None,
                                                        desc: Some("test".to_string()),
                                                        len: Some(4),
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
                                        match assets.get_current_asset("img") {
                                            Some(asset) => {
                                                asset_vec.push(Asset {
                                                    id: asset.id,
                                                    req: 1,
                                                    img: Some(ImageAsset {
                                                        url: "https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png".to_string(),
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
                                        match assets.get_current_asset("video") {
                                            Some(video) => {
                                                asset_vec.push(Asset {
                                                    id: video.id,
                                                    req: 1,
                                                    video: Some(VideoAsset {
                                                        url: "https://media.w3.org/2010/05/sintel/trailer.mp4".to_string(),
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

                                                match &video.video.clone().unwrap().comp {
                                                    Some(comp) => {
                                                        for companion in comp {
                                                            match &companion.display.nativefmt {
                                                                Some (nativefmt_embedded) => {
                                                                    for asset in &nativefmt_embedded.asset {
                                                                        if asset.title.is_some() {
                                                                            asset_vec.push(Asset {
                                                                                id: asset.id,
                                                                                req: 1,
                                                                                title: Some(TitleAsset {
                                                                                    text: "test".to_string(),
                                                                                    subtitle: None,
                                                                                    desc: Some("test".to_string()),
                                                                                    len: Some(4),
                                                                                }),
                                                                                img: None,
                                                                                video: None,
                                                                                data: None,
                                                                                html: None,
                                                                                app: None,
                                                                            });
                                                                        }
                                                                        if asset.img.is_some() {
                                                                            asset_vec.push(Asset {
                                                                                id: asset.id,
                                                                                req: 1,
                                                                                img: Some(ImageAsset {
                                                                                    url: "https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png".to_string(),
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
                                                                            match &asset.data {
                                                                                Some(data) => {
                                                                                    asset_vec.push(Asset {
                                                                                        id: asset.id,
                                                                                        req: 1,
                                                                                        data: Some(DataAsset {
                                                                                            value: "test".to_string(),
                                                                                            len: Some(4),
                                                                                            datatype: Some(data.datatype),
                                                                                        }),
                                                                                        title: None,
                                                                                        img: None,
                                                                                        video: None,
                                                                                        html: None,
                                                                                        app: None,
                                                                                    });
                                                                                },
                                                                                None => (),
                                                                            }
                                                                        }
                                                                    }
                                                                },
                                                                None => (),
                                                            }
                                                        }
                                                    },
                                                    None => (),
                                                }
                                            },
                                            None => (),
                                        }
                                        match assets.get_current_asset("data") {
                                            Some(data) => {
                                                asset_vec.push(Asset {
                                                    id: data.id,
                                                    req: 1,
                                                    data: Some(DataAsset {
                                                        value: "test".to_string(),
                                                        len: Some(4),
                                                        datatype: Some(data.data.clone().unwrap().datatype),
                                                    }),
                                                    title: None,
                                                    img: None,
                                                    video: None,
                                                    html: None,
                                                    app: None,
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
                                    let event_vec = vec![];

                                    event_vec
                                }
                            },
                            advertiser: None,
                            advertisericon: None,
                        },
                    };
                    bids.push(bid);
                    bids
                }
            }].to_vec()),
        };

        Ok(response)
    }

    async fn bidding_notify_win(_url: String, _win_price: i32, _next_price: i32, _iv: &String, _connection: &Connection, _pool: &HttpPool) {

    }

    async fn bidding_notify_lose(_url: String, _lose_price: i32, _lose_reason: i32, _lose_adn_name: &String, _iv: &String, _connection: &Connection, _pool: &HttpPool) {

    }

    fn encrypt_price(price: i32, iv: &String, connection: &Connection) -> String {
        let mut ekey_base64 = connection.client_tag_id.split("|").nth(1).unwrap().replace("-", "+").replace("_", "/");
        while ekey_base64.len() % 4 != 0 {
            ekey_base64.push_str("=");
        }
        let ekey = BASE64_STANDARD.decode(ekey_base64);
        let mut ikey_base64 = connection.client_tag_id.split("|").nth(2).unwrap().replace("-", "+").replace("_", "/");
        while ikey_base64.len() % 4 != 0 {
            ikey_base64.push_str("=");
        }
        let ikey = BASE64_STANDARD.decode(ikey_base64);
        if ekey.is_err() || ikey.is_err() {
            return "".to_string();
        }
        let ekey = ekey.unwrap();
        let ikey = ikey.unwrap();

        let price_bytes = u64::to_be_bytes(price as u64).to_vec();
        let iv_bytes = match iv.parse::<u128>() {
            Ok(number) => u128::to_be_bytes(number),
            Err(_) => u128::to_be_bytes(0)
        };

        let mut price_pad: Vec<u8> = [].to_vec();
        let mac_ekey = HmacSha1::new_from_slice(&ekey);
        match mac_ekey {
            Ok(mut mac) => {
                mac.update(&iv_bytes.to_vec());
                price_pad = mac.finalize().into_bytes().to_vec();
            },
            Err(_) => (),
        }

        let enc_price: Vec<u8> = price_bytes.iter()
            .zip(price_pad[0..8].iter())
            .map(|(&x1, &x2)| x1 ^ x2)
            .collect();

        let mut sig = [].to_vec();
        let mac_ikey = HmacSha1::new_from_slice(&ikey);
        match mac_ikey {
            Ok(mut mac) => {
                mac.update(&[price_bytes.to_vec(), iv_bytes.to_vec()].concat());
                sig = mac.finalize().into_bytes().to_vec();
            },
            Err(_) => (),
        }
        let signature = sig[0..4].to_vec();

        let message_base64 = BASE64_STANDARD.encode(&[iv_bytes.to_vec(), enc_price.to_vec(), signature.to_vec()].concat());

        message_base64.replace("+", "-").replace("/", "_").replace("=", "")
    }

}
