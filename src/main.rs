mod pid;
mod visualizer;
mod input;

use pid::PID;
use visualizer::Visualizer;
use input::Input;
use crate::visualizer::VisualizerConfig;

fn main() -> anyhow::Result<()> {
    let input = Input::set(1.5, 1.0);//ingest();
    let mut pid = PID::new(10.0, 0.0, 0.0);//ingest();

    // TODO: try different init setup, investigate, make configurable
    let mut measurement = 0.0; // Initial value
    let dt = 0.01; // Time step
    let sim_time = 5.0; // Total simulation time

    let mut plot_data = vec![];

    for step in 0..(sim_time / dt) as usize {
        // feedback loop with previous measurement
        let control = pid.update(input.target_value, measurement, dt);

        // TODO: replace with inertia, mass-damping and noise modelling
        measurement += control * dt;

        plot_data.push((step as f64 * dt, measurement));
    }

    println!("{:#?}", plot_data.iter().map(|(t, v)| v.to_string()).collect::<Vec<_>>().join(", "));

    // Visualize the result
    let v = Visualizer::new(VisualizerConfig::new("PID".to_string(), "pid_response.png".to_string(), 1000, 800));
    v.plot_response(&input, &plot_data, &sim_time)?;

    Ok(())
}