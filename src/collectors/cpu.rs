use sysinfo::{System, Components};
use std::sync::OnceLock;

static CPU_TEMP_IDX: OnceLock<usize> = OnceLock::new();

//x|use crate::collectors::cpu;
#[derive(Clone)]
pub struct CpuSnapshot {
    cpu_percent: f32,
    pub cpu_per_core: Vec<f32>,
    cpu_temp_celsius: f32,
}

impl CpuSnapshot {
    pub fn new(sys: &System, components: &mut Components) -> Self {
        let mut v: Vec<f32> = Vec::new();
        
        for cpu in sys.cpus() {
            v.push(cpu.cpu_usage());
        }

        Self {
            //cpu_percent: CpuSnapshot::cpu_percent(&v),
            cpu_percent: sys.global_cpu_usage(),
            cpu_per_core: v,
            cpu_temp_celsius: CpuSnapshot::calc_cpu_temp_celsius(components)
        }
    }   
}

impl CpuSnapshot {
    pub fn cpu_percent(&self) -> f32 {
        self.cpu_percent
    }

    pub fn calc_cpu_temp_celsius(components: &mut Components) -> f32 {
        components.refresh(false);

        let idx = CpuSnapshot::cpu_index(components);
        
        if idx < 1000 {
            components
                .get(idx)
                .and_then(|c| Some(c.temperature()?))
                .unwrap_or(0.0)
        } else {
            0.0
        }
    }

    fn cpu_index(components: &Components) -> usize {
        *CPU_TEMP_IDX.get_or_init(|| {
            components
                .iter()
                .position(|c| {
                    let label = c.label().to_lowercase();
                    label.contains("cpu") || label.contains("core") || label.contains("tctl")
                })
                .unwrap_or(1000)
        })
    }

    pub fn cpu_usage(&self) -> Vec<f32> {
        let cpu = self.cpu_per_core.clone();
        cpu
    }

    pub fn cpu_temp_celsius(&self) -> f32 {
        self.cpu_temp_celsius
    }
}