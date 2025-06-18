use zbus::{Connection, Proxy};
use std::error::Error;

pub async fn get_battery_info() -> Result<String, Box<dyn Error>> {
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
        .ok_or("No battery device found")?;

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
    let charging = state == 1;

    let icon = get_battery_icon(percentage_i32, charging);
    let info = format!("{} {}%", icon, percentage_i32);

    Ok(info)
}

// Icons according to battery state and percentage 
fn get_battery_icon(percentage: i32, charging: bool) -> &'static str {
    if charging {
        match percentage {
            0..=10 => "󰢜",
            11..=20 => "󰂆",
            21..=30 => "󰂇",
            31..=40 => "󰂈",
            41..=50 => "󰢝",
            51..=60 => "󰂉",
            61..=70 => "󰢞",
            71..=80 => "󰂊",
            81..=90 => "󰂋",
            91..=100 => "󰂅",
            _ => "󰂄",
        }
    } else {
        match percentage {
            0..=10 => "󰂎",
            11..=20 => "󰁻",
            21..=30 => "󰁼",
            31..=40 => "󰁽",
            41..=50 => "󰁾",
            51..=60 => "󰁿",
            61..=70 => "󰂀",
            71..=80 => "󰂁",
            81..=90 => "󰂂",
            91..=100 => "󰁹",
            _ => "󰂃",
        }
    }
}
