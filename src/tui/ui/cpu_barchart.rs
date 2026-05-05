//! CPU usage barchart rendering.
use ratatui::{
    prelude::*,
    widgets::{Bar, BarChart, BarGroup, Block, Borders},
    layout::{Layout, Constraint},
};

use crate::tui::app::App;
use crate::tui::ui::utils::create_chart_area;

/// Renders a barchart displaying CPU usage per core.
///
/// Each bar represents a CPU core, color-coded by usage:
/// - Green: < 50%
/// - Yellow: 50-80%
/// - Red: > 80%
pub fn draw_cpu_barchart(frame: &mut Frame, app: &App) {
    let size = create_chart_area(frame, 40, 30, 0, 0).unwrap();

    let (bars, xd) = extract_cpu_bars_data(app);
    if bars.is_empty() {
        return;
    }

    // Sidebar de 8 columnas + resto para la gráfica
    let layout = Layout::horizontal([
        Constraint::Length(8),
        Constraint::Fill(1),
    ]);
    let [_, chart_area] = layout.areas(size);

    // MEJORA 1: max(100) fuerza escala real 0-100%
    // MEJORA 2: bar_width(4) + bar_gap(1) dan espacio a labels y separación visual
    let group = build_bars(bars);
    let chart = build_barchart_chart(group, xd);

    frame.render_widget(chart, chart_area);
}

/// Extracts CPU usage data and builds styled bars for each core.
///
/// Returns a tuple of (bars, max_usage) where bars are ready to render
/// and max_usage is used for chart scaling.
fn extract_cpu_bars_data(app: &App) -> (Vec<Bar<'static>>, f32) {
    let Ok(tui_data) = app.tui_data.lock() else {
        return (Vec::new(), 0.0);
    };
    let cpu_data = &tui_data.cpu_usage;

    let max_usage = cpu_data.iter().cloned().fold(0.0_f32, f32::max);

    let xd = cpu_data
        .iter()
        .enumerate()
        .map(|(idx, &usage)| {
            let value = usage.round() as u64;

            // MEJORA: label corto "C0"–"C99" para que quepa en bar_width(4)
            let label = format!("C{:<2}", idx);

            // MEJORA: text_value muestra "50%" en lugar del número crudo
            let text_value = format!("{value:2}%");

            let style = if usage < 50.0 {
                Style::default().fg(Color::Green)
            } else if usage < 80.0 {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::Red)
            };

            Bar::default()
                .value(value)
                .label(label)
                .text_value(text_value)
                .style(style)
                // Fondo tenue para cores en 0% (visible como placeholder)
                .value_style(
                    Style::default()
                        .fg(Color::Black)
                        .bg(if usage < 50.0 { Color::Green }
                            else if usage < 80.0 { Color::Yellow }
                            else { Color::Red }),
                )
        })
        .collect();
    (xd, max_usage)
}

/// Creates a bar group from the individual bars.
fn build_bars(bars: Vec<Bar<'_>>) -> BarGroup<'_> {
    BarGroup::default()
    .bars(&bars)
}

/// Builds the barchart widget with styling and dynamic scaling.
fn build_barchart_chart(group: BarGroup<'_>, max_usage: f32) -> BarChart<'_> {

    let dynamic_max = (max_usage as u64).max(25);  // piso de 25%

    let chart = BarChart::default()
        .direction(Direction::Vertical)
        .data(group)
        .bar_width(4)
        .bar_gap(1)
        .max(dynamic_max)                          // ← crítico: escala fija 0-100
        .block(
            Block::default()
                .title(Span::styled(
                    " CPU Usage per Core (%) ",
                    Style::default().fg(Color::Cyan).bold(),
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );

    chart
}