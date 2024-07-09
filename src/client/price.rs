use crate::{entity::PORT_TYPE_SHARE, Connection};

pub struct Price {

}

impl Price {

    pub fn to_client(connection: &Connection, price: Option<i32>) -> i32 {
        if connection.vendor_mode == PORT_TYPE_SHARE {
            connection.default_price
        } else {
            match price {
                Some(price) => {
                    if price > 0 {
                        (price as f64 / connection.cost_ratio) as i32
                    } else {
                        connection.default_price
                    }
                },
                None => {
                    connection.default_price
                },
            }
        }
    }

    pub fn to_vendor(connection: &Connection, price: Option<i32>) -> i32 {
        match price {
            Some(price) => {
                if price > 0 {
                    (price as f64 * connection.cost_ratio) as i32
                } else {
                    (connection.default_price as f64 * connection.cost_ratio) as i32
                }
            },
            None => {
                (connection.default_price as f64 * connection.cost_ratio) as i32
            },
        }
    }

}
