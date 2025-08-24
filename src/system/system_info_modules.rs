// System info widget module
// mainly this is right side modules

use crate::config::config_helper;
use crate::ui::modules::{
    cpu::cpu_widget::create_cpu_widget,
    // memory::memory_widget::_create_memory_widget_percentage,
};
use gtk::{prelude::*, Box as GtkBox, Label};

pub struct SystemInfoModule;

impl SystemInfoModule {
    pub fn new() -> Self {
        Self
    }

    pub fn create(
        &self,
        container: &GtkBox,
    ) -> Option<(
        Label,
        Label,
    )> {
        let config = config_helper::get_config().expect("Failed to get configuration");

        // let memory_label = _create_memory_widget_percentage();
        // memory_label.set_widget_name("memory");
        // memory_label.add_css_class("modules");
        // container.append(&memory_label);

        let cpu_label = create_cpu_widget();
        cpu_label.set_widget_name("cpu");
        cpu_label.add_css_class("modules");
        if config.modules.cpu {
            container.append(&cpu_label);
        }

        let network_label = Label::new(Some("󰖩 --"));
        network_label.set_widget_name("network");
        network_label.add_css_class("modules");
        if config.modules.network {
            container.append(&network_label);
        }

        if config.modules.cpu || config.modules.network {
            Some((
                cpu_label,
                network_label,
            ))
        } else {
            None
        }
    }
}
