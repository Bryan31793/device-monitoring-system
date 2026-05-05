//! Memory usage chart rendering.

use std::io;

use ratatui::{
    prelude::*,
    widgets::*,
};

use crate::tui::app::App;

/// Renders a line chart displaying memory usage over time.
///
/// Draws the memory consumption history from app data as a cyan line chart
/// with proper scaling and axis labels. Does nothing if the terminal is too small.
pub fn draw_memory_chart(frame: &mut Frame, app: &App) {
    // *** Cambiar unwrap para produccion ***
    let size = create_chart_area(frame, 100, 40).unwrap();

    let data_vec = extract_memory_data(app);
    if data_vec.is_empty() {
        return;
    }

    let data: &[(f64, f64)] = &data_vec;
    let datasets = create_datasets(data);
    let x_labels = create_x_labels(app);
    let (min_y, max_y) = calculate_y_bounds(&data_vec);

    let chart = build_chart(datasets, x_labels, data.len(), min_y, max_y);
    frame.render_widget(chart, size);
}

/// Extracts memory usage data from the app's shared state.
fn extract_memory_data(app: &App) -> Vec<(f64, f64)> {
    if let Ok(tui_data) = app.tui_data.lock() {
        tui_data
            .used_memory
            .iter()
            .enumerate()
            .map(|(i, &v)| (i as f64, v))
            .collect()
    } else {
        Vec::new()
    }
}

/// Creates the dataset for the memory chart.
fn create_datasets<'a>(data: &'a[(f64, f64)]) -> Vec<Dataset<'a>> {
    vec![Dataset::default()
        //.name("Used Memory (GB)")
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Cyan))
        .data(data)]
}

/// Creates the area in which the chart would be placed.
fn create_chart_area(frame: &mut Frame, percent_width: u16, percent_height: u16) -> Result<Rect, io::Error> {
    let size = frame.area();

    let chart_width = size.width * percent_width / 100;
    let chart_height = size.height * percent_height / 100;

    if chart_width < 10 || chart_height < 5 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData, 
            "El area resultante es demasiado pequeña para el grafico"
        ));
    }

    Ok(Rect {
        x: (size.width - chart_width) / 2,
        y: (size.height - chart_height) / 2,
        width: chart_width,
        height: chart_height,
    })
} 

/// Creates X-axis labels with 5-second intervals.
fn create_x_labels<'a>(app: &'a App) -> Vec<Span<'a>> {
    if let Ok(tui_data) = app.tui_data.lock() {
        tui_data
            .used_memory
            .iter()
            .enumerate()
            .filter_map(|(i, _)| if i % 5 == 0 { Some(Span::raw(i.to_string())) } else { None })
            .collect()
    } else {
        Vec::new()
    }
}

/// Calculates Y-axis bounds with 15% padding.
fn calculate_y_bounds(data: &[(f64, f64)]) -> (f64, f64) {
    let max_y = data.iter()
        .map(|(_, v)| *v)
        .fold(0.0_f64, f64::max);

    let min_y = data.iter()
        .map(|(_, v)| *v)
        .fold(f64::MAX, f64::min);

    let padding = (max_y - min_y) * 0.15;
    ((min_y - padding).max(0.0), max_y + padding)
}

/// Builds the configured chart widget.
fn build_chart<'a>(
    datasets: Vec<Dataset<'a>>,
    x_labels: Vec<Span<'a>>,
    data_len: usize,
    min_y: f64,
    max_y: f64,
) -> Chart<'a> {
    Chart::new(datasets)
        .block(
            Block::default()
                .title("Memory Usage")
                .borders(Borders::ALL)
                .style(Color::Gray),
        )
        .x_axis(
            Axis::default()
                .title("Time (seconds)")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, (data_len - 1) as f64])
                .labels(x_labels),
        )
        .y_axis(
            Axis::default()
                .title("GB")
                .style(Style::default().fg(Color::Gray))
                .bounds([min_y, max_y])
                .labels(vec![
                    Span::raw(format!("{:.1}", min_y)),
                    Span::raw(format!("{:.1}", (min_y + max_y) / 2.0)),
                    Span::raw(format!("{:.1}", max_y)),
                ]),
        )
}

