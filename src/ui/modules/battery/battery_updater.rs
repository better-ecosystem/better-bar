use gtk::{glib, prelude::*};
use lazy_static::lazy_static;
use crate::ui::logger::{LogLevel, Logger};
use super::battery_info::get_battery_info;

lazy_static! {
    static ref LOG: Logger = Logger::new("battery",LogLevel::Debug);
}

pub struct BatteryUpdater;

impl BatteryUpdater {
    pub fn start(battery_label: gtk::Label) {
        LOG.debug("started battery updates");
        
         glib::timeout_add_seconds_local(3, move || {
            let label_clone = battery_label.clone();
            
            glib::spawn_future_local(async move {
                match get_battery_info().await {
                    Ok(battery_info) => {
                        Self::update_battery_display(&label_clone, &battery_info);
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
    
    fn update_battery_display(battery_label: &gtk::Label, battery_info: &str) {
        battery_label.set_text(battery_info);
        battery_label.remove_css_class("warning");
        battery_label.remove_css_class("critical");
        
        if let Some(percentage_str) = battery_info.split_whitespace().nth(1) {
            if let Ok(level) = percentage_str.trim_end_matches('%').parse::<f32>() {
                if level < 15.0 {
                    battery_label.add_css_class("critical");
                } else if level < 30.0 {
                    battery_label.add_css_class("warning");
                }
            }
        }
    }
}