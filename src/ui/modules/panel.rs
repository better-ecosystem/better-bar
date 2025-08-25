use gtk::{
    Align, ApplicationWindow, Box as GtkBox, CenterBox, EventSequenceState, Label, Orientation,
    PopoverMenu, prelude::*,
};

use super::clock::ClockModule;
use crate::{
    config::config::Config,
    system::{
        global::_is_hyprland_session, system_info_modules::SystemInfoModule, updater::SystemUpdater,
    },
    ui::modules::{
        battery::battery::Battery,
        hyprland::{window::window_title::WindowWidget, workspace::workspaces::WorkspaceWidget},
        launcher::app_launcher::LauncherWidget,
        network::network::Network,
        volume::volume::Volume,
    },
};

use crate::config::config_helper::get_config;
use crate::ui::settings::settings::show_panel_settings;
use gio::{Menu, SimpleActionGroup};
use std::rc::Rc;

#[derive(Clone)]
pub struct PanelState {
    pub _launcher: Rc<LauncherWidget>,
    pub _workspace_widget: Option<Rc<WorkspaceWidget>>,
    pub _window_title: Option<Rc<WindowWidget>>,
    pub _time_label: Label,
    pub _cpu_label: Option<Label>,
    // pub _memory_label: Label,
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
        let config = get_config().unwrap();

        let main_box = CenterBox::new();
        main_box.set_widget_name("bar");
        main_box.set_margin_top(4);
        main_box.set_margin_bottom(4);
        main_box.set_margin_start(8);
        main_box.set_margin_end(8);
        main_box.set_hexpand(true);
        main_box.set_vexpand(true);

        // Right click menu
        setup_context_menu(&window, &main_box);

        // Left section
        let left_box = GtkBox::new(Orientation::Horizontal, 0);
        left_box.set_halign(Align::Start);

        let _launcher = Rc::new(LauncherWidget::new());
        left_box.append(_launcher.widget());

        // Show only for hyprland session
        let mut _workspace_widget: Option<Rc<WorkspaceWidget>> = None;
        let mut _window_title: Option<Rc<WindowWidget>> = None;

        // Only try to initialize them if we're in a Hyprland session
        if _is_hyprland_session() {
            if config.modules.workspaces {
                let widget = Rc::new(WorkspaceWidget::new());
                left_box.append(widget.widget());
                _workspace_widget = Some(widget);
            }

            if config.modules.window_title {
                let widget = Rc::new(WindowWidget::new());
                left_box.append(widget.widget());
                _window_title = Some(widget);
            }
        }

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

        // Only show menu when click on empty area of the bar
        // else using this workaround to ignore right click on other boxes
        // until i find a better way :)
        add_gesture_blocker(&left_box);
        add_gesture_blocker(&center_box);
        add_gesture_blocker(&right_box);

        let system_info = SystemInfoModule::new();
        let _cpu_label = if let Some(cpu) = system_info.create(&right_box) {
            Some(cpu)
        } else {
            None
        };

        
        let network_config = Config::load().unwrap().network;
        let network = Network::new(network_config.clone());
        right_box.append(network.widget());

        if config.modules.network {
            network.start_updates();
        }

        let battery_config = Config::load().unwrap().battery;
        let battery = Battery::new(battery_config.clone());
        right_box.append(battery.widget());

        if config.modules.battery {
            battery.start_updates();
        }

        let volume_config = Config::load().unwrap().volume;
        let volume = Volume::new(volume_config.clone());
        right_box.append(volume.widget());

        if config.modules.volume {
            volume.start_updates();
        }

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
        }
    }
}

fn add_gesture_blocker(widget: &GtkBox) {
    let gesture = gtk::GestureClick::new();
    gesture.set_button(gtk::gdk::BUTTON_SECONDARY);
    gesture.connect_pressed(move |gesture, _, _, _| {
        gesture.set_state(EventSequenceState::Claimed);
    });
    widget.add_controller(gesture);
}

fn setup_context_menu(window: &ApplicationWindow, main_box: &CenterBox) {
    let menu = Menu::new();
    menu.append(Some("Panel Settings"), Some("bar.settings"));

    let popover = PopoverMenu::from_model(Some(&menu));
    popover.set_parent(window);

    let actions = SimpleActionGroup::new();

    let settings_action = gio::SimpleAction::new("settings", None);
    settings_action.connect_activate(move |_, _| {
        show_panel_settings();
    });

    actions.add_action(&settings_action);
    window.insert_action_group("bar", Some(&actions));

    let gesture = gtk::GestureClick::new();
    gesture.set_button(gtk::gdk::BUTTON_SECONDARY);

    let popover_clone = popover.clone();
    gesture.connect_pressed(move |gesture, _, x, y| {
        let rect = gtk::gdk::Rectangle::new(x as i32, y as i32, 1, 200);
        popover_clone.set_pointing_to(Some(&rect));
        popover_clone.popup();

        gesture.set_state(gtk::EventSequenceState::Claimed);
    });
    main_box.add_controller(gesture);
}
