#[derive(Debug, Clone, Copy)]
pub struct Input {
    pub target_value: f64,
    pub acceptable_time: f64, // in seconds
}
impl Default for Input {
    fn default() -> Self {
        Self {
            target_value: 1.0,
            acceptable_time: 1.0,
        }
    }
}
impl Input {
    pub fn set(value: f64, time: f64) -> Input {
        Self {
            target_value: value,
            acceptable_time: time,
        }
    }
}
