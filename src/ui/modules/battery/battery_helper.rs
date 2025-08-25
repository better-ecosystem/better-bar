
use std::time::Duration;
use battery::{units::time::second, State};

use crate::config::config::BatteryConfig;

pub fn quantity_to_duration(q: Option<battery::units::Time>) -> Option<Duration> {
    q.map(|t| Duration::from_secs_f32(t.get::<second>()))
}

// Map battery percentage and charging state to icon
pub fn get_battery_icon(percentage: f32, charging: bool) -> &'static str {
    match percentage {
        90.0..=100.0 => if charging { "battery-full-charging-symbolic" } else { "battery-full-symbolic" },
        60.0..=89.0 => if charging { "battery-good-charging-symbolic" } else { "battery-good-symbolic" },
        30.0..=59.0 => if charging { "battery-medium-charging-symbolic" } else { "battery-medium-symbolic" },
        10.0..=29.0 => if charging { "battery-low-charging-symbolic" } else { "battery-low-symbolic" },
        0.0..=9.0 => if charging { "battery-caution-charging-symbolic" } else { "battery-caution-symbolic" },
        _ => "battery-missing-symbolic",
    }
}

// Format main widget text
pub fn format_battery_text(
    percentage: f32,
    state: State,
    time_to_full: Option<Duration>,
    time_to_empty: Option<Duration>,
    config: &BatteryConfig,
) -> String {
    let mut text = config.format.clone();
    text = text.replace("{percentage}", &format!("{:.0}%", percentage));
    text = text.replace("{state}", state_to_str(state));
    text = text.replace("{time}", &format_time_label(percentage, state, time_to_full, time_to_empty));
    text.replace("{icon}", "")
}

// Format tooltip
pub fn format_battery_tooltip(
    percentage: f32,
    state: State,
    time_to_full: Option<Duration>,
    time_to_empty: Option<Duration>,
    config: &BatteryConfig,
) -> String {
    let mut tooltip = config.tooltip_format.clone();
    tooltip = tooltip.replace("{percentage}", &format!("{:.0}%", percentage));
    tooltip = tooltip.replace("{state}", state_to_str(state));
    tooltip = tooltip.replace("{time}", &format_time_label(percentage, state, time_to_full, time_to_empty));
    tooltip.replace("{icon}", "")
}

// Convert State enum to string
fn state_to_str(state: State) -> &'static str {
    match state {
        State::Charging => "Charging",
        State::Discharging => "Discharging",
        State::Empty => "Empty",
        State::Full => "Full",
        State::Unknown => "Unknown",
        _ => "ERROR_STATE"
    }
}

// Format duration into a nice label
fn format_time_label(
    percentage: f32,
    state: State,
    time_to_full: Option<Duration>,
    time_to_empty: Option<Duration>,
) -> String {
    match state {
        State::Charging => {
            if percentage >= 100.0 {
                return "Full".to_string();
            }
            if let Some(duration) = time_to_full {
                let total_secs = duration.as_secs();
                let hours = total_secs / 3600;
                let minutes = (total_secs % 3600) / 60;
                if hours > 0 {
                    format!("Full in {:02}:{:02}", hours, minutes)
                } else {
                    format!("Full in {}m", minutes)
                }
            } else {
                "Full in Unknown".to_string()
            }
        }
        State::Discharging => {
            if let Some(duration) = time_to_empty {
                let total_secs = duration.as_secs();
                let hours = total_secs / 3600;
                let minutes = (total_secs % 3600) / 60;
                if hours > 0 {
                    format!("Empty in {:02}:{:02}", hours, minutes)
                } else {
                    format!("Empty in {}m", minutes)
                }
            } else {
                "Empty in Unknown".to_string()
            }
        }
        State::Full => "Full".to_string(),
        _ => "Unknown".to_string(),
    }
}

