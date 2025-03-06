use std::{sync::{Arc, RwLock}, time::Duration};

use reqwest::Client;

#[derive(Clone)]
pub struct HttpPool {
    pub pool_adwanji: Arc<RwLock<Client>>,
    pub pool_fanglin: Arc<RwLock<Client>>,
    pub pool_fwb: Arc<RwLock<Client>>,
    pub pool_jmedium: Arc<RwLock<Client>>,
    pub pool_kkmh: Arc<RwLock<Client>>,
    pub pool_mfocus: Arc<RwLock<Client>>,
    pub pool_mobrtb: Arc<RwLock<Client>>,
    pub pool_mygolbs: Arc<RwLock<Client>>,
    pub pool_richmob: Arc<RwLock<Client>>,
    pub pool_yiba: Arc<RwLock<Client>>,
}

impl HttpPool {

    pub fn new() -> Self {
        HttpPool {
            pool_adwanji: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
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
            pool_jmedium: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
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
            pool_yiba: Arc::new(RwLock::new(reqwest::ClientBuilder::new()
                .tcp_keepalive(Duration::from_secs(60))
                .gzip(true)
                .no_brotli()
                .no_deflate()
                .pool_idle_timeout(Duration::from_millis(1000))
                .build().unwrap())),
        }
    }

}
