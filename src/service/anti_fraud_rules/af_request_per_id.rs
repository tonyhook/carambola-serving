use chrono::{DateTime, FixedOffset};

use crate::{client::Identifiers, entity::AntiFraud, service::Cache, AntiFraudRule, Request};

pub struct AfRequestPerId {

}

impl AntiFraudRule for AfRequestPerId {

    fn pass(cache: &Cache, time: DateTime<FixedOffset>, client_port: i32, af: &AntiFraud, request: &Request) -> bool {
        let identifiers = Identifiers::new(request);
        for identifier in identifiers.get_flatten_ids() {
            match identifier.atype {
                501..=510 | 513..=521 => {
                    if !identifier.id.is_empty() {
                        let amount = cache.get_anti_fraud_amount(time, client_port, af.period, &af.rule, &identifier.id) as f64;
                        if amount >= af.limitation {
                            return false;
                        }
                    }
                },
                _ => (),
            }
        }

        true
    }

    fn set(cache: &Cache, time: DateTime<FixedOffset>, client_port: i32, af: &AntiFraud, request: &Request) {
        let identifiers = Identifiers::new(request);
        for identifier in identifiers.get_flatten_ids() {
            cache.set_anti_fraud_amount(time, client_port, af.period, &af.rule, &identifier.id, 1);
        }
    }

}
