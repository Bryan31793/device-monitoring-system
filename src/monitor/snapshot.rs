use std::collections::VecDeque;
use sysinfo::System;
use crate::collectors::memory::MemorySnapshot;
use crate::collectors::cpu::CpuSnapshot;

// ─── MetricBuffer ────────────────────────────────────────────────────────────

pub struct MetricBuffer<T> {
    values: VecDeque<T>,
    capacity: usize,
}

impl<T: MetricSnapshots> MetricBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            values: VecDeque::<T>::with_capacity(capacity),
            capacity,
        }
    }

    /// Empuja un valor solo si ha pasado el intervalo configurado.
    /// Retorna true si efectivamente se hizo push.
    pub fn update_buffer(&mut self, sys: &System) where T: Clone {
        if self.values.len() == self.capacity {
            self.values.pop_front();
        }   

        self.values.push_back(T::get_snapshot(sys));
        
    }

    /// Los últimos N valores (para la TUI).
    pub fn last_n(&self, n: usize) -> impl Iterator<Item = &T> {
        let skip = self.values.len().saturating_sub(n);
        self.values.iter().skip(skip)
    }

    pub fn last(&self) -> Option<&T> {
        self.values.back()
    }

    /// Todos los valores (para el modelo).
    pub fn all(&self) -> impl Iterator<Item = &T> {
        self.values.iter()
    }

}

pub trait MetricSnapshots {
    fn get_snapshot(sys: &System) -> Self;
}

impl MetricSnapshots for MemorySnapshot {
    fn get_snapshot(sys: &System) -> Self {
        MemorySnapshot::from(sys)
    }
}

impl MetricSnapshots for CpuSnapshot {
    fn get_snapshot(sys: &System) -> Self {
        CpuSnapshot::from(sys)
    }
}