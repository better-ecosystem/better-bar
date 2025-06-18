use std::error::Error;
use zbus::{Connection, Proxy};
use zvariant::OwnedObjectPath;

pub async fn get_network_info() -> Result<String, Box<dyn Error>> {
    // Connect to system bus
    let connection = Connection::system().await?;
    // Create proxy to NNetworkManager
    let nm = Proxy::new(
        &connection,
        "org.freedesktop.NetworkManager",
        "/org/freedesktop/NetworkManager",
        "org.freedesktop.NetworkManager",
    )
    .await?;

    // Get all active connections
    let active_connections: Vec<OwnedObjectPath> = nm.get_property("ActiveConnections").await?;

    for conn_path in active_connections {
        let conn = Proxy::new(
            &connection,
            "org.freedesktop.NetworkManager",
            conn_path.as_str(),
            "org.freedesktop.NetworkManager.Connection.Active",
        )
        .await?;

        // Get connection type and state
        let conn_type: String = conn.get_property("Type").await?;
        let state: u32 = conn.get_property("State").await?;

        if state == 2 {
            match conn_type.as_str() {
                "802-11-wireless" => {
                    // Get the specific connection details
                    let specific_object: OwnedObjectPath =
                        conn.get_property("SpecificObject").await?;

                    if !specific_object.as_str().is_empty() {
                        let ap = Proxy::new(
                            &connection,
                            "org.freedesktop.NetworkManager",
                            specific_object.as_str(),
                            "org.freedesktop.NetworkManager.AccessPoint",
                        )
                        .await?;

                        let ssid_bytes: Vec<u8> = ap.get_property("Ssid").await?;
                        let ssid = String::from_utf8_lossy(&ssid_bytes);

                        // Get signal strength for more detailed icon
                        let strength: u8 = ap.get_property("Strength").await.unwrap_or(0);
                        let icon = get_wifi_icon(strength);

                        return Ok(format!("{} {}", icon, ssid));
                    }
                }
                "802-3-ethernet" => {
                    return Ok("󰈁 Wired".to_string());
                }
                _ => continue,
            }
        }
    }

    // Check if WiFi is available but not connected
    let devices: Vec<OwnedObjectPath> = nm.call("GetDevices", &()).await?;

    for device_path in devices {
        let device = Proxy::new(
            &connection,
            "org.freedesktop.NetworkManager",
            device_path.as_str(),
            "org.freedesktop.NetworkManager.Device",
        )
        .await?;

        let device_type: u32 = device.get_property("DeviceType").await?;
        let state: u32 = device.get_property("State").await?;

        // Device type 2 = WiFi, state 30 = connected
        if device_type == 2 && state < 30 {
            return Ok("󰖪 WiFi Disconnected".to_string());
        }
    }

    Ok("󰖪 Disconnected".to_string())
}

// WiFi signal strength icons
fn get_wifi_icon(strength: u8) -> &'static str {
    match strength {
        0..=25 => "󰤟",   // Weak signal
        26..=50 => "󰤢",  // Medium signal
        51..=75 => "󰤥",  // Good signal
        76..=100 => "󰤨", // Excellent signal
        _ => "󰖩",        // Default WiFi icon
    }
}
