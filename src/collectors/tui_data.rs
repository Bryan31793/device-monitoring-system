use std::collections::VecDeque;

#[derive(Clone)]
#[derive(Debug)]
pub struct TuiData {
    pub used_memory: VecDeque<f64>,
    //pub used_swap: VecDeque<f64>,
    capacity: usize,
}

impl TuiData {
    pub fn new(capacity: usize) -> Self {
        Self {
            used_memory: VecDeque::with_capacity(capacity),
            //used_swap: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    ///en el futuro tendra que ser generic
    pub fn update_tui_buffer(&mut self, value: f64) {
        if self.used_memory.len() == self.capacity {
            self.used_memory.pop_front();
        }

        self.used_memory.push_back(value);
    }
}
