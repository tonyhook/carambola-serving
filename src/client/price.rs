use crate::{entity::PORT_TYPE_SHARE, Connection};

pub struct Price {

}

impl Price {

    pub fn to_client(connection: &Connection, price: Option<f64>) -> f64 {
        if connection.vendor_mode == PORT_TYPE_SHARE {
            connection.default_price as f64
        } else {
            match price {
                Some(price) => {
                    if price > 0.0 {
                        price / connection.downstream_ratio
                    } else {
                        connection.default_price as f64
                    }
                },
                None => {
                    connection.default_price as f64
                },
            }
        }
    }

    pub fn to_upstream(connection: &Connection, price: Option<f64>) -> f64 {
        match price {
            Some(price) => {
                if price > 0.0 {
                    price * connection.upstream_ratio
                } else {
                    connection.default_price as f64 * connection.upstream_ratio
                }
            },
            None => {
                connection.default_price as f64 * connection.upstream_ratio
            },
        }
    }

    pub fn to_rebate(connection: &Connection, price: Option<f64>) -> f64 {
        match price {
            Some(price) => {
                if price > 0.0 {
                    price * connection.rebate_ratio
                } else {
                    connection.default_price as f64 * connection.rebate_ratio
                }
            },
            None => {
                connection.default_price as f64 * connection.rebate_ratio
            },
        }
    }

    pub fn to_vendor(connection: &Connection, price: Option<f64>) -> f64 {
        match price {
            Some(price) => {
                if price > 0.0 {
                    price * connection.downstream_ratio
                } else {
                    connection.default_price as f64 * connection.downstream_ratio
                }
            },
            None => {
                connection.default_price as f64 * connection.downstream_ratio
            },
        }
    }

}
