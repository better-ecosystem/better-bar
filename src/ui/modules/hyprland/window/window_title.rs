// hyprland window title module
use gtk::pango::EllipsizeMode;
use gtk::prelude::*;
use gtk::{Box, Label, Orientation};
use hyprland::data::Client;
use hyprland::event_listener::EventListener;
use hyprland::shared::HyprDataActiveOptional;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;

use crate::ui::logger::{LogLevel, Logger};
use lazy_static::lazy_static;

lazy_static! {
    static ref LOG: Logger = Logger::new("window_title",LogLevel::Debug);
}
pub struct WindowWidget {
    container: Box,
    title_label: Rc<RefCell<Label>>,
}

impl WindowWidget {
    pub fn new() -> Self {
        let container = Box::new(Orientation::Horizontal, 0);
        container.set_spacing(4);
        container.set_widget_name("window");
        container.add_css_class("modules");

        // Title label
        let title_label = Label::new(None);
        title_label.set_ellipsize(EllipsizeMode::End);
        title_label.set_max_width_chars(34);
        container.append(&title_label);

        let widget = Self {
            container,
            title_label: Rc::new(RefCell::new(title_label)),
        };

        widget.update_title();
        widget.start_event_listener();
        widget
    }

    pub fn widget(&self) -> &Box {
        &self.container
    }

    fn update_title(&self) {
        LOG.debug("Updating title");
        let title = match Client::get_active() {
            Ok(Some(client)) if !client.title.trim().is_empty() => client.title,
            _ => String::from("Desktop"),
        };
        self.title_label.borrow().set_text(&title);
    }

    pub fn start_event_listener(&self) {
        LOG.debug("started event listener");
        let (tx, rx) = mpsc::channel();
        let label = self.title_label.clone();

        // Spawn the hyprland event listener in a separate thread
        thread::spawn(move || {
            let mut event_listener = EventListener::new();

            // Helper function to get current window title
            let get_current_title = || -> String {
                match Client::get_active() {
                    Ok(Some(client)) if !client.title.trim().is_empty() => client.title,
                    _ => "Desktop".to_string(),
                }
            };

            let tx1 = tx.clone();
            event_listener.add_active_window_changed_handler(move |_| {
                LOG.debug("active window changed");
                let title = get_current_title();
                if let Err(e) = tx1.send(title) {
                    eprintln!("Failed to send window title: {}", e);
                }
            });

            let tx2 = tx.clone();
            event_listener.add_workspace_changed_handler(move |_| {
                LOG.debug("active workspace changed");
                let title = get_current_title();
                if let Err(e) = tx2.send(title) {
                    eprintln!("Failed to send window title: {}", e);
                }
            });

            let tx3 = tx.clone();
            event_listener.add_window_closed_handler(move |_| {
                LOG.debug("active window closed");
                let title = get_current_title();
                if let Err(e) = tx3.send(title) {
                    LOG.error(&format!("Failed to send window title: {}", e));
                }
            });

            let tx4 = tx.clone();
            event_listener.add_window_opened_handler(move |_| {
                LOG.debug("hyprland-window -> active window closed");
                let title = get_current_title();
                if let Err(e) = tx4.send(title) {
                    eprintln!("Failed to send window title: {}", e);
                }
            });

            if let Err(e) = event_listener.start_listener() {
                LOG.error("hyprland-window -> Failed to start window title listener");
                LOG.error(&e.to_string());
            }
        });

        let mut last_update = std::time::Instant::now();
        let debounce_duration = std::time::Duration::from_millis(100);

        glib::timeout_add_local(std::time::Duration::from_millis(100),  move || {
            if let Ok(title) = rx.try_recv() {
                let now = std::time::Instant::now();
                if now.duration_since(last_update) >= debounce_duration {
                    label.borrow().set_text(&title);
                    last_update = now;
                }
            }

            if !label.borrow().is_visible() {
                glib::ControlFlow::Break
            } else {
                glib::ControlFlow::Continue
            }
        });
    }
}
