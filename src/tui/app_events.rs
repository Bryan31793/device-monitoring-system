use std::thread;
use std::time::Duration;
use crossterm::event::{KeyCode, KeyModifiers};
use crossbeam_channel::{Receiver, unbounded};

/// Application event types.
#[derive(Clone)]
pub enum AppEvent {
    DataUpdated,
    Shutdown,
}

/// Starts a background thread that monitors keyboard input.
///
/// Detects Ctrl+C and sends `AppEvent::Shutdown` through the returned channel.
/// This thread runs independently from the main event loop.
pub fn start_keyboard_thread() -> Receiver<AppEvent> {
    let (tx, rx) = unbounded::<AppEvent>();

    thread::spawn(move || {
        loop {
            // Poll keyboard with timeout
            if crossterm::event::poll(Duration::from_millis(100)).unwrap_or(false) {
                if let Ok(crossterm::event::Event::Key(key)) = crossterm::event::read() {
                    if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                        // Send shutdown and terminate thread
                        let _ = tx.send(AppEvent::Shutdown);
                        break;
                    }
                }
            }
        }
    });

    rx
}