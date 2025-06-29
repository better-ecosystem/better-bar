use gtk::{prelude::*, Box, Label, Orientation, SpinButton, Switch, DropDown};

pub fn create_config_page() -> Box {
    let config_page = Box::new(Orientation::Vertical, 12);
    config_page.set_margin_top(20);
    config_page.set_margin_bottom(20);
    config_page.set_margin_end(20);
    config_page.set_margin_start(20);
    
    let config_label = Label::new(Some("Panel Configuration"));
    config_label.add_css_class("heading");
    config_page.append(&config_label);
    
    // Panel height setting
    let height_box = Box::new(Orientation::Horizontal, 12);
    let height_label = Label::new(Some("Panel Height:"));
    let height_spin = SpinButton::with_range(20.0, 100.0, 1.0);
    height_box.set_can_focus(false);
    height_spin.set_value(40.0);
    height_box.append(&height_label);
    height_box.append(&height_spin);
    config_page.append(&height_box);
    
    // Auto hide switch
    let autohide_box = Box::new(Orientation::Horizontal, 12);
    let autohide_label = Label::new(Some("Auto-hide Panel:"));
    let autohide_switch = Switch::new();
    autohide_box.set_can_focus(false);
    autohide_box.append(&autohide_label);
    autohide_box.append(&autohide_switch);
    config_page.append(&autohide_box);
    
    // Panel position
    let position_box = Box::new(Orientation::Horizontal, 12);
    let position_label = Label::new(Some("Panel Position:"));
    let position_dropdown = DropDown::from_strings(&["Top", "Bottom", "Left", "Right"]);
    position_dropdown.set_selected(0); // Default to "Top"
    position_box.append(&position_label);
    position_box.append(&position_dropdown);
    config_page.append(&position_box);
    
    config_page
}