use crate::config::config_helper;
use crate::utils::logger::{LogLevel, Logger};
use gtk::{
    prelude::*,
    Align, Box, DropDown, Label, ListBox, ListBoxRow, Orientation, Separator, SpinButton,
};
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::rc::Rc;

lazy_static! {
    static ref LOG: Logger = Logger::new("config", LogLevel::Debug);
}

pub fn create_config_page() -> Box {
    // Load config
    let config = match config_helper::get_config() {
        Ok(cfg) => cfg.clone(),
        Err(e) => {
            LOG.error(&format!("Failed to load config: {}", e));
            return Box::new(Orientation::Vertical, 12);
        }
    };
    let config_state = Rc::new(RefCell::new(config.clone()));

    // Outer vertical container with margin
    let page = Box::new(Orientation::Vertical, 16);
    page.set_margin_top(20);
    page.set_margin_bottom(20);
    page.set_margin_start(20);
    page.set_margin_end(20);

    page.append(&section_title("Panel Configuration"));

    let card = card_container();

    // List of settings as rows
    let list = ListBox::new();
    list.add_css_class("boxed-list");

    // Panel Height
    {
        let row = settings_row(
            "Panel Height",
            {
                let spin = SpinButton::with_range(20.0, 60.0, 1.0);
                spin.set_value(config.panel.height as f64);
                spin
            },
        );
        // connect
        {
            let config_ref = Rc::clone(&config_state);
            let right = row.child().and_then(|c| c.downcast::<Box>().ok()).unwrap();
            let spin: SpinButton = right.last_child().unwrap().downcast().unwrap();
            spin.connect_value_changed(move |spin_button| {
                let mut cfg = config_ref.borrow_mut();
                let new_height = spin_button.value() as u32;
                if cfg.panel.height != new_height {
                    cfg.panel.height = new_height;
                    LOG.debug(&format!("Panel height updated locally: {}", new_height));
                }
            });
        }
        list.append(&row);
    }

    // Panel Position
    {
        let row = settings_row(
            "Panel Position",
            {
                let dd = DropDown::from_strings(&["Top", "Bottom", "Left", "Right"]);
                let current_position = match config.panel.position.as_str() {
                    "top" => 0,
                    "bottom" => 1,
                    "left" => 2,
                    "right" => 3,
                    _ => 0,
                };
                dd.set_selected(current_position);
                dd
            },
        );
        // connect
        {
            let config_ref = Rc::clone(&config_state);
            let right = row.child().and_then(|c| c.downcast::<Box>().ok()).unwrap();
            let dropdown: DropDown = right.last_child().unwrap().downcast().unwrap();
            dropdown.connect_selected_notify(move |dd| {
                let mut cfg = config_ref.borrow_mut();
                let idx = dd.selected();
                let position_str = match idx {
                    0 => "top",
                    1 => "bottom",
                    2 => "left",
                    3 => "right",
                    _ => "top",
                };
                if cfg.panel.position != position_str {
                    cfg.panel.position = position_str.to_string();
                    LOG.debug(&format!("Panel position updated locally: {}", position_str));
                }
            });
        }
        list.append(&row);
    }

    card.append(&list);
    page.append(&card);

    // Separator for visual separation
    page.append(&Separator::new(Orientation::Horizontal));

    page
}

/// Title for sections
fn section_title(text: &str) -> Label {
    let lbl = Label::new(Some(text));
    lbl.add_css_class("section-title");
    lbl.set_xalign(0.0);
    lbl
}

/// A card container with padding and rounded border 
/// styled using css
fn card_container() -> Box {
    let card = Box::new(Orientation::Vertical, 12);
    card.add_css_class("settings-card");
    card.set_margin_top(8);
    card
}

/// One settings row left label & right widget aligned
fn settings_row<T: IsA<gtk::Widget>>(label_text: &str, right_widget: T) -> ListBoxRow {
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

    let right: gtk::Widget = right_widget.upcast();
    right.set_halign(Align::End);

    h.append(&label);
    h.append(&right);
    row.set_child(Some(&h));
    row
}
