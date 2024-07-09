use super::Configuration;

pub const PORT_TYPE_SHARE:   i32 = 1;
pub const PORT_TYPE_BIDDING: i32 = 2;

#[derive(Clone)]
pub struct Connection {
    pub id: i32,
    pub client_code: String,
    pub client_port: i32,
    pub client_tag_id: String,
    pub client_mode: i32,
    pub client_ekey: String,
    pub client_ikey: String,
    pub test: bool,
    pub vendor_port: i32,
    pub vendor_mode: i32,
    pub vendor_ekey: String,
    pub vendor_ikey: String,
    pub timeout: u64,
    pub priority: i32,
    pub cost_ratio: f64,
    pub default_price: i32,
    pub configuration: Configuration,
}
