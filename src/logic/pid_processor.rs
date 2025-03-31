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

impl Default for PIDProcessor {
    fn default() -> Self {
        Self {
            measurement: 0.0,
            motor_output: 0.0,
            k: 1.0,
            dt: 0.01,
            pid: PID::default(),
            phx: Physics::default(),
        }
    }
}
impl PIDProcessor {
    pub fn motor_output(mut self, value: f64) -> Self {
        self.motor_output = value;
        self
    }

    pub fn starting_position(mut self, value: f64) -> Self {
        self.measurement = value;
        self
    }

    pub fn time_resolution(mut self, value: f64) -> Self {
        self.dt = value;
        self
    }

    pub fn pid(mut self, value: PID) -> Self {
        self.pid = value;
        self
    }

    pub fn phx(mut self, value: Physics) -> Self {
        self.phx = value;
        self
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
