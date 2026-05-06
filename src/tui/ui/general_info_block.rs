//! System information display block.
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType};
use ratatui::Frame;

use ratatui::widgets::Paragraph;
use crate::tui::app::App;

/// Renders a styled block displaying current system metrics.
///
/// Shows real-time memory and CPU usage in the left panel with color-coded values.
pub fn draw_general_info_block(frame: &mut Frame, app: &App) {
    let vertical = Layout::vertical([Constraint::Length(23), Constraint::Fill(1)]).spacing(1);
    let horizontal = Layout::horizontal([Constraint::Percentage(33); 3]).spacing(1);
    let [_, main] = frame.area().layout(&vertical);
    let [left, _, _] = main.layout(&horizontal);

    
    //render_bordered_block(frame, left);
    let text = extract_general_information(app);
    render_styled_block(frame, left, text);
    //render_custom_bordered_block(frame, right);
}


/// Extracts current RAM and CPU usage statistics.
fn extract_general_information(app: &App) -> Vec<Line<'_>> {
    if let Ok(tui_data) = app.tui_data.lock() {
        let text = vec![
            Line::from(vec![
                Span::styled("RAM usage: ", Style::new().cyan().bold()),
                Span::styled(
                    format!("{} GB", tui_data.used_memory.back().unwrap()),
                    Style::new().green().bold(),
                ),
            ]),
            Line::from(vec![
                Span::styled("CPU usage: ", Style::new().cyan().bold()),
                Span::styled(
                    format!("{}%", tui_data.cpu_usage_percentage()),
                    Style::new().green().bold(),
                ),
            ]),
            Line::from(vec![
                Span::styled("CPU temp: ", Style::new().cyan().bold()),
                Span::styled(
                    format!("{} °C", tui_data.cpu_temp_celsius),
                    Style::new().green().bold(),
                ),
            ]),
            
        ];
        text
    } else {
        Vec::new()
    }
}


/// Renders styled text in a bordered block with fixed positioning.
fn render_styled_block(frame: &mut Frame, area: Rect, text: Vec<Line<'_>>) {
    let block = Block::bordered()
        .style(Style::new().blue().on_black().bold().italic())
        .border_type(BorderType::Rounded)
        .title("System information");

    // Crear el párrafo con el bloque
    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(ratatui::layout::Alignment::Center);

    frame.render_widget(paragraph, area);
}