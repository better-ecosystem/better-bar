use crate::config::config_helper;
use crate::utils::logger::{LogLevel, Logger};
use gtk::{
    prelude::*,
    Align, Box, Label, ListBox, ListBoxRow, Orientation, Separator, Switch,
};
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::rc::Rc;

lazy_static! {
    static ref LOG: Logger = Logger::new("modules", LogLevel::Debug);
}

/// Public entry: returns a composed, styled page
pub fn create_modules_page() -> Box {

    // Load config file
    let config = match config_helper::get_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            LOG.error(&format!("Failed to load config: {}", e));
            return Box::new(Orientation::Vertical, 12);
        }
    };
    let config_state = Rc::new(RefCell::new(config.clone()));

    let page = Box::new(Orientation::Vertical, 16);
    page.set_margin_top(20);
    page.set_margin_bottom(20);
    page.set_margin_start(20);
    page.set_margin_end(20);

    page.append(&section_title("Panel Modules"));

    let card = card_container();
    let list = ListBox::new();
    list.add_css_class("boxed-list");

    // CPU
    {
        let row = switch_row("CPU Usage", config.modules.cpu);
        let cfg = Rc::clone(&config_state);
        attach_switch_handler(&row, move |state| {
            let mut c = cfg.borrow_mut();
            c.modules.cpu = state;
            LOG.debug(&format!("CPU module set to: {}", state));
        });
        list.append(&row);
    }

    // Memory
    {
        let row = switch_row("Memory Usage", config.modules.memory);
        let cfg = Rc::clone(&config_state);
        attach_switch_handler(&row, move |state| {
            let mut c = cfg.borrow_mut();
            c.modules.memory = state;
            LOG.debug(&format!("Memory module set to: {}", state));
        });
        list.append(&row);
    }

    // Window Title, only on hyprland for now
    {
        let row = switch_row("Window Title", config.modules.window_title);
        let cfg = Rc::clone(&config_state);
        attach_switch_handler(&row, move |state| {
            let mut c = cfg.borrow_mut();
            c.modules.window_title = state;
            LOG.debug(&format!("Window title module set to: {}", state));
        });
        list.append(&row);
    }

    // Workspaces, only on hyprland for now
    {
        let row = switch_row("Workspaces", config.modules.workspaces);
        let cfg = Rc::clone(&config_state);
        attach_switch_handler(&row, move |state| {
            let mut c = cfg.borrow_mut();
            c.modules.workspaces = state;
            LOG.debug(&format!("Workspaces module set to: {}", state));
        });
        list.append(&row);
    }

    // Battery
    {
        let row = switch_row("Battery Info", config.modules.battery);
        let cfg = Rc::clone(&config_state);
        attach_switch_handler(&row, move |state| {
            let mut c = cfg.borrow_mut();
            c.modules.battery = state;
            LOG.debug(&format!("Battery module set to: {}", state));
        });
        list.append(&row);
    }

    // Network
    {
        let row = switch_row("Network Info", config.modules.network);
        let cfg = Rc::clone(&config_state);
        attach_switch_handler(&row, move |state| {
            let mut c = cfg.borrow_mut();
            c.modules.network = state;
            LOG.debug(&format!("Network module set to: {}", state));
        });
        list.append(&row);
    }

    // Volume
    {
        let row = switch_row("Volume", config.modules.volume);
        let cfg = Rc::clone(&config_state);
        attach_switch_handler(&row, move |state| {
            let mut c = cfg.borrow_mut();
            c.modules.volume = state;
            LOG.debug(&format!("Volume module set to: {}", state));
        });
        list.append(&row);
    }

    card.append(&list);
    page.append(&card);

    page.append(&Separator::new(Orientation::Horizontal));
    page
}

fn section_title(text: &str) -> Label {
    let lbl = Label::new(Some(text));
    lbl.add_css_class("section-title");
    lbl.set_xalign(0.0);
    lbl
}

fn card_container() -> Box {
    let card = Box::new(Orientation::Vertical, 12);
    card.add_css_class("settings-card");
    card.set_margin_top(8);
    card
}

/// Create a row with a right-aligned toggle switch
fn switch_row(label_text: &str, initial: bool) -> ListBoxRow {
    let row = ListBoxRow::new();

    let h = Box::new(Orientation::Horizontal, 12);
    h.set_margin_top(8);
    h.set_margin_bottom(8);
    h.set_margin_start(12);
    h.set_margin_end(12);

    let label = Label::new(Some(label_text));
    label.set_xalign(0.0);
    label.set_halign(Align::Start);
    label.set_hexpand(true);

    let sw = Switch::new();
    sw.set_active(initial);
    sw.set_halign(Align::End);

    h.append(&label);
    h.append(&sw);
    row.set_child(Some(&h));
    row
}

fn attach_switch_handler<F: 'static + Fn(bool)>(row: &ListBoxRow, f: F) {
    let h: Box = row
        .child()
        .and_then(|c| c.downcast::<Box>().ok())
        .expect("row child must be a Box");

    let sw: Switch = h.last_child().unwrap().downcast().unwrap();
    sw.connect_state_set(move |_s, state| {
        f(state);
        glib::Propagation::Proceed
    });
}
