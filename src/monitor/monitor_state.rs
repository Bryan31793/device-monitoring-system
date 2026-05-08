use std::thread;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;
use sysinfo::{System, RefreshKind, CpuRefreshKind, MemoryRefreshKind, Components};
use crossbeam_channel::{Receiver, unbounded};
use crossbeam_channel::select;
use crate::collectors::tui_data::TuiData;
use crate::monitor::snapshot::MetricBuffer;
use crate::collectors::system_snapshot::ram::MemorySnapshot;
use crate::collectors::system_snapshot::cpu::CpuSnapshot;
use crate::tui::app_events::AppEvent;

static BUFFER_CAPACITY: usize = 120;

pub struct MonitorState {
    sys: Arc<Mutex<System>>,
    components: Arc<Mutex<Components>>,
    pub mem: Arc<Mutex<MetricBuffer<MemorySnapshot>>>,
    pub cpu: Arc<Mutex<MetricBuffer<CpuSnapshot>>>,
    pub tui_data: Arc<Mutex<TuiData>>,
}

impl MonitorState {
    pub fn new() -> Self {
        Self {
            sys: Arc::new(
                Mutex::new(
                    System::new())
            ),
            components: Arc::new(
                Mutex::new(
                    Components::new_with_refreshed_list())
            ),
            mem: Arc::new(
                Mutex::new(
                MetricBuffer::<MemorySnapshot>::new(BUFFER_CAPACITY),
            )),
            cpu: Arc::new(Mutex::new(
                MetricBuffer::<CpuSnapshot>::new(BUFFER_CAPACITY),
            )),
            tui_data: Arc::new(
                Mutex::new(
                    TuiData::new(BUFFER_CAPACITY))),
        }
    }

    /// Starts the thread where data is collected
    pub fn start_main_thread(&self, shutdown_rx: Receiver<()>) -> Receiver<AppEvent>{
        let (tx, rx) = unbounded::<AppEvent>();

        let sys_shared = Arc::clone(&self.sys);
        let components_shared = Arc::clone(&self.components);
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
                        if MonitorState::update_data(
                            Arc::clone(&sys_shared),
                            Arc::clone(&components_shared),
                            Arc::clone(&mem_shared),
                            Arc::clone(&cpu_shared),
                            Arc::clone(&tui_shared),
                        ) {
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

    fn update_data(
        sys_shared: Arc<Mutex<System>>,
        components_shared: Arc<Mutex<Components>>,
        mem_shared: Arc<Mutex<MetricBuffer<MemorySnapshot>>>,
        cpu_shared: Arc<Mutex<MetricBuffer<CpuSnapshot>>>,
        tui_shared: Arc<Mutex<TuiData>>
    ) -> bool {
        if let Ok(mut sys) = sys_shared.lock() && let Ok(mut comp) = components_shared.lock() {
            MonitorState::refresh_components(&mut *sys);

            // Actualizar CPU
            if let Ok(mut cpu_buffer) = cpu_shared.lock() {
                MonitorState::update_cpu(&mut cpu_buffer, &mut sys, &mut comp, Arc::clone(&tui_shared));
            }

            // Actualizar Memoria
            if let Ok(mut mem_buffer) = mem_shared.lock() {
                MonitorState::update_ram(&mut mem_buffer, &mut sys, &mut comp, tui_shared);
            }
            
            return true;
        }
        
        false
    }

    fn update_cpu(cpu_buffer: &mut MutexGuard<'_, MetricBuffer<CpuSnapshot>>, 
        sys:  &mut MutexGuard<'_, System>, 
        comp: &mut MutexGuard<'_, Components>,
        tui_shared: Arc<Mutex<TuiData>>
    ) {
        cpu_buffer.update_buffer((&mut *sys, &mut *comp));

        //refactorize
        //update tui_buffer
        if let Some(last_snapshot) = cpu_buffer.last() {
            let cpu_usage = last_snapshot.cpu_usage();
            let cpu_temp = last_snapshot.cpu_temp_celsius();
            if let Ok(mut tui) = tui_shared.lock() {
                tui.update_tui_buffer_cpu(cpu_usage);
                tui.update_tui_cpu_temp_celsius(cpu_temp);
            }
        }
    }

    fn update_ram(mem_buffer: &mut MutexGuard<'_, MetricBuffer<MemorySnapshot>>, 
        sys:  &mut MutexGuard<'_, System>, 
        comp: &mut MutexGuard<'_, Components>,
        tui_shared: Arc<Mutex<TuiData>>
    ) {
        mem_buffer.update_buffer(&mut *sys);

        //refactorize
        //update_tui_buffer
        if let Some(last_snapshot) = mem_buffer.last() {
            let used_memory = last_snapshot.used_memory_tui();
            if let Ok(mut tui) = tui_shared.lock() {
                tui.update_tui_buffer_ram(used_memory);
            }
        }
    }
}


    
    
