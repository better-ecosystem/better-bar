// System info widget module
// mainly this is right side modules

use crate::config::config_helper;
use crate::ui::modules::volume::volume::{volume_down, volume_up};
use crate::ui::modules::{
    cpu::cpu_widget::create_cpu_widget,
    // memory::memory_widget::_create_memory_widget_percentage,
};
use gtk::{prelude::*, Box as GtkBox, EventControllerScroll, Label};

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
        GtkBox,
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

        let battery_box = GtkBox::new(gtk::Orientation::Horizontal, 2);
        battery_box.set_widget_name("battery");
        battery_box.add_css_class("modules");
        if config.modules.battery {
            container.append(&battery_box);
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

            // Add scroll behaviour on volume label
            let scroll = EventControllerScroll::new(gtk::EventControllerScrollFlags::VERTICAL);

            scroll.connect_scroll(|_controller, _dx,dy | {
                if dy > 0.0 {
                    
                    tokio::spawn(async { let _ = volume_down().await; });
                }else if dy < 0.0{

                    tokio::spawn(async { let _ = volume_up().await; });
                }

                true.into()
            });

            volume_label.add_controller(scroll);
            
        }

        if config.modules.cpu || config.modules.battery || config.modules.network || config.modules.volume {
            Some((
                cpu_label,
                battery_box,
                network_label,
                volume_label,
            ))
        } else {
            None
        }
    }
}
