use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct InputParams {
    pub launch_height: f64,
    pub landing_height: f64,
    pub distance: f64,
    pub angle_min_deg: f64,
    pub angle_max_deg: f64,
    pub angle_step_deg: f64,
    pub gravity: f64,
}

#[derive(Debug, Clone)]
pub struct SolutionPoint {
    pub angle_deg: f64,
    pub speed_mps: f64,
}

#[derive(Debug, Clone)]
pub struct SolveResult {
    pub points: Vec<SolutionPoint>,
    pub slowest: SolutionPoint,
    pub fastest: SolutionPoint,
}

pub fn solve(params: &InputParams) -> Result<SolveResult, AppError> {
    validate(params)?;

    let mut points = Vec::new();
    let delta_h = params.landing_height - params.launch_height;

    let mut angle = params.angle_min_deg;
    while angle <= params.angle_max_deg + 1e-12 {
        if let Some(speed) =
            required_speed_for_angle(params.distance, delta_h, angle, params.gravity)
        {
            points.push(SolutionPoint {
                angle_deg: angle,
                speed_mps: speed,
            });
        }
        angle += params.angle_step_deg;
    }

    if points.is_empty() {
        return Err(AppError::NoFeasibleAngles {
            angle_min: params.angle_min_deg,
            angle_max: params.angle_max_deg,
            step: params.angle_step_deg,
        });
    }

    let mut slowest = points[0].clone();
    let mut fastest = points[0].clone();

    for point in &points[1..] {
        if point.speed_mps < slowest.speed_mps {
            slowest = point.clone();
        }
        if point.speed_mps > fastest.speed_mps {
            fastest = point.clone();
        }
    }

    Ok(SolveResult {
        points,
        slowest,
        fastest,
    })
}

fn validate(params: &InputParams) -> Result<(), AppError> {
    if params.distance <= 0.0 {
        return Err(AppError::NonPositiveDistance(params.distance));
    }
    if params.gravity <= 0.0 {
        return Err(AppError::NonPositiveGravity(params.gravity));
    }
    if params.angle_step_deg <= 0.0 {
        return Err(AppError::NonPositiveAngleStep(params.angle_step_deg));
    }
    if params.angle_min_deg >= params.angle_max_deg {
        return Err(AppError::InvalidAngleRange {
            min: params.angle_min_deg,
            max: params.angle_max_deg,
        });
    }
    Ok(())
}

/// Computes required launch speed for a single angle.
///
/// Formula:
/// v0 = sqrt(g * d^2 / (2 * cos(theta)^2 * (d * tan(theta) - delta_h)))
/// where delta_h = landing_height - launch_height.
pub fn required_speed_for_angle(
    distance: f64,
    delta_h: f64,
    angle_deg: f64,
    gravity: f64,
) -> Option<f64> {
    let theta = angle_deg.to_radians();
    let cos_theta = theta.cos();
    if cos_theta.abs() < 1e-10 {
        return None;
    }

    let tan_theta = theta.tan();
    let denominator = 2.0 * cos_theta * cos_theta * (distance * tan_theta - delta_h);
    if denominator <= 0.0 || !denominator.is_finite() {
        return None;
    }

    let v_sq = gravity * distance * distance / denominator;
    if v_sq <= 0.0 || !v_sq.is_finite() {
        return None;
    }

    Some(v_sq.sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn speed_at_45_deg_same_height_matches_expected() {
        let speed = required_speed_for_angle(10.0, 0.0, 45.0, 9.80665).unwrap();
        assert!((speed - 9.902853).abs() < 1e-3);
    }

    #[test]
    fn solve_finds_feasible_angles() {
        let params = InputParams {
            launch_height: 1.0,
            landing_height: 1.0,
            distance: 12.0,
            angle_min_deg: 10.0,
            angle_max_deg: 80.0,
            angle_step_deg: 5.0,
            gravity: 9.80665,
        };

        let result = solve(&params).unwrap();
        assert!(!result.points.is_empty());
        assert!(result.slowest.speed_mps <= result.fastest.speed_mps);
    }
}
