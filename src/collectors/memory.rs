use sysinfo::{System, MemoryRefreshKind};

const BYTES_IN_GB: f64 = 1_000_000_000.0;

#[derive(Clone, Copy)]
pub struct MemorySnapshot {
    total_memory: f64,  //esto esta de mas porque siempre es igual
    used_memory: f64,
    total_swap: f64,
    used_swap: f64,
    available_memory: f64
}

impl From<&System> for MemorySnapshot {
    fn from(sys: &System) -> Self {
        Self {
            total_memory: MemorySnapshot::bytes_to_gb(sys.total_memory()),
            used_memory: MemorySnapshot::bytes_to_gb(sys.used_memory()),
            total_swap: MemorySnapshot::bytes_to_gb(sys.total_swap()),
            used_swap: MemorySnapshot::bytes_to_gb(sys.used_swap()),
            available_memory: MemorySnapshot::bytes_to_gb(sys.available_memory())
        }
    }
}

impl MemorySnapshot {
    ///esta ya no va.
    fn refresh_memory(sys: &mut System) {   
        sys.refresh_memory_specifics(
            MemoryRefreshKind::nothing()
            .with_ram()
            .with_swap());
    }

    fn bytes_to_gb(value: u64) -> f64 {
        value as f64 / BYTES_IN_GB
    }

    pub fn total_memory(&self) -> f64{
        self.total_memory
    }

    pub fn used_memory(&self) -> f64 {
        self.used_memory
    }

    pub fn total_swap(&self) -> f64 {
        self.total_swap
    }

    pub fn used_swap(&self) -> f64 {
        self.used_swap
    }

    pub fn percentage_use_memory(&self) -> f64 {
        self.used_memory() / self.total_memory * 100.0
    }

    pub fn percentage_use_swap(&self) -> f64 {
        self.used_swap / self.total_swap * 100.0
    }

    pub fn cache(&self) -> f64 {
        self.used_memory() - self.available_memory
    }
}