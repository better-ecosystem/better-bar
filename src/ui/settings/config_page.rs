use crate::config::config_helper;
use crate::ui::logger::{LogLevel, Logger};
use gtk::{Box, DropDown, Label, Orientation, SpinButton, prelude::*};
use std::cell::RefCell;
use std::rc::Rc;

lazy_static::lazy_static! {
    static ref LOG: Logger = Logger::new("config", LogLevel::Debug);
}

pub fn create_config_page() -> Box {
    let config_page = Box::new(Orientation::Vertical, 12);
    config_page.set_margin_top(20);
    config_page.set_margin_bottom(20);
    config_page.set_margin_end(20);
    config_page.set_margin_start(20);

    // Get current configuration
    let config = match config_helper::get_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            LOG.error(&format!("Failed to load config: {}", e));
            return Box::new(Orientation::Vertical, 12); // Return empty box on error
        }
    };
    let config_state = Rc::new(RefCell::new(config.clone()));

    let config_label = Label::new(Some("Panel Configuration"));
    config_label.add_css_class("heading");
    config_page.append(&config_label);

    // Panel height setting
    let height_box = Box::new(Orientation::Horizontal, 12);
    let height_label = Label::new(Some("Panel Height:"));
    let height_spin = SpinButton::with_range(20.0, 50.0, 1.0);
    height_box.set_can_focus(false);
    height_spin.set_value(config.panel.height as f64); // Set from config
    height_box.append(&height_label);
    height_box.append(&height_spin);
    config_page.append(&height_box);

    // Connect height spin
    let config_ref = config_state.clone();
    height_spin.connect_value_changed(move |spin_button| {
        let mut config = config_ref.borrow_mut();
        config.panel.height = spin_button.value() as u32;
        LOG.debug(&format!("Panel height changed to: {}", config.panel.height));
    });

    // Panel position
    let position_box = Box::new(Orientation::Horizontal, 12);
    let position_label = Label::new(Some("Panel Position:"));
    let position_options = ["Top", "Bottom", "Left", "Right"];
    let position_dropdown = DropDown::from_strings(&position_options);

    let current_position = match config.panel.position.as_str() {
        "Top" => 0,
        "Bottom" => 1,
        "Left" => 2,
        "Right" => 3,
        _ => 0, // Default to Top
    };
    position_dropdown.set_selected(current_position);

    position_box.append(&position_label);
    position_box.append(&position_dropdown);
    config_page.append(&position_box);

    let config_ref = config_state.clone();
    position_dropdown.connect_selected_notify(move |drop_down| {
        let mut config = config_ref.borrow_mut();
        let selected_index = drop_down.selected();
        let position = match selected_index {
            0 => "Top",
            1 => "Bottom",
            2 => "Left",
            3 => "Right",
            _ => "Top", // Default fallback
        };
        config.panel.position = position.to_string().to_lowercase();
        LOG.debug(&format!("Panel position set to: {}", config.panel.position));
    });
    
    config_page
}
