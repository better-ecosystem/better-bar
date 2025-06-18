use gtk::{
    Button,
    prelude::*,
};
use tokio::process::Command;

// no quick menu for now
pub struct LauncherWidget {
    button: Button,
}

impl LauncherWidget {
    pub fn new() -> Self {
        let button = Button::builder().label("").build();
        button.set_widget_name("launcher");

        button.connect_clicked(|_| {
            glib::spawn_future_local(async move {
                if let Err(e) = Command::new("sh").args(["-c", "better-launcher"]).spawn() {
                    eprintln!("Failed to launch better-launcher: {}", e);
                }
            });
        });

        Self { button }
    }

    pub fn widget(&self) -> &Button {
        &self.button
    }
}
