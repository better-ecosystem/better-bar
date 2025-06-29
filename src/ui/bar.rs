use gio::{Menu, SimpleActionGroup};
use gtk::{Application, ApplicationWindow};
use gtk::{PopoverMenu, prelude::*};
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};

use crate::ui::logger::{LogLevel, Logger};
use crate::ui::modules::panel::PanelBuilder;
use crate::ui::settings::show_panel_settings;
use crate::ui::styles::load_css;
use lazy_static::lazy_static;

lazy_static! {
    static ref LOG: Logger = Logger::new(LogLevel::Debug);
}
pub fn create_main_bar(app: &Application) {
    load_css();
    LOG.debug("Loaded css");

    let window = ApplicationWindow::builder().application(app).build();

    setup_layer_shell(&window);
    LOG.debug("Layer Shell setup complete");

    let panel_builder = PanelBuilder::new();
    let panel_state = panel_builder.build(&window);
    panel_state.start_updates();
    panel_state.refresh_workspaces();

    setup_context_menu(&window);

    window.connect_close_request(|_| {
        LOG.debug("bar window closed");
        gtk::glib::Propagation::Proceed
    });

    window.present();
}

fn setup_context_menu(window: &ApplicationWindow) {
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
    window.add_controller(gesture);
}

fn setup_layer_shell(window: &ApplicationWindow) {
    LayerShell::init_layer_shell(window);
    LayerShell::set_layer(window, Layer::Top);
    LayerShell::set_anchor(window, gtk4_layer_shell::Edge::Top, true);
    LayerShell::set_anchor(window, gtk4_layer_shell::Edge::Left, true);
    LayerShell::set_anchor(window, gtk4_layer_shell::Edge::Right, true);
    LayerShell::set_exclusive_zone(window, 40);
    LayerShell::set_keyboard_mode(window, KeyboardMode::None);
}
