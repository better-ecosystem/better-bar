use crate::ui::modules::volume::volume_info::{VolumeError, VolumeInfo};
use tokio::process::Command;

pub async fn get_current_volume() -> Result<VolumeInfo, VolumeError> {
    let (vol, mute) = tokio::join!(get_volume_percentage(), is_muted());
    Ok(VolumeInfo {
        percentage: vol?,
        is_muted: mute?,
    })
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
        return Err(VolumeError::CommandFailed(
            "Failed to get mute status".into(),
        ));
    }
    let s = String::from_utf8(output.stdout).map_err(|e| VolumeError::ParseError(e.to_string()))?;
    Ok(s.trim().ends_with("yes"))
}

fn parse_volume(output: &str) -> Result<u8, VolumeError> {
    output
        .split_whitespace()
        .find_map(|w| w.strip_suffix('%').map(|v| v.parse::<u8>().ok()))
        .flatten()
        .ok_or_else(|| VolumeError::ParseError("No volume percentage found".into()))
}

pub async fn _set_volume(percent: u8) -> Result<(), VolumeError> {
    let percent = percent.min(100);
    let output = Command::new("pactl")
        .args(&[
            "set-sink-volume",
            "@DEFAULT_SINK@",
            &format!("{}%", percent),
        ])
        .output()
        .await?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(VolumeError::CommandFailed(stderr.into()));
    }
    Ok(())
}

pub async fn change_volume(amount: i8) -> Result<(), VolumeError> {
    let current_volume = get_volume_percentage().await?;

    let new_volume = if amount >= 0 {
        // Inc volume
        (current_volume as i16 + amount as i16).min(100) as u8
    } else {
        // Dec volume
        (current_volume as i16 + amount as i16).max(0) as u8
    };

    if new_volume != current_volume {
        let output = Command::new("pactl")
            .args(&[
                "set-sink-volume",
                "@DEFAULT_SINK@",
                &format!("{}%", new_volume),
            ])
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(VolumeError::CommandFailed(stderr.into()));
        }
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
    change_volume(amount.min(100) as i8).await
}

/// Decrease volume by specified amount
pub async fn decrease_volume(amount: u8) -> Result<(), VolumeError> {
    change_volume(-(amount.min(100) as i8)).await
}
