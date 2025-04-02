#[derive(Debug, Clone, Copy)]
pub struct Physics {
    pub sim_time: f64,
    pub tau_motor: f64,
    pub tau_inertia: f64,
}
impl Default for Physics {
    fn default() -> Self {
        Self {
            sim_time: 0.3,
            tau_motor: 0.1,
            tau_inertia: 0.01,
        }
    }
}
// TODO: implement vibrations on rapid change and gyro noise
impl Physics {
    pub fn new(sim_time: f64, tau_motor: f64, inertia: f64, effective_damping: f64) -> Self {
        let tau_inertia = inertia / effective_damping;
        Self {
            sim_time,
            tau_motor,
            tau_inertia,
        }
    }

    pub fn cetus_pro(sim_time: f64, effective_damping: f64) -> Self {
        let inertia = Self::cetus_real_inertia() * 3 as f64;
        let tau_motor = 0.05; // PID-to-motor
        Self::new(sim_time, tau_motor, inertia, effective_damping)
    }

    fn cetus_real_inertia() -> f64 {
        // Component masses explicitly
        let battery_mass = 0.015; // kg
        let fc_mass = 0.004; // kg
        let frame_mass = 0.010; // kg
        let motor_total_mass = 0.015; // kg (all 4 motors together)

        // Component distances explicitly from CG (approx. guesses, measure accurately if possible)
        let battery_distance: f64 = 0.02; // 2 cm (0.02 m) battery slightly offset
        let fc_distance: f64 = 0.0; // FC at drone center
        let frame_distance: f64 = 0.035; // 3.5 cm frame mass evenly distributed around center
        let motor_distance: f64 = 0.058; // 5.8 cm (half-diagonal of 117 mm frame approx.)

        // Explicit calculation (sum of each mass contribution clearly)
        let battery_inertia = battery_mass * battery_distance.powi(2);
        let fc_inertia = fc_mass * fc_distance.powi(2);
        let frame_inertia = frame_mass * frame_distance.powi(2);
        let motors_inertia = motor_total_mass * motor_distance.powi(2);

        battery_inertia + fc_inertia + frame_inertia + motors_inertia
    }
}
