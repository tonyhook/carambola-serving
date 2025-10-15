use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(prost::Message)]
pub struct JinmoTrack {
    #[prost(string, repeated, tag="1")]
    pub win_notice_urls: Vec<String>,
    #[prost(string, repeated, tag="2")]
    pub view_monitor_urls: Vec<String>,
    #[prost(string, repeated, tag="3")]
    pub click_monitor_urls: Vec<String>,
    #[prost(string, repeated, tag="4")]
    pub download_start_monitor_urls: Vec<String>,
    #[prost(string, repeated, tag="5")]
    pub download_finish_monitor_urls: Vec<String>,
    #[prost(string, repeated, tag="6")]
    pub install_start_monitor_urls: Vec<String>,
    #[prost(string, repeated, tag="7")]
    pub installed_monitor_urls: Vec<String>,
    #[prost(string, repeated, tag="8")]
    pub dp_click_monitor_urls: Vec<String>,
    #[prost(string, repeated, tag="9")]
    pub dp_success_monitor_urls: Vec<String>,
    #[prost(string, repeated, tag="10")]
    pub dp_failed_monitor_urls: Vec<String>,
    #[prost(string, repeated, tag="11")]
    pub video_play_start_monitor_urls: Vec<String>,
    #[prost(string, repeated, tag="12")]
    pub video_play_end_monitor_urls: Vec<String>,
    #[prost(string, repeated, tag="13")]
    pub video_play_25_monitor_urls: Vec<String>,
    #[prost(string, repeated, tag="14")]
    pub video_play_50_monitor_urls: Vec<String>,
    #[prost(string, repeated, tag="15")]
    pub video_play_75_monitor_urls: Vec<String>,
    #[prost(string, repeated, tag="16")]
    pub video_pause_monitor_urls: Vec<String>,
    #[prost(string, repeated, tag="17")]
    pub video_continue_monitor_urls: Vec<String>,
    #[prost(string, repeated, tag="18")]
    pub video_skip_monitor_urls: Vec<String>,
}
