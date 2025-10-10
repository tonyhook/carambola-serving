use std::time::Duration;

use urlencoding::encode;

use crate::{protocol::*, Assets, Cache, Client, Connection, HttpPool, Identifiers, Price, ResultMessage};

pub mod caid;
pub mod data;
pub mod request;
pub mod response;

pub use caid::RuiangCaid;
pub use data::RuiangData;
pub use request::RuiangRequest;
pub use response::RuiangResponse;

pub struct Ruiang {

}

impl Client for Ruiang {

    async fn request(request: &Request, connection: &Connection, pool: &HttpPool, cache: &Cache) -> Result<Response, ResultMessage> {
        let request_id = cache.get_sequence();

        let mut assets = Assets::new(request);
        let identifiers = Identifiers::new(request);

        let request_ruiang = RuiangRequest {
            id: {
                request_id.to_string()
            },
            ver: {
                "2.0".to_string()
            },
            ad_id: {
                connection.client_tag_id.clone()
            },
            ad_width: {
                match request.item[0].spec.display.w {
                    Some(w) => w,
                    None => 0,
                }
            },
            ad_height: {
                match request.item[0].spec.display.h {
                    Some(h) => h,
                    None => 0,
                }
            },
            app_name: {
                match &request.context.app {
                    Some(app) => app.name.clone(),
                    None => "".to_string(),
                }
            },
            pkg_name: {
                match &request.context.app {
                    Some(app) => {
                        match &app.bundle {
                            Some(bundle) => bundle.clone(),
                            None => "".to_string(),
                        }
                    },
                    None => "".to_string(),
                }
            },
            apv: {
                match &request.context.app {
                    Some(app) => {
                        match &app.ver {
                            Some(ver) => ver.clone(),
                            None => "".to_string(),
                        }
                    },
                    None => "".to_string(),
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
            density: {
                match request.context.device.pxratio {
                    Some(pxratio) => pxratio,
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.device.pxratio is required for upstream".to_string(),
                    }),
                }
            },
            os: {
                match request.context.device.os {
                    Some(2) => 1,
                    Some(13) => 2,
                    _ => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.device.os should be 2/13 for upstream".to_string(),
                    }),
                }
            },
            os_ver: {
                match &request.context.device.osv {
                    Some(osv) => osv.clone(),
                    None => "".to_string(),
                }
            },
            api_level: {
                request.context.device.oslevel.clone()
            },
            dev_width: {
                match request.context.device.w {
                    Some(w) => w,
                    None => 0,
                }
            },
            dev_height: {
                match request.context.device.h {
                    Some(h) => h,
                    None => 0,
                }
            },
            dev_orient: {
                match request.context.device.orientation {
                    Some(501) => 0,
                    Some(502) => 1,
                    _ => 0,
                }
            },
            appstore_ver: {
                match &request.context.device.storev {
                    Some(storev) => storev.clone(),
                    _ => "".to_string(),
                }
            },
            hmscore: {
                match &request.context.device.hmsv {
                    Some(hmsv) => hmsv.clone(),
                    _ => "".to_string(),
                }
            },
            ui_ver: {
                match &request.context.device.uiv {
                    Some(uiv) => uiv.clone(),
                    _ => "".to_string(),
                }
            },
            dev_type: {
                match request.context.device.devicetype {
                    Some(1) => 0,
                    Some(4) => 0,
                    Some(5) => 1,
                    _ => 0,
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
            operator_nop: {
                match &request.context.device.carrier {
                    Some(carrier) => {
                        match carrier.as_str() {
                            "cmcc" => "46000".to_string(),
                            "unicom" => "46001".to_string(),
                            "telecom" => "46003".to_string(),
                            _ => "0".to_string(),
                        }
                    },
                    None => "0".to_string(),
                }
            },
            net_type: {
                match &request.context.device.contype {
                    Some(2) => 3,
                    Some(4) => 1,
                    Some(5) => 2,
                    Some(6) => 5,
                    Some(7) => 6,
                    _ => 0,
                }
            },
            imsi: {
                match identifiers.get_id(503, 0) {
                    Some(uid) => uid.id.clone(),
                    None => "".to_string(),
                }
            },
            mac: {
                match identifiers.get_id(511, 0) {
                    Some(uid) => uid.id.clone(),
                    None => "".to_string(),
                }
            },
            bid_price: {
                Some(Price::to_client(connection, request.item[0].flr.map(f64::from)) as i32)
            },
            imei: {
                match identifiers.get_id(501, 0) {
                    Some(uid) => uid.id.clone(),
                    None => "".to_string(),
                }
            },
            imei_md5: {
                match identifiers.get_id(502, 0) {
                    Some(uid) => uid.id.clone(),
                    None => "".to_string(),
                }
            },
            android_id: {
                match identifiers.get_id(509, 0) {
                    Some(uid) => uid.id.clone(),
                    None => "".to_string(),
                }
            },
            oaid: {
                match identifiers.get_id(505, 0) {
                    Some(uid) => uid.id.clone(),
                    None => "".to_string(),
                }
            },
            user_agent: {
                request.context.device.ua.clone()
            },
            idfa: {
                match identifiers.get_id(507, 0) {
                    Some(uid) => uid.id.clone(),
                    None => "".to_string(),
                }
            },
            idfa_md5: {
                match identifiers.get_id(508, 0) {
                    Some(uid) => uid.id.clone(),
                    None => "".to_string(),
                }
            },
            idfv: {
                match identifiers.get_id(515, 0) {
                    Some(uid) => uid.id.clone(),
                    None => "".to_string(),
                }
            },
            udid: {
                "".to_string()
            },
            ip: {
                match &request.context.device.ip {
                    Some(ip) => ip.clone(),
                    None => "".to_string(),
                }
            },
            ipv6: {
                match &request.context.device.ipv6 {
                    Some(ipv6) => ipv6.clone(),
                    None => "".to_string(),
                }
            },
            ppi: {
                match request.context.device.ppi {
                    Some(ppi) => ppi,
                    None => return Err(ResultMessage {
                        code: 998,
                        message: "request.context.device.ppi is required for upstream".to_string(),
                    }),
                }
            },
            lon: {
                match &request.context.device.geo {
                    Some(geo) => {
                        match geo.lon {
                            Some(lon) => lon.to_string(),
                            None => "".to_string(),
                        }
                    },
                    None => "".to_string(),
                }
            },
            lat: {
                match &request.context.device.geo {
                    Some(geo) => {
                        match geo.lat {
                            Some(lat) => lat.to_string(),
                            None => "".to_string(),
                        }
                    },
                    None => "".to_string(),
                }
            },
            boot_mark: {
                match &request.context.device.bootmark {
                    Some(bootmark) => bootmark.clone(),
                    None => "".to_string(),
                }
            },
            update_mark: {
                match &request.context.device.updatemark {
                    Some(updatemark) => updatemark.clone(),
                    None => "".to_string(),
                }
            },
            serialno: {
                match &request.context.device.serial {
                    Some(serial) => serial.clone(),
                    None => "".to_string(),
                }
            },
            rom_ver: {
                match &request.context.device.romv {
                    Some(romv) => romv.clone(),
                    None => "".to_string(),
                }
            },
            sys_compiling_time: {
                match &request.context.device.romtime {
                    Some(romtime) => romtime.clone(),
                    None => "".to_string(),
                }
            },
            dev_name_md5: {
                match identifiers.get_id(528, 0) {
                    Some(uid) => uid.id.clone(),
                    None => "".to_string(),
                }
            },
            startup_time: {
                match &request.context.device.boottime {
                    Some(boottime) => boottime.split(".").nth(0).unwrap().to_string(),
                    None => "".to_string(),
                }
            },
            upgrade_time: {
                match &request.context.device.updatetime {
                    Some(updatetime) => updatetime.split(".").nth(0).unwrap().to_string(),
                    None => "".to_string(),
                }
            },
            sys_start_up_nano_time: {
                match &request.context.device.boottime {
                    Some(boottime) => boottime.clone(),
                    None => "".to_string(),
                }
            },
            sys_update_nano_time: {
                match &request.context.device.updatetime {
                    Some(updatetime) => updatetime.clone(),
                    None => "".to_string(),
                }
            },
            dev_init_nano_time: {
                match &request.context.device.inittime {
                    Some(inittime) => inittime.clone(),
                    None => "".to_string(),
                }
            },
            timezone: {
                match &request.context.device.timezone {
                    Some(timezone) => timezone.clone(),
                    None => "".to_string(),
                }
            },
            hardware_model: {
                match &request.context.device.hwmodel {
                    Some(hwmodel) => hwmodel.clone(),
                    None => "".to_string(),
                }
            },
            hardware_machine: {
                match &request.context.device.hwmachine {
                    Some(hwmachine) => hwmachine.clone(),
                    None => "".to_string(),
                }
            },
            memory: {
                match request.context.device.sysmemory {
                    Some(sysmemory) => sysmemory,
                    None => 0,
                }
            },
            hard_disk: {
                match request.context.device.sysdisksize {
                    Some(sysdisksize) => sysdisksize,
                    None => 0,
                }
            },
            cpu_num: {
                match request.context.device.syscpu {
                    Some(syscpu) => syscpu,
                    None => 0,
                }
            },
            cpu_freq: {
                match request.context.device.syscpufreq {
                    Some(syscpufreq) => syscpufreq,
                    None => 0.0,
                }
            },
            idfa_policy: {
                match request.context.device.lmt {
                    Some(lmt) => lmt,
                    None => 0,
                }
            },
            battery_status: {
                match request.context.device.sysbatterystatus {
                    Some(sysbatterystatus) => sysbatterystatus,
                    None => 0,
                }
            },
            battery_power: {
                match request.context.device.sysbatterypower {
                    Some(sysbatterypower) => sysbatterypower,
                    None => 0,
                }
            },
            dev_file_time: {
                match &request.context.device.inittime {
                    Some(inittime) => inittime.split(".").nth(0).unwrap().to_string(),
                    None => "".to_string(),
                }
            },
            country_code: {
                match &request.context.device.country {
                    Some(country) => country.clone(),
                    None => "".to_string(),
                }
            },
            language: {
                match &request.context.device.lang {
                    Some(lang) => lang.clone(),
                    None => "".to_string(),
                }
            },
            paid: {
                match identifiers.get_id(519, 0) {
                    Some(uid) => Some(uid.id.clone()),
                    None => None,
                }
            },
            pkgs: {
                match &request.context.device.app {
                    Some(app) => app.split(",").map(|s| s.to_string()).collect(),
                    None => [].to_vec(),
                }
            },
            caids: {
                let mut caids = vec![];

                match identifiers.get_id(513, 0) {
                    Some(uid) => {
                        caids.push(RuiangCaid {
                            caid: {
                                uid.id.clone()
                            },
                            caid_version: {
                                match &uid.ver {
                                    Some(ver) => ver.clone(),
                                    None => "".to_string(),
                                }
                            },
                        });
                    },
                    None => (),
                };

                match identifiers.get_id(513, 1) {
                    Some(uid) => {
                        caids.push(RuiangCaid {
                            caid: {
                                uid.id.clone()
                            },
                            caid_version: {
                                match &uid.ver {
                                    Some(ver) => ver.clone(),
                                    None => "".to_string(),
                                }
                            },
                        });
                    },
                    None => (),
                };

                caids
            },
        };

        let response_ruiang: RuiangResponse;

        let client = {
            let pool_ruiang_lock = pool.pool_ruiang.clone();
            let pool_ruiang = pool_ruiang_lock.read().unwrap();
            pool_ruiang.clone()
        };
        let response_ruiang_raw = client.post("http://dk.ruiaiad.com/td/ad")
            .json(&request_ruiang)
            .header("Accept-Encoding", "gzip")
            .header("Connection", "keep-alive")
            .header("Content-Type", "application/json")
            .timeout(Duration::from_millis(connection.timeout))
            .send().await;

        match response_ruiang_raw {
            Ok(response_ruiang_raw) => {
                let status = response_ruiang_raw.status();
                if status == 200 {
                    match response_ruiang_raw.text().await {
                        Ok(text) => {
                            match serde_json::from_str::<RuiangResponse>(&text) {
                                Ok(json) => {
                                    if json.code == 200 {
                                        response_ruiang = json;
                                    } else if json.code == 500 {
                                        return Err(ResultMessage {
                                            code: 993,
                                            message: "".to_string(),
                                        });
                                    } else {
                                        return Err(ResultMessage {
                                            code: 994,
                                            message: format!("upstream error {}: {}", json.code, json.message),
                                        });
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
                        message: {
                            match response_ruiang_raw.text().await {
                                Ok(text) => format!("upstream error {}: {}", status, text),
                                Err(_) => format!("upstream error {}", status),
                            }
                        },
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
                Some([Seatbid {
                    bid: {
                        let mut bids = vec![];

                        for data in response_ruiang.data.unwrap() {
                            let link_asset = LinkAsset {
                                linktype: {
                                    match data.act {
                                        2 => 1,
                                        1 => 2,
                                        3 => 3,
                                        _ => 1,
                                    }
                                },
                                universallink: None,
                                storeid: None,
                                deeplink: {
                                    match &data.dp_url {
                                        Some(dp_url) => Some(replace_macro(&dp_url)),
                                        None => None,
                                    }
                                },
                                quickapplink: None,
                                wechatmppath: None,
                                wechatmpid: None,
                                marketurl: None,
                                downloadurl: None,
                                url: {
                                    replace_macro(&data.clk_url)
                                },
                                urlfb: None,
                            };

                            let bid = Bid {
                                id: Some(request_id.to_string()),
                                item: request.item[0].id.clone(),
                                price: { // update later
                                    if data.bid_price > 0 {
                                        data.bid_price
                                    } else {
                                        connection.default_price
                                    }
                                },
                                burl: {
                                    let mut burl = Vec::<String>::new();
                                    for notice in &data.notice_trace {
                                        let mut nurl = notice.clone();
                                        nurl = nurl.replace("__PRICE__", "__WIN_PRICE__");
                                        burl.push(replace_macro(&nurl));
                                    }
                                    Some(burl)
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
                                                if data.imgs.len() > 0 {
                                                    Some(Banner {
                                                        img: data.imgs[0].clone(),
                                                        link: Some(link_asset.clone()),
                                                    })
                                                } else {
                                                    None
                                                }
                                            } else {
                                                None
                                            }
                                        },
                                        native: {
                                            if assets.get_asset_total_size() > 0 {
                                                let mut asset_vec = vec![];

                                                if data.imgs.len() > 0 {
                                                    if assets.get_asset_size("img") > 0 {
                                                        for image_url in &data.imgs {
                                                            asset_vec.push(Asset {
                                                                id: assets.consume_asset("img"),
                                                                req: 1,
                                                                img: Some(ImageAsset {
                                                                    url: image_url.clone(),
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
                                                    if assets.get_asset_size("thumb") > 0 {
                                                        for image_url in &data.imgs {
                                                            asset_vec.push(Asset {
                                                                id: assets.consume_asset("thumb"),
                                                                req: 1,
                                                                img: Some(ImageAsset {
                                                                    url: image_url.clone(),
                                                                    mime: None,
                                                                    w: None,
                                                                    h: None,
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
                                                }

                                                if data.title.len() > 0 {
                                                    asset_vec.push(Asset {
                                                        id: assets.consume_asset("title"),
                                                        req: 1,
                                                        title: Some(TitleAsset {
                                                            text: data.title.clone(),
                                                            subtitle: None,
                                                            desc: Some(data.atx.clone()),
                                                            len: Some(data.title.len() as i32),
                                                        }),
                                                        img: None,
                                                        video: None,
                                                        data: None,
                                                        html: None,
                                                        app: None,
                                                    });
                                                }

                                                if data.bt_txt.len() > 0 {
                                                    asset_vec.push(Asset {
                                                        id: assets.consume_asset("data#ctatext"),
                                                        req: 1,
                                                        title: None,
                                                        img: None,
                                                        video: None,
                                                        data: Some(DataAsset {
                                                            value: data.bt_txt.clone(),
                                                            len: None,
                                                            datatype: Some(12),
                                                        }),
                                                        html: None,
                                                        app: None,
                                                    });
                                                }

                                                if data.appname.len() > 0 {
                                                    asset_vec.push(Asset {
                                                        id: assets.consume_asset("app"),
                                                        req: 0,
                                                        app: Some(AppAsset {
                                                            name: data.appname.clone(),
                                                            desc: None,
                                                            descurl: None,
                                                            domain: None,
                                                            bundle: data.pack.clone(),
                                                            ver: None,
                                                            developer: None,
                                                            icon: None,
                                                            storeid: None,
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
                                                        title: None,
                                                        img: None,
                                                        video: None,
                                                        data: None,
                                                        html: None,
                                                    });
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

                                            for event in &data.show_trace {
                                                event_vec.push(Event {
                                                    eventtype: 501,
                                                    method: 1,
                                                    url: replace_macro(event),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                            for event in &data.clk_trace {
                                                event_vec.push(Event {
                                                    eventtype: 502,
                                                    method: 1,
                                                    url: replace_macro(event),
                                                    header: None,
                                                    content: None,
                                                });
                                            }
                                            match &data.dp_try_trace {
                                                Some(dp_try_trace) => {
                                                    for event in dp_try_trace {
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
                                            match &data.dp_suc_trace {
                                                Some(dp_suc_trace) => {
                                                    for event in dp_suc_trace {
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
                                            match &data.dp_fail_trace {
                                                Some(dp_fail_trace) => {
                                                    for event in dp_fail_trace {
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
                                            match &data.down_trace {
                                                Some(down_trace) => {
                                                    for event in down_trace {
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
                                            match &data.downed_trace {
                                                Some(downed_trace) => {
                                                    for event in downed_trace {
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
                                            match &data.ins_trace {
                                                Some(ins_trace) => {
                                                    for event in ins_trace {
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
                                            match &data.insed_trace {
                                                Some(insed_trace) => {
                                                    for event in insed_trace {
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
                                            match &data.ac_trace {
                                                Some(ac_trace) => {
                                                    for event in ac_trace {
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

                                            event_vec
                                        }
                                    },
                                    advertiser: None,
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
            let pool_ruiang_lock = pool.pool_ruiang.clone();
            let pool_ruiang = pool_ruiang_lock.read().unwrap();
            pool_ruiang.clone()
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

    replaced = replaced.replace("__ADOWN_X__", "__ABS_DOWN_X__");
    replaced = replaced.replace("__ADOWN_Y__", "__ABS_DOWN_Y__");
    replaced = replaced.replace("__AUP_X__", "__ABS_UP_X__");
    replaced = replaced.replace("__AUP_Y__", "__ABS_UP_Y__");

    replaced = replaced.replace("__DISPLAY_LUX__", "__LT_X__");
    replaced = replaced.replace("__DISPLAY_LUY__", "__LT_Y__");
    replaced = replaced.replace("__DISPLAY_RDX__", "__RB_X__");
    replaced = replaced.replace("__DISPLAY_RDY__", "__RB_Y__");

    replaced = replaced.replace("__BUTTON_LUX__", "__BUTTON_LT_X__");
    replaced = replaced.replace("__BUTTON_LUY__", "__BUTTON_LT_Y__");
    replaced = replaced.replace("__BUTTON_RDX__", "__BUTTON_RB_X__");
    replaced = replaced.replace("__BUTTON_RDY__", "__BUTTON_RB_Y__");

    replaced = replaced.replace("__VIDEO_STARTTIME__", "__VIDEO_BEGIN_TIME__");
    replaced = replaced.replace("__VIDEO_ENDTIME__", "__VIDEO_END_TIME__");
    replaced = replaced.replace("__VIDEO_EVENTTIME__", "__TS__");
    replaced = replaced.replace("__VIDEO_PROCESS__", "__VIDEO_PLAY_PROGRESS__");
    replaced = replaced.replace("__VIDEO_MS_PROCESS__", "__VIDEO_PLAY_PROGRESS_S__");
    replaced = replaced.replace("__VIDEO_TIME_PROCESS__", "__VIDEO_PLAY_DURATION__");
    replaced = replaced.replace("__VIDEO_RATE__", "__VIDEO_PLAY_RATIO__");
    replaced = replaced.replace("__VIDEO_DURATION__", "__VIDEO_TIME__");
    replaced = replaced.replace("__BEHAVIOR__", "__VIDEO_PLAY_TRIGGER_0__");
    replaced = replaced.replace("__PLAY_FIRST_FRAME__", "__VIDEO_FIRST_FRAME__");
    replaced = replaced.replace("__PLAY_LAST_FRAME__", "__VIDEO_LAST_FRAME__");
    replaced = replaced.replace("__SCENE__", "__VIDEO_PLAY_SCENE__");
    replaced = replaced.replace("__PLAY_TYPE__", "__VIDEO_PLAY_TYPE__");
    replaced = replaced.replace("__VIDEO_STATUS__", "__VIDEO_PLAY_STATUS__");

    replaced = replaced.replace("__EVENT_TIME_START__", "__EVENT_TIME_START__");
    replaced = replaced.replace("__EVENT_TIME_END__", "__EVENT_TIME_END__");

    replaced = replaced.replace("__TARGET_APP_INSTALL__", "__DP_TARGET__");

    replaced
}
