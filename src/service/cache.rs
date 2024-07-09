use std::sync::{Arc, Mutex};

use chrono::{DateTime, Timelike, Utc};
use redis::Client;

use crate::{EnvConfig, GLOBAL_CONFIG};

pub const PERFORMANCE_NO_PROTOCOL:          &str = "A"; // -1 as client port
pub const PERFORMANCE_BAD_PROTOCOL_VER:     &str = "B"; // -1 as client port
pub const PERFORMANCE_NO_ITEM:              &str = "C"; // -1 as client port
pub const PERFORMANCE_NOT_REGISTERED:       &str = "D"; // -1 as client port
pub const PERFORMANCE_NO_MATCH_CONNECTION:  &str = "E"; // -1 as client port
pub const PERFORMANCE_BEYOND_VENDOR_QPS:    &str = "F"; // -1 as client port
pub const PERFORMANCE_ANTI_FRAUD:           &str = "G"; // -1 as client port
pub const PERFORMANCE_NO_RESPONSE:          &str = "H"; // -1 as client port
pub const PERFORMANCE_SHARE_SUCCESS:        &str = "I"; // one in each request
pub const PERFORMANCE_BIDDING_SUCCESS:      &str = "J"; // one in each request

// by connection, multiple in each request
pub const PERFORMANCE_TIMEOUT:              &str = "ZA";
pub const PERFORMANCE_REQUEST_FAILED:       &str = "ZB";
pub const PERFORMANCE_NOT_BIDDING:          &str = "ZC";
pub const PERFORMANCE_SHARE_OK:             &str = "ZD";
pub const PERFORMANCE_BIDDING_OK:           &str = "ZE";
pub const PERFORMANCE_TRANS_FROM_FAILED:    &str = "ZF";
pub const PERFORMANCE_TRANS_TO_FAILED:      &str = "ZG";
pub const PERFORMANCE_UNKNOWN_CLIENT:       &str = "ZH";
pub const PERFORMANCE_BIDDING_LOSE:         &str = "ZI";
pub const PERFORMANCE_BIDDING_WIN:          &str = "ZJ";
pub const PERFORMANCE_BIDDING_INVALID:      &str = "ZK";
pub const PERFORMANCE_BEYOND_CLIENT_QPS:    &str = "ZL";
pub const PERFORMANCE_LOST_KEY_FIELD:       &str = "ZM";

#[derive(Clone)]
pub struct Cache {
    pub pa: Arc<Mutex<Client>>, // performance
    pub fa: Arc<Mutex<Client>>, // anti fraud
    pub sa: Arc<Mutex<Client>>, // id generator
    pub na: Arc<Mutex<Client>>, // notification url & cost
}

impl Cache {

    pub fn new(config: &EnvConfig) -> Self {
        Self {
            pa: Arc::new(Mutex::new(redis::Client::open(config.performance_connection.clone()).unwrap())),
            fa: Arc::new(Mutex::new(redis::Client::open(config.flowcontrol_connection.clone()).unwrap())),
            sa: Arc::new(Mutex::new(redis::Client::open(config.idgenerator_connection.clone()).unwrap())),
            na: Arc::new(Mutex::new(redis::Client::open(config.notification_connection.clone()).unwrap())),
        }
    }

    pub fn update_performance(&self, client_port: i32, vendor_port: i32, event: &str) {
        let cache = self.clone();
        let event = Arc::new(event.to_string());
        tokio::spawn({
            async move {
                cache.update_performance_async(client_port, vendor_port, &event).await;
            }
        });
    }

    async fn update_performance_async(&self, client_port: i32, vendor_port: i32, event: &str) {
        let utc: DateTime<Utc> = Utc::now();
        let hour = utc.hour();
        let minute_aligned = utc.minute() / GLOBAL_CONFIG.get().unwrap().performance_interval * GLOBAL_CONFIG.get().unwrap().performance_interval;
        let minute_fragment = utc.minute() - minute_aligned;
        let second = utc.second();

        let key = format!("P{:0>2}{:0>2}:{}:{}:{}", hour, minute_aligned, client_port, vendor_port, event);
        let expire = 14400 - minute_fragment * 60 - second - GLOBAL_CONFIG.get().unwrap().performance_interval * 60;

        let connection = {
            let cl = self.pa.clone();
            let rs_client = cl.lock().unwrap();
            rs_client.get_connection()
        };

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

    pub fn get_request_amount(&self, client_port: i32, vendor_port: i32) -> i32 {
        let utc: DateTime<Utc> = Utc::now();
        let hour = utc.hour();
        let minute = utc.minute();

        let key = format!("Q{:0>2}{:0>2}:{}:{}", hour, minute, client_port, vendor_port);

        let connection = {
            let cl = self.fa.clone();
            let rs_client = cl.lock().unwrap();
            rs_client.get_connection()
        };

        match connection {
            Ok(mut connection) => {
                let result = redis::cmd("GET").arg(key).query::<Option<i32>>(&mut connection);
                match result {
                    Ok(amount) => {
                        match amount {
                            Some(amount) => amount,
                            None => 0,
                        }
                    },
                    Err(_) => std::i32::MAX,
                }
            },
            Err(_) => std::i32::MAX,
        }
    }

    pub fn set_request_amount(&self, client_port: i32, vendor_port: i32) {
        let utc: DateTime<Utc> = Utc::now();
        let hour = utc.hour();
        let minute = utc.minute();
        let second = utc.second();

        let key = format!("Q{:0>2}{:0>2}:{}:{}", hour, minute, client_port, vendor_port);
        let expire = 60 - second + GLOBAL_CONFIG.get().unwrap().performance_interval * 60;

        let connection = {
            let cl = self.fa.clone();
            let rs_client = cl.lock().unwrap();
            rs_client.get_connection()
        };

        match connection {
            Ok(mut connection) => {
                let result = redis::cmd("INCR").arg(&key).query::<Option<i32>>(&mut connection);
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

    pub fn get_sequence(&self) -> u64 {
        let mut sequence = 0;
        let connection = {
            let cl = self.sa.clone();
            let rs_client = cl.lock().unwrap();
            rs_client.get_connection()
        };

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
                                sequence = timestamp << 32 | result as u64;
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

    pub fn get_notification_url(&self, request_id: &String, group: &str) -> Option<Vec<String>> {
        let connection = {
            let cl = self.na.clone();
            let rs_client = cl.lock().unwrap();
            rs_client.get_connection()
        };

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

    pub fn set_notification_url(&self, request_id: &String, group: &str, urls: &Vec<String>) {
        let connection = {
            let cl = self.na.clone();
            let rs_client = cl.lock().unwrap();
            rs_client.get_connection()
        };

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

    pub fn set_notification_cost(&self, request_id: &str, client_id: i32, vendor_id: i32, client_win_price: i32, vendor_win_price: i32) {
        let connection = {
            let cl = self.na.clone();
            let rs_client = cl.lock().unwrap();
            rs_client.get_connection()
        };

        match connection {
            Ok(mut connection) => {
                let key = format!("cost:{}", request_id);
                let value = format!("{}:{}:{}:{}", client_id, vendor_id, client_win_price, vendor_win_price);

                let result = redis::cmd("SET").arg(&key).arg(&value).query::<Option<i32>>(&mut connection);
                match result {
                    Ok(result) => {
                        match result {
                            Some(result) => {
                                if result == 1 {
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
