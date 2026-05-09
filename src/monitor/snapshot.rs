use std::collections::VecDeque;
use sysinfo::{Components, Disks, Networks, System};
use crate::collectors::system_snapshot::network::NetworkSnapshot;
//use procfs::DiskStat;
use crate::collectors::system_snapshot::ram::MemorySnapshot;
use crate::collectors::system_snapshot::cpu::CpuSnapshot;
use crate::collectors::system_snapshot::disk::DiskSnapshot;


pub struct MetricBuffer<T> {
    values: VecDeque<T>,
    capacity: usize,
}

impl<'a, T: MetricSnapshots<'a>> MetricBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            values: VecDeque::<T>::with_capacity(capacity),
            capacity,
        }
    }

    /// Empuja un valor solo si ha pasado el intervalo configurado.
    /// Retorna true si efectivamente se hizo push.
    pub fn update_buffer(&mut self, params: T::Params) where T: Clone {
        if self.values.len() == self.capacity {
            self.values.pop_front();
        }   

        self.values.push_back(T::get_snapshot(params));
        
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

pub trait MetricSnapshots<'a> {
    type Params;
    fn get_snapshot(params: Self::Params) -> Self;
}

impl<'a> MetricSnapshots<'a> for MemorySnapshot {
    type Params = &'a System;
    fn get_snapshot(sys: Self::Params) -> Self {
        MemorySnapshot::from(sys)
    }
}

impl<'a> MetricSnapshots<'a> for CpuSnapshot {
    type Params = (&'a System, &'a mut Components);
    fn get_snapshot((sys, components): Self::Params) -> Self {
        CpuSnapshot::new(sys, components)
    }
}

impl<'a> MetricSnapshots<'a> for DiskSnapshot {
    type Params = &'a Disks;
    fn get_snapshot(disks: Self::Params) -> Self {
        DiskSnapshot::new(disks)
    }
}

impl<'a> MetricSnapshots<'a> for NetworkSnapshot {
    type Params = &'a Networks;
    fn get_snapshot(networks: Self::Params) -> Self {
        NetworkSnapshot::new(networks)
    }
}
    