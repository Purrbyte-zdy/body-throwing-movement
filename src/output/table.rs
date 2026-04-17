use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::error::AppError;
use crate::physics::ballistics::{SolutionPoint, SolveResult};

pub fn render_stdout(result: &SolveResult) -> String {
    let mut out = String::new();
    out.push_str("Feasible angle-speed pairs\n");
    out.push_str("--------------------------------\n");
    out.push_str("Angle (deg) | Required Speed (m/s)\n");
    out.push_str("------------+----------------------\n");

    for point in &result.points {
        out.push_str(&format!(
            "{:>10.2} | {:>20.4}\n",
            point.angle_deg, point.speed_mps
        ));
    }

    out.push('\n');
    out.push_str(&format!(
        "Slowest launch: angle = {:.2} deg, speed = {:.4} m/s\n",
        result.slowest.angle_deg, result.slowest.speed_mps
    ));
    out.push_str(&format!(
        "Fastest launch: angle = {:.2} deg, speed = {:.4} m/s\n",
        result.fastest.angle_deg, result.fastest.speed_mps
    ));

    out
}

pub fn write_csv(path: &str, points: &[SolutionPoint]) -> Result<(), AppError> {
    if let Some(parent) = Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).map_err(|source| AppError::CsvWrite {
                path: path.to_string(),
                source,
            })?;
        }
    }

    let mut file = File::create(path).map_err(|source| AppError::CsvWrite {
        path: path.to_string(),
        source,
    })?;

    file.write_all(b"angle_deg,speed_mps\n")
        .map_err(|source| AppError::CsvWrite {
            path: path.to_string(),
            source,
        })?;

    for p in points {
        let line = format!("{:.6},{:.6}\n", p.angle_deg, p.speed_mps);
        file.write_all(line.as_bytes())
            .map_err(|source| AppError::CsvWrite {
                path: path.to_string(),
                source,
            })?;
    }

    Ok(())
}
