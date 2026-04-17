use clap::Parser;

use body_throwing_movement::cli::Cli;
use body_throwing_movement::output::plot::render_speed_plot;
use body_throwing_movement::output::table::{render_stdout, write_csv};
use body_throwing_movement::physics::ballistics::{solve, InputParams};

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), body_throwing_movement::error::AppError> {
    let cli = Cli::parse();

    let params = InputParams {
        launch_height: cli.launch_height,
        landing_height: cli.landing_height,
        distance: cli.distance,
        angle_min_deg: cli.angle_min,
        angle_max_deg: cli.angle_max,
        angle_step_deg: cli.angle_step,
        gravity: cli.gravity,
    };

    let result = solve(&params)?;

    println!("{}", render_stdout(&result));

    if let Some(csv_path) = cli.csv_out {
        write_csv(&csv_path, &result.points)?;
        println!("CSV written to: {csv_path}");
    }

    if let Some(plot_path) = cli.plot_out {
        render_speed_plot(&plot_path, &result.points)?;
        println!("Plot written to: {plot_path}");
    }

    Ok(())
}

