pub struct MathUtils {}
impl MathUtils {
    pub fn max_measurement(data: &[(f64, f64)]) -> Option<f64> {
        data.iter()
            .map(|&(_time, measurement)| measurement)
            .fold(None, |max, val| match max {
                None => Some(val),
                Some(current_max) => Some(current_max.max(val)),
            })
    }
}
