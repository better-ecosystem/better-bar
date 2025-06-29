use gtk::prelude::*;
use gtk::Application;

mod ui;
mod system;
mod config;
use ui::bar::create_main_bar;
use crate::ui::logger::{LogLevel, Logger};
use crate::config::config_helper;

pub const APP_ID: &str = "com.better-ecosystem.bar";
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log = Logger::new(LogLevel::Debug);
    config_helper::init_config()?;
    log.debug("main --> config initialized");
    let app = Application::builder().application_id(APP_ID).build();
    
    app.connect_activate(|app| {
        create_main_bar(app);
    });

    log.debug("main -->Running main app");
    app.run();
    
    Ok(())
}