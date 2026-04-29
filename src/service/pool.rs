use std::{sync::{Arc, RwLock}, time::Duration};

use reqwest::Client;

#[derive(Clone)]
pub struct HttpPool {
    pub pool_adxflow: Arc<RwLock<Client>>,
    pub pool_adxwork: Arc<RwLock<Client>>,
    pub pool_adwanji: Arc<RwLock<Client>>,
    pub pool_billowlink: Arc<RwLock<Client>>,
    pub pool_fanglin: Arc<RwLock<Client>>,
    pub pool_fwb: Arc<RwLock<Client>>,
    pub pool_huichuan: Arc<RwLock<Client>>,
    pub pool_huoli: Arc<RwLock<Client>>,
    pub pool_onenmob: Arc<RwLock<Client>>,
    pub pool_jinmo: Arc<RwLock<Client>>,
    pub pool_jmedium: Arc<RwLock<Client>>,
    pub pool_kaka: Arc<RwLock<Client>>,
    pub pool_kkmh: Arc<RwLock<Client>>,
    pub pool_leidong: Arc<RwLock<Client>>,
    pub pool_mfocus: Arc<RwLock<Client>>,
    pub pool_mobrtb: Arc<RwLock<Client>>,
    pub pool_mvpmob: Arc<RwLock<Client>>,
    pub pool_mygolbs: Arc<RwLock<Client>>,
    pub pool_richmob: Arc<RwLock<Client>>,
    pub pool_ruiang: Arc<RwLock<Client>>,
    pub pool_spinview: Arc<RwLock<Client>>,
    pub pool_sweet: Arc<RwLock<Client>>,
    pub pool_tengmei: Arc<RwLock<Client>>,
    pub pool_tianzhuo: Arc<RwLock<Client>>,
    pub pool_ustars: Arc<RwLock<Client>>,
    pub pool_yiba: Arc<RwLock<Client>>,
    pub pool_yiwei: Arc<RwLock<Client>>,
    pub pool_zhanqing: Arc<RwLock<Client>>,
}

impl HttpPool {

    pub fn new() -> Self {
        HttpPool {
            pool_adxflow: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
                .tcp_keepalive(Duration::from_secs(60))
                .gzip(true)
                .no_brotli()
                .no_deflate()
                .pool_idle_timeout(Duration::from_millis(1000))
                .build().unwrap())),
            pool_adxwork: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
                .tcp_keepalive(Duration::from_secs(60))
                .gzip(true)
                .no_brotli()
                .no_deflate()
                .pool_idle_timeout(Duration::from_millis(1000))
                .build().unwrap())),
            pool_adwanji: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
                .tcp_keepalive(Duration::from_secs(60))
                .gzip(true)
                .no_brotli()
                .no_deflate()
                .pool_idle_timeout(Duration::from_millis(1000))
                .build().unwrap())),
            pool_billowlink: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
                .tcp_keepalive(Duration::from_secs(60))
                .gzip(true)
                .no_brotli()
                .no_deflate()
                .pool_idle_timeout(Duration::from_millis(1000))
                .build().unwrap())),
            pool_fanglin: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
                .tcp_keepalive(Duration::from_secs(60))
                .gzip(true)
                .no_brotli()
                .no_deflate()
                .pool_idle_timeout(Duration::from_millis(1000))
                .build().unwrap())),
            pool_fwb: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
                .tcp_keepalive(Duration::from_secs(60))
                .gzip(true)
                .no_brotli()
                .no_deflate()
                .pool_idle_timeout(Duration::from_millis(1000))
                .build().unwrap())),
            pool_huichuan: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
                .tcp_keepalive(Duration::from_secs(60))
                .gzip(true)
                .no_brotli()
                .no_deflate()
                .pool_idle_timeout(Duration::from_millis(1000))
                .build().unwrap())),
            pool_huoli: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
                .tcp_keepalive(Duration::from_secs(60))
                .gzip(true)
                .no_brotli()
                .no_deflate()
                .pool_idle_timeout(Duration::from_millis(1000))
                .build().unwrap())),
            pool_onenmob: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
                .tcp_keepalive(Duration::from_secs(60))
                .gzip(true)
                .no_brotli()
                .no_deflate()
                .pool_idle_timeout(Duration::from_millis(1000))
                .build().unwrap())),
            pool_jinmo: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
                .tcp_keepalive(Duration::from_secs(60))
                .gzip(true)
                .no_brotli()
                .no_deflate()
                .pool_idle_timeout(Duration::from_millis(1000))
                .build().unwrap())),
            pool_jmedium: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
                .tcp_keepalive(Duration::from_secs(60))
                .gzip(true)
                .no_brotli()
                .no_deflate()
                .pool_idle_timeout(Duration::from_millis(1000))
                .build().unwrap())),
            pool_kaka: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
                .tcp_keepalive(Duration::from_secs(60))
                .gzip(true)
                .no_brotli()
                .no_deflate()
                .pool_idle_timeout(Duration::from_millis(1000))
                .build().unwrap())),
            pool_kkmh: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
                .tcp_keepalive(Duration::from_secs(60))
                .gzip(true)
                .no_brotli()
                .no_deflate()
                .pool_idle_timeout(Duration::from_millis(1000))
                .build().unwrap())),
            pool_leidong: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
                .tcp_keepalive(Duration::from_secs(60))
                .gzip(true)
                .no_brotli()
                .no_deflate()
                .pool_idle_timeout(Duration::from_millis(1000))
                .build().unwrap())),
            pool_mfocus: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
                .tcp_keepalive(Duration::from_secs(60))
                .gzip(true)
                .no_brotli()
                .no_deflate()
                .pool_idle_timeout(Duration::from_millis(1000))
                .build().unwrap())),
            pool_mobrtb: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
                .tcp_keepalive(Duration::from_secs(60))
                .gzip(true)
                .no_brotli()
                .no_deflate()
                .pool_idle_timeout(Duration::from_millis(1000))
                .build().unwrap())),
            pool_mvpmob: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
                .tcp_keepalive(Duration::from_secs(60))
                .gzip(true)
                .no_brotli()
                .no_deflate()
                .pool_idle_timeout(Duration::from_millis(1000))
                .build().unwrap())),
            pool_mygolbs: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
                .tcp_keepalive(Duration::from_secs(60))
                .gzip(true)
                .no_brotli()
                .no_deflate()
                .pool_idle_timeout(Duration::from_millis(1000))
                .build().unwrap())),
            pool_richmob: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
                .tcp_keepalive(Duration::from_secs(60))
                .gzip(true)
                .no_brotli()
                .no_deflate()
                .pool_idle_timeout(Duration::from_millis(1000))
                .build().unwrap())),
            pool_ruiang: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
                .tcp_keepalive(Duration::from_secs(60))
                .gzip(true)
                .no_brotli()
                .no_deflate()
                .pool_idle_timeout(Duration::from_millis(1000))
                .build().unwrap())),
            pool_spinview: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
                .tcp_keepalive(Duration::from_secs(60))
                .gzip(true)
                .no_brotli()
                .no_deflate()
                .pool_idle_timeout(Duration::from_millis(1000))
                .build().unwrap())),
            pool_sweet: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
                .tcp_keepalive(Duration::from_secs(60))
                .gzip(true)
                .no_brotli()
                .no_deflate()
                .pool_idle_timeout(Duration::from_millis(1000))
                .build().unwrap())),
            pool_tengmei: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
                .tcp_keepalive(Duration::from_secs(60))
                .gzip(true)
                .no_brotli()
                .no_deflate()
                .pool_idle_timeout(Duration::from_millis(1000))
                .build().unwrap())),
            pool_tianzhuo: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
                .tcp_keepalive(Duration::from_secs(60))
                .gzip(true)
                .no_brotli()
                .no_deflate()
                .pool_idle_timeout(Duration::from_millis(1000))
                .build().unwrap())),
            pool_ustars: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
                .tcp_keepalive(Duration::from_secs(60))
                .gzip(true)
                .no_brotli()
                .no_deflate()
                .pool_idle_timeout(Duration::from_millis(1000))
                .build().unwrap())),
            pool_yiba: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
                .tcp_keepalive(Duration::from_secs(60))
                .gzip(true)
                .no_brotli()
                .no_deflate()
                .pool_idle_timeout(Duration::from_millis(1000))
                .build().unwrap())),
            pool_yiwei: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
                .tcp_keepalive(Duration::from_secs(60))
                .gzip(true)
                .no_brotli()
                .no_deflate()
                .pool_idle_timeout(Duration::from_millis(1000))
                .build().unwrap())),
            pool_zhanqing: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
                .tcp_keepalive(Duration::from_secs(60))
                .gzip(true)
                .no_brotli()
                .no_deflate()
                .pool_idle_timeout(Duration::from_millis(1000))
                .build().unwrap())),
        }
    }

}
