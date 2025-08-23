// Hyprand workspace module
use gtk::{Box, Button, Label, Orientation};
use gtk::{glib, prelude::*};
use hyprland::data::{Workspace, Workspaces};
use hyprland::dispatch::{Dispatch, DispatchType, WorkspaceIdentifierWithSpecial};
use hyprland::event_listener::EventListener;
use hyprland::shared::{HyprData, HyprDataActive, HyprDataVec};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;

use crate::utils::logger::{LogLevel, Logger};
use lazy_static::lazy_static;

lazy_static! {
    static ref LOG: Logger = Logger::new("workspaces",LogLevel::Debug);
}

#[derive(Clone)]
pub struct WorkspaceWidget {
    container: Box,
    workspaces: Rc<RefCell<Vec<i32>>>,
    current_workspace: Rc<RefCell<i32>>,
}

impl WorkspaceWidget {
    pub fn new() -> Self {
        let container = Box::new(Orientation::Horizontal, 5);
        container.set_widget_name("workspaces");

        let widget = Self {
            container,
            workspaces: Rc::new(RefCell::new(Vec::new())),
            current_workspace: Rc::new(RefCell::new(1)),
        };

        widget.update_workspaces();
        widget.start_event_listener();

        widget
    }

    pub fn widget(&self) -> &Box {
        &self.container
    }

    fn start_event_listener(&self) {
        let (tx, rx) = mpsc::channel();
        let widget_clone = self.clone();


        thread::spawn(move || {
            let mut event_listener = EventListener::new();

            let tx_clone = tx.clone();
            event_listener.add_workspace_changed_handler(move |_| {
                if let Err(e) = tx_clone.send(WorkspaceEvent::Changed) {
                    LOG.error(&format!("Failed to send workspace change event: {}", e));
                }
            });
            
            let tx_clone = tx.clone();
            event_listener.add_workspace_added_handler(move |_| {
                if let Err(e) = tx_clone.send(WorkspaceEvent::Added) {
                    LOG.error(&format!("Failed to send workspace added event: {}", e));
                }
            });

            let tx_clone = tx.clone();
            event_listener.add_workspace_deleted_handler(move |_| {
                if let Err(e) = tx_clone.send(WorkspaceEvent::Destroyed) {
                    LOG.error(&format!("Failed to send workspace destroyed event: {}", e));
                }
            });

            // Start listening
            if let Err(e) = event_listener.start_listener() {
                LOG.error(&format!("Failed to start event listener: {}", e));
            }
        });

        glib::spawn_future_local(async move {
            loop {
                match rx.try_recv() {
                    Ok(_) => {
                        widget_clone.update_workspaces();
                        LOG.debug("workspaces updated workspace");
                    }
                    Err(mpsc::TryRecvError::Empty) => {
                        LOG.debug("mpcs: Workspade event -> Empty");
                        // No events, wait a bit
                        glib::timeout_future(std::time::Duration::from_millis(200)).await;
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        LOG.error("mpcs: Workspade event ->  disconnected");
                        eprintln!("Event listener disconnected");
                        break;
                    }
                }
            }
        });
    }

    pub fn update_workspaces(&self) {
        let (new_workspace_ids, new_current_id) = match self.get_workspace_info() {
            Ok(info) => info,
            Err(e) => {
                self.show_error(&format!("Workspace error: {:?}", e));
                LOG.error("workspace error");
                LOG.error(&e.to_string());
                return;
            }
        };

        // Check if anything actually changed
        let workspaces_changed = {
            let current_workspaces = self.workspaces.borrow();
            *current_workspaces != new_workspace_ids
        };

        let current_workspace_changed = {
            let current_ws = self.current_workspace.borrow();
            *current_ws != new_current_id
        };

        // If only current workspace changed, just update button styles (fastest)
        if !workspaces_changed && current_workspace_changed {
            self.update_button_styles(&new_workspace_ids, new_current_id);
            *self.current_workspace.borrow_mut() = new_current_id;
            return;
        }

        // If nothing changed, do nothing
        if !workspaces_changed && !current_workspace_changed {
            return;
        }

        // Full rebuild needed
        *self.workspaces.borrow_mut() = new_workspace_ids.clone();
        *self.current_workspace.borrow_mut() = new_current_id;

        // Clear and rebuild
        while let Some(child) = self.container.first_child() {
            self.container.remove(&child);
        }

        self.create_workspace_buttons(&new_workspace_ids, new_current_id);
    }

    // only update css classes 
    fn update_button_styles(&self, workspace_ids: &[i32], current_id: i32) {
        let mut child = self.container.first_child();
        let mut index = 0;

        while let Some(widget) = child {
            if let Some(button) = widget.downcast_ref::<Button>() {
                if index < workspace_ids.len() {
                    let workspace_id = workspace_ids[index];
                    // Remove old classes and add new ones
                    button.remove_css_class("active");

                    if workspace_id == current_id {
                        button.add_css_class("active");
                    }
                }
                index += 1;
            }
            child = widget.next_sibling();
        }
    }

    // Create workspace buttons according to workspaces
    fn create_workspace_buttons(&self, workspace_ids: &[i32], current_id: i32) {
        for &workspace_id in workspace_ids {
            let button = Button::with_label(&workspace_id.to_string());
            button.set_size_request(30, 24);
            button.set_widget_name("ws-button");
            // button.add_css_class("flat");

            // it sucks without flat 
            if workspace_id == current_id {
                button.add_css_class("active");
            }
            button.connect_clicked(move |_| {
                Self::switch_to_workspace(workspace_id);
            });

            self.container.append(&button);
        }
    }

    fn show_error(&self, error_msg: &str) {
        while let Some(child) = self.container.first_child() {
            self.container.remove(&child);
        }

        let error_label = Label::new(Some("WS Error"));
        error_label.set_tooltip_text(Some(error_msg));
        self.container.append(&error_label);
    }

    fn get_workspace_info(&self) -> Result<(Vec<i32>, i32), hyprland::shared::HyprError> {
        let workspaces = Workspaces::get()?.to_vec();
        let mut workspace_ids: Vec<i32> = workspaces.iter().map(|w| w.id).collect();
        workspace_ids.sort();

        let current_id = match Workspace::get_active() {
            Ok(workspace) => workspace.id,
            Err(_) => 1, // Default fallback
        };

        Ok((workspace_ids, current_id))
    }

    fn switch_to_workspace(workspace_id: i32) {
        // Use hyprland dispatch for immediate switching
        if let Err(e) = Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Id(
            workspace_id,
        ))) {
            eprintln!("Failed to switch to workspace {}: {}", workspace_id, e);
        }
    }

    pub fn refresh(&self) {
        self.update_workspaces();
    }
}

#[derive(Debug, Clone)]
enum WorkspaceEvent {
    Changed,
    Added,
    Destroyed,
}
