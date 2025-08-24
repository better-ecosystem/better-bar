use anyhow::Result;
use gtk::{Box, Image, Label, Orientation, glib, prelude::*};
use zbus::{Connection, Proxy};

use crate::config::config::BatteryConfig;

pub struct Battery {
    widget: Box,           // container shown in the bar
    label: Label,          // percentage/state label
    icon: Image,           // battery icon
    config: BatteryConfig, // user config
}

impl Battery {
    // Retuns a battery box with battery info
    // uses config to show the widget
    // by default it will show icon and percentage
    // and a tooltip with time to full/ time to empty according to the battery state
    pub fn new(config: BatteryConfig) -> Self {
        let hbox = Box::new(Orientation::Horizontal, 2);
        hbox.add_css_class("modules");

        let icon = Image::new();
        let label = Label::new(None);
        hbox.append(&icon);
        hbox.append(&label);

        Self {
            widget: hbox,
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
        let config = self.config.clone();
        let widget = self.widget.clone();

        glib::timeout_add_seconds_local(3, move || {
            let label = label.clone();
            let icon = icon.clone();
            let config = config.clone();
            let widget = widget.clone();

            glib::spawn_future_local(async move {
                match get_battery_info().await {
                    Ok(b) => {
                        if let Some(perc) = b.percentage {
                            let charging = matches!(b.state.as_deref(), Some("Charging"));
                            let icon_name = get_battery_icon(perc, charging);
                            icon.set_icon_name(Some(icon_name));
                        }

                        // Text based on format
                        let mut text = config.format.clone();

                        if let Some(perc) = b.percentage {
                            text = text.replace("{percentage}", &format!("{}", perc));
                        }
                        if let Some(ref state) = b.state {
                            text = text.replace("{state}", &state);
                        }
                        if let Some(ref time) = b.time {
                            text = text.replace("{time}", &format!("{}m", time));
                        }

                        // icons can't be text right ?
                        text = text.replace("{icon}", "");
                        label.set_text(&text);

                        // Tooltip for the widget
                        if config.tooltip {
                            let mut tooltip = config.tooltip_format.clone();
                            if let Some(perc) = b.percentage {
                                tooltip = tooltip.replace("{percentage}", &format!("{}", perc));
                            }
                            if let Some(state) = &b.state {
                                tooltip = tooltip.replace("{state}", state);
                            }
                            if let Some(time) = b.time {
                                tooltip = tooltip.replace("{time}", &format!("{}m", time));
                            }
                            tooltip = tooltip.replace("{icon}", "");

                            widget.set_tooltip_text(Some(&tooltip));
                        }
                    }
                    Err(e) => {
                        label.set_text(&format!("Battery Error: {}", e));
                    }
                }
            });

            glib::ControlFlow::Continue
        });
    }
}

pub struct BatteryInfo {
    pub icon: Option<Image>,
    pub percentage: Option<i32>,
    pub state: Option<String>,
    pub time: Option<String>,
}

pub async fn get_battery_info() -> Result<BatteryInfo> {
    let connection = Connection::system().await?;

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

    let battery_proxy = Proxy::new(
        &connection,
        "org.freedesktop.UPower",
        battery_path.as_str(),
        "org.freedesktop.UPower.Device",
    )
    .await?;

    let percentage: f64 = battery_proxy.get_property("Percentage").await?;
    let state: u32 = battery_proxy.get_property("State").await?;
    let time_to_full: f64 = battery_proxy
        .get_property("TimeToFull")
        .await
        .unwrap_or(0.0);
    let time_to_empty: f64 = battery_proxy
        .get_property("TimeToEmpty")
        .await
        .unwrap_or(0.0);

    let time = if state == 1 {
        Some(format!("{}", time_to_full.round() as i32))
    } else if state == 2 {
        Some(format!("{}", time_to_empty.round() as i32))
    } else {
        None
    };

    Ok(BatteryInfo {
        icon: Some(Image::from_icon_name(get_battery_icon(
            percentage.round() as i32,
            state == 1,
        ))),
        percentage: Some(percentage.round() as i32),
        state: Some(match state {
            1 => "Charging".into(),
            2 => "Discharging".into(),
            4 => "Fully Charged".into(),
            _ => "Unknown".into(),
        }),
        time,
    })
}

/// Icon mapping function
pub fn get_battery_icon(percentage: i32, charging: bool) -> &'static str {
    match percentage {
        90..=100 => {
            if charging {
                "battery-full-charging-symbolic"
            } else {
                "battery-full-symbolic"
            }
        }
        60..=89 => {
            if charging {
                "battery-good-charging-symbolic"
            } else {
                "battery-good-symbolic"
            }
        }
        30..=59 => {
            if charging {
                "battery-medium-charging-symbolic"
            } else {
                "battery-medium-symbolic"
            }
        }
        10..=29 => {
            if charging {
                "battery-low-charging-symbolic"
            } else {
                "battery-low-symbolic"
            }
        }
        0..=9 => {
            if charging {
                "battery-caution-charging-symbolic"
            } else {
                "battery-caution-symbolic"
            }
        }
        _ => "battery-missing-symbolic",
    }
}
