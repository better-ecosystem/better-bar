use std::time::Duration;

use gtk::{prelude::WidgetExt, Label};

use crate::ui::modules::cpu::cpu::CpuMonitor;

pub fn create_cpu_widget() -> Label {
    let label = Label::new(Some("CPU: 0%"));
    
    let label_clone = label.clone();
    let mut cpu_monitor = CpuMonitor::new();
    
    glib::timeout_add_local(Duration::from_secs(1), move || {
        let usage = match cpu_monitor.get_cpu_usage() {
            Ok(usage) => usage,
            Err(err) => cpu_monitor.handle_error(err),
        };
        
        label_clone.set_text(&format!("CPU: {:.1}%", usage as i64));
        
        // Remove classes for conflict
        label_clone.remove_css_class("low");
        label_clone.remove_css_class("medium");
        label_clone.remove_css_class("high");
        
        // Classes for styling 
        let css_class = match usage {
            u if u < 30.0 => "low",
            u if u < 70.0 => "medium",
            _ => "high",
        };
        label_clone.add_css_class(css_class);
        
        glib::ControlFlow::Continue
    });
    
    label
}