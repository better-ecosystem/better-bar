// Global file for reuseable functions and components

use sysinfo::{System, Users};

pub fn _get_first_username() -> Option<String> {
    let users = Users::new_with_refreshed_list();
    users.list().first().map(|user| user.name().to_string())
}

pub fn _get_os_name() -> String {
    let os_name = System::name();

    os_name.unwrap().to_string()
}

pub fn _get_hostname() -> String {
    let host_name = System::host_name();

    host_name.unwrap().to_string()
}

pub fn reload_bar(){
    println!("reload bar");
}

// pub fn _get_current_session() -> String {
//     std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "Unknown".to_string())
// }

pub fn _is_hyprland_session() -> bool {
    match std::env::var("XDG_CURRENT_DESKTOP") {
        Ok(session) => session.to_lowercase().contains("hyprland"),
        Err(_) => false
    }
}