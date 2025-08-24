use std::fmt;
use std::io;

#[derive(Debug)]
pub enum VolumeError {
    CommandFailed(String),
    ParseError(String),
    IoError(io::Error),
}

impl fmt::Display for VolumeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VolumeError::CommandFailed(msg) => write!(f, "Command failed: {}", msg),
            VolumeError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            VolumeError::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl std::error::Error for VolumeError {}

impl From<io::Error> for VolumeError {
    fn from(err: io::Error) -> Self {
        VolumeError::IoError(err)
    }
}

#[derive(Debug, Clone)]
pub struct VolumeInfo {
    pub percentage: u8,
    pub is_muted: bool,
}

impl fmt::Display for VolumeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let icon = if self.is_muted {
            "audio-volume-muted"
        } else if self.percentage == 0 {
            "audio-volume-muted"
        } else if self.percentage <= 33 {
            "audio-volume-low"
        } else if self.percentage <= 66 {
            "audio-volume-medium"
        } else {
            "audio-volume-high"
        };
        write!(f, "{} {}%", icon, self.percentage)
    }
}
