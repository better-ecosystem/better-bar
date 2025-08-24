use tokio::process::Command;
use crate::ui::modules::volume::volume_info::{VolumeError, VolumeInfo};


pub async fn get_current_volume() -> Result<VolumeInfo, VolumeError> {
    let (vol, mute) = tokio::join!(get_volume_percentage(), is_muted());
    Ok(VolumeInfo { percentage: vol?, is_muted: mute? })
}


pub async fn get_volume_percentage() -> Result<u8, VolumeError> {
    let output = Command::new("pactl")
        .args(&["get-sink-volume", "@DEFAULT_SINK@"])
        .output()
        .await?;
    if !output.status.success() {
        return Err(VolumeError::CommandFailed("Failed to get volume".into()));
    }
    let s = String::from_utf8(output.stdout).map_err(|e| VolumeError::ParseError(e.to_string()))?;
    parse_volume(&s)
}


pub async fn is_muted() -> Result<bool, VolumeError> {
    let output = Command::new("pactl")
        .args(&["get-sink-mute", "@DEFAULT_SINK@"])
        .output()
        .await?;
    if !output.status.success() {
        return Err(VolumeError::CommandFailed("Failed to get mute status".into()));
    }
    let s = String::from_utf8(output.stdout).map_err(|e| VolumeError::ParseError(e.to_string()))?;
    Ok(s.trim().ends_with("yes"))
}


fn parse_volume(output: &str) -> Result<u8, VolumeError> {
    output.split_whitespace()
        .find_map(|w| w.strip_suffix('%').map(|v| v.parse::<u8>().ok()))
        .flatten()
        .ok_or_else(|| VolumeError::ParseError("No volume percentage found".into()))
}


pub async fn _set_volume(percent: u8) -> Result<(), VolumeError> {
    let percent = percent.min(100);
    let output = Command::new("pactl")
        .args(&["set-sink-volume", "@DEFAULT_SINK@", &format!("{}%", percent)])
        .output()
        .await?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(VolumeError::CommandFailed(stderr.into()));
    }
    Ok(())
}


pub async fn change_volume(amount: i8) -> Result<(), VolumeError> {
    let sign = if amount >= 0 { "+" } else { "-" };
    let output = Command::new("pactl")
        .args(&["set-sink-volume", "@DEFAULT_SINK@", &format!("{}{}%", sign, amount.abs())])
        .output()
        .await?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(VolumeError::CommandFailed(stderr.into()));
    }
    Ok(())
}


pub async fn toggle_mute() -> Result<(), VolumeError> {
    let output = Command::new("pactl")
        .args(&["set-sink-mute", "@DEFAULT_SINK@", "toggle"])
        .output()
        .await?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(VolumeError::CommandFailed(stderr.into()));
    }
    Ok(())
}


/// Increase volume by specified amount (1-100)
pub async fn increase_volume(amount: u8) -> Result<(), VolumeError> {
    let amount = amount.min(100); // Cap at 100
    
    let output = Command::new("pactl")
        .args(&["set-sink-volume", "@DEFAULT_SINK@", &format!("+{}%", amount)])
        .output()
        .await?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(VolumeError::CommandFailed(format!("Failed to increase volume: {}", stderr)));
    }
    
    Ok(())
}


/// Decrease volume by specified amount
pub async fn decrease_volume(amount: u8) -> Result<(), VolumeError> {
    let amount = amount.min(100);
    
    let output = Command::new("pactl")
        .args(&["set-sink-volume", "@DEFAULT_SINK@", &format!("-{}%", amount)])
        .output()
        .await?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(VolumeError::CommandFailed(format!("Failed to decrease volume: {}", stderr)));
    }
    
    Ok(())
}


pub async fn volume_up() -> Result<(), VolumeError> {
    increase_volume(1).await
}


pub async fn volume_down() -> Result<(), VolumeError> {
    decrease_volume(1).await
}

