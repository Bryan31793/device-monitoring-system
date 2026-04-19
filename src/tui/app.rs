//! Application state for the TUI.

use std::sync::{Arc, Mutex};
use crate::collectors::tui_data::TuiData;

/// Manages the application state and lifecycle.
pub struct App {
    /// Signal to exit the event loop.
    pub should_quit: bool,
    /// Shared reference to monitoring data.
    pub tui_data: Arc<Mutex<TuiData>>,
}

impl App {
    /// Creates a new app instance.
    pub fn new(tui_data: Arc<Mutex<TuiData>>) -> Self {
        Self {
            should_quit: false,
            tui_data,
        }
    }

    /// Handles key press event - initiates shutdown.
    pub fn on_key(&mut self) {
        self.should_quit = true;
    }
}