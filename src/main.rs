use color_eyre::eyre::Result;
use crossbeam_channel::unbounded;
use device_monitoring_system::monitor::monitor_state::MonitorState;
use device_monitoring_system::tui::Tui;
use device_monitoring_system::tui::app_events::start_keyboard_thread;

fn main() -> Result<()> {
    color_eyre::install()?;

    let monitor_state = MonitorState::new();
    let (shutdown_tx, shutdown_rx) = unbounded::<()>();
    let tui_data_arc = monitor_state.tui_data.clone();

    // Inicia el thread del monitor
    let monitor_rx = monitor_state.start_main_thread(shutdown_rx.clone());

    // Inicia el thread del teclado (polling)
    let keyboard_rx = start_keyboard_thread();

    // La TUI escucha ambos threads
    let mut tui = Tui::new(tui_data_arc)?;
    tui.run(monitor_rx, keyboard_rx, shutdown_tx)
}

