use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(
    author,
    version,
    about = "Compute launch speeds for feasible projectile angles",
    long_about = "Given launch height, landing height, and horizontal distance, this tool scans an angle range and computes the required launch speed for each feasible angle."
)]
pub struct Cli {
    #[arg(long, help = "Launch height in meters")]
    pub launch_height: f64,

    #[arg(long, help = "Landing height in meters")]
    pub landing_height: f64,

    #[arg(long, help = "Horizontal distance in meters")]
    pub distance: f64,

    #[arg(long, default_value_t = 1.0, help = "Minimum scanned angle in degrees")]
    pub angle_min: f64,

    #[arg(long, default_value_t = 89.0, help = "Maximum scanned angle in degrees")]
    pub angle_max: f64,

    #[arg(long, default_value_t = 0.5, help = "Angle step in degrees")]
    pub angle_step: f64,

    #[arg(long, default_value_t = 9.80665, help = "Gravitational acceleration in m/s^2")]
    pub gravity: f64,

    #[arg(long, help = "Optional CSV output path")]
    pub csv_out: Option<String>,

    #[arg(long, help = "Optional chart output path (.png or .svg)")]
    pub plot_out: Option<String>,
}

