use serde::{Deserialize, Serialize};

use super::RichmobUrlVisit;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct RichmobMonitorUrlVisit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landingpagetracklist: Option<Vec<RichmobUrlVisit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imptracklist: Option<Vec<RichmobUrlVisit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicktracklist: Option<Vec<RichmobUrlVisit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closetracklist: Option<Vec<RichmobUrlVisit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startdowntracklist: Option<Vec<RichmobUrlVisit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finishdowntracklist: Option<Vec<RichmobUrlVisit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downpausetracklist: Option<Vec<RichmobUrlVisit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downlandingpagetracklist: Option<Vec<RichmobUrlVisit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downgoontracklist: Option<Vec<RichmobUrlVisit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downdeletetracklist: Option<Vec<RichmobUrlVisit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startinstalltracklist: Option<Vec<RichmobUrlVisit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finishinstalltracklist: Option<Vec<RichmobUrlVisit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activedtracklist: Option<Vec<RichmobUrlVisit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplinktracklist: Option<Vec<RichmobUrlVisit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplinkfailedtracklist: Option<Vec<RichmobUrlVisit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videostarttracklist: Option<Vec<RichmobUrlVisit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firstquartiletracklist: Option<Vec<RichmobUrlVisit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub midpointtracklist: Option<Vec<RichmobUrlVisit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thirdquartiletracklist: Option<Vec<RichmobUrlVisit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videoendtracklist: Option<Vec<RichmobUrlVisit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videomutetracklist: Option<Vec<RichmobUrlVisit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videoskiptracklist: Option<Vec<RichmobUrlVisit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videoclosetracklist: Option<Vec<RichmobUrlVisit>>,
}
