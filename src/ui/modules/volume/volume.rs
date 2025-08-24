use crate::{
    config::config::VolumeConfig,
    ui::modules::volume::{
        monitor::start_volume_monitor,
        volume_helper::{change_volume, toggle_mute},
        volume_info::VolumeInfo,
    },
};
use gtk::{
    Box, Image, Label,
    prelude::{BoxExt, GestureSingleExt, WidgetExt},
};
use std::sync::{Arc, Mutex};
use tokio::time::{Duration, sleep};

pub struct Volume {
    widget: Box,
    label: Label,
    icon: Image,
    config: VolumeConfig,
}

impl Volume {
    pub fn new(config: VolumeConfig) -> Self {
        let container = Box::new(gtk::Orientation::Horizontal, 4);
        container.add_css_class("modules");

        let icon = Image::new();
        let label = Label::new(None);

        container.append(&icon);
        container.append(&label);

        let gesture = gtk::GestureClick::new();
        gesture.set_button(gtk::gdk::BUTTON_PRIMARY);
        gesture.connect_pressed(move |_, _, _, _| {
            let _ = tokio::spawn(async move {
                if let Err(e) = toggle_mute().await {
                    eprintln!("Failed to toggle mute: {:?}", e);
                }
            });
        });

        container.add_controller(gesture);

        let pending_delta = Arc::new(Mutex::new(0i8));
        let pending_delta_clone = pending_delta.clone();

        let scroll = gtk::EventControllerScroll::new(gtk::EventControllerScrollFlags::VERTICAL);
        scroll.connect_scroll(move |_ctrl, _dx, dy| {
            let mut delta = pending_delta.lock().unwrap();
            if dy < 0.0 {
                *delta += 1;
            } else if dy > 0.0 {
                *delta -= 1;
            }
            true.into()
        });

        container.add_controller(scroll);

        tokio::spawn(async move {
            loop {
                sleep(Duration::from_millis(50)).await;
                let mut delta = pending_delta_clone.lock().unwrap();
                if *delta != 0 {
                    let amount = *delta;
                    *delta = 0;
                    tokio::spawn(async move {
                        let _ = change_volume(amount).await;
                    });
                }
            }
        });

        Self {
            widget: container,
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

        // Spawn async task for live updates
        glib::spawn_future_local(async move {
            match std::panic::catch_unwind(|| start_volume_monitor()) {
                Ok(mut rx) => {
                    while widget.is_visible() {
                        match rx.recv().await {
                            Some(volume) => update_widget(&config, &widget, &label, &icon, &volume),

                            None => break,
                        }
                    }
                }
                Err(e) => eprintln!("Failed to start volume monitor: {:?}", e),
            }
        });
    }
}

/// Updates volume in ui
fn update_widget(
    config: &VolumeConfig,
    widget: &Box,
    label: &Label,
    icon: &Image,
    volume: &VolumeInfo,
) {
    label.set_text(&format!("{}%", volume.percentage));

    let icon_name = if volume.is_muted {
        "audio-volume-muted-symbolic"
    } else if volume.percentage > 70 {
        "audio-volume-high-symbolic"
    } else if volume.percentage > 30 {
        "audio-volume-medium-symbolic"
    } else if volume.percentage > 0 {
        "audio-volume-low-symbolic"
    } else {
        "audio-volume-muted-symbolic"
    };

    icon.set_icon_name(Some(icon_name));

    if config.tooltip {
        let mut tooltip = config.tooltip_format.clone();

        let muted_text = if volume.is_muted {
            "muted".to_string()
        } else {
            format!("{}%", volume.percentage)
        };

        tooltip = tooltip.replace("{percentage}", &format!("{}", volume.percentage));
        tooltip = tooltip.replace("{state}", &muted_text);
        tooltip = tooltip.replace("{icon}", "");

        widget.set_tooltip_text(Some(&tooltip));
    }
}
