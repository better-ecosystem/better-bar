// Updates info of all modules
use crate::ui::{
    modules::{
        panel::PanelState,
    },
};

use crate::utils::logger::{LogLevel, Logger};
use gtk::{glib};
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
        self.start_clock_updates();
    }

    fn start_clock_updates(&self) {
        let time_label = self.panel_state._time_label.clone();
        glib::timeout_add_seconds_local(1, move || {
            let now = chrono::Local::now();
            time_label.set_text(&now.format("%I:%M").to_string());
            glib::ControlFlow::Continue
        });
    }
}
