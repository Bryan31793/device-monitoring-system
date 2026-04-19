use sysinfo::{ProcessRefreshKind, RefreshKind, System, MemoryRefreshKind};

struct Memory {
  sys: System,
  total_memory: u64,
  used_memory: u64, 
  total_swap: u64,
  used_swap: u64
}

impl Memory {

    fn new_memory() -> Memory {
        let sys = Memory::get_sys();
        let mem = Memory {
            total_memory: sys.total_memory(),
            used_memory: sys.used_memory(),
            total_swap: sys.total_swap(),
            used_swap: sys.used_swap(),
            sys
        };

        mem
    }

    fn get_sys() -> System {
        let sys = System::new_with_specifics(
        RefreshKind::nothing()
        .with_memory(MemoryRefreshKind::everything())
        .with_processes(ProcessRefreshKind::nothing().with_memory())
        );  

        sys
    }

    fn snapshot(&mut self) {
        self.refresh_mem();

        self.total_memory = self.sys.total_memory();
        self.used_memory = self.sys.used_memory();
        self.total_swap = self.sys.total_swap();
        self.used_swap = self.sys.total_swap();
    }

    fn refresh_mem(&mut self) {
        self.sys.refresh_memory_specifics(MemoryRefreshKind::everything());
    }

    fn bytes_to_gb(bytes: u64) -> u64 {
        bytes / 1_000_000_000
    }

    fn total_memory(self) -> u64 {
        self.total_memory
    }

    fn used_memory(self) -> u64 {
        self.used_memory
    }

    fn total_swap(self) -> u64 {
        self.total_swap
    }

    fn used_swap(self) -> u64 {
        self.used_swap
    }
}

pub fn mem_usage() {
    let mut mem = Memory::new_memory();
    mem.snapshot();    

    println!("=> system:");
    // RAM and swap information:
    println!("total memory: {} bytes", mem.total_memory);
    println!("used memory : {} bytes", mem.used_memory);
    println!("total swap  : {} bytes", mem.total_swap);
    println!("used swap   : {} bytes", mem.used_swap);
}

// ... tu código actual en mem.rs ...

/* 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mem_usage() {
        mem_usage();
        
    }
}*/