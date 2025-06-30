use gtk::{gdk::Key, prelude::*,Box, Orientation, EventControllerKey, Label, Notebook};
use lazy_static::lazy_static;
use std::{env, process::Command};

use crate::ui::{logger::{LogLevel, Logger}, settings::{config_page::{create_config_page}, modules_page::create_modules_page}};

lazy_static! {
    static ref LOG: Logger = Logger::new("settings",LogLevel::Debug);
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

    let notebook = Notebook::new();

    let config_page = create_config_page();
    let modules_page = create_modules_page();

    // Add Configuration page to notebook
    let config_tab_label = Label::new(Some("Configuration"));
    notebook.append_page(&config_page, Some(&config_tab_label));

    // Add Modules page to notebook
    let modules_tab_label = gtk::Label::new(Some("Modules"));
    notebook.append_page(&modules_page, Some(&modules_tab_label));

    // Close button
    let button_box = Box::new(Orientation::Horizontal, 12);
    button_box.set_halign(gtk::Align::End);
    button_box.set_margin_top(20);
    button_box.set_margin_end(20);
    button_box.set_margin_bottom(20);

    let close_button = gtk::Button::with_label("Close");
    let window_clone = settings_window.clone();
    close_button.connect_clicked(move |_| {
        window_clone.close();
    });
    button_box.append(&close_button);

    // Main box
    let main_vbox = Box::new(Orientation::Vertical, 0);
    main_vbox.append(&notebook);
    main_vbox.append(&button_box);

    settings_window.set_child(Some(&main_vbox));

    // Keybinds for settings window
    let key_controller = EventControllerKey::new();
    let settings_window_clone = settings_window.clone();
    key_controller.connect_key_pressed(move |_controller, key, _keycode, _state| match key {
        Key::Escape => {
            LOG.debug("Closing setting window");
            settings_window_clone.close();
            true.into()
        }
        Key::q | Key::Q => {
            LOG.debug("Closing setting window");
            settings_window_clone.close();
            true.into()
        }
        _ => false.into(),
    });
    settings_window.add_controller(key_controller);
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
