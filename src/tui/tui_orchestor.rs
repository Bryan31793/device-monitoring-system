//! Terminal User Interface (TUI) for system monitoring.
//!
//! Manages the interactive terminal display, event handling, and rendering
//! of real-time system metrics using ratatui and crossterm.

//pub mod app;
//pub mod app_events;
//pub mod ui;

use std::io::{stdout, Stdout};
use std::sync::{Arc, Mutex};

use color_eyre::eyre::Result;
use crossbeam_channel::{Receiver, Sender, select};
use ratatui::{prelude::*, Terminal};

use crate::collectors::tui_data::TuiData;
use crate::tui::app::App;
use crate::tui::app_events::AppEvent;
use crate::tui::ui::cpu_barchart::draw_cpu_barchart;
use crate::tui::ui::memory_chart::draw_memory_chart;
use crate::tui::ui::general_info_block::draw_general_info_block;

/// The main TUI controller.
///
/// Handles terminal initialization, the event loop, keyboard input,
/// and rendering of monitoring data to the screen.
pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    app: App,
}

impl Tui {
    /// Initializes the TUI with the given monitoring data.
    pub fn new(tui_data: Arc<Mutex<TuiData>>) -> Result<Self> {
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        Ok(Self {
            terminal,
            app: App::new(tui_data),
        })
    }

    /// Starts the main event loop - displays and handles user input.
    ///
    /// - Listens for keyboard events (Ctrl+C to quit)
    /// - Processes data updates from the monitoring system
    /// - Uses event-driven architecture with `select!` to listen to both threads
    pub fn run(&mut self, monitor_rx: Receiver<AppEvent>, keyboard_rx: Receiver<AppEvent>, shutdown_tx: Sender<()>) -> Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(
            self.terminal.backend_mut(),
            crossterm::terminal::EnterAlternateScreen,
            crossterm::event::EnableMouseCapture
        )?;

        loop {
            if self.app.should_quit {
                break;
            }

            // Event-driven puro: escucha ambos threads
            // - monitor_rx: eventos del monitor de sistema
            // - keyboard_rx: evento de Ctrl+C
            select! {
                recv(monitor_rx) -> msg => {
                    match msg {
                        Ok(AppEvent::DataUpdated) => {
                            // Los datos ya están actualizados en el Arc<Mutex<TuiData>>
                            // Renderizamos con los datos frescos
                            self.terminal.draw(|frame| {
                                draw_memory_chart(frame, &self.app);
                                draw_cpu_barchart(frame, &self.app);
                                draw_general_info_block(frame, &self.app);

                            })?;
                        }
                        Ok(AppEvent::Shutdown) => {
                            self.app.should_quit = true;
                            shutdown_tx.send(()).unwrap();
                        }
                        Err(_) => {
                            // Canal cerrado
                            break;
                        }
                    }
                }
                recv(keyboard_rx) -> msg => {
                    match msg {
                        Ok(AppEvent::Shutdown) => {
                            self.app.should_quit = true;
                            shutdown_tx.send(()).unwrap();
                        }
                        _ => {}
                    }
                }
            }
        }

        self.exit()?;
        Ok(())
    }

    /// Cleans up the terminal and restores normal mode.
    fn exit(&mut self) -> Result<()> {
        crossterm::execute!(
            self.terminal.backend_mut(),
            crossterm::terminal::LeaveAlternateScreen,
            crossterm::event::DisableMouseCapture
        )?;
        crossterm::terminal::disable_raw_mode()?;
        Ok(())
    }
}

