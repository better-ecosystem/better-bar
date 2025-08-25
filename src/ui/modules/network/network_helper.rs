use std::{fs, process::Command};
use anyhow::Result;

use crate::ui::modules::network::network::NetworkInfo;

pub fn get_ip_address(interface: &str) -> Result<String> {
    let output = Command::new("ip")
        .args(&["addr", "show", interface])
        .output()?;

    let addr_info = String::from_utf8_lossy(&output.stdout);
    for line in addr_info.lines() {
        if line.contains("inet ") && !line.contains("inet6") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(addr_part) = parts.iter().find(|&&x| x.contains('.')) {
                if let Some(ip) = addr_part.split('/').next() {
                    return Ok(ip.to_string());
                }
            }
        }
    }
    Ok("0.0.0.0".to_string())
}

pub fn get_wifi_name(interface: &str) -> Result<String> {
    // Try iwconfig first
    if let Ok(output) = Command::new("iwconfig").arg(interface).output() {
        let config = String::from_utf8_lossy(&output.stdout);
        for line in config.lines() {
            if line.contains("ESSID:") {
                if let Some(essid_part) = line.split("ESSID:").nth(1) {
                    let essid = essid_part.trim().trim_matches('"');
                    if !essid.is_empty() && essid != "off/any" {
                        return Ok(essid.to_string());
                    }
                }
            }
        }
    }

    // use nmcli as fallback if iwconfig failed by any chance
    if let Ok(output) = Command::new("nmcli")
        .args(&["-t", "-f", "active,ssid", "dev", "wifi"])
        .output()
    {
        let wifi_info = String::from_utf8_lossy(&output.stdout);
        for line in wifi_info.lines() {
            if line.starts_with("yes:") {
                if let Some(ssid) = line.strip_prefix("yes:") {
                    if !ssid.is_empty() {
                        return Ok(ssid.to_string());
                    }
                }
            }
        }
    }

    Ok("Unknown Network".to_string())
}

pub fn get_wifi_signal_strength(interface: &str) -> i32 {
    // Try iwconfig
    if let Ok(output) = Command::new("iwconfig").arg(interface).output() {
        let config = String::from_utf8_lossy(&output.stdout);
        for line in config.lines() {
            if line.contains("Signal level=") {
                if let Some(signal_part) = line.split("Signal level=").nth(1) {
                    if let Some(signal_str) = signal_part.split_whitespace().next() {
                        if let Ok(signal_dbm) = signal_str.trim_end_matches("dBm").parse::<i32>() {
                            // Convert dBm to percentage (rough approximation)
                            return ((signal_dbm + 100) * 2).max(0).min(100);
                        }
                    }
                }
            }
        }
    }

    // Try /proc/net/wireless
    if let Ok(wireless) = fs::read_to_string("/proc/net/wireless") {
        for line in wireless.lines().skip(2) {
            if line.contains(interface) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    if let Ok(quality) = parts[2].trim_end_matches('.').parse::<f32>() {
                        return (quality * 100.0 / 70.0) as i32; // Assume max quality is 70
                    }
                }
            }
        }
    }

    0
}

pub fn get_network_speeds(interface: &str) -> (String, String) {
    if let Ok(rx_bytes) =
        fs::read_to_string(format!("/sys/class/net/{}/statistics/rx_bytes", interface))
    {
        if let Ok(tx_bytes) =
            fs::read_to_string(format!("/sys/class/net/{}/statistics/tx_bytes", interface))
        {
            let rx = rx_bytes.trim().parse::<u64>().unwrap_or(0);
            let tx = tx_bytes.trim().parse::<u64>().unwrap_or(0);

            return (
                format_bytes(rx / 1000), // Rough estimation
                format_bytes(tx / 1000),
            );
        }
    }
    ("0 B/s".to_string(), "0 B/s".to_string())
}

pub fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B/s", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB/s", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB/s", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1} GB/s", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}


pub fn get_wifi_frequency(interface: &str) -> Result<String> {
    if let Ok(output) = Command::new("iwconfig").arg(interface).output() {
        let config = String::from_utf8_lossy(&output.stdout);
        for line in config.lines() {
            if line.contains("Frequency:") {
                if let Some(freq_part) = line.split("Frequency:").nth(1) {
                    let freq = freq_part.split_whitespace().next().unwrap_or("-");
                    return Ok(freq.to_string());
                }
            }
        }
    }
    Ok("N/A".to_string())
}

/// Map icons according to connection
/// TODO: map other icons also
pub fn get_network_icon(info: &NetworkInfo) -> &'static str {
    if !info.is_connected {
        return "network-offline-symbolic";
    }

    match info.connection_type.as_str() {
        "WiFi" => match info.signal_strength {
            76..=100 => "network-wireless-signal-excellent-symbolic",
            51..=75 => "network-wireless-signal-good-symbolic",
            26..=50 => "network-wireless-signal-ok-symbolic",
            1..=25 => "network-wireless-signal-weak-symbolic",
            _ => "network-wireless-signal-none-symbolic",
        },
        "Ethernet" => "network-wired-symbolic",
        _ => "network-error-symbolic",
    }
}
