use std::io;

pub struct PID {
    pub kp: f64,     // Proportional gain
    pub ki: f64,     // Integral gain
    pub kd: f64,     // Derivative gain
    integral: f64,   // Integral accumulator
    prev_error: f64, // Previous error for derivative calculation
}

impl PID {
    pub fn new(kp: f64, ki: f64, kd: f64) -> Self {
        Self {
            kp, ki, kd,
            integral: 0.0,
            prev_error: 0.0,
        }
    }

    pub fn ingest() -> Self{
        println!("Type P value");

        let mut p_value_s = String::new();
        io::stdin()
            .read_line(&mut p_value_s)
            .expect("Failed to read P value.");
        let p_value = p_value_s.trim().parse()
            .expect("Please enter a valid float number.");

        println!("Type I value");
        let mut i_value_s = String::new();
        io::stdin()
            .read_line(&mut i_value_s)
            .expect("Failed to read I value.");
        let i_value = i_value_s.trim().parse()
            .expect("Please enter a valid float number.");

        println!("Type D value");
        let mut d_value_s = String::new();
        io::stdin()
            .read_line(&mut d_value_s)
            .expect("Failed to read D value.");
        let d_value = d_value_s.trim().parse()
            .expect("Please enter a valid float number.");

        PID::new(p_value, i_value, d_value)
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