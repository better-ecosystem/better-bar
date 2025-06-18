use gtk::{Align, ApplicationWindow, Box as GtkBox, CenterBox, Label, Orientation, prelude::*};

use super::clock::ClockModule;
use crate::{
    system::{global::_is_hyprland_session, system_info_modules::SystemInfoModule, updater::SystemUpdater},
    ui::modules::{
        hyprland::{window::window_title::WindowWidget, workspace::workspaces::WorkspaceWidget},
        launcher::app_launcher::LauncherWidget,
    },
};

use std::rc::Rc;

#[derive(Clone)]
pub struct PanelState {
    pub _launcher: Rc<LauncherWidget>,
    pub _workspace_widget: Option<Rc<WorkspaceWidget>>,
    pub _window_title: Option<Rc<WindowWidget>>,
    pub _time_label: Label,
    pub _cpu_label: Label, 
    // pub _memory_label: Label,
    pub _battery_label: Label,
    pub _network_label: Label,
    pub _volume_label: Label,
}

impl PanelState {
    pub fn start_updates(&self) {
        let updater = SystemUpdater::new(self.clone());
        updater.start();

        if let Some(ref window_title) = self._window_title {
            window_title.start_event_listener();
        }
    }

    pub fn refresh_workspaces(&self) {
        if let Some(ref workspace_widget) = self._workspace_widget {
            workspace_widget.refresh();
        }
    }
}

pub struct PanelBuilder;

impl PanelBuilder {
    pub fn new() -> Self {
        Self
    }

    pub fn build(&self, window: &ApplicationWindow) -> PanelState {
        let main_box = CenterBox::new();
        main_box.set_widget_name("bar");
        main_box.set_margin_top(4);
        main_box.set_margin_bottom(4);
        main_box.set_margin_start(8);
        main_box.set_margin_end(8);
        main_box.set_hexpand(true);
        main_box.set_vexpand(true);

        // Left section
        let left_box = GtkBox::new(Orientation::Horizontal, 0);
        left_box.set_halign(Align::Start);
        
        let _launcher = Rc::new(LauncherWidget::new());
        left_box.append(_launcher.widget());

        // Show only for hyprland session
        let (_workspace_widget, _window_title) = if _is_hyprland_session() {
            let workspace_widget = Rc::new(WorkspaceWidget::new());
            let window_title = Rc::new(WindowWidget::new());
            
            left_box.append(workspace_widget.widget());
            left_box.append(window_title.widget());
            
            (Some(workspace_widget), Some(window_title))
        } else {
            (None, None)
        };

        // Center section
        let center_box = GtkBox::new(Orientation::Horizontal, 0);
        center_box.set_halign(Align::Center);
        center_box.set_hexpand(true);

        let clock_module = ClockModule::new();
        let _time_label = clock_module.create();
        center_box.append(&_time_label);

        // Right section
        let right_box = GtkBox::new(Orientation::Horizontal, 0);
        right_box.set_halign(Align::End);

        let system_info = SystemInfoModule::new();
        let (
            _cpu_label,
            // _memory_label,
            _battery_label,
            _network_label,
            _volume_label,
        ) = system_info.create(&right_box);

        main_box.set_start_widget(Some(&left_box));
        main_box.set_center_widget(Some(&center_box));
        main_box.set_end_widget(Some(&right_box));
        window.set_child(Some(&main_box));

        PanelState {
            _launcher,
            _workspace_widget,
            _window_title,
            _time_label,
            // _memory_label,
            _cpu_label,
            _battery_label,
            _network_label,
            _volume_label,
        }
    }
}
