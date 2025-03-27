use plotters::prelude::*;
use crate::input::Input;

pub struct VisualizerConfig{
    pub caption: String,
    pub output_path: String,
    pub width: u32,
    pub height: u32
}
impl VisualizerConfig {
    pub fn new(caption: String, output_path: String, width: u32, height: u32) -> Self{ Self{
        caption,
        output_path, width, height
    }}
}

pub struct Visualizer {
    config: VisualizerConfig
}
impl Visualizer {
    pub fn new(config: VisualizerConfig) -> Self { Self{
        config
    }}

    pub fn plot_response(&self, target: &Input , data: &[(f64, f64)], time_window: &f64) -> anyhow::Result<()> {
        let root = BitMapBackend::new(&self.config.output_path, (self.config.width, self.config.height)).into_drawing_area();
        root.fill(&WHITE)?;

        let extra_space = 0.5;

        let max_x = (target.acceptable_time + extra_space).max(*time_window);
        let max_y = target.target_value.max(max_measurement(&data).unwrap_or(1.0)) + extra_space;

        let mut chart = ChartBuilder::on(&root)
            .caption(&self.config.caption, ("sans-serif", 30))
            .margin(20)
            .x_label_area_size(30)
            .y_label_area_size(40)
            .build_cartesian_2d(0.0..max_x, 0.0..max_y)?;

        chart.configure_mesh().draw()?;

        // Your main line series
        let data = (0..data.len()).map(|x| {
            let x = x as f64 * 0.01;
            (x, (1.0 - (-x).exp())) // example data
        });

        chart.draw_series(LineSeries::new(data, &BLUE))?;

        // Highlight a target value explicitly:
        let target_point = (target.acceptable_time, target.target_value);

        chart.draw_series(PointSeries::of_element(
            [target_point],
            5, // Size of the marker
            &RED,
            &|c, s, st| {
                return EmptyElement::at(c)
                    + Circle::new((0,0), s, st.filled())
                    + Text::new("Target", (10, -10), ("sans-serif", 15).into_font().color(&BLACK));
            },
        ))?;

        root.present()?;
        Ok(())
    }
}

fn max_measurement(data: &[(f64, f64)]) -> Option<f64> {
    data.iter()
        .map(|&(_time, measurement)| measurement)
        .fold(None, |max, val| match max {
            None => Some(val),
            Some(current_max) => Some(current_max.max(val)),
        })
}