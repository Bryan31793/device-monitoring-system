use std::thread;
use std::time::Duration;
use crossterm::event::{KeyCode, KeyModifiers};
use crossbeam_channel::{Receiver, unbounded};

/// Define todos los eventos posibles de la app
#[derive(Clone)]
pub enum AppEvent {
    DataUpdated,
    Shutdown,
}

/// Inicia un thread que hace polling del teclado
/// 
/// Este thread corre independientemente y detecta Ctrl+C,
/// luego envía `AppEvent::Shutdown` por el canal retornado.
pub fn start_keyboard_thread() -> Receiver<AppEvent> {
    let (tx, rx) = unbounded::<AppEvent>();

    thread::spawn(move || {
        loop {
            // Polling del teclado con timeout pequeño
            if crossterm::event::poll(Duration::from_millis(100)).unwrap_or(false) {
                if let Ok(crossterm::event::Event::Key(key)) = crossterm::event::read() {
                    if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                        // Envía shutdown y termina el thread
                        let _ = tx.send(AppEvent::Shutdown);
                        break;
                    }
                }
            }
        }
    });

    rx
}