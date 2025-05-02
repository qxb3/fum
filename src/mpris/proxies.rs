use anyhow::Context;
use zbus::{Connection, Proxy};

use crate::FumResult;

/// Proxy for "org.freedesktop.DBUS" interface.
pub async fn create_dbus_proxy(connection: &Connection) -> FumResult<Proxy> {
    let proxy = Proxy::new(
        connection,
        "org.freedesktop.DBus",
        "/org/freedesktop/DBus",
        "org.freedesktop.DBus",
    )
    .await
    .context("Failed to create D-Bus proxy")?;

    Ok(proxy)
}

/// Creates a proxy for "org.freedesktop.DBus.Properties".
pub async fn create_properties_proxy(
    connection: &Connection,
    bus_name: String,
) -> FumResult<Proxy<'static>> {
    let properties_proxy = Proxy::new(
        connection,
        bus_name,
        "/org/mpris/MediaPlayer2",
        "org.freedesktop.DBus.Properties",
    )
    .await?;

    Ok(properties_proxy)
}

/// Proxy for "org.mpris.MediaPlayer2.Player" interface.
pub async fn create_player_proxy(connection: &Connection, bus: &str) -> FumResult<Proxy<'static>> {
    let proxy: Proxy = zbus::proxy::Builder::new(connection)
        .destination(bus.to_string())?
        .path("/org/mpris/MediaPlayer2")?
        .interface("org.mpris.MediaPlayer2.Player")?
        .cache_properties(zbus::proxy::CacheProperties::No)
        .build()
        .await?;

    Ok(proxy)
}
