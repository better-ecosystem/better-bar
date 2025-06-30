use gtk::prelude::*;
use gtk::Application;
use clap::{Parser, ArgAction};

mod ui;
mod system;
mod config;
use ui::bar::create_main_bar;
use crate::ui::logger::{LogLevel, Logger};
use crate::config::config_helper;

pub const APP_ID: &str = "com.better-ecosystem.bar";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Show debug logs
    #[clap(short, long, action = ArgAction::SetTrue)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    Logger::set_logging_enabled(args.debug);
    let log = Logger::new("main", LogLevel::Debug);
    config_helper::init_config()?;
    log.debug("config initialized");
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(|app| {
        create_main_bar(app);
    });

    log.debug("Running main app");
    
    let args: Vec<String> = Vec::new();
    app.run_with_args(&args);

    Ok(())
}