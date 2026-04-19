use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use sysinfo::{System, RefreshKind, CpuRefreshKind, MemoryRefreshKind};
use crossbeam_channel::{Receiver, unbounded};
use crossbeam_channel::select;
use crate::collectors::tui_data::TuiData;
use crate::monitor::snapshot::MetricBuffer;
use crate::collectors::memory::MemorySnapshot;
use crate::collectors::cpu::CpuSnapshot;
use crate::tui::app_events::AppEvent;

static BUFFER_CAPACITY: usize = 120;

pub struct MonitorState {
    sys: Arc<Mutex<System>>,
    pub mem: Arc<Mutex<MetricBuffer<MemorySnapshot>>>,
    pub cpu: Arc<Mutex<MetricBuffer<CpuSnapshot>>>,
    pub tui_data: Arc<Mutex<TuiData>>,
}

impl MonitorState {
    pub fn new() -> Self {
        Self {
            sys: Arc::new(Mutex::new(System::new())),
            mem: Arc::new(Mutex::new(
                MetricBuffer::<MemorySnapshot>::new(BUFFER_CAPACITY),
            )),
            cpu: Arc::new(Mutex::new(
                MetricBuffer::<CpuSnapshot>::new(BUFFER_CAPACITY),
            )),
            tui_data: Arc::new(Mutex::new(TuiData::new(BUFFER_CAPACITY))),
        }
    }

    /// Starts the thread where data is collected
    pub fn start_main_thread(&self, shutdown_rx: Receiver<()>) -> Receiver<AppEvent>{
        let (tx, rx) = unbounded::<AppEvent>();

        let sys_shared = Arc::clone(&self.sys);
        let mem_shared = Arc::clone(&self.mem);
        let cpu_shared = Arc::clone(&self.cpu);
        let tui_shared = Arc::clone(&self.tui_data);

        // thread collector
        thread::spawn(move || {
            loop {

                select! {
                    recv(shutdown_rx) -> _ => {
                        break;
                    }
                    default(Duration::from_secs(2)) => {
                        if let Ok(mut sys) = sys_shared.lock() {
                            MonitorState::refresh_components(&mut *sys);

                            // esto puede ser una funcion generic
                            if let Ok(mut cpu_buffer) = cpu_shared.lock() {
                                cpu_buffer.update_buffer(&mut *sys);
                            }

                            if let Ok(mut mem_buffer) = mem_shared.lock() {
                                mem_buffer.update_buffer(&mut *sys);

                                if let Some(last_snapshot) = mem_buffer.last() {
                                    let used_memory = last_snapshot.used_memory();
                                    if let Ok(mut tui) = tui_shared.lock() {
                                        tui.update_tui_buffer(used_memory);
                                        //println!("{:#?}", tui);
                                    }
                                }
                            }

                            //  Notifica a la TUI — ella decide si re-renderizar
                            if tx.send(AppEvent::DataUpdated).is_err() {
                                break; // La TUI cerró el canal, salimos limpiamente
                            }
                        }
                    }
                }
                

                //thread::sleep(Duration::from_secs(2));
            }
        });

        rx // La TUI recibe este Receiver
    }

    /// Refresh components needed for data collection
    fn refresh_components(sys: &mut System) {
        sys.refresh_specifics(
            RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::nothing().with_cpu_usage())
            .with_memory(MemoryRefreshKind::nothing().with_ram().with_swap())
        );
    }
}




    
    
