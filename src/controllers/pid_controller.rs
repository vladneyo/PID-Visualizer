use crate::logic::input::Input;
use crate::logic::physics::Physics;
use crate::logic::pid::PID;
use crate::logic::pid_processor::PIDProcessor;
use crate::logic::visualizer::{Visualizer, VisualizerConfig};
use timetrap::*;

pub struct PIDController {}
impl PIDController {
    pub fn update(input: Input, pid: PID, phx: Physics) -> Option<anyhow::Result<()>> {
        let visualizer = Visualizer::new(VisualizerConfig::new(
            "PID Response".to_string(),
            "pid_response.png".to_string(),
            2000,
            1400,
            30,
        ));

        println!("{:?}", phx);

        let mut pid_processor = PIDProcessor::default()
            .starting_position(0.0)
            .motor_output(0.0)
            .time_resolution(0.001)
            .pid(pid)
            .phx(phx);

        let mut plot_data: Vec<(f64, f64)> = vec![];

        trap!("pid_processor.process()", {
            plot_data = pid_processor.process(&input);
        });

        trap_mem!("visualizer.plot_response", MemUnits::Kb, {
            Some(visualizer.plot_response(&input, &plot_data, &phx.sim_time))
        })
    }
}
