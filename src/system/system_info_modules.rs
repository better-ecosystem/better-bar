// System info widget module
// mainly this is right side modules

use crate::config::config_helper;
use crate::ui::modules::{
    cpu::cpu_widget::create_cpu_widget,
    // memory::memory_widget::_create_memory_widget_percentage,
};
use gtk::{Box as GtkBox, Label, prelude::*};

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

        let battery_label = Label::new(Some("󰂎 --"));
        battery_label.set_widget_name("battery");
        battery_label.add_css_class("modules");
        if config.modules.battery {
            container.append(&battery_label);
        }

        let network_label = Label::new(Some("󰖩 --"));
        network_label.set_widget_name("network");
        network_label.add_css_class("modules");
        if config.modules.network {
            container.append(&network_label);
        }

        let volume_label = Label::new(Some("󰕿 --"));
        volume_label.set_widget_name("volume");
        volume_label.add_css_class("modules");
        if config.modules.volume {
            container.append(&volume_label);
        }

        if config.modules.cpu || config.modules.battery || config.modules.network || config.modules.volume {
            Some((
                cpu_label,
                battery_label,
                network_label,
                volume_label,
            ))
        } else {
            None
        }
    }
}
