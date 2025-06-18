// Updates info of all modules
use crate::ui::{
    logger::{LogLevel, Logger},
    modules::{
        battery::battery_updater::BatteryUpdater,
        network::network_updater::NetworkUpdater,
        panel::PanelState,
        volume::volume::start_volume_monitor,
    },
};
use gtk::{glib, prelude::*};
use lazy_static::lazy_static;

lazy_static! {
    static ref LOG: Logger = Logger::new(LogLevel::Debug);
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
        
        self.start_clock_updates();
        self.start_network_updates();
        self.start_battery_updates();
        self.start_volume_updates();
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
        let label_clone = self.panel_state._battery_label.clone();
        glib::spawn_future_local(async move {
            BatteryUpdater::start(label_clone);
        });
    }

    // Update network info 
    fn start_network_updates(&self) {
        let label_clone = self.panel_state._network_label.clone();
        
        glib::spawn_future_local(async move {
            NetworkUpdater::start(label_clone);
        });
    }


    // For updating volume
    fn start_volume_updates(&self) {
        LOG.debug("started volume update");
        let volume_label = self.panel_state._volume_label.clone();
        
        glib::spawn_future_local(async move {
            match std::panic::catch_unwind(|| start_volume_monitor()) {
                Ok(mut volume_rx) => {
                    LOG.debug("updater -> volume monitor started successfully");
                    
                    while volume_label.is_visible() {
                        match volume_rx.recv().await {
                            Some(volume) => {
                                volume_label.set_text(&volume);
                                LOG.debug("updater ->  updated volume label");
                            }
                            None => {
                                LOG.debug("updater -> volume monitor channel closed");
                                break;
                            }
                        }
                    }
                    LOG.debug("updater -> volume updater stopped");
                }
                Err(e) => {
                    LOG.error(&format!("updater -> Failed to start volume monitor: {:?}", e));
                }
            }
        });
    }
}