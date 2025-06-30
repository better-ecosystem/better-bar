use gio::{Menu, SimpleActionGroup};
use gtk::{Application, ApplicationWindow};
use gtk::{PopoverMenu, prelude::*};
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};

use crate::ui::logger::{LogLevel, Logger};
use crate::ui::modules::panel::PanelBuilder;
use crate::ui::settings::settings::show_panel_settings;
use crate::ui::styles::load_css;
use lazy_static::lazy_static;
use crate::config::config_helper::{get_config};

lazy_static! {
    static ref LOG: Logger = Logger::new("bar",LogLevel::Debug);
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
    
    window.connect_close_request(|_| {
        LOG.debug("bar window closed");
        glib::Propagation::Proceed
    });

    window.present();
}

fn setup_layer_shell(window: &ApplicationWindow) {
    let config = get_config();

    LayerShell::init_layer_shell(window);
    LayerShell::set_layer(window, Layer::Top);
    LayerShell::set_anchor(window, gtk4_layer_shell::Edge::Top, true);
    LayerShell::set_anchor(window, gtk4_layer_shell::Edge::Left, true);
    LayerShell::set_anchor(window, gtk4_layer_shell::Edge::Right, true);
    LayerShell::set_exclusive_zone(window, config.unwrap().panel.height as i32);
    LayerShell::set_keyboard_mode(window, KeyboardMode::None);
}
