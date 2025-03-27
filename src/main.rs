mod pid;
mod visualizer;
mod input;
mod physics;
mod console_input;

use pid::PID;
use visualizer::Visualizer;
use input::Input;
use crate::physics::Physics;
use crate::visualizer::VisualizerConfig;
use crate::console_input::ConsoleInput;

fn main() -> anyhow::Result<()> {
    let input = Input::set(1.5, 0.2);
    let mut pid = PID::new(40.0, 61.0, 10.0);
    let phx = Physics::new(3.0, 0.5, 2.0, 0.0);

    // TODO: try different init setup, investigate, make configurable
    // System state clearly tracked
    let mut measurement = 0.0;  // Initial position
    let mut velocity = 0.0;     // Initial velocity

    let dt = 0.01; // Time step / resolution

    // Parameters clearly defined
    let k = 1.0; // System gain (PID coefficient)

    let mut plot_data = vec![];

    for step in 0..(phx.sim_time / dt) as usize {
        // feedback loop with previous measurement
        // PID output as control input
        let control = pid.update(input.target_value, measurement, dt);

        // First-order dynamics applied to control input (with inertia)
        let effective_input = (-measurement + k * control) / phx.inertia;

        // Second-order dynamics (mass-spring-damper system) applied clearly:
        let acceleration = effective_input - 2.0 * phx.damping * phx.n_freq * velocity - phx.n_freq.powi(2) * measurement;

        velocity += acceleration * dt;
        measurement += velocity * dt;

        plot_data.push((step as f64 * dt, measurement));
    }

    println!("{:#?}", plot_data.iter().map(|(t, v)| v.to_string()).collect::<Vec<_>>().join(", "));

    // Visualize the result
    let v = Visualizer::new(VisualizerConfig::new("PID".to_string(), "pid_response.png".to_string(), 1000, 800));
    v.plot_response(&input, &plot_data, &phx.sim_time)?;

    Ok(())
}