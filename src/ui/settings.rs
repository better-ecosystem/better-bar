use std::{env, process::Command};

use gtk::prelude::*;
use lazy_static::lazy_static;

use crate::ui::logger::{LogLevel, Logger};

lazy_static! {
    static ref LOG: Logger = Logger::new(LogLevel::Debug);
}

pub fn show_panel_settings() {
    LOG.debug("Opening panel settings window");
    set_window_floating_rules();

    // Create a regular window for settings
    let settings_window = gtk::Window::builder()
        .title("panel_settings")
        .default_width(600)
        .default_height(600)
        .resizable(true)
        .build();

    // Add dummyy options for now
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 12);
    vbox.set_margin_top(20);
    vbox.set_margin_bottom(20);
    vbox.set_margin_start(20);
    vbox.set_margin_end(20);

    let label = gtk::Label::new(Some("Panel Configuration"));
    label.add_css_class("heading");
    vbox.append(&label);

    let height_box = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    let height_label = gtk::Label::new(Some("Panel Height:"));
    let height_spin = gtk::SpinButton::with_range(20.0, 100.0, 1.0);
    height_spin.set_value(40.0);
    height_box.append(&height_label);
    height_box.append(&height_spin);
    vbox.append(&height_box);

    let autohide_box = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    let autohide_label = gtk::Label::new(Some("Auto-hide Panel:"));
    let autohide_switch = gtk::Switch::new();
    autohide_box.append(&autohide_label);
    autohide_box.append(&autohide_switch);
    vbox.append(&autohide_box);

    // Close button
    let close_button = gtk::Button::with_label("Close");
    let window_clone = settings_window.clone();
    close_button.connect_clicked(move |_| {
        window_clone.close();
    });

    vbox.append(&close_button);
    settings_window.set_child(Some(&vbox));
    settings_window.present();
}

fn set_window_floating_rules() {
    let xdg = env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or_default()
        .to_lowercase();
    let sway_sock = env::var("SWAYSOCK").unwrap_or_default().to_lowercase();

    if xdg.contains("hyprland") {
        match Command::new("hyprctl")
            .args(["keyword", "windowrule", "float,title:^(panel_settings)$"])
            .output()
        {
            Ok(_) => {
                LOG.debug("Successfully set hyprland window rule");
            }
            Err(e) => {
                LOG.error(&format!("Failed to set hyprland window rule: {}", e));
            }
        }
    } else if sway_sock.contains("sway") {
        match Command::new("swaymsg")
            .args([
                "for_window",
                "[title=\"^panel_settings$\"]",
                "floating",
                "enable",
            ])
            .output()
        {
            Ok(_) => {
                LOG.debug("Successfully set sway window rule");
            }
            Err(e) => {
                LOG.error(&format!("Failed to set sway window rule: {}", e));
            }
        }
    }
}
