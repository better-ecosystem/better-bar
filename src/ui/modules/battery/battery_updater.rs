use super::battery_info::get_battery_info;
use crate::utils::logger::{LogLevel, Logger};
use gtk::{glib, prelude::*};
use lazy_static::lazy_static;

lazy_static! {
    static ref LOG: Logger = Logger::new("battery", LogLevel::Debug);
}

pub struct BatteryUpdater;

impl BatteryUpdater {
    pub fn start(battery_label: gtk::Label, battery_icon: gtk::Image) {
        LOG.debug("started battery updates");

        glib::timeout_add_seconds_local(3, move || {
            let label_clone = battery_label.clone();
            let icon_clone = battery_icon.clone();

            glib::spawn_future_local(async move {
                match get_battery_info().await {
                    Ok(percentage) => {
                        Self::update_battery_display(
                            &label_clone,
                            icon_clone,
                            percentage,
                        );
                        LOG.debug("updated battery label");
                    }
                    Err(e) => {
                        LOG.error(&format!("Battery update error: {}", e));
                        label_clone.set_text("Battery Error");
                    }
                }
            });

            if battery_label.is_visible() {
                glib::ControlFlow::Continue
            } else {
                LOG.debug("battery updater stopped");
                glib::ControlFlow::Break
            }
        });
    }

    fn update_battery_display(
        battery_label: &gtk::Label,
        battery_icon: gtk::Image,
        percentage: i32,
    ) {
        battery_label.set_text(&format!("{}%", percentage));

        let icon_name = if percentage < 15 {
            "battery-empty"
        } else if percentage < 30 {
            "battery-low"
        } else if percentage < 80 {
            "battery-medium"
        } else {
            "battery-full"
        };
        battery_icon.set_icon_name(Some(icon_name));

        if let Some(parent) = battery_label.parent() {
            parent.remove_css_class("warning");
            parent.remove_css_class("critical");

            if percentage < 15 {
                parent.add_css_class("critical");
            } else if percentage < 30 {
                parent.add_css_class("warning");
            }
        }
    }
}
