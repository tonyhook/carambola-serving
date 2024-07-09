use std::{collections::HashMap, sync::{Arc, RwLock}};

use mysql::{*, prelude::*};

use crate::entity::{Configuration, Connection};

#[derive(Clone)]
pub struct Database {
    pub conn_pool: Pool,

    pub cma: Arc<RwLock<HashMap<i32, Vec<Connection>>>>,
    pub cpla: Arc<RwLock<HashMap<String, (i32, String, i32)>>>,
    pub vpla: Arc<RwLock<HashMap<String, (i32, String, i32)>>>,
    pub cla: Arc<RwLock<HashMap<i32, Connection>>>,
}

impl Database {

    pub fn new(db_url: &str) -> Self {
        Self {
            conn_pool: Pool::new(db_url).unwrap(),
            cma: Arc::new(RwLock::new(HashMap::<i32, Vec<Connection>>::new())),
            cpla: Arc::new(RwLock::new(HashMap::<String, (i32, String, i32)>::new())),
            vpla: Arc::new(RwLock::new(HashMap::<String, (i32, String, i32)>::new())),
            cla: Arc::new(RwLock::new(HashMap::<i32, Connection>::new())),
        }
    }

    pub fn get_connections(&self) {
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

        let mut configurations = Vec::<Configuration>::new();

        let result = conn.query_iter(
            "SELECT
                id,
                log_transaction,
                limit_request_frequency,
                af_ip_frequency_hourly,
                af_ip_frequency_daily,
                af_id_frequency_hourly,
                af_id_frequency_daily,
                af_ua_per_id_hourly,
                af_ua_per_id_daily,
                af_ip_per_id_hourly,
                af_ip_per_id_daily
            FROM ad_configuration",
        ).unwrap();

        for row in result {
            let mut row = row.unwrap();
            let log_transaction: Vec<u8> = row.take(1).unwrap();

            let configuration = Configuration {
                id: row.take(0).unwrap(),
                log_transaction: log_transaction[0] == 1,
                limit_request_frequency: row.take(2).unwrap(),
                af_ip_frequency_hourly: row.take(3).unwrap(),
                af_ip_frequency_daily: row.take(4).unwrap(),
                af_id_frequency_hourly: row.take(5).unwrap(),
                af_id_frequency_daily: row.take(6).unwrap(),
                af_ua_per_id_hourly: row.take(7).unwrap(),
                af_ua_per_id_daily: row.take(8).unwrap(),
                af_ip_per_id_hourly: row.take(9).unwrap(),
                af_ip_per_id_daily: row.take(10).unwrap()
            };

            configurations.push(configuration);
        }

        // let configurations = conn.query_map(
        //     "SELECT
        //         id,
        //         log_transaction,
        //         limit_request_frequency,
        //         af_ip_frequency_hourly,
        //         af_ip_frequency_daily,
        //         af_id_frequency_hourly,
        //         af_id_frequency_daily,
        //         af_ua_per_id_hourly,
        //         af_ua_per_id_daily,
        //         af_ip_per_id_hourly,
        //         af_ip_per_id_daily
        //     FROM ad_configuration",
        //     | (id, log_transaction, limit_request_frequency, af_ip_frequency_hourly, af_ip_frequency_daily, af_id_frequency_hourly, af_id_frequency_daily, af_ua_per_id_hourly, af_ua_per_id_daily, af_ip_per_id_hourly, af_ip_per_id_daily)
        //         : (i32, Vec<u8>, i32, i32, i32, i32, i32, i32, i32, i32, i32)
        //     | Configuration {
        //         id,
        //         log_transaction: log_transaction[0] == 1,
        //         limit_request_frequency,
        //         af_ip_frequency_hourly,
        //         af_ip_frequency_daily,
        //         af_id_frequency_hourly,
        //         af_id_frequency_daily,
        //         af_ua_per_id_hourly,
        //         af_ua_per_id_daily,
        //         af_ip_per_id_hourly,
        //         af_ip_per_id_daily
        //     }
        // ).unwrap();

        let mut connections = Vec::<Connection>::new();

        let result = conn.query_iter(
            "SELECT
                ad_connection.id,
                ad_client.code,
                ad_client_port.id,
                ad_client_port.tag_id,
                ad_client_port.mode,
                ad_client_port.ekey,
                ad_client_port.ikey,
                ad_connection.test,
                ad_vendor_port.id,
                ad_vendor_port.mode,
                ad_vendor.ekey,
                ad_vendor.ikey,
                ad_vendor_port.timeout,
                ad_connection.priority,
                ad_connection.cost_ratio,
                ad_connection.default_price,
                configuration_id
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
            let test: Vec<u8> = row.take(7).unwrap();
            let connection_id: i32 = row.take(16).unwrap();

            let connection = Connection {
                id: row.take(0).unwrap(),
                client_code: row.take(1).unwrap(),
                client_port: row.take(2).unwrap(),
                client_tag_id: row.take(3).unwrap(),
                client_mode: row.take(4).unwrap(),
                client_ekey: row.take(5).unwrap(),
                client_ikey: row.take(6).unwrap(),
                test: test[0] == 1,
                vendor_port: row.take(8).unwrap(),
                vendor_mode: row.take(9).unwrap(),
                vendor_ekey: row.take(10).unwrap(),
                vendor_ikey: row.take(11).unwrap(),
                timeout: row.take(12).unwrap(),
                priority: row.take(13).unwrap(),
                cost_ratio: row.take(14).unwrap(),
                default_price: row.take(15).unwrap(),
                configuration: {
                    *configurations.iter().find(|c|c.id == connection_id).unwrap()
                },
            };
            connections.push(connection);
        }

        // let connections = conn.query_map(
        //     "SELECT
        //         ad_connection.id,
        //         ad_client.code,
        //         ad_client_port.id,
        //         ad_client_port.tag_id,
        //         ad_client_port.mode,
        //         ad_client_port.ekey,
        //         ad_client_port.ikey,
        //         ad_connection.test,
        //         ad_vendor_port.id,
        //         ad_vendor_port.mode,
        //         ad_vendor.ekey,
        //         ad_vendor.ikey,
        //         ad_vendor_port.timeout,
        //         ad_connection.priority,
        //         ad_connection.cost_ratio,
        //         ad_connection.default_price,
        //         configuration_id
        //     FROM ad_connection, ad_client, ad_client_port, ad_vendor, ad_vendor_media, ad_vendor_port
        //     WHERE ad_connection.enabled AND NOT ad_connection.deleted
        //     AND ad_connection.valid_from <= NOW()
        //     AND ad_connection.valid_to >= NOW()
        //     AND ad_connection.client_port_id = ad_client_port.id
        //     AND ad_client_port.client_id = ad_client.id
        //     AND ad_connection.vendor_port_id = ad_vendor_port.id
        //     AND ad_vendor_port.vendor_media_id = ad_vendor_media.id
        //     AND ad_vendor_media.vendor_id = ad_vendor.id;",
        //     | (id, client_code, client_port, client_tag_id, client_mode, client_ekey, client_ikey, test, vendor_port, vendor_mode, vendor_ekey, vendor_ikey, timeout, priority, cost_ratio, default_price, configuration_id)
        //         : (i32, String, i32, String, i32, String, String, Vec<u8>, i32, i32, String, String, u64, i32, f64, i32, i32)
        //     | Connection {
        //         id,
        //         client_code,
        //         client_port,
        //         client_tag_id,
        //         client_mode,
        //         client_ekey,
        //         client_ikey,
        //         test: test[0] == 1,
        //         vendor_port,
        //         vendor_mode,
        //         vendor_ekey,
        //         vendor_ikey,
        //         timeout,
        //         priority,
        //         cost_ratio,
        //         default_price,
        //         configuration: {
        //             *configurations.iter().find(|c|c.id == configuration_id).unwrap()
        //         },
        //     }
        // ).unwrap();

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
    }

}
