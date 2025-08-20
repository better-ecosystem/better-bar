use zbus::{Connection, Proxy};
use anyhow::Result;

pub async fn get_battery_info() -> Result<i32> {
    // Connect to system bus
    let connection = Connection::system().await?;

    // Create proxy to UPower manager
    let upower = Proxy::new(
        &connection,
        "org.freedesktop.UPower",
        "/org/freedesktop/UPower",
        "org.freedesktop.UPower",
    )
    .await?;

    let devices: Vec<zvariant::OwnedObjectPath> = upower.call("EnumerateDevices", &()).await?;
    let battery_path = devices
        .iter()
        .find(|p| p.as_str().contains("battery"))
        .ok_or_else(|| anyhow::anyhow!("No battery device found"))?;

    let battery = Proxy::new(
        &connection,
        "org.freedesktop.UPower",
        battery_path.as_str(),
        "org.freedesktop.UPower.Device",
    )
    .await?;

    // Get battery values
    let percentage: f64 = battery.get_property("Percentage").await?;
    let state: u32 = battery.get_property("State").await?;

    let percentage_i32 = percentage.round() as i32;

    Ok(percentage_i32)
}
