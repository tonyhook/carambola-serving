use chrono::{DateTime, FixedOffset};

use crate::{entity::AntiFraud, service::Cache, AntiFraudRule, Request};

pub struct AfRequestPerIp {

}

impl AntiFraudRule for AfRequestPerIp {

    fn pass(cache: &Cache, time: DateTime<FixedOffset>, client_port: i32, af: &AntiFraud, request: &Request) -> bool {
        match &request.context.device.ip {
            Some(ip) => {
                if !ip.is_empty() {
                    let amount = cache.get_anti_fraud_amount(time, client_port, af.period, &af.rule, &ip) as f64;
                    if amount >= af.limitation {
                        return false;
                    }
                }
            },
            None => (),
        }
        match &request.context.device.ipv6 {
            Some(ipv6) => {
                if !ipv6.is_empty() {
                    let amount = cache.get_anti_fraud_amount(time, client_port, af.period, &af.rule, &ipv6) as f64;
                    if amount >= af.limitation {
                        return false;
                    }
                }
            },
            None => (),
        }

        true
    }

    fn set(cache: &Cache, time: DateTime<FixedOffset>, client_port: i32, af: &AntiFraud, request: &Request) {
        match &request.context.device.ip {
            Some(ip) => {
                cache.set_anti_fraud_amount(time, client_port, af.period, &af.rule, &ip, 1);
            },
            None => (),
        }
        match &request.context.device.ipv6 {
            Some(ipv6) => {
                cache.set_anti_fraud_amount(time, client_port, af.period, &af.rule, &ipv6, 1);
            },
            None => (),
        }
    }

}
