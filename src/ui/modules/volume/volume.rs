use tokio::io::AsyncBufReadExt;
use tokio::process::Command;

// Listen to pactl evnents
pub fn start_volume_monitor() -> tokio::sync::mpsc::Receiver<String> {
    let (tx, rx) = tokio::sync::mpsc::channel(100);
    
    tokio::spawn(async move {
        let mut child = Command::new("pactl")
            .args(&["subscribe"])
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to start pactl subscribe");
        
        let stdout = child.stdout.take().expect("Failed to get stdout");
        let mut reader = tokio::io::BufReader::new(stdout);
        let mut line = String::new();
        
        // initial volume
        if let Ok(initial_volume) = get_current_volume().await {
            let _ = tx.send(initial_volume).await;
        }
        
        while let Ok(_) = reader.read_line(&mut line).await {
            if line.contains("sink") && (line.contains("change") || line.contains("new")) {
                if let Ok(volume) = get_current_volume().await {
                    let _ = tx.send(volume).await;
                }
            }
            line.clear();
        }
    });
    
    rx
}

async fn get_current_volume() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let output = Command::new("pactl")
        .args(&["get-sink-volume", "@DEFAULT_SINK@"])
        .output()
        .await?;
    
    let volume_output = String::from_utf8(output.stdout)?;
    
    let mute_output = tokio::process::Command::new("pactl")
        .args(&["get-sink-mute", "@DEFAULT_SINK@"])
        .output()
        .await?;
    
    let mute_str = String::from_utf8(mute_output.stdout)?;
    let muted = mute_str.trim().ends_with("yes");
    
    if muted {
        return Ok(" ".to_string());
    }
    
    // send volume percentage
    if let Some(percent_pos) = volume_output.find('%') {
        let mut volume_start = percent_pos;
        while volume_start > 0 && volume_output.chars().nth(volume_start - 1).unwrap().is_ascii_digit() {
            volume_start -= 1;
        }
        let volume_str = &volume_output[volume_start..percent_pos];
        Ok(format!(" {}%", volume_str))
    } else {
        Ok(" --".to_string())
    }
}