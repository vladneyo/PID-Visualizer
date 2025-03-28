use crate::logic::input::Input;
use crate::logic::physics::Physics;
use crate::logic::pid::PID;

pub struct PIDProcessor {
    pub measurement: f64, // init position
    pub motor_output: f64,
    pub k: f64,
    pub dt: f64,
    pub pid: PID,
    pub phx: Physics,
}

impl PIDProcessor {
    pub fn new(
        measurement: f64,
        motor_output: f64,
        k: f64,
        dt: f64,
        pid: PID,
        phx: Physics,
    ) -> Self {
        Self {
            measurement,
            motor_output,
            k,
            dt,
            pid,
            phx,
        }
    }

    pub fn process(&mut self, input: &Input) -> Vec<(f64, f64)> {
        let mut plot_data = vec![];

        for step in 0..(self.phx.sim_time / self.dt) as usize {
            let control = self
                .pid
                .update(input.target_value, self.measurement, self.dt);

            // Motor delay
            let du = (self.k * control - self.motor_output) / self.phx.tau_motor;
            self.motor_output += du * self.dt;

            // Inertia delay clearly explicit:
            let d_measurement = (self.motor_output - self.measurement) / self.phx.tau_inertia;
            self.measurement += d_measurement * self.dt;

            plot_data.push((step as f64 * self.dt, self.measurement));
        }

        plot_data
    }
}
