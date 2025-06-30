use gtk::{glib, prelude::WidgetExt, Label};
use lazy_static::lazy_static;
use crate::ui::logger::{LogLevel, Logger};
use super::network_info::get_network_info;

lazy_static! {
    static ref LOG: Logger = Logger::new("network",LogLevel::Debug);
}

pub struct NetworkUpdater;

impl NetworkUpdater {
    pub fn start(network_label: Label) {
        LOG.debug("network updater -> started network updates");
        
         glib::timeout_add_seconds_local(3, move || {
            let label_clone = network_label.clone();
            
            glib::spawn_future_local(async move {
                match get_network_info().await {
                    Ok(network_info) => {
                        label_clone.set_text(&network_info);
                        LOG.debug("network updater: updated network label");
                    }
                    Err(e) => {
                        LOG.error(&format!("network updater -> network update error: {}", e));
                        label_clone.set_text("Network Error");
                    }
                }
            });
            
            if network_label.is_visible() {
                glib::ControlFlow::Continue
            } else {
                LOG.debug("network updater -> network updater stopped");
                glib::ControlFlow::Break
            }
        });

    }
}