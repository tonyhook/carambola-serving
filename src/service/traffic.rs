use chrono::{FixedOffset, Timelike, Utc};

use crate::{client::Identifiers, entity::{Request, TC_INDICATOR_REQUEST, TC_PERIOD_DAY, TC_PERIOD_HOUR, TC_PERIOD_SECOND}};

use super::{AfIpPerId, AfRequestPerId, AfRequestPerIp, AfUaPerId, AntiFraudRule, Cache, Database};

#[derive(Clone)]
pub struct Traffic {
    pub database: Database,
    pub cache: Cache,
}

impl Traffic {

    pub fn new(database: Database, cache: Cache) -> Self {
        Traffic {
            database: database,
            cache: cache,
        }
    }

    pub fn pass_traffic_control(&self, client_port: i32, vendor_port: i32, bundle: &String) -> bool {
        let time = Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap());

        let tcs = {
            let tcl = self.database.tcla.clone();
            let tc = tcl.read().unwrap();

            let tcs = tc.get(&format!("{}|{}|{}", client_port, vendor_port, bundle));

            match tcs {
                Some(tcs) => {
                    tcs.clone()
                },
                None => {
                    [].to_vec()
                },
            }
        };

        for tc in &tcs {
            let indicator = tc.indicator;
            let period = tc.period;
            let mut limitation = tc.limitation;

            if limitation == 0 {
                return false;
            }

            if period == TC_PERIOD_DAY {
                let hour = time.hour();
                let minute = time.minute();
                limitation = limitation * (hour as i64 * 60 + minute as i64) / 1440 + 1;
            }
            if period == TC_PERIOD_HOUR {
                let minute = time.minute();
                limitation = limitation * (minute as i64) / 60 + 1;
            }
            if period == TC_PERIOD_SECOND {
                limitation = limitation * 60;
            }

            let q = self.cache.get_traffic_control_amount(time, client_port, vendor_port, bundle, indicator, period);
            if q >= limitation {
                return false;
            }
        }

        for tc in &tcs {
            let indicator = tc.indicator;
            let period = tc.period;

            if indicator == TC_INDICATOR_REQUEST {
                self.cache.set_traffic_control_amount(time, client_port, vendor_port, bundle, indicator, period, 1);
            }
        }

        return true;
    }

    pub fn pass_anti_fraud(&self, client_port: i32, request: &Request) -> bool {
        let mut passed = true;
        let time = Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap());

        let afs = {
            let afl = self.database.afla.clone();
            let af = afl.read().unwrap();

            let afs = af.get(&format!("{}", client_port));

            match afs {
                Some(afs) => {
                    afs.clone()
                },
                None => {
                    [].to_vec()
                },
            }
        };

        for af in &afs {
            if af.rule == "REQUEST_PER_IP" {
                passed = AfRequestPerIp::pass(&self.cache, time, client_port, af, request);
            }
            if af.rule == "REQUEST_PER_ID" {
                passed = AfRequestPerId::pass(&self.cache, time, client_port, af, request);
            }
            if af.rule == "UA_PER_ID" {
                passed = AfUaPerId::pass(&self.cache, time, client_port, af, request);
            }
            if af.rule == "IP_PER_ID" {
                passed = AfIpPerId::pass(&self.cache, time, client_port, af, request);
            }

            if !passed {
                return false;
            }
        }

        for af in &afs {
            if af.rule == "REQUEST_PER_IP" {
                AfRequestPerIp::set(&self.cache, time, client_port, af, request);
            }
            if af.rule == "REQUEST_PER_ID" {
                AfRequestPerId::set(&self.cache, time, client_port, af, request);
            }
            if af.rule == "UA_PER_ID" {
                AfUaPerId::set(&self.cache, time, client_port, af, request);
            }
            if af.rule == "IP_PER_ID" {
                AfIpPerId::set(&self.cache, time, client_port, af, request);
            }
        }

        return true;
    }

    pub fn prepare_for_imp(&self, client_port: i32, request_id: &String, request: &Request) {
        let afs = {
            let afl = self.database.afla.clone();
            let af = afl.read().unwrap();

            let afs = af.get(&format!("{}", client_port));

            match afs {
                Some(afs) => {
                    afs.clone()
                },
                None => {
                    [].to_vec()
                },
            }
        };

        for af in &afs {
            if af.rule == "IMP_PER_ID" {
                let identifiers = Identifiers::new(request);
                self.cache.set_ids(request_id, &identifiers.get_flatten_ids());
            }
        }
    }

}
