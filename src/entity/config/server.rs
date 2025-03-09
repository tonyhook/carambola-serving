use serde::Deserialize;

#[derive(Deserialize)]
#[derive(Clone)]
pub struct Server {
    pub node: i32,
}
