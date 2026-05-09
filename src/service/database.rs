use std::{collections::HashMap, sync::{Arc, RwLock}};

use mysql::{*, prelude::*};

use crate::entity::{AntiFraud, Connection, TrafficControl};

#[derive(Clone)]
pub struct Database {
    pub conn_pool: Pool,

    // connection map: vendor_port => connection list
    pub cma: Arc<RwLock<HashMap<i32, Vec<Connection>>>>,

    // connection map: id => connection
    pub cla: Arc<RwLock<HashMap<i32, Connection>>>,

    // client port map: tag_id => (id, tag_id, mode)
    pub cpla: Arc<RwLock<HashMap<String, (i32, String, i32)>>>,

    // vendor port map: tag_id => (id, tag_id, mode)
    pub vpla: Arc<RwLock<HashMap<String, (i32, String, i32)>>>,

    // traffic control map: client_port + "|" + vendor_port + "|" + bundle => tc[]
    pub tcla: Arc<RwLock<HashMap<String, Vec<TrafficControl>>>>,

    // anti fraud map: client_port => af[]
    pub afla: Arc<RwLock<HashMap<String, Vec<AntiFraud>>>>,

}

impl Database {

    pub fn new(db_url: &str) -> Self {
        Self {
            conn_pool: Pool::new(db_url).unwrap(),
            cma: Arc::new(RwLock::new(HashMap::<i32, Vec<Connection>>::new())),
            cla: Arc::new(RwLock::new(HashMap::<i32, Connection>::new())),
            cpla: Arc::new(RwLock::new(HashMap::<String, (i32, String, i32)>::new())),
            vpla: Arc::new(RwLock::new(HashMap::<String, (i32, String, i32)>::new())),
            tcla: Arc::new(RwLock::new(HashMap::<String, Vec<TrafficControl>>::new())),
            afla: Arc::new(RwLock::new(HashMap::<String, Vec<AntiFraud>>::new())),
        }
    }

    pub fn get_connections(&self) {
        // use query_map if there's no boolean type in result
        // otherwise, use query_iter

        let mut conn = match self.conn_pool.get_conn() {
            Ok(conn) => {
                conn
            },
            Err(_) => {
                return;
            }
        };

        let cps = conn.query_map(
            "SELECT ad_client_port.id, ad_client_port.tag_id, ad_client_port.mode
            FROM ad_client_port;",
            | (id, tag_id, mode)
                : (i32, String, i32)
            | (id, tag_id, mode),
        ).unwrap();

        let cpll = self.cpla.clone();
        let mut cpl = cpll.write().unwrap();
        cpl.clear();
        for cp in cps.iter() {
            cpl.insert(cp.1.clone(), cp.clone());
        }

        let vps = conn.query_map(
            "SELECT ad_vendor_port.id, ad_vendor_port.tag_id, ad_vendor_port.mode
            FROM ad_vendor_port;",
            | (id, tag_id, mode)
                : (i32, String, i32)
            | (id, tag_id, mode),
        ).unwrap();

        let vpll = self.vpla.clone();
        let mut vpl = vpll.write().unwrap();
        vpl.clear();
        for vp in vps.iter() {
            vpl.insert(vp.1.clone(), vp.clone());
        }

        let mut connections = Vec::<Connection>::new();

        let result = conn.query_iter(
            "SELECT
                ad_connection.id,
                ad_client.code,
                ad_client_port.id,
                ad_client_port.tag_id,
                ad_client_port.apppackage,
                ad_client_port.appname,
                ad_client_port.mode,
                ad_client_port.ekey,
                ad_client_port.ikey,
                ad_client_port.filter,
                ad_vendor_port.id,
                ad_vendor_port.mode,
                ad_vendor.ekey,
                ad_vendor.ikey,
                ad_connection.test,
                ad_vendor_port.timeout,
                ad_connection.priority,
                ad_connection.upstream_ratio,
                ad_connection.rebate_ratio,
                ad_connection.downstream_ratio,
                ad_connection.default_price
            FROM ad_connection, ad_client, ad_client_port, ad_vendor, ad_vendor_media, ad_vendor_port
            WHERE ad_connection.enabled AND NOT ad_connection.deleted
            AND ad_connection.valid_from <= NOW()
            AND ad_connection.valid_to >= NOW()
            AND ad_connection.client_port_id = ad_client_port.id
            AND ad_client_port.mode <> 3
            AND ad_client_port.client_id = ad_client.id
            AND ad_connection.vendor_port_id = ad_vendor_port.id
            AND ad_vendor_port.mode <> 3
            AND ad_vendor_port.vendor_media_id = ad_vendor_media.id
            AND ad_vendor_media.vendor_id = ad_vendor.id;"
        ).unwrap();

        for row in result {
            let mut row = row.unwrap();
            let test: Vec<u8> = row.take(14).unwrap();
            let apppackage: Option<Value> = row.take(4);
            let appname: Option<Value> = row.take(5);
            let filter: Option<Value> = row.take(9);

            let apppackage = match apppackage {
                Some(Value::Bytes(apppackage)) => {
                    if apppackage.len() > 0 {
                        Some(String::from_utf8(apppackage).unwrap())
                    } else {
                        None
                    }
                },
                _ => None,
            };

            let appname = match appname {
                Some(Value::Bytes(appname)) => {
                    if appname.len() > 0 {
                        Some(String::from_utf8(appname).unwrap())
                    } else {
                        None
                    }
                },
                _ => None,
            };

            let rule_set = match filter {
                Some(filter) => {
                    match filter {
                        Value::Bytes(filter) => {
                            if filter.len() == 0 {
                                None
                            } else {
                                let filter = String::from_utf8(filter).unwrap();
                                let rule_set = serde_json::from_str(&filter);
                                match rule_set {
                                    Ok(rule_set) => {
                                        Some(rule_set)
                                    },
                                    Err(_) => {
                                        println!("Error parsing filter: {}", filter);
                                        None
                                    },
                                }
                            }
                        },
                        _ => None,
                    }
                },
                None => None,
            };

            let connection = Connection {
                id: row.take(0).unwrap(),
                client_code: row.take(1).unwrap(),
                client_port: row.take(2).unwrap(),
                client_tag_id: row.take(3).unwrap(),
                client_media_apppackage: apppackage,
                client_media_appname: appname,
                client_mode: row.take(6).unwrap(),
                client_ekey: row.take(7).unwrap(),
                client_ikey: row.take(8).unwrap(),
                filter: rule_set.clone(),
                vendor_port: row.take(10).unwrap(),
                vendor_mode: row.take(11).unwrap(),
                vendor_ekey: row.take(12).unwrap(),
                vendor_ikey: row.take(13).unwrap(),
                test: test[0] == 1,
                timeout: row.take(15).unwrap(),
                priority: row.take(16).unwrap(),
                upstream_ratio: row.take(17).unwrap(),
                rebate_ratio: row.take(18).unwrap(),
                downstream_ratio: row.take(19).unwrap(),
                default_price: row.take(20).unwrap(),
            };
            connections.push(connection);
        }

        let cml = self.cma.clone();
        let mut cm = cml.write().unwrap();
        cm.clear();
        for connection in connections.iter() {
            if !cm.contains_key(&connection.vendor_port) {
                cm.insert(connection.vendor_port.clone(), Vec::<Connection>::new());
            }
            let mut map: Vec<Connection> = cm.get(&connection.vendor_port).unwrap().to_vec();
            map.push(connection.clone());
            cm.insert(connection.vendor_port.clone(), map);
        }

        let cll = self.cla.clone();
        let mut cl = cll.write().unwrap();
        cl.clear();
        for connection in connections.iter() {
            cl.insert(connection.id, connection.clone());
        }

        let tcs = conn.query_map(
            "SELECT
                ad_traffic_control.client_port,
                ad_traffic_control.vendor_port,
                ad_traffic_control.bundle,
                ad_traffic_control.indicator,
                ad_traffic_control.period,
                ad_traffic_control.limitation
            FROM ad_traffic_control;",
            | (client_port, vendor_port, bundle, indicator, period, limitation)
                : (i32, i32, String, i32, i32, i64)
            | (client_port, vendor_port, bundle, indicator, period, limitation),
        ).unwrap();

        let tcll = self.tcla.clone();
        let mut tcl = tcll.write().unwrap();
        tcl.clear();
        for tc in tcs.iter() {
            let traffic_control = TrafficControl {
                client_port: tc.0,
                vendor_port: tc.1,
                bundle: tc.2.clone(),
                indicator: tc.3,
                period: tc.4,
                limitation: tc.5,
            };
            let key = format!("{}|{}|{}", traffic_control.client_port, traffic_control.vendor_port, traffic_control.bundle);
            if !tcl.contains_key(&key) {
                tcl.insert(key.clone(), Vec::<TrafficControl>::new());
            }
            tcl.get_mut(&key).unwrap().push(traffic_control);
        }

        let afs = conn.query_map(
            "SELECT
                ad_anti_fraud.client_port,
                ad_anti_fraud.rule,
                ad_anti_fraud.period,
                ad_anti_fraud.limitation
            FROM ad_anti_fraud, ad_anti_fraud_rule
            WHERE ad_anti_fraud.rule = ad_anti_fraud_rule.code
            AND ad_anti_fraud_rule.enabled;",
            | (client_port, rule, period, limitation)
                : (i32, String, i32, f64)
            | (client_port, rule, period, limitation),
        ).unwrap();

        let afll = self.afla.clone();
        let mut afl = afll.write().unwrap();
        afl.clear();
        for af in afs.iter() {
            let anti_fraud = AntiFraud {
                client_port: af.0,
                rule: af.1.clone(),
                period: af.2,
                limitation: af.3,
            };
            let key = format!("{}", anti_fraud.client_port);
            if !afl.contains_key(&key) {
                afl.insert(key.clone(), Vec::<AntiFraud>::new());
            }
            afl.get_mut(&key).unwrap().push(anti_fraud);
        }
    }

}
