use gtk::{prelude::*, Box, Label, Orientation, SpinButton, DropDown, Button};
use crate::config::config_helper;
use std::cell::RefCell;
use std::rc::Rc;
use crate::ui::logger::{LogLevel, Logger};

pub fn create_config_page() -> Box {
    let log = Logger::new("config_page", LogLevel::Debug);
    let config_page = Box::new(Orientation::Vertical, 12);
    config_page.set_margin_top(20);
    config_page.set_margin_bottom(20);
    config_page.set_margin_end(20);
    config_page.set_margin_start(20);
    
    // Get current configuration
    let config = config_helper::get_config().expect("Failed to get configuration");
    
    // Create a structure to hold pending changes
    #[derive(Default)]
    struct PendingChanges {
        position: Option<String>,
        height: Option<u32>,
    }
    
    let pending_changes = Rc::new(RefCell::new(PendingChanges::default()));
    
    let config_label = Label::new(Some("Panel Configuration"));
    config_label.add_css_class("heading");
    config_page.append(&config_label);
    
    // Panel height setting
    let height_box = Box::new(Orientation::Horizontal, 12);
    let height_label = Label::new(Some("Panel Height:"));
    let height_spin = SpinButton::with_range(20.0, 100.0, 1.0);
    height_box.set_can_focus(false);
    height_spin.set_value(config.panel.height as f64); // Set from config
    height_box.append(&height_label);
    height_box.append(&height_spin);
    config_page.append(&height_box);
    
    // Panel position
    let position_box = Box::new(Orientation::Horizontal, 12);
    let position_label = Label::new(Some("Panel Position:"));
    let position_options = ["Top", "Bottom", "Left", "Right"];
    let position_dropdown = DropDown::from_strings(&position_options);
    
    // Set the current position from config
    let current_pos = config.panel.position.as_str();
    let selected_index = position_options.iter()
        .position(|&pos| pos.eq_ignore_ascii_case(current_pos))
        .unwrap_or(0); // Default to "Top" if not found
    position_dropdown.set_selected(selected_index as u32);
    
    position_box.append(&position_label);
    position_box.append(&position_dropdown);
    config_page.append(&position_box);
    
    // Store pending changes when dropdown selection changes
    let pending_changes_clone = pending_changes.clone();
    position_dropdown.connect_selected_notify(move |dropdown| {
        let selected_index = dropdown.selected();
        if let Some(position) = position_options.get(selected_index as usize) {
            pending_changes_clone.borrow_mut().position = Some(position.to_string());
        }
    });
    
    // Store pending changes when height changes
    let pending_changes_clone = pending_changes.clone();
    height_spin.connect_value_changed(move |spin| {
        let height = spin.value() as u32;
        pending_changes_clone.borrow_mut().height = Some(height);
    });
    
    // Create a button container
    let button_box = Box::new(Orientation::Horizontal, 12);
    button_box.set_halign(gtk::Align::End);
    button_box.set_margin_top(20);
    
    // Add Apply button
    let apply_button = Button::with_label("Apply");
    button_box.append(&apply_button);
    config_page.append(&button_box);
    
    // Connect Apply button to save configuration
    apply_button.connect_clicked(move |_| {
        if let Ok(mut config) = config_helper::get_config_mut() {
            let changes = pending_changes.borrow();
            
            // Apply pending changes to config
            if let Some(position) = &changes.position {
                log.debug(&format!("Changing panel position to: {}", position));
                config.panel.position = position.clone();
            }
            
            if let Some(height) = changes.height {
                log.debug(&format!("Changing panel height to: {}", height));
                config.panel.height = height;
            }
            
            // Save the updated configuration and notify listeners
            if let Err(err) = config_helper::save_config() {
                log.error(&format!("Failed to save configuration: {}", err));
            } else {
                // Clear pending changes after successful save
                pending_changes.borrow_mut().position = None;
                pending_changes.borrow_mut().height = None;
                log.debug("Configuration applied successfully");
            }
        }
    });
    
    config_page
}