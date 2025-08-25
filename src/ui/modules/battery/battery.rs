/* BATTERY MODULE */

use gtk::{Box, Image, Label, Orientation, glib, prelude::*};
use battery::{units::{ratio::percent}, Manager, State};
use crate::{config::config::BatteryConfig, ui::modules::battery::battery_helper::{format_battery_text, format_battery_tooltip, get_battery_icon, quantity_to_duration}};

pub struct Battery {
    widget: Box,
    label: Label,
    icon: Image,
    config: BatteryConfig,
}

impl Battery {
    pub fn new(config: BatteryConfig) -> Self {
        let battery_box = Box::new(Orientation::Horizontal, 2);
        battery_box.add_css_class("modules");

        let icon = Image::new();
        let label = Label::new(None);
        battery_box.append(&icon);
        battery_box.append(&label);

        Self {
            widget: battery_box,
            label,
            icon,
            config,
        }
    }

    pub fn widget(&self) -> &Box {
        &self.widget
    }

    pub fn start_updates(self) {
        let label = self.label.clone();
        let icon = self.icon.clone();
        let widget = self.widget.clone();
        let config = self.config.clone();

        glib::timeout_add_seconds_local(3, move || {
            let label = label.clone();
            let icon = icon.clone();
            let widget = widget.clone();
            let config = config.clone();

            glib::spawn_future_local(async move {
                let manager = match Manager::new() {
                    Ok(m) => m,
                    Err(e) => {
                        label.set_text(&format!("Battery Error: {}", e));
                        return;
                    }
                };

                let battery = match manager.batteries().ok().and_then(|mut b| b.next()) {
                    Some(Ok(b)) => b,
                    _ => {
                        label.set_text("No Battery Found");
                        return;
                    }
                };

                let percentage = battery.state_of_charge().get::<percent>();
                let state = battery.state();

                let time_to_full = quantity_to_duration(battery.time_to_full());
                let time_to_empty = quantity_to_duration(battery.time_to_empty());

                icon.set_icon_name(Some(get_battery_icon(percentage, state == State::Charging)));

                label.set_text(&format_battery_text(percentage, state, time_to_full, time_to_empty, &config));

                if config.tooltip {
                    widget.set_tooltip_markup(Some(&format_battery_tooltip(
                        percentage,
                        state,
                        time_to_full,
                        time_to_empty,
                        &config,
                    )));
                }
            });

            glib::ControlFlow::Continue
        });
    }
}
