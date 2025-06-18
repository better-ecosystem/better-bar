use gtk::{gdk, CssProvider};
use crate::config::get_primary_css_path;
use std::fs;

pub fn load_css() {
    let provider = CssProvider::new();
    let default_path = get_primary_css_path();
    
    let mut css_content = String::new();
    
    // Then load default styles
    if let Ok(default) = fs::read_to_string(default_path) {
        let filtered = default
            .lines()
            .filter(|line| !line.trim().starts_with("@import"))
            .collect::<Vec<&str>>()
            .join("\n");
        css_content.push_str(&filtered);
    } else {
        eprintln!("Failed to load default.css");
    }
    
    // let provider = CssProvider::new();
    provider.load_from_string(&css_content);
    
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER,
    );
}