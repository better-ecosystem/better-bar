/* NETWORK MODULE */

use gtk::{Box, Image, Label, Orientation, glib, prelude::*};
use std::fs;
use std::process::Command;

use crate::config::config::NetworkConfig;
use crate::ui::modules::network::network_helper::{get_ip_address, get_network_icon, get_network_speeds, get_wifi_frequency, get_wifi_name, get_wifi_signal_strength};

pub struct Network {
    widget: Box,           // container shown in the bar
    label: Label,          // network info label
    icon: Image,           // network icon
    config: NetworkConfig, // user config
}

impl Network {
    /// Returns a network box with network info
    /// uses config to show the widget
    /// by default it will show icon and network name/status
    /// and a tooltip with detailed network information
    pub fn new(config: NetworkConfig) -> Self {
        let network_box = Box::new(Orientation::Horizontal, 2);
        network_box.add_css_class("modules");

        let icon = Image::new();
        let label = Label::new(None);
        network_box.append(&icon);
        network_box.append(&label);

        Self {
            widget: network_box,
            label,
            icon,
            config,
        }
    }

    pub fn widget(&self) -> &Box {
        &self.widget
    }

    /// Start periodic updates
    pub fn start_updates(self) {
        let label = self.label.clone();
        let icon = self.icon.clone();
        let widget = self.widget.clone();
        let config = self.config.clone();

        glib::timeout_add_seconds_local(2, move || {
            let label = label.clone();
            let icon = icon.clone();
            let widget = widget.clone();
            let config = config.clone();

            let info = Self::get_network_info();

            // Update UI on main thread
            // FIXME: FIX THE UI FREEZE
            label.set_text(&Self::format_network_text(&info, &config));
            icon.set_icon_name(Some(get_network_icon(&info)));
            if config.tooltip {
                widget.set_tooltip_markup(Some(&Self::format_network_tooltip(&info, &config)));
            }

            glib::ControlFlow::Continue
        });
    }

    fn format_network_text(info: &NetworkInfo, config: &NetworkConfig) -> String {
        let mut text = config.format.clone();
        text = text
            .replace("{device}", &info.device)
            .replace("{ip}", &info.ip_address)
            .replace("{name}", &info.wifi_name)
            .replace("{signal}", &format!("{}%", info.signal_strength))
            .replace("{type}", &info.connection_type)
            .replace(
                "{status}",
                if info.is_connected {
                    "Connected"
                } else {
                    "Disconnected"
                },
            )
            .replace("{download}", &info.download_speed)
            .replace("{upload}", &info.upload_speed)
            .replace("{frequency}", &info.frequency)
            .replace("{icon}", "");
        text
    }

    fn format_network_tooltip(info: &NetworkInfo, config: &NetworkConfig) -> String {
        let mut tooltip = config.tooltip_format.clone();
        tooltip = tooltip
            .replace("{device}", &info.device)
            .replace("{ip}", &info.ip_address)
            .replace("{name}", &info.wifi_name)
            .replace("{signal}", &format!("{}%", info.signal_strength))
            .replace("{type}", &info.connection_type)
            .replace(
                "{status}",
                if info.is_connected {
                    "Connected"
                } else {
                    "Disconnected"
                },
            )
            .replace("{download}", &info.download_speed)
            .replace("{upload}", &info.upload_speed)
            .replace("{frequency}", &info.frequency)
            .replace("{icon}", "");

        tooltip
    }

    fn get_network_info() -> NetworkInfo {
        let mut info = NetworkInfo::default();

        if let Some(interface) = get_active_interface() {
            info.device = interface.clone();
            info.ip_address = get_ip_address(&interface).unwrap_or_else(|_| "0.0.0.0".to_string());
            info.is_connected = !info.ip_address.is_empty() && info.ip_address != "0.0.0.0";

            if interface.starts_with("wl") || interface.contains("wifi") {

                info.connection_type = "WiFi".to_string();
                info.wifi_name = get_wifi_name(&interface).unwrap_or_else(|_| "Unknown Network".to_string());
                info.signal_strength = get_wifi_signal_strength(&interface);
                info.frequency = get_wifi_frequency(&interface).unwrap_or("-".to_string());
                
            } else if interface.starts_with("en") || interface.starts_with("eth") {
                info.connection_type = "Ethernet".to_string();
                info.wifi_name = "Wired Connection".to_string();
                info.signal_strength = 100;
                info.frequency = "-".to_string();
            }

            let (down, up) = get_network_speeds(&interface);
            info.download_speed = down;
            info.upload_speed = up;
        } else {
            // No active interface
            info.connection_type = "None".to_string();
            info.wifi_name = "Disconnected".to_string();
            info.signal_strength = 0;
            info.ip_address = "-".to_string();
            info.frequency = "-".to_string();
        }

        info
    }
}

#[derive(Debug, Clone)]
pub struct NetworkInfo {
    pub device: String,
    pub ip_address: String,
    pub wifi_name: String,
    pub signal_strength: i32,
    pub connection_type: String,
    pub is_connected: bool,
    pub download_speed: String,
    pub upload_speed: String,
    pub frequency: String,
}

impl Default for NetworkInfo {
    fn default() -> Self {
        Self {
            device: "No Device".to_string(),
            ip_address: "0.0.0.0".to_string(),
            wifi_name: "Disconnected".to_string(),
            signal_strength: 0,
            connection_type: "None".to_string(),
            is_connected: false,
            download_speed: "0 B/s".to_string(),
            upload_speed: "0 B/s".to_string(),
            frequency: "N/A".to_string(),
        }
    }
}

fn get_active_interface() -> Option<String> {
    // Try to get the default route interface
    if let Ok(output) = Command::new("ip")
        .args(&["route", "show", "default"])
        .output()
    {
        let route_info = String::from_utf8_lossy(&output.stdout);
        for line in route_info.lines() {
            if line.contains("default") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(dev_index) = parts.iter().position(|&x| x == "dev") {
                    if let Some(interface) = parts.get(dev_index + 1) {
                        return Some(interface.to_string());
                    }
                }
            }
        }
    }

    // Get first active interface
    // this is for the fallback
    if let Ok(interfaces) = fs::read_to_string("/proc/net/dev") {
        for line in interfaces.lines().skip(2) {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 {
                let interface = parts[0].trim();
                if !interface.is_empty() && interface != "lo" {
                    return Some(interface.to_string());
                }
            }
        }
    }

    None
}
