// Clock module

use gtk::{prelude::*, Label};

pub struct ClockModule;

impl ClockModule {
    pub fn new() -> Self {
        Self
    }
    pub fn create(&self) -> Label {
        let time_label = Label::new(Some(""));
        time_label.set_widget_name("clock");
        time_label.add_css_class("modules");
        time_label
    }
}