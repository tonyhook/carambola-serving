use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct RuiangData {
    #[serde(rename(deserialize = "clkUrl", serialize = "clkUrl"))]
    pub clk_url: String,
    #[serde(rename(deserialize = "dpUrl", serialize = "dpUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dp_url: Option<String>,
    #[serde(rename(deserialize = "dpAd", serialize = "dpAd"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dp_ad: Option<i32>,
    pub mt: i32,
    pub imgs: Vec<String>,
    pub width: i32,
    pub height: i32,
    pub title: String,
    pub atx: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pack: Option<String>,
    pub appname: String,
    #[serde(rename(deserialize = "btTxt", serialize = "btTxt"))]
    pub bt_txt: String,
    pub act: i32,
    #[serde(rename(deserialize = "bidPrice", serialize = "bidPrice"))]
    pub bid_price: i32,
    #[serde(rename(deserialize = "showTrace", serialize = "showTrace"))]
    pub show_trace: Vec<String>,
    #[serde(rename(deserialize = "clkTrace", serialize = "clkTrace"))]
    pub clk_trace: Vec<String>,
    #[serde(rename(deserialize = "dpTryTrace", serialize = "dpTryTrace"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dp_try_trace: Option<Vec<String>>,
    #[serde(rename(deserialize = "dpSucTrace", serialize = "dpSucTrace"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dp_suc_trace: Option<Vec<String>>,
    #[serde(rename(deserialize = "dpFailTrace", serialize = "dpFailTrace"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dp_fail_trace: Option<Vec<String>>,
    #[serde(rename(deserialize = "downTrace", serialize = "downTrace"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub down_trace: Option<Vec<String>>,
    #[serde(rename(deserialize = "downedTrace", serialize = "downedTrace"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downed_trace: Option<Vec<String>>,
    #[serde(rename(deserialize = "insTrace", serialize = "insTrace"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ins_trace: Option<Vec<String>>,
    #[serde(rename(deserialize = "insedTrace", serialize = "insedTrace"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insed_trace: Option<Vec<String>>,
    #[serde(rename(deserialize = "acTrace", serialize = "acTrace"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ac_trace: Option<Vec<String>>,
    #[serde(rename(deserialize = "noticeTrace", serialize = "noticeTrace"))]
    pub notice_trace: Vec<String>,
}
