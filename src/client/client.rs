use crate::{entity::{Connection, Request, Response}, Cache, HttpPool, ResultMessage};

pub trait Client {
    async fn request(request: &Request, connection: &Connection, pool: &HttpPool, cache: &Cache) -> Result<Response, ResultMessage>;
    async fn bidding_notify_win(url: String, win_price: i32, next_price: i32, iv: &String, connection: &Connection, pool: &HttpPool);
    async fn bidding_notify_lose(url: String, lose_price: i32, lose_reason: i32, lose_adn_name: &String, iv: &String, connection: &Connection, pool: &HttpPool);
    fn encrypt_price(price: i32, iv: &String, connection: &Connection) -> String;
}
