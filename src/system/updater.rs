// Updates info of all modules
use crate::config::config_helper::get_config;
use crate::ui::{
    modules::{
        battery::battery_updater::BatteryUpdater, network::network_updater::NetworkUpdater,
        panel::PanelState, volume::volume::start_volume_monitor,
    },
};

use crate::utils::logger::{LogLevel, Logger};
use gtk::{glib, prelude::*};
use lazy_static::lazy_static;

lazy_static! {
    static ref LOG: Logger = Logger::new("updater", LogLevel::Debug);
}

pub struct SystemUpdater {
    panel_state: PanelState,
}

impl SystemUpdater {
    pub fn new(panel_state: PanelState) -> Self {
        Self { panel_state }
    }

    pub fn start(&self) {
        LOG.debug("Starting system updaters");
        let config = get_config().unwrap();
        self.start_clock_updates();
        if config.modules.network {
            self.start_network_updates();
        }
        if config.modules.battery {
            self.start_battery_updates();
        }
        if config.modules.volume {
            self.start_volume_updates();
        }
    }

    fn start_clock_updates(&self) {
        let time_label = self.panel_state._time_label.clone();
        glib::timeout_add_seconds_local(1, move || {
            let now = chrono::Local::now();
            time_label.set_text(&now.format("%I:%M").to_string());
            glib::ControlFlow::Continue
        });
    }

    fn start_battery_updates(&self) {
        if let Some(ref battery_box) = self.panel_state._battery_box {

            let label = gtk::Label::new(None);
            let icon = gtk::Image::new();
            battery_box.append(&icon);
            battery_box.append(&label);
            

            glib::spawn_future_local(async move {
                BatteryUpdater::start(label, icon);
            });
        } else {
            LOG.debug("Battery module disabled, skipping battery updates");
        }
    }

    // Update network info
    fn start_network_updates(&self) {
        if let Some(ref network_label) = self.panel_state._network_label {
            let label_clone = network_label.clone();
            glib::spawn_future_local(async move {
                NetworkUpdater::start(label_clone);
            });
        } else {
            LOG.debug("Network module disabled, skipping network updates");
        }
    }

    // For updating volume
    fn start_volume_updates(&self) {
        if let Some(ref volume_label) = self.panel_state._volume_label {
            LOG.debug("started volume update");
            let volume_label_clone = volume_label.clone();
            glib::spawn_future_local(async move {
                match std::panic::catch_unwind(|| start_volume_monitor()) {
                    Ok(mut volume_rx) => {
                        LOG.debug("volume monitor started successfully");
                        while volume_label_clone.is_visible() {
                            match volume_rx.recv().await {
                                Some(volume) => {
                                    volume_label_clone.set_text(&volume.percentage.to_string());
                                    LOG.debug("updated volume label");
                                }
                                None => {
                                    LOG.debug("volume monitor channel closed");
                                    break;
                                }
                            }
                        }
                        LOG.debug("volume updater stopped");
                    }
                    Err(e) => {
                        LOG.error(&format!("Failed to start volume monitor: {:?}", e));
                    }
                }
            });
        } else {
            LOG.debug("Volume module disabled, skipping volume updates");
        }
    }
}
