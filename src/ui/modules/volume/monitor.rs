use crate::ui::modules::volume::{volume_helper::get_current_volume, volume_info::VolumeInfo};
use tokio::io::AsyncBufReadExt;
use tokio::sync::mpsc;

pub fn start_volume_monitor() -> mpsc::Receiver<VolumeInfo> {
    let (tx, rx) = mpsc::channel(100);
    tokio::spawn(async move {
        if let Ok(vol) = get_current_volume().await {
            let _ = tx.send(vol).await;
        }
        // use pactl subscribe for real-time updates
        let mut child = tokio::process::Command::new("pactl")
            .args(&["subscribe"])
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap();

        let stdout = child.stdout.take().unwrap();
        let reader = tokio::io::BufReader::new(stdout);
        let mut lines = reader.lines();

        while let Ok(Some(line)) = lines.next_line().await {
            if line.contains("sink") && (line.contains("change") || line.contains("new")) {
                if let Ok(vol) = get_current_volume().await {
                    if tx.send(vol).await.is_err() {
                        break;
                    }
                }
            }
        }
    });
    rx
}
