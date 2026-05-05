use std::io;
use ratatui::Frame;
use ratatui::layout::Rect;

/// Calculates a chart area based on percentage dimensions.
///
/// Returns a centered rectangle within the terminal frame, scaled by percent_width and percent_height.
/// Errors if the resulting area is too small to render (< 10x5).
pub fn create_chart_area(frame: &mut Frame, percent_width: u16, percent_height: u16, x: u16, y: u16) -> Result<Rect, io::Error> {
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
        x,
        y,
        width: chart_width,
        height: chart_height,
    })
} 