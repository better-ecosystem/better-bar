use gtk::{glib::ExitCode, prelude::*};
use gtk::Application;

mod ui;
mod system;
mod config;

use ui::bar::create_main_bar;

use crate::ui::logger::{LogLevel, Logger};

pub const APP_ID: &str = "com.better-ecosystem.bar";
#[tokio::main]
async fn main() -> ExitCode {
    let log = Logger::new(LogLevel::Debug);
    let app = Application::builder().application_id(APP_ID).build();
    
    app.connect_activate(|app| {
        create_main_bar(app);
    });

    log.debug("Running main app");
    app.run()
}