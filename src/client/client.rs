use crate::{entity::{Connection, Request, Response}, Cache, ResultMessage};

pub trait Client {
    async fn request(request: &Request, connection: &Connection, cache: &Cache) -> Result<Response, ResultMessage>;
    async fn bidding_notify_win(url: String, win_price: i32, next_price: i32, iv: &String, connection: &Connection);
    async fn bidding_notify_lose(url: String, lose_price: i32, lose_reason: i32, lose_adn_name: &String, iv: &String, connection: &Connection);
    fn encrypt_price(price: i32, iv: &String, connection: &Connection) -> String;
}
