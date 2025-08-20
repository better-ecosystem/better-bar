use tokio::io::AsyncBufReadExt;
use tokio::process::Command;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum VolumeError {
    CommandFailed(String),
    ParseError(String),
    IoError(std::io::Error),
}

impl fmt::Display for VolumeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VolumeError::CommandFailed(msg) => write!(f, "Command failed: {}", msg),
            VolumeError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            VolumeError::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl Error for VolumeError {}

impl From<std::io::Error> for VolumeError {
    fn from(err: std::io::Error) -> Self {
        VolumeError::IoError(err)
    }
}

#[derive(Debug, Clone)]
pub struct VolumeInfo {
    pub percentage: u8,
    pub is_muted: bool,
}

impl fmt::Display for VolumeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_muted {
            write!(f, "audio-volume-muted")
        } else {
            let icon = match self.percentage {
                0 => "audio-volume-muted",
                1..=33 => "audio-volume-low",
                34..=66 => "audio-volume-medium",
                _ => "audio-volume-high",
            };
            write!(f, "{} {}%", icon, self.percentage)
        }
    }
}

/// Listen to pactl events and monitor volume changes
pub fn start_volume_monitor() -> tokio::sync::mpsc::Receiver<VolumeInfo> {
    let (tx, rx) = tokio::sync::mpsc::channel(100);
    
    tokio::spawn(async move {
        // Send initial volume
        if let Ok(initial_volume) = get_current_volume().await {
            let _ = tx.send(initial_volume).await;
        }

        // Start monitoring
        if let Err(e) = monitor_volume_changes(tx).await {
            eprintln!("Volume monitor error: {}", e);
        }
    });
    
    rx
}

async fn monitor_volume_changes(tx: tokio::sync::mpsc::Sender<VolumeInfo>) -> Result<(), VolumeError> {
    let mut child = Command::new("pactl")
        .args(&["subscribe"])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()?;

    let stdout = child.stdout.take()
        .ok_or_else(|| VolumeError::CommandFailed("Failed to get stdout".to_string()))?;
    
    let reader = tokio::io::BufReader::new(stdout);
    let mut lines = reader.lines();
    
    while let Some(line) = lines.next_line().await? {

        // Check for sink volume/mute changes
        if line.contains("sink") && (line.contains("change") || line.contains("new")) {
            if let Ok(volume) = get_current_volume().await {
                if tx.send(volume).await.is_err() {
                    break;
                }
            }
        }
    }
    
    Ok(())
}

/// Get current volume and mute status
pub async fn get_current_volume() -> Result<VolumeInfo, VolumeError> {
    // Get volume and mute status concurrently
    let (volume_result, mute_result) = tokio::join!(
        get_volume_percentage(),
        is_muted()
    );
    
    let percentage = volume_result?;
    let is_muted = mute_result?;
    
    Ok(VolumeInfo { percentage, is_muted })
}

async fn get_volume_percentage() -> Result<u8, VolumeError> {
    let output = Command::new("pactl")
        .args(&["get-sink-volume", "@DEFAULT_SINK@"])
        .output()
        .await?;
    
    if !output.status.success() {
        return Err(VolumeError::CommandFailed("Failed to get volume".to_string()));
    }
    
    let volume_output = String::from_utf8(output.stdout)
        .map_err(|e| VolumeError::ParseError(format!("Invalid UTF-8: {}", e)))?;
    
    parse_volume_from_output(&volume_output)
}

async fn is_muted() -> Result<bool, VolumeError> {
    let output = Command::new("pactl")
        .args(&["get-sink-mute", "@DEFAULT_SINK@"])
        .output()
        .await?;
    
    if !output.status.success() {
        return Err(VolumeError::CommandFailed("Failed to get mute status".to_string()));
    }
    
    let mute_output = String::from_utf8(output.stdout)
        .map_err(|e| VolumeError::ParseError(format!("Invalid UTF-8: {}", e)))?;
    
    Ok(mute_output.trim().ends_with("yes"))
}

fn parse_volume_from_output(output: &str) -> Result<u8, VolumeError> {
    // Look for percentage pattern like "50%"
    for word in output.split_whitespace() {
        if let Some(percent_pos) = word.find('%') {
            let volume_str = &word[..percent_pos];
            return volume_str.parse::<u8>()
                .map_err(|_| VolumeError::ParseError(format!("Invalid volume: {}", volume_str)));
        }
    }
    
    Err(VolumeError::ParseError("No volume percentage found".to_string()))
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

/// Set absolute volume
pub async fn set_volume(percentage: u8) -> Result<(), VolumeError> {
    let percentage = percentage.min(100); // well.. may be over-amplification ??
    
    let output = Command::new("pactl")
        .args(&["set-sink-volume", "@DEFAULT_SINK@", &format!("{}%", percentage)])
        .output()
        .await?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(VolumeError::CommandFailed(format!("Failed to set volume: {}", stderr)));
    }
    
    Ok(())
}

/// Toggle mute status
pub async fn toggle_mute() -> Result<(), VolumeError> {
    let output = Command::new("pactl")
        .args(&["set-sink-mute", "@DEFAULT_SINK@", "toggle"])
        .output()
        .await?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(VolumeError::CommandFailed(format!("Failed to toggle mute: {}", stderr)));
    }
    
    Ok(())
}

pub async fn volume_up() -> Result<(), VolumeError> {
    increase_volume(1).await
}

pub async fn volume_down() -> Result<(), VolumeError> {
    decrease_volume(1).await
}
