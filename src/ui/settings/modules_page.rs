use gtk::{prelude::*, Box, Label, Orientation, Switch};

pub fn create_modules_page() -> Box {
    let modules_page = Box::new(Orientation::Vertical, 12);
    modules_page.set_margin_top(20);
    modules_page.set_margin_bottom(20);
    modules_page.set_margin_end(20);
    modules_page.set_margin_start(20);

    let modules_label = Label::new(Some("Panel Modules"));
    modules_label.add_css_class("heading");
    modules_page.append(&modules_label);

    // Clock module
    let clock_box = Box::new(Orientation::Horizontal, 12);
    let clock_label = Label::new(Some("Show Clock:"));
    let clock_switch = Switch::new();
    clock_switch.set_active(true);
    clock_box.append(&clock_label);
    clock_box.append(&clock_switch);
    modules_page.append(&clock_box);

    // System tray module
    let tray_box = Box::new(Orientation::Horizontal, 12);
    let tray_label = Label::new(Some("System Tray:"));
    let tray_switch = Switch::new();
    tray_switch.set_active(true);
    tray_box.append(&tray_label);
    tray_box.append(&tray_switch);
    modules_page.append(&tray_box);

    // Workspace module
    let workspace_box = Box::new(Orientation::Horizontal, 12);
    let workspace_label = Label::new(Some("Workspaces:"));
    let workspace_switch = Switch::new();
    workspace_switch.set_active(true);
    workspace_box.append(&workspace_label);
    workspace_box.append(&workspace_switch);
    modules_page.append(&workspace_box);

    // Battery module
    let battery_box = Box::new(Orientation::Horizontal, 12);
    let battery_label = Label::new(Some("Battery Info:"));
    let battery_switch = Switch::new();
    battery_switch.set_active(false);
    battery_box.append(&battery_label);
    battery_box.append(&battery_switch);
    modules_page.append(&battery_box);

    modules_page
}