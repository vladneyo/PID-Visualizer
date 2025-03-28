#[derive(Debug, Clone, Copy)]
pub struct PID {
    pub kp: f64,     // Proportional gain
    pub ki: f64,     // Integral gain
    pub kd: f64,     // Derivative gain
    integral: f64,   // Integral accumulator
    prev_error: f64, // Previous error for derivative calculation
}

impl PID {
    pub fn default() -> PID {
        PID {
            kp: 2.0,
            ki: 2.0,
            kd: 0.01,
            integral: 0.0,
            prev_error: 0.0,
        }
    }
    pub fn new(kp: f64, ki: f64, kd: f64) -> Self {
        Self {
            kp,
            ki,
            kd,
            integral: 0.0,
            prev_error: 0.0,
        }
    }

    pub fn update(&mut self, target_point: f64, measurement: f64, dt: f64) -> f64 {
        let error = target_point - measurement;
        // integrate errors over time
        self.integral += error * dt;
        // speed of error changes
        let derivative = (error - self.prev_error) / dt;
        self.prev_error = error;

        // PID formula
        self.kp * error + self.ki * self.integral + self.kd * derivative
    }
}
