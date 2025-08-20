use gtk::{Application, ApplicationWindow};
use gtk::{prelude::*};
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};

use crate::ui::logger::{LogLevel, Logger};
use crate::ui::modules::panel::PanelBuilder;
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
    let config = get_config().unwrap();
    let edge = string_to_edge(&config.panel.position);

    LayerShell::init_layer_shell(window);
    LayerShell::set_layer(window, Layer::Top);
    LayerShell::set_anchor(window, edge, true);
    LayerShell::set_anchor(window, gtk4_layer_shell::Edge::Left, true);
    LayerShell::set_anchor(window, gtk4_layer_shell::Edge::Right, true);
    LayerShell::set_exclusive_zone(window, config.panel.height as i32);
    LayerShell::set_keyboard_mode(window, KeyboardMode::None);
}
fn string_to_edge(s: &str) -> gtk4_layer_shell::Edge {
        match s.to_lowercase().as_str() {
            "top" => gtk4_layer_shell::Edge::Top,
            "bottom" => gtk4_layer_shell::Edge::Bottom,
            "left" => gtk4_layer_shell::Edge::Left,
            "right" => gtk4_layer_shell::Edge::Right,
            _ => gtk4_layer_shell::Edge::Top,
        }
    }