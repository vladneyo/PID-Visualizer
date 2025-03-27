pub struct Physics {
    pub sim_time: f64,
    pub inertia: f64, // First-order inertia
    pub n_freq: f64, // Natural frequency (second-order)
    pub damping: f64, // Damping ratio (second-order, lower = more oscillations)
}
impl Physics {
    pub fn new(sim_time: f64, inertia: f64, n_freq: f64, damping: f64) -> Self {
        Self{sim_time, inertia, n_freq, damping}
    }

    pub fn cetus(sim_time: f64) -> Self {
        Self::new(sim_time, 0.0, 0.1, 0.0)
    }
}