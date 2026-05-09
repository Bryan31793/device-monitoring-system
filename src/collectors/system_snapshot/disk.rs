use sysinfo::Disks;
//use procfs::DiskStat;

#[derive(Clone)]
pub struct DiskSnapshot {
    pub disk_read_bytes_s: u64,
    pub disk_write_bytes_s: u64,
    pub disk_io_wait_ms: u64,
}

impl DiskSnapshot {
    pub fn new(disks: &Disks) -> Self {
        let disk_stats = procfs::diskstats().unwrap_or_default();

        // Sumar todos los discos
        let total_read_bytes: u64 = disks
            .list()
            .iter()
            .map(|disk| disk.usage().read_bytes)
            .sum();
        
        let total_write_bytes: u64 = disks
            .list()
            .iter()
            .map(|disk| disk.usage().written_bytes)
            .sum();

        let time_in_progress: u64 = disk_stats
            .iter()
            .map(|stat| stat.time_in_progress)
            .sum();

        Self {
            disk_read_bytes_s: total_read_bytes,
            disk_write_bytes_s: total_write_bytes,
            disk_io_wait_ms: time_in_progress,
        }
    }
}

