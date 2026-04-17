use std::path::Path;

use plotters::prelude::*;

use crate::error::AppError;
use crate::physics::ballistics::SolutionPoint;

pub fn render_speed_plot(path: &str, points: &[SolutionPoint]) -> Result<(), AppError> {
    if points.is_empty() {
        return Ok(());
    }

    if let Some(parent) = Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).map_err(|e| AppError::PlotWrite {
                path: path.to_string(),
                reason: e.to_string(),
            })?;
        }
    }

    let x_min = points
        .iter()
        .map(|p| p.angle_deg)
        .fold(f64::INFINITY, f64::min);
    let x_max = points
        .iter()
        .map(|p| p.angle_deg)
        .fold(f64::NEG_INFINITY, f64::max);
    let y_min = points
        .iter()
        .map(|p| p.speed_mps)
        .fold(f64::INFINITY, f64::min);
    let y_max = points
        .iter()
        .map(|p| p.speed_mps)
        .fold(f64::NEG_INFINITY, f64::max);

    if Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.eq_ignore_ascii_case("svg"))
        .unwrap_or(false)
    {
        draw_svg(path, points, x_min, x_max, y_min, y_max)
    } else {
        draw_png(path, points, x_min, x_max, y_min, y_max)
    }
}

fn draw_png(
    path: &str,
    points: &[SolutionPoint],
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
) -> Result<(), AppError> {
    let root = BitMapBackend::new(path, (1280, 720)).into_drawing_area();
    draw_chart(root, path, points, x_min, x_max, y_min, y_max)
}

fn draw_svg(
    path: &str,
    points: &[SolutionPoint],
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
) -> Result<(), AppError> {
    let root = SVGBackend::new(path, (1280, 720)).into_drawing_area();
    draw_chart(root, path, points, x_min, x_max, y_min, y_max)
}

fn draw_chart<DB: DrawingBackend>(
    root: DrawingArea<DB, plotters::coord::Shift>,
    path: &str,
    points: &[SolutionPoint],
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
) -> Result<(), AppError> {
    root.fill(&WHITE).map_err(|e| AppError::PlotWrite {
        path: path.to_string(),
        reason: e.to_string(),
    })?;

    // Add a small margin so points near boundaries remain visible.
    let y_margin = ((y_max - y_min) * 0.1).max(0.1);
    let mut chart = ChartBuilder::on(&root)
        .caption("Required Launch Speed vs Angle", ("sans-serif", 36))
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(70)
        .build_cartesian_2d(x_min..x_max, (y_min - y_margin)..(y_max + y_margin))
        .map_err(|e| AppError::PlotWrite {
            path: path.to_string(),
            reason: e.to_string(),
        })?;

    chart
        .configure_mesh()
        .x_desc("Angle (deg)")
        .y_desc("Required speed (m/s)")
        .draw()
        .map_err(|e| AppError::PlotWrite {
            path: path.to_string(),
            reason: e.to_string(),
        })?;

    chart
        .draw_series(LineSeries::new(
            points.iter().map(|p| (p.angle_deg, p.speed_mps)),
            &BLUE,
        ))
        .map_err(|e| AppError::PlotWrite {
            path: path.to_string(),
            reason: e.to_string(),
        })?
        .label("speed")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()
        .map_err(|e| AppError::PlotWrite {
            path: path.to_string(),
            reason: e.to_string(),
        })?;

    root.present().map_err(|e| AppError::PlotWrite {
        path: path.to_string(),
        reason: e.to_string(),
    })?;

    Ok(())
}
