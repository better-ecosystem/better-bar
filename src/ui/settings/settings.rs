use crate::config::config_helper;
use crate::ui::settings::{config_tab::create_config_page, modules_tab::create_modules_page};
use crate::utils::logger::{LogLevel, Logger};
use gtk::{
    prelude::*,
    gdk::Key,
    Box, Button, EventControllerKey, HeaderBar, Label, Notebook, Orientation, Window,
};
use lazy_static::lazy_static;
use std::env;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};

lazy_static! {
    static ref LOG: Logger = Logger::new("settings", LogLevel::Debug);
    static ref SETTINGS_WINDOW_OPEN: AtomicBool = AtomicBool::new(false);
}

pub fn show_panel_settings() {
    if SETTINGS_WINDOW_OPEN.load(Ordering::SeqCst) {
        return;
    }
    SETTINGS_WINDOW_OPEN.store(true, Ordering::SeqCst);

    LOG.debug("Opening panel settings window");

    set_window_floating_rules();

    let settings_window = Window::builder()
        .title("Better Bar – Settings")
        .default_width(720)
        .default_height(520)
        .resizable(true)
        .build();

    // Header bar
    let header = HeaderBar::new();
    header.set_title_widget(Some(&Label::new(Some("Better Bar Settings"))));

    let apply_button = Button::with_label("Apply");
    apply_button.add_css_class("suggested-action");

    let close_button = Button::with_label("Close");
    close_button.add_css_class("flat");

    header.pack_start(&close_button);
    header.pack_end(&apply_button);
    settings_window.set_titlebar(Some(&header));

    let notebook = Notebook::new();

    let config_page = create_config_page();
    let modules_page = create_modules_page();

    notebook.append_page(&config_page, Some(&Label::new(Some("Configuration"))));
    notebook.append_page(&modules_page, Some(&Label::new(Some("Modules"))));

    let main_vbox = Box::new(Orientation::Vertical, 0);
    main_vbox.append(&notebook);
    settings_window.set_child(Some(&main_vbox));

    apply_button.connect_clicked(move |_| match config_helper::save_config() {
        Ok(_) => LOG.debug("Configuration applied successfully"),
        Err(e) => LOG.error(&format!("Failed to save config: {}", e)),
    });

    let window_clone = settings_window.clone();
    close_button.connect_clicked(move |_| {
        SETTINGS_WINDOW_OPEN.store(false, Ordering::SeqCst);
        window_clone.close();
    });

    // Close on ESC, 'q'
    let key_controller = EventControllerKey::new();
    let settings_window_clone = settings_window.clone();
    key_controller.connect_key_pressed(move |_controller, key, _keycode, _state| match key {
        Key::Escape | Key::q | Key::Q => {
            LOG.debug("Closing settings window");
            SETTINGS_WINDOW_OPEN.store(false, Ordering::SeqCst);
            settings_window_clone.close();
            true.into()
        }
        _ => false.into(),
    });
    settings_window.add_controller(key_controller);

    settings_window.connect_close_request(move |_| {
        LOG.debug("Settings window closed");
        SETTINGS_WINDOW_OPEN.store(false, Ordering::SeqCst);
        glib::Propagation::Proceed
    });

    settings_window.present();
}

// Set window to floating on hyprland/sway 
fn set_window_floating_rules() {
    let xdg = env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or_default()
        .to_lowercase();
    let sway_sock = env::var("SWAYSOCK").unwrap_or_default().to_lowercase();

    if xdg.contains("hyprland") {
        match Command::new("hyprctl")
            .args(["keyword", "windowrule", "float,title:^(Better Bar – Settings)$"])
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
                "[title=\"^Better Bar – Settings$\"]",
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
