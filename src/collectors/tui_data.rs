use std::collections::VecDeque;

#[derive(Clone)]
#[derive(Debug)]
pub struct TuiData {
    pub used_memory: VecDeque<f64>,
    //pub used_swap: VecDeque<f64>,
    pub cpu_cores_usage: Vec<f32>,
    pub cpu_temp_celsius: f32,
    capacity: usize,
}

impl TuiData {
    pub fn new(capacity: usize) -> Self {
        Self {
            used_memory: VecDeque::with_capacity(capacity),
            //used_swap: VecDeque::with_capacity(capacity),
            cpu_cores_usage: Vec::with_capacity(capacity),
            cpu_temp_celsius: 0.0,
            capacity,
        }
    }

    ///en el futuro tendra que ser generic
    pub fn update_tui_buffer_ram(&mut self, value: f64) {
        if self.used_memory.len() == self.capacity {
            self.used_memory.pop_front();
        }

        self.used_memory.push_back(value);
    }

    pub fn update_tui_buffer_cpu(&mut self, value: Vec<f32>) {
        self.cpu_cores_usage = value;
    }

    pub fn update_tui_cpu_temp_celsius(&mut self, value: f32) {
        self.cpu_temp_celsius = value;
    }

    pub fn cpu_usage_percentage(&self) -> f32 {
        let mut general_cpu_usage = 0.0;
        for core in &self.cpu_cores_usage {
            general_cpu_usage += core;
        }

        general_cpu_usage/self.cpu_cores_usage.len() as f32
    }
}
