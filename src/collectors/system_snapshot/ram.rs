use sysinfo::System;

const BYTES_IN_MB: f64 = 1_000_000.0;
const MB_IN_GB: f64 = 1000.0;

#[derive(Clone, Copy)]
pub struct MemorySnapshot {
    ram_used_mb: f64,
    ram_total_mb: f64,
    ram_percentage: f64,
    swap_used_mb: f64,
}

impl From<&System> for MemorySnapshot {
    fn from(sys: &System) -> Self {
        let ram_used = sys.used_memory();
        let ram_total = sys.total_memory(); 
        Self {
            ram_percentage: MemorySnapshot::ram_percentage(ram_total, ram_used),
            ram_used_mb: MemorySnapshot::bytes_to_mb(ram_used),
            ram_total_mb: MemorySnapshot::bytes_to_mb(ram_total),
            swap_used_mb: MemorySnapshot::bytes_to_mb(sys.used_swap()),
        }
    }
}

impl MemorySnapshot {
    fn bytes_to_mb(value: u64) -> f64 {
        value as f64 / BYTES_IN_MB  
    }

    fn mb_to_gb(value: f64) -> f64 {
        value as f64 / MB_IN_GB
    }

    pub fn ram_percentage(ram_total_mb: u64, ram_used_mb: u64) -> f64 {
        ram_used_mb as f64 / ram_total_mb as f64
    }

    pub fn total_memory_gb(&self) -> f64{
        MemorySnapshot::mb_to_gb(self.ram_total_mb)
    }

    pub fn used_memory_tui(&self) -> f64 {
        MemorySnapshot::mb_to_gb(self.ram_used_mb)
    }

    pub fn percentage_use_memory(&self) -> f64 {
        self.ram_percentage
    }

    pub fn swap_used_memory_gb(&self) -> f64 {
        MemorySnapshot::mb_to_gb(self.swap_used_mb)
    }
}