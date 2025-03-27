mod console_input;
mod input;
mod math_utils;
mod physics;
mod pid;
mod visualizer;

use crate::console_input::ConsoleInput;
use crate::physics::Physics;
use crate::visualizer::VisualizerConfig;
use input::Input;
use pid::PID;
use visualizer::Visualizer;

fn main() -> anyhow::Result<()> {
    let visualizer = Visualizer::new(VisualizerConfig::new(
        "PID Response".to_string(),
        "pid_response.png".to_string(),
        2000,
        1400,
        30,
    ));

    let input = Input::set(1.5, 0.2);
    let mut pid = PID::new(2.0, 13.0, 0.02);
    let phx = Physics::cetus_pro(3.0, 0.0007);
    println!("{:?}", phx);
    let mut measurement = 0.0; // Initial position
    let mut motor_output = 0.0; // intermediate motor response
    let dt = 0.01; // Time step / resolution
    let k = 1.0; // System gain (PID coefficient)

    let mut plot_data = vec![];

    for step in 0..(phx.sim_time / dt) as usize {
        let control = pid.update(input.target_value, measurement, dt);

        // Motor delay
        let du = (k * control - motor_output) / phx.tau_motor;
        motor_output += du * dt;

        // Inertia delay clearly explicit:
        let d_measurement = (motor_output - measurement) / phx.tau_inertia;
        measurement += d_measurement * dt;

        plot_data.push((step as f64 * dt, measurement));
    }

    // println!(
    //     "{:#?}",
    //     plot_data
    //         .iter()
    //         .map(|(t, v)| v.to_string())
    //         .collect::<Vec<_>>()
    //         .join(", ")
    // );

    // Visualize the result
    visualizer.plot_response(&input, &plot_data, &phx.sim_time)?;

    Ok(())
}
