use zbus::{Connection, Proxy};

use crate::FumResult;

#[derive(Debug)]
pub struct Player<'a> {
    pub connection: &'a Connection,
    pub proxy: Proxy<'a>,
    pub bus_name: String,
}

impl<'a> Player<'a> {
    /// Creates a new player struct.
    pub async fn new(connection: &'a Connection, bus_name: String) -> FumResult<Self> {
        let proxy = Proxy::new(
            connection,
            bus_name.to_string(),
            "/org/freedesktop/DBus",
            "org.freedesktop.DBus"
        ).await?;

        Ok(Self {
            connection,
            proxy,
            bus_name
        })
    }
}
