use sysinfo::{Networks};

#[derive(Clone)]
pub struct NetworkSnapshot {
    pub net_sent_bytes_s: u64,
    pub net_recv_byets_s: u64,
}

impl NetworkSnapshot {
    pub fn new(networks: &Networks) -> Self {
        // Agregar datos de todas las interfaces de red
        let total_transmitted: u64 = networks
            .iter()
            .map(|(_, data)| data.transmitted())
            .sum();
        
        let total_received: u64 = networks
            .iter()
            .map(|(_, data)| data.received())
            .sum();

        Self {
            net_sent_bytes_s: total_transmitted,
            net_recv_byets_s: total_received,
        }
    }
}