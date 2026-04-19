use serde::{Deserialize, Serialize};

use super::Rule;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Entry {
    RuleSet(RuleSet),
    Rule(Rule),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct RuleSet {
    pub condition: String,
    pub rules: Vec<Entry>,
}
