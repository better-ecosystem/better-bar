use std::time::Duration;

use gtk::{prelude::*, Label,};
use crate::ui::modules::memory::memory::{MemoryMonitor, MemoryStats};

#[derive(Debug, Clone, Copy)]
pub enum MemoryDisplayMode {
    _UsedOnly,           // SHows used only memory
    _UsedWithTotal,      // Shows as 4gb/12gb
    _UsedPercentage,     // Shows memory usage percentage
    _Available,          // Shows available memory
    _AvailablePercentage, // Shows available memory in percentage
    _Detailed,           // Shows memory in detail eg: "Used: 8.2 gb, Available: 7.8 gb"
}

// Shows used only memory
pub fn _create_memory_widget() -> Label {
    create_memory_widget_with_mode(MemoryDisplayMode::_UsedOnly)
}

// Memory widget with different display modes
pub fn create_memory_widget_with_mode(mode: MemoryDisplayMode) -> Label {
    let label = Label::new(Some("RAM: 0 MB"));
    let label_clone = label.clone();
    let mut memory_monitor = MemoryMonitor::new();
    
    glib::timeout_add_local(Duration::from_secs(2), move || {
        if !label_clone.is_visible() {
            return glib::ControlFlow::Break;
        }
        
        let stats = match memory_monitor.get_memory_stats() {
            Ok(stats) => stats,
            Err(err) => memory_monitor.handle_error(err),
        };

        let text = _format_memory_display(&stats, mode);
        label_clone.set_text(&text);
        
        // Update CSS classes
        label_clone.remove_css_class("low");
        label_clone.remove_css_class("medium");
        label_clone.remove_css_class("high");

        let usage_percent = stats.used_percentage();
        let css_class = match usage_percent {
            u if u < 50.0 => "low",
            u if u < 80.0 => "medium", 
            _ => "high",
        };
        label_clone.add_css_class(css_class);
        
        glib::ControlFlow::Continue
    });

    label
}


// Helper function to format memory display based on mode
// Added this to show different memory informations
fn _format_memory_display(stats: &MemoryStats, mode: MemoryDisplayMode) -> String {
    match mode {
        MemoryDisplayMode::_UsedOnly => {
            format!("RAM: {}", MemoryStats::format_bytes(stats.used()))
        }
        MemoryDisplayMode::_UsedWithTotal => {
            format!("RAM: {} / {}", 
                MemoryStats::format_bytes(stats.used()),
                MemoryStats::format_bytes(stats._total)
            )
        }
        MemoryDisplayMode::_UsedPercentage => {
            format!("RAM: {}%", stats.used_percentage() as u32)
        }
        MemoryDisplayMode::_Available => {
            format!("RAM: {} available", MemoryStats::format_bytes(stats._available))
        }
        MemoryDisplayMode::_AvailablePercentage => {
            format!("RAM: {}% available", stats.available_percentage() as u32)
        }
        MemoryDisplayMode::_Detailed => {
            format!("Used: {}, Available: {}", 
                MemoryStats::format_bytes(stats.used()),
                MemoryStats::format_bytes(stats._available)
            )
        }
    }
}

// Functions for different display modes
pub fn _create_memory_widget_used_only() -> Label {
    create_memory_widget_with_mode(MemoryDisplayMode::_UsedOnly)
}

pub fn _create_memory_widget_with_total() -> Label {
    create_memory_widget_with_mode(MemoryDisplayMode::_UsedWithTotal)
}

pub fn _create_memory_widget_percentage() -> Label {
    create_memory_widget_with_mode(MemoryDisplayMode::_UsedPercentage)
}

pub fn _create_memory_widget_available() -> Label {
    create_memory_widget_with_mode(MemoryDisplayMode::_Available)
}