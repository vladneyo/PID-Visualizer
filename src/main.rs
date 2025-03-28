mod console_input;
mod input;
mod math_utils;
mod physics;
mod pid;
mod pid_processor;
mod visualizer;

use crate::physics::Physics;
use crate::pid_processor::PIDProcessor;
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
    let pid = PID::new(2.0, 13.0, 0.02);
    let phx = Physics::cetus_pro(3.0, 0.0007);
    println!("{:?}", phx);

    let mut pid_processor = PIDProcessor::new(0.0, 0.0, 1.0, 0.01, pid, phx.clone());
    let plot_data = pid_processor.process(&input);

    visualizer.plot_response(&input, &plot_data, &phx.sim_time)?;

    Ok(())
}
