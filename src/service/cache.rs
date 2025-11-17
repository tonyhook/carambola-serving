use std::sync::Arc;

use chrono::{DateTime, Datelike, FixedOffset, Timelike, Utc};
use r2d2::Pool;
use redis::Client;

use crate::{entity::{Uid, AF_PERIOD_DAY, AF_PERIOD_HOUR, AF_PERIOD_MINUTE, AF_PERIOD_SECOND, TC_INDICATOR_COST, TC_INDICATOR_REQUEST, TC_PERIOD_DAY, TC_PERIOD_HOUR, TC_PERIOD_MINUTE, TC_PERIOD_SECOND}, EnvConfig, GLOBAL_CONFIG, NODE_ID};

pub const PERFORMANCE_NO_PROTOCOL:                   &str = "A"; // -1 as client port
pub const PERFORMANCE_BAD_PROTOCOL_VER:              &str = "B"; // -1 as client port
pub const PERFORMANCE_NO_ITEM:                       &str = "C"; // -1 as client port
pub const PERFORMANCE_NOT_REGISTERED:                &str = "D"; // -1 as client port
pub const PERFORMANCE_NO_MATCH_CONNECTION:           &str = "E"; // -1 as client port
pub const PERFORMANCE_BEYOND_VENDOR_TRAFFIC_CONTROL: &str = "F"; // -1 as client port
pub const PERFORMANCE_VENDOR_ANTI_FRAUD:             &str = "G"; // -1 as client port
pub const PERFORMANCE_NO_RESPONSE:                   &str = "H"; // -1 as client port
pub const PERFORMANCE_SHARE_SUCCESS:                 &str = "I"; // one in each request
pub const PERFORMANCE_BIDDING_SUCCESS:               &str = "J"; // one in each request

// by connection, multiple in each request
pub const PERFORMANCE_TIMEOUT:                       &str = "ZA";
pub const PERFORMANCE_REQUEST_FAILED:                &str = "ZB";
pub const PERFORMANCE_NOT_BIDDING:                   &str = "ZC";
pub const PERFORMANCE_SHARE_OK:                      &str = "ZD";
pub const PERFORMANCE_BIDDING_OK:                    &str = "ZE";
pub const PERFORMANCE_TRANS_FROM_FAILED:             &str = "ZF";
pub const PERFORMANCE_TRANS_TO_FAILED:               &str = "ZG";
pub const PERFORMANCE_UNKNOWN_CLIENT:                &str = "ZH";
pub const PERFORMANCE_BIDDING_LOSE:                  &str = "ZI";
pub const PERFORMANCE_BIDDING_WIN:                   &str = "ZJ";
pub const PERFORMANCE_BIDDING_INVALID:               &str = "ZK";
pub const PERFORMANCE_BEYOND_CLIENT_TRAFFIC_CONTROL: &str = "ZL";
pub const PERFORMANCE_LOST_KEY_FIELD:                &str = "ZM";
pub const PERFORMANCE_CLIENT_ANTI_FRAUD:             &str = "ZN";
pub const PERFORMANCE_REQUEST_REJECTED:              &str = "ZO";

#[derive(Clone)]
pub struct Cache {
    pub pw: Pool<Client>, // performance (write)
    pub lw: Pool<Client>, // response tracker list (write)
    pub nw: Pool<Client>, // notification bundle, url & cost (write)
    pub nr: Pool<Client>, // notification bundle, url & cost (read)
    pub s: Pool<Client>, // id generator
    pub tcw: Pool<Client>, // traffic control (write)
    pub tcr: Pool<Client>, // traffic control (read)
    pub afw: Pool<Client>, // anti fraud (write)
    pub afr: Pool<Client>, // anti fraud (read)
}

impl Cache {

    // performance

    pub fn new(config: &EnvConfig) -> Self {
        Self {
            pw: Pool::builder().build(redis::Client::open(config.performance_connection_write.clone()).unwrap()).unwrap(),
            lw: Pool::builder().build(redis::Client::open(config.log_connection_write.clone()).unwrap()).unwrap(),
            nw: Pool::builder().build(redis::Client::open(config.notification_connection_write.clone()).unwrap()).unwrap(),
            nr: Pool::builder().build(redis::Client::open(config.notification_connection_read.clone()).unwrap()).unwrap(),
            s: Pool::builder().build(redis::Client::open(config.idgenerator_connection.clone()).unwrap()).unwrap(),
            tcw: Pool::builder().build(redis::Client::open(config.trafficcontrol_connection_write.clone()).unwrap()).unwrap(),
            tcr: Pool::builder().build(redis::Client::open(config.trafficcontrol_connection_read.clone()).unwrap()).unwrap(),
            afw: Pool::builder().build(redis::Client::open(config.antifraud_connection_write.clone()).unwrap()).unwrap(),
            afr: Pool::builder().build(redis::Client::open(config.antifraud_connection_read.clone()).unwrap()).unwrap(),
        }
    }

    pub fn update_performance(&self, client_port: i32, vendor_port: i32, bundle: &String, event: &str) {
        let cache = self.clone();
        let bundle = Arc::new(bundle.to_string());
        let event = Arc::new(event.to_string());
        tokio::spawn({
            async move {
                cache.update_performance_async(client_port, vendor_port, &bundle, &event).await;
            }
        });
    }

    async fn update_performance_async(&self, client_port: i32, vendor_port: i32, bundle: &String, event: &str) {
        let utc: DateTime<Utc> = Utc::now();
        let hour = utc.hour();
        let minute_aligned = utc.minute() / GLOBAL_CONFIG.get().unwrap().performance_interval * GLOBAL_CONFIG.get().unwrap().performance_interval;
        let minute_fragment = utc.minute() - minute_aligned;
        let second = utc.second();

        let key = format!("P{:0>2}{:0>2}:{}:{}:{}:{}", hour, minute_aligned, client_port, vendor_port, bundle.replace(":", "_"), event);
        let expire = 14400 - minute_fragment * 60 - second - GLOBAL_CONFIG.get().unwrap().performance_interval * 60;

        let connection = self.pw.get();

        match connection {
            Ok(mut connection) => {
                let result = redis::cmd("INCR").arg(&key).query::<Option<u32>>(&mut connection);
                match result {
                    Ok(result) => {
                        match result {
                            Some(result) => {
                                if result == 1 {
                                    let _ = redis::cmd("EXPIRE").arg(&key).arg(expire).query::<Option<u32>>(&mut connection);
                                }
                            },
                            None => (),
                        }
                    },
                    Err(_) => (),
                }
            },
            Err(_) => (),
        }
    }

    // upstream response

    pub fn update_response_tracker(&self, client_port: i32, tracker: String) {
        let cache = self.clone();
        tokio::spawn({
            async move {
                cache.update_response_tracker_async(client_port, tracker).await;
            }
        });
    }

    async fn update_response_tracker_async(&self, client_port: i32, tracker: String) {
        let key = format!("LS:{}", client_port);

        let connection = self.lw.get();

        match connection {
            Ok(mut connection) => {
                let _ = redis::cmd("SET").arg(&key).arg(&tracker).query::<Option<String>>(&mut connection);
            },
            Err(_) => (),
        }
    }

    // traffic control & anti fraud

    pub fn get_traffic_control_amount(&self, time: DateTime<FixedOffset>, client_port: i32, vendor_port: i32, bundle: &String, indicator: i32, period: i32) -> i64 {
        let day = time.day();
        let hour = time.hour();
        let minute = time.minute();

        let mut indicator_code = "";
        if indicator == TC_INDICATOR_REQUEST {
            indicator_code = "R";
        }
        if indicator == TC_INDICATOR_COST {
            indicator_code = "C";
        }

        let mut key = "".to_string();
        if period == TC_PERIOD_DAY {
            key = format!("Q{}D{:0>2}:{}:{}:{}", indicator_code, day, client_port, vendor_port, bundle.replace(":", "_"));
        }
        if period == TC_PERIOD_HOUR {
            key = format!("Q{}H{:0>2}:{}:{}:{}", indicator_code, hour, client_port, vendor_port, bundle.replace(":", "_"));
        }
        if period == TC_PERIOD_MINUTE {
            key = format!("Q{}M{:0>2}{:0>2}:{}:{}:{}", indicator_code, hour, minute, client_port, vendor_port, bundle.replace(":", "_"));
        }
        if period == TC_PERIOD_SECOND {
            key = format!("Q{}S{:0>2}{:0>2}:{}:{}:{}", indicator_code, hour, minute, client_port, vendor_port, bundle.replace(":", "_"));
        }

        let connection = self.tcr.get();

        match connection {
            Ok(mut connection) => {
                let result = redis::cmd("GET").arg(&key).query::<Option<i64>>(&mut connection);
                match result {
                    Ok(amount) => {
                        match amount {
                            Some(amount) => amount,
                            None => 0,
                        }
                    },
                    Err(_) => std::i64::MAX,
                }
            },
            Err(_) => std::i64::MAX,
        }
    }

    pub fn set_traffic_control_amount(&self, time: DateTime<FixedOffset>, client_port: i32, vendor_port: i32, bundle: &String, indicator: i32, period: i32, amount: i64) {
        let cache = self.clone();
        let bundle = Arc::new(bundle.to_string());
        tokio::spawn({
            async move {
                cache.set_traffic_control_amount_async(time, client_port, vendor_port, &bundle, indicator, period, amount).await;
            }
        });
    }

    async fn set_traffic_control_amount_async(&self, time: DateTime<FixedOffset>, client_port: i32, vendor_port: i32, bundle: &String, indicator: i32, period: i32, amount: i64) {
        let day = time.day();
        let hour = time.hour();
        let minute = time.minute();

        let mut indicator_code = "";
        if indicator == TC_INDICATOR_REQUEST {
            indicator_code = "R";
        }
        if indicator == TC_INDICATOR_COST {
            indicator_code = "C";
        }

        let mut key = "".to_string();
        let mut expire = 0;
        if period == TC_PERIOD_DAY {
            key = format!("Q{}D{:0>2}:{}:{}:{}", indicator_code, day, client_port, vendor_port, bundle.replace(":", "_"));
            expire = 86400 + 60;
        }
        if period == TC_PERIOD_HOUR {
            key = format!("Q{}H{:0>2}:{}:{}:{}", indicator_code, hour, client_port, vendor_port, bundle.replace(":", "_"));
            expire = 3600 + 60;
        }
        if period == TC_PERIOD_MINUTE {
            key = format!("Q{}M{:0>2}{:0>2}:{}:{}:{}", indicator_code, hour, minute, client_port, vendor_port, bundle.replace(":", "_"));
            expire = 60 + 60;
        }
        if period == TC_PERIOD_SECOND {
            key = format!("Q{}S{:0>2}{:0>2}:{}:{}:{}", indicator_code, hour, minute, client_port, vendor_port, bundle.replace(":", "_"));
            expire = 60 + 60;
        }

        let connection = self.tcw.get();

        match connection {
            Ok(mut connection) => {
                let result = redis::cmd("INCRBY").arg(&key).arg(amount).query::<Option<i64>>(&mut connection);
                match result {
                    Ok(result) => {
                        match result {
                            Some(result) => {
                                if result == amount {
                                    let _ = redis::cmd("EXPIRE").arg(&key).arg(expire).query::<Option<u32>>(&mut connection);
                                }
                            },
                            None => (),
                        }
                    },
                    Err(_) => (),
                }
            },
            Err(_) => (),
        }
    }

    pub fn get_anti_fraud_amount(&self, time: DateTime<FixedOffset>, client_port: i32, period: i32, code: &String, identifier: &String) -> i64 {
        let day = time.day();
        let hour = time.hour();
        let minute = time.minute();
        let second = time.second();

        let mut key = "".to_string();
        if period == AF_PERIOD_DAY {
            key = format!("AD{:0>2}:{}:{}:{}", day, client_port, code, identifier);
        }
        if period == AF_PERIOD_HOUR {
            key = format!("AH{:0>2}:{}:{}:{}", hour, client_port, code, identifier);
        }
        if period == AF_PERIOD_MINUTE {
            key = format!("AM{:0>2}{:0>2}:{}:{}:{}", hour, minute, client_port, code, identifier);
        }
        if period == AF_PERIOD_SECOND {
            key = format!("AS{:0>2}{:0>2}{:0>2}:{}:{}:{}", hour, minute, second, client_port, code, identifier);
        }

        let connection = self.afr.get();

        match connection {
            Ok(mut connection) => {
                let result = redis::cmd("GET").arg(&key).query::<Option<i64>>(&mut connection);
                match result {
                    Ok(amount) => {
                        match amount {
                            Some(amount) => amount,
                            None => 0,
                        }
                    },
                    Err(_) => 0,
                }
            },
            Err(_) => 0,
        }
    }

    pub fn set_anti_fraud_amount(&self, time: DateTime<FixedOffset>, client_port: i32, period: i32, code: &String, identifier: &String, amount: i64) {
        let cache = self.clone();
        let code = Arc::new(code.to_string());
        let identifier = Arc::new(identifier.to_string());
        tokio::spawn({
            async move {
                cache.set_anti_fraud_amount_async(time, client_port, period, &code, &identifier, amount).await;
            }
        });
    }

    async fn set_anti_fraud_amount_async(&self, time: DateTime<FixedOffset>, client_port: i32, period: i32, code: &String, identifier: &String, amount: i64) {
        let day = time.day();
        let hour = time.hour();
        let minute = time.minute();
        let second = time.second();

        let mut key = "".to_string();
        let mut expire = 0;
        if period == AF_PERIOD_DAY {
            key = format!("AD{:0>2}:{}:{}:{}", day, client_port, code, identifier);
            expire = 86400 + 60;
        }
        if period == AF_PERIOD_HOUR {
            key = format!("AH{:0>2}:{}:{}:{}", hour, client_port, code, identifier);
            expire = 3600 + 60;
        }
        if period == AF_PERIOD_MINUTE {
            key = format!("AM{:0>2}{:0>2}:{}:{}:{}", hour, minute, client_port, code, identifier);
            expire = 60 + 60;
        }
        if period == AF_PERIOD_SECOND {
            key = format!("AS{:0>2}{:0>2}{:0>2}:{}:{}:{}", hour, minute, second, client_port, code, identifier);
            expire = 60;
        }

        let connection = self.afw.get();

        match connection {
            Ok(mut connection) => {
                let result = redis::cmd("INCRBY").arg(&key).arg(amount).query::<Option<i64>>(&mut connection);
                match result {
                    Ok(result) => {
                        match result {
                            Some(result) => {
                                if result == amount {
                                    let _ = redis::cmd("EXPIRE").arg(&key).arg(expire).query::<Option<u32>>(&mut connection);
                                }
                            },
                            None => (),
                        }
                    },
                    Err(_) => (),
                }
            },
            Err(_) => (),
        }
    }

    pub fn get_anti_fraud_member(&self, time: DateTime<FixedOffset>, client_port: i32, period: i32, code: &String, identifier: &String) -> i64 {
        let day = time.day();
        let hour = time.hour();
        let minute = time.minute();
        let second = time.second();

        let mut key = "".to_string();
        if period == AF_PERIOD_DAY {
            key = format!("AD{:0>2}:{}:{}:{}", day, client_port, code, identifier);
        }
        if period == AF_PERIOD_HOUR {
            key = format!("AH{:0>2}:{}:{}:{}", hour, client_port, code, identifier);
        }
        if period == AF_PERIOD_MINUTE {
            key = format!("AM{:0>2}{:0>2}:{}:{}:{}", hour, minute, client_port, code, identifier);
        }
        if period == AF_PERIOD_SECOND {
            key = format!("AS{:0>2}{:0>2}{:0>2}:{}:{}:{}", hour, minute, second, client_port, code, identifier);
        }

        let connection = self.afr.get();

        match connection {
            Ok(mut connection) => {
                let result = redis::cmd("SCARD").arg(&key).query::<Option<i64>>(&mut connection);
                match result {
                    Ok(amount) => {
                        match amount {
                            Some(amount) => amount,
                            None => 0,
                        }
                    },
                    Err(_) => std::i64::MAX,
                }
            },
            Err(_) => std::i64::MAX,
        }
    }

    pub fn set_anti_fraud_member(&self, time: DateTime<FixedOffset>, client_port: i32, period: i32, code: &String, identifier: &String, member: &String) {
        let cache = self.clone();
        let code = Arc::new(code.to_string());
        let identifier = Arc::new(identifier.to_string());
        let member = Arc::new(member.to_string());
        tokio::spawn({
            async move {
                cache.set_anti_fraud_member_async(time, client_port, period, &code, &identifier, &member).await;
            }
        });
    }

    async fn set_anti_fraud_member_async(&self, time: DateTime<FixedOffset>, client_port: i32, period: i32, code: &String, identifier: &String, member: &String) {
        let day = time.day();
        let hour = time.hour();
        let minute = time.minute();
        let second = time.second();

        let mut key = "".to_string();
        let mut expire = 0;
        if period == AF_PERIOD_DAY {
            key = format!("AD{:0>2}:{}:{}:{}", day, client_port, code, identifier);
            expire = 86400 + 60;
        }
        if period == AF_PERIOD_HOUR {
            key = format!("AH{:0>2}:{}:{}:{}", hour, client_port, code, identifier);
            expire = 3600 + 60;
        }
        if period == AF_PERIOD_MINUTE {
            key = format!("AM{:0>2}{:0>2}:{}:{}:{}", hour, minute, client_port, code, identifier);
            expire = 60 + 60;
        }
        if period == AF_PERIOD_SECOND {
            key = format!("AS{:0>2}{:0>2}{:0>2}:{}:{}:{}", hour, minute, second, client_port, code, identifier);
            expire = 60;
        }

        let connection = self.afw.get();

        match connection {
            Ok(mut connection) => {
                let result = redis::cmd("SADD").arg(&key).arg(member).query::<Option<i64>>(&mut connection);
                match result {
                    Ok(result) => {
                        match result {
                            Some(result) => {
                                if result == 1 {
                                    let _ = redis::cmd("EXPIRE").arg(&key).arg(expire).query::<Option<u32>>(&mut connection);
                                }
                            },
                            None => (),
                        }
                    },
                    Err(_) => (),
                }
            },
            Err(_) => (),
        }
    }

    // id generator

    pub fn get_sequence(&self) -> u64 {
        let mut sequence = 0;
        let connection = self.s.get();

        match connection {
            Ok(mut connection) => {
                let utc: DateTime<Utc> = Utc::now();
                let timestamp = utc.timestamp() as u64 / 60 * 60;
                let minute = utc.minute();
                let result = redis::cmd("INCR").arg(minute).query::<Option<u32>>(&mut connection);
                match result {
                    Ok(result) => {
                        match result {
                            Some(result) => {
                                if result == 1 {
                                    let _ = redis::cmd("EXPIRE").arg(minute).arg(120).query::<Option<u32>>(&mut connection);
                                }
                                sequence = timestamp << 32 | ((*NODE_ID.get().unwrap() as u64) << 24 & 0xFF000000) | (result as u64 & 0x00FFFFFF);
                            },
                            None => (),
                        }
                    },
                    Err(_) => (),
                }
            },
            Err(_) => (),
        }
        sequence
    }

    // notification bundle, url & cost

    pub fn get_bundle(&self, request_id: &String) -> Option<String> {
        let connection = self.nr.get();

        match connection {
            Ok(mut connection) => {
                let key = format!("bundle:{}", request_id);

                let result = redis::cmd("GET").arg(&key).query::<Option<String>>(&mut connection);
                match result {
                    Ok(result) => {
                        return result;
                    },
                    Err(_) => {
                        return None;
                    }
                }
            },
            Err(_) => {
                return None;
            },
        }
    }

    pub fn set_bundle(&self, request_id: &String, bundle: &String) {
        let cache = self.clone();
        let request_id = Arc::new(request_id.to_string());
        let bundle = Arc::new(bundle.to_string());
        tokio::spawn({
            async move {
                cache.set_bundle_async(&request_id, &bundle).await;
            }
        });
    }

    async fn set_bundle_async(&self, request_id: &String, bundle: &String) {
        let connection = self.nw.get();

        match connection {
            Ok(mut connection) => {
                let key = format!("bundle:{}", request_id);
                let value = format!("{}", bundle.replace(":", "_"));

                let result = redis::cmd("SET").arg(&key).arg(&value).query::<Option<String>>(&mut connection);
                match result {
                    Ok(result) => {
                        match result {
                            Some(result) => {
                                if result == "OK" {
                                    let _ = redis::cmd("EXPIRE").arg(&key).arg(86400).query::<Option<u32>>(&mut connection);
                                }
                            },
                            None => (),
                        }
                    },
                    Err(_) => (),
                }
            },
            Err(_) => (),
        }
    }

    pub fn set_ids(&self, request_id: &String, ids: &Vec<&Uid>) {
        let cache = self.clone();
        let request_id = Arc::new(request_id.to_string());
        let mut cloned_ids = Vec::<Uid>::new();
        for &id in ids {
            cloned_ids.push(id.clone());
        }
        tokio::spawn({
            async move {
                cache.set_ids_async(&request_id, &cloned_ids).await;
            }
        });
    }

    async fn set_ids_async(&self, request_id: &String, ids: &Vec<Uid>) {
        let connection = self.nw.get();

        match connection {
            Ok(mut connection) => {
                let key = format!("ids:{}", request_id);
                for id in ids {
                    let value = format!("{}", id.id.replace(":", "_"));

                    let _ = redis::cmd("SADD").arg(&key).arg(&value).query::<Option<String>>(&mut connection);
                }
                let _ = redis::cmd("EXPIRE").arg(&key).arg(86400).query::<Option<u32>>(&mut connection);
            },
            Err(_) => (),
        }
    }

    pub fn get_notification_url(&self, request_id: &String, group: &str) -> Option<Vec<String>> {
        let connection = self.nw.get();

        match connection {
            Ok(mut connection) => {
                let key = format!("{}:{}", group, request_id);

                let result = redis::cmd("SMEMBERS").arg(&key).query::<Option<Vec<String>>>(&mut connection);
                match result {
                    Ok(result) => {
                        match result {
                            Some(result) => {
                                let _ = redis::cmd("DEL").arg(&key).query::<Option<i32>>(&mut connection);
                                return Some(result);
                            },
                            None => (),
                        }
                    },
                    Err(_) => (),
                }
            },
            Err(_) => (),
        }

        return None;
    }

    pub fn set_notification_url(&self, request_id: &str, group: &str, urls: &Vec<String>) {
        let cache = self.clone();
        let request_id = Arc::new(request_id.to_string());
        let group = Arc::new(group.to_string());
        let urls = Arc::new(urls.clone());
        tokio::spawn({
            async move {
                cache.set_notification_url_async(&request_id, &group, &urls).await;
            }
        });
    }

    async fn set_notification_url_async(&self, request_id: &str, group: &str, urls: &Vec<String>) {
        let connection = self.nw.get();

        match connection {
            Ok(mut connection) => {
                let key = format!("{}:{}", group, request_id);

                for url in urls {
                    let result = redis::cmd("SADD").arg(&key).arg(&url).query::<Option<i32>>(&mut connection);
                    match result {
                        Ok(result) => {
                            match result {
                                Some(result) => {
                                    if result == 1 {
                                        let _ = redis::cmd("EXPIRE").arg(&key).arg(60).query::<Option<u32>>(&mut connection);
                                    }
                                },
                                None => (),
                            }
                        },
                        Err(_) => (),
                    }
                }
            },
            Err(_) => (),
        }
    }

    pub fn set_notification_cost(&self, request_id: &String, client_id: i32, vendor_id: i32, income: i32, outcome_upstream: f64, outcome_rebate: f64, outcome_downstream: f64) {
        let cache = self.clone();
        let request_id = Arc::new(request_id.to_string());
        tokio::spawn({
            async move {
                cache.set_notification_cost_async(&request_id, client_id, vendor_id, income, outcome_upstream, outcome_rebate, outcome_downstream).await;
            }
        });
    }

    async fn set_notification_cost_async(&self, request_id: &String, client_id: i32, vendor_id: i32, income: i32, outcome_upstream: f64, outcome_rebate: f64, outcome_downstream: f64) {
        let connection = self.nw.get();

        match connection {
            Ok(mut connection) => {
                // income and outcome_downstream should be placed at the first for compatibility
                let key = format!("cost:{}", request_id);
                let value = format!("{}:{}:{}:{}:{}:{}", client_id, vendor_id, income, outcome_downstream, outcome_upstream, outcome_rebate);

                let result = redis::cmd("SET").arg(&key).arg(&value).query::<Option<String>>(&mut connection);
                match result {
                    Ok(result) => {
                        match result {
                            Some(result) => {
                                if result == "OK" {
                                    let _ = redis::cmd("EXPIRE").arg(&key).arg(86400).query::<Option<u32>>(&mut connection);
                                }
                            },
                            None => (),
                        }
                    },
                    Err(_) => (),
                }
            },
            Err(_) => (),
        }
    }

}
