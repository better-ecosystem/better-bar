use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use lazy_static::lazy_static;

use crate::ui::logger::{LogLevel, Logger};

lazy_static! {
    static ref LOG: Logger = Logger::new("memory",LogLevel::Debug);
}

// Derived from waybar
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub _total: i64,
    pub _available: i64,
    pub _used: i64,
    pub _zfs_size: i64,
}

impl MemoryStats {
    pub fn used(&self) -> i64 {
        self._used
    }

    pub fn used_percentage(&self) -> f64 {
        (self._used as f64 / self._total as f64) * 100.0
    }

    pub fn available_percentage(&self) -> f64 {
        (self._available as f64 / self._total as f64) * 100.0
    }

    pub fn format_bytes(kb: i64) -> String {
        let gb = kb as f64 / 1024.0 / 1024.0;
        format!("{:.1} GB", gb)
    }
}

pub struct MemoryMonitor;

impl MemoryMonitor {
    pub fn new() -> Self {
        Self
    }

    pub fn get_memory_stats(&mut self) -> Result<MemoryStats, String> {
        let info = parse_meminfo();

        let total = match info.get("MemTotal") {
            Some(val) => *val,
            None => {
                LOG.error("memory: Failed to get MemTotal from /proc/meminfo");
                0
            }
        };

        let available = match info.get("MemAvailable") {
            Some(val) => *val,
            None => {
                LOG.error("memory: Failed to get MemAvailable from /proc/meminfo");
                0
            }
        };
        let zfs = match info.get("zfs_size") {
            Some(val) => *val,
            None => {
                LOG.error("memory: Failed to get ZFS ARC size");
                0
            }
        };
        let used = total - available + zfs;

        Ok(MemoryStats {
            _total: total,
            _available: available,
            _used: used,
            _zfs_size: zfs,
        })
    }

    pub fn handle_error(&self, err: String) -> MemoryStats {
        eprintln!("MemoryMonitor error: {}", err);
        MemoryStats {
            _total: 0,
            _available: 0,
            _used: 0,
            _zfs_size: 0,
        }
    }
}

fn zfs_arc_size() -> u64 {
    LOG.debug("memory: passing /proc/spl/kstat/zfs/arcstats");
    let file = File::open("/proc/spl/kstat/zfs/arcstats");
    if let Ok(file) = file {
        let reader = BufReader::new(file);

        for line in reader.lines().flatten() {
            let mut parts = line.split_whitespace();
            let name = parts.next();
            let _type = parts.next();
            let data = parts.next();

            if let (Some("size"), Some(_), Some(data_str)) = (name, _type, data) {
                if let Ok(size) = data_str.parse::<u64>() {
                    return size / 1024; // Convert to kB
                }
            }
        }
    } else {
        LOG.error("memory: error while performing zfs arc size");
    }
    0
}

/// Parses /proc/meminfo and adds zfs_size if available.
fn parse_meminfo() -> HashMap<String, i64> {
    LOG.debug("memory: passing /proc/meminfo");
    let file = match File::open("/proc/meminfo") {
        Ok(f) => f,
        Err(e) => {
            LOG.error(&format!("memory: Failed to open /proc/meminfo: {}", e));
            return HashMap::new();
        }
    };

    let reader = BufReader::new(file);
    let mut meminfo = HashMap::new();

    for line in reader.lines().flatten() {
        if let Some((key, val)) = line.split_once(':') {
            let val = val.trim().split_whitespace().next().unwrap_or("0");
            if let Ok(value) = val.parse::<i64>() {
                meminfo.insert(key.to_string(), value);
            } else {
                LOG.error(&format!("memory: Failed to parse value for key: {}", key));
            }
        }
    }

    // Insert ZFS ARC size under key "zfs_size"
    meminfo.insert("zfs_size".to_string(), zfs_arc_size() as i64);
    meminfo
}
