use sysinfo::DiskUsage;
//use procfs::DiskStat;

#[derive(Clone)]
pub struct DiskSnapshot {
    pub disk_read_bytes_s: u64,
    pub disk_write_bytes_s: u64,
    pub disk_io_wait_ms: u64,
}

impl DiskSnapshot {
    pub fn new(disk_usage: &DiskUsage,) -> Self {
        let disk_stats = procfs::diskstats().unwrap_or_default();

        // Opción 3: Sumar todos los discos
        let time_in_progress: u64 = disk_stats
            .iter()
            .map(|stat| stat.time_in_progress)
            .sum();

        Self {
            disk_read_bytes_s: disk_usage.read_bytes,
            disk_write_bytes_s: disk_usage.written_bytes,
            disk_io_wait_ms: time_in_progress,
        }
    }
}

