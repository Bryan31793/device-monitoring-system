use sysinfo::NetworkData;

pub struct NetworkSnapshot {
    pub net_sent_bytes_s: u64,
    pub net_recv_byets_s: u64,
}

impl From<&NetworkData> for NetworkSnapshot {
    fn from(net_data: &NetworkData) -> Self {
        Self {
            net_sent_bytes_s: net_data.received(),
            net_recv_byets_s: net_data.received(),
        }
    }
}