use sysinfo::{System, CpuRefreshKind};

//x|use crate::collectors::cpu;
#[derive(Clone)]
pub struct CpuSnapshot {
    cpu_usage: Vec<f32>,
    cpu_avg: f32,
    cpu_max: f32,
    cpu_std: f32
}

impl From<&System> for CpuSnapshot {
    fn from(sys: &System) -> Self {
        let mut v: Vec<f32> = Vec::new();
        
        for cpu in sys.cpus() {
            v.push(cpu.cpu_usage());
        }

        Self {
            cpu_avg: CpuSnapshot::calc_cpu_avg(&v),
            cpu_max: CpuSnapshot::calc_cpu_max(&v),
            cpu_std: CpuSnapshot::calc_cpu_std(&v),
            cpu_usage: v
        }
    }   
}

impl CpuSnapshot {
    pub fn avg(&self) -> f32 {
        self.cpu_avg
    }

    pub fn max(&self) -> f32 {
        self.cpu_max
    }

    pub fn std(&self) -> f32 {
        self.cpu_std
    }

    ///esta ya no va
    fn refresh_cpu(sys: &mut System) {
        sys.refresh_cpu_specifics(
            CpuRefreshKind::nothing()
            .with_cpu_usage());
    }

    fn calc_cpu_avg(cpu_usage: &[f32]) -> f32{
        if cpu_usage.is_empty() {
            return 0.0;
        }

        let sum: f32 = cpu_usage.iter().copied().sum();
        sum / cpu_usage.len() as f32
    }

    fn calc_cpu_max(cpu_usage: &[f32]) -> f32 {
        if cpu_usage.is_empty() {
            return 0.0;
        }
        cpu_usage.iter().copied().fold(f32::NEG_INFINITY, f32::max)
    }

    fn calc_cpu_std(cpu_usage: &[f32]) -> f32 {
        if cpu_usage.len() < 2 {
            return 0.0;
        }
        let n: f32 = cpu_usage.len() as f32;
        let avg: f32 = CpuSnapshot::calc_cpu_avg(cpu_usage);

        let variance: f32 = cpu_usage.iter().copied()
            .map(|x| {
                let diff = x - avg;
                diff * diff
            })
            .sum::<f32>() / n;

        variance.sqrt()
    }
}