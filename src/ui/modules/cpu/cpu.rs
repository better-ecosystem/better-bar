use std::time::Instant;

use lazy_static::lazy_static;

use crate::ui::logger::{LogLevel, Logger};

lazy_static! {
    static ref LOG: Logger = Logger::new(LogLevel::Debug);
}

#[derive(Debug, Clone)]
struct CpuStats {
    user: u64,
    nice: u64,
    system: u64,
    idle: u64,
    iowait: u64,
    irq: u64,
    softirq: u64,
    steal: u64,
}
impl CpuStats {
    fn total(&self) -> u64 {
        self.user
            + self.nice
            + self.system
            + self.idle
            + self.iowait
            + self.irq
            + self.softirq
            + self.steal
    }

    fn active(&self) -> u64 {
        self.total() - self.idle - self.iowait
    }
}
pub struct CpuMonitor {
    previous_stats: Option<CpuStats>,
    last_update: Instant,
    error_count: u32,
}

impl CpuMonitor {
    pub fn new() -> Self {
        Self {
            previous_stats: None,
            last_update: Instant::now(),
            error_count: 0,
        }
    }

    fn parse_proc_stat() -> Result<CpuStats, String> {
        LOG.debug("cpu -> Parsed /proc/stat");
        
        let content = std::fs::read_to_string("/proc/stat")
        .map_err(|e| format!("Failed to read /proc/stat: {}", e))?;
    
        let first_line = content.lines().next().ok_or("Empty /proc/stat file")?;
        
        let values: Result<Vec<u64>, _> = first_line
            .split_whitespace()
            .skip(1)
            .take(8)
            .map(|s| s.parse::<u64>())
            .collect();
        
        let values = values.map_err(|e| format!("Failed to parse CPU values: {}", e))?;
        
        if values.len() < 4 {
            LOG.debug("cpu -> Not enough CPU stats in /proc/stat");
            return Err("Not enough CPU stats in /proc/stat".to_string());
        }

        let mut padded_values = values;
        padded_values.resize(8, 0);

        Ok(CpuStats {
            user: padded_values[0],
            nice: padded_values[1],
            system: padded_values[2],
            idle: padded_values[3],
            iowait: padded_values[4],
            irq: padded_values[5],
            softirq: padded_values[6],
            steal: padded_values[7],
        })
    }

    pub fn get_cpu_usage(&mut self) -> Result<f64, String> {
        let current_stats = Self::parse_proc_stat()?;
        let now = Instant::now();

        self.error_count = 0;

        if let Some(prev_stats) = &self.previous_stats {
            // Calculate deltas
            let total_delta = current_stats.total().saturating_sub(prev_stats.total());
            let active_delta = current_stats.active().saturating_sub(prev_stats.active());

            if total_delta > 0 {
                let cpu_usage = (active_delta as f64 / total_delta as f64) * 100.0;

                let cpu_usage = cpu_usage.max(0.0).min(100.0);

                self.previous_stats = Some(current_stats);
                self.last_update = now;
                return Ok(cpu_usage);
            }
        }
        self.previous_stats = Some(current_stats);
        self.last_update = now;
        Ok(0.0)
    }
    pub fn handle_error(&mut self, error: String) -> f64 {
        self.error_count += 1;
        eprintln!("CPU monitoring error #{}: {}", self.error_count, error);
        LOG.error(&format!("cpu -> Cpu monitoring error {}", error));

        if self.error_count > 5 { 0.0 } else { 50.0 }
    }
}
