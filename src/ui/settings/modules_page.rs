use gtk::{prelude::*, Box, Label, Orientation, Switch};
use std::rc::Rc;
use std::cell::RefCell;

use crate::config::config_helper;
use lazy_static::lazy_static;
use crate::ui::logger::{LogLevel, Logger};

lazy_static! {
    static ref LOG: Logger = Logger::new("modules", LogLevel::Debug);
}

pub fn create_modules_page() -> Box {
    let config = match config_helper::get_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            LOG.error(&format!("Failed to load config: {}", e));
            return Box::new(Orientation::Vertical, 12); // Return empty box on error
        }
    };

    let modules_page = Box::new(Orientation::Vertical, 12);
    modules_page.set_margin_top(20);
    modules_page.set_margin_bottom(20);
    modules_page.set_margin_end(20);
    modules_page.set_margin_start(20);

    let modules_label = Label::new(Some("Panel Modules"));
    modules_label.add_css_class("heading");
    modules_page.append(&modules_label);

    // Create a shared config state to be updated by all switches
    let config_state = Rc::new(RefCell::new(config.clone()));

    // CPU module
    let cpu_box = Box::new(Orientation::Horizontal, 12);
    let cpu_label = Label::new(Some("Show CPU usage:"));
    let cpu_switch = Switch::new();
    cpu_switch.set_active(config.modules.cpu);
    cpu_box.append(&cpu_label);
    cpu_box.append(&cpu_switch);
    modules_page.append(&cpu_box);

    // Connect CPU switch
    let config_ref = config_state.clone();
    cpu_switch.connect_state_set(move |_, state| {
        let mut config = config_ref.borrow_mut();
        config.modules.cpu = state;
        LOG.debug(&format!("CPU module set to: {}", state));
        glib::Propagation::Proceed
    });

    // Memory module
    let memory_box = Box::new(Orientation::Horizontal, 12);
    let memory_label = Label::new(Some("Show Memory usage:"));
    let memory_switch = Switch::new();
    memory_switch.set_active(config.modules.memory);
    memory_box.append(&memory_label);
    memory_box.append(&memory_switch);
    modules_page.append(&memory_box);

    // Connect Memory switch
    let config_ref = config_state.clone();
    memory_switch.connect_state_set(move |_, state| {
        let mut config = config_ref.borrow_mut();
        config.modules.memory = state;
        LOG.debug(&format!("Memory module set to: {}", state));
        glib::Propagation::Proceed
    });

    // Window title module
    let window_title_box = Box::new(Orientation::Horizontal, 12);
    let window_title_label = Label::new(Some("Show window title:"));
    let window_title_switch = Switch::new();
    window_title_switch.set_active(config.modules.window_title);
    window_title_box.append(&window_title_label);
    window_title_box.append(&window_title_switch);
    modules_page.append(&window_title_box);

    // Connect window title switch
    let config_ref = config_state.clone();
    window_title_switch.connect_state_set(move |_, state| {
        let mut config = config_ref.borrow_mut();
        config.modules.window_title = state;
        LOG.debug(&format!("Window title module set to: {}", state));
        glib::Propagation::Proceed
    });

    // Workspace module
    let workspace_box = Box::new(Orientation::Horizontal, 12);
    let workspace_label = Label::new(Some("Workspaces:"));
    let workspace_switch = Switch::new();
    workspace_switch.set_active(config.modules.workspaces);
    workspace_box.append(&workspace_label);
    workspace_box.append(&workspace_switch);
    modules_page.append(&workspace_box);

    // Connect workspace switch
    let config_ref = config_state.clone();
    workspace_switch.connect_state_set(move |_, state| {
        let mut config = config_ref.borrow_mut();
        config.modules.workspaces = state;
        LOG.debug(&format!("Workspaces module set to: {}", state));
        glib::Propagation::Proceed
    });

    // Battery module
    let battery_box = Box::new(Orientation::Horizontal, 12);
    let battery_label = Label::new(Some("Battery Info:"));
    let battery_switch = Switch::new();
    battery_switch.set_active(config.modules.battery);
    battery_box.append(&battery_label);
    battery_box.append(&battery_switch);
    modules_page.append(&battery_box);

    // Connect battery switch
    let config_ref = config_state.clone();
    battery_switch.connect_state_set(move |_, state| {
        let mut config = config_ref.borrow_mut();
        config.modules.battery = state;
        LOG.debug(&format!("Battery module set to: {}", state));
        glib::Propagation::Proceed
    });

    // Network module
    let network_box = Box::new(Orientation::Horizontal, 12);
    let network_label = Label::new(Some("Network Info:"));
    let network_switch = Switch::new();
    network_switch.set_active(config.modules.network);
    network_box.append(&network_label);
    network_box.append(&network_switch);
    modules_page.append(&network_box);

    // Connect network switch
    let config_ref = config_state.clone();
    network_switch.connect_state_set(move |_, state| {
        let mut config = config_ref.borrow_mut();
        config.modules.network = state;
        LOG.debug(&format!("Network module set to: {}", state));
        glib::Propagation::Proceed
    });

    // Volume module
    let volume_box = Box::new(Orientation::Horizontal, 12);
    let volume_label = Label::new(Some("Volume Info:"));
    let volume_switch = Switch::new();
    volume_switch.set_active(config.modules.volume);
    volume_box.append(&volume_label);
    volume_box.append(&volume_switch);
    modules_page.append(&volume_box);

    // Connect volume switch
    let config_ref = config_state.clone();
    volume_switch.connect_state_set(move |_, state| {
        let mut config = config_ref.borrow_mut();
        config.modules.volume = state;
        LOG.debug(&format!("Volume module set to: {}", state));
        glib::Propagation::Proceed
    });
    
    modules_page
}