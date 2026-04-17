use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("distance must be positive, got {0}")]
    NonPositiveDistance(f64),

    #[error("gravity must be positive, got {0}")]
    NonPositiveGravity(f64),

    #[error("angle step must be positive, got {0}")]
    NonPositiveAngleStep(f64),

    #[error("angle range is invalid: min={min}, max={max}")]
    InvalidAngleRange { min: f64, max: f64 },

    #[error("no feasible launch angle found in [{angle_min}, {angle_max}] with step {step}")]
    NoFeasibleAngles {
        angle_min: f64,
        angle_max: f64,
        step: f64,
    },

    #[error("failed to write csv to {path}: {source}")]
    CsvWrite {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to render plot to {path}: {reason}")]
    PlotWrite { path: String, reason: String },
}

