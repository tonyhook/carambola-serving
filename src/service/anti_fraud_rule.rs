use chrono::{DateTime, FixedOffset};

use crate::entity::{AntiFraud, Request};

use super::Cache;

pub trait AntiFraudRule {
    fn pass(cache: &Cache, time: DateTime<FixedOffset>, client_port: i32, af: &AntiFraud, request: &Request) -> bool;
    fn set(cache: &Cache, time: DateTime<FixedOffset>, client_port: i32, af: &AntiFraud, request: &Request);
}
