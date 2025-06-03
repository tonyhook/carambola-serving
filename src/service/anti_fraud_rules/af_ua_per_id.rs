use chrono::{DateTime, FixedOffset};

use crate::{client::Identifiers, entity::AntiFraud, service::Cache, AntiFraudRule, Request};

pub struct AfUaPerId {

}

impl AntiFraudRule for AfUaPerId {

    fn pass(cache: &Cache, time: DateTime<FixedOffset>, client_port: i32, af: &AntiFraud, request: &Request) -> bool {
        let identifiers = Identifiers::new(request);
        for identifier in identifiers.get_flatten_ids() {
            let amount = cache.get_anti_fraud_member(time, client_port, af.period, &af.rule, &identifier.id) as f64;
            if amount >= af.limitation {
                return false;
            }
        }

        true
    }

    fn set(cache: &Cache, time: DateTime<FixedOffset>, client_port: i32, af: &AntiFraud, request: &Request) {
        let identifiers = Identifiers::new(request);
        for identifier in identifiers.get_flatten_ids() {
            cache.set_anti_fraud_member(time, client_port, af.period, &af.rule, &identifier.id, &request.context.device.ua);
        }
    }

}
