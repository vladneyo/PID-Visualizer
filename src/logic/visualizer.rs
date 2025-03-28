use crate::logic::input::Input;
use crate::utils::math_utils::MathUtils;
use plotters::prelude::*;
use plotters::style::full_palette::GREY_A200;

#[derive(Debug, Clone)]
pub struct VisualizerConfig {
    pub caption: String,
    pub output_path: String,
    pub width: u32,
    pub height: u32,
    pub font_size: u32,
    pub font_family: String,
}
impl VisualizerConfig {
    pub fn new(
        caption: String,
        output_path: String,
        width: u32,
        height: u32,
        font_size: u32,
    ) -> Self {
        Self {
            caption,
            output_path,
            width,
            height,
            font_size,
            font_family: FontFamily::SansSerif.clone().as_str().to_string(),
        }
    }
}

pub struct Visualizer {
    config: VisualizerConfig,
}
impl Visualizer {
    pub fn new(config: VisualizerConfig) -> Self {
        Self { config }
    }

    pub fn plot_response(
        &self,
        target: &Input,
        data: &[(f64, f64)],
        time_window: &f64,
    ) -> anyhow::Result<()> {
        let root = BitMapBackend::new(
            &self.config.output_path,
            (self.config.width, self.config.height),
        )
        .into_drawing_area();
        root.fill(&WHITE)?;

        let extra_space = 0.5;

        // adapt to max values of result data
        let max_x = (target.acceptable_time + extra_space).max(*time_window);
        let max_y = target
            .target_value
            .max(MathUtils::max_measurement(&data).unwrap_or(1.0))
            + extra_space;

        let mut chart = ChartBuilder::on(&root)
            .caption(
                &self.config.caption,
                (*&self.config.font_family.as_str(), self.config.font_size),
            )
            .margin(50)
            .x_label_area_size(80)
            .y_label_area_size(80)
            .build_cartesian_2d(0.0..max_x, 0.0..max_y)?;

        chart
            .configure_mesh()
            .x_desc("Time")
            .x_label_style((*&self.config.font_family.as_str(), self.config.font_size))
            .y_desc("Value")
            .y_label_style((*&self.config.font_family.as_str(), self.config.font_size))
            .bold_line_style(ShapeStyle {
                color: BLACK.into(),
                filled: false,
                stroke_width: 1,
            })
            .light_line_style(ShapeStyle {
                color: GREY_A200.into(),
                filled: false,
                stroke_width: 1,
            })
            .draw()?;

        chart.draw_series(LineSeries::new(
            data.iter().copied(),
            ShapeStyle {
                color: BLUE.into(),
                filled: false,
                stroke_width: 3,
            },
        ))?;

        // Draw a dashed horizontal line at the target value.
        chart.draw_series(DashedLineSeries::new(
            vec![(0.0, target.target_value), (max_x, target.target_value)],
            30,
            20,
            ShapeStyle {
                color: RED.into(),
                filled: false,
                stroke_width: 3,
            },
        ))?;

        // Highlight a target value explicitly:
        let target_point = (target.acceptable_time, target.target_value);

        chart.draw_series(PointSeries::of_element(
            [target_point],
            5, // Size of the marker
            &RED,
            &|c, s, st| {
                return EmptyElement::at(c)
                    + Circle::new((0, 0), s, st.filled())
                    + Text::new(
                        "Target",
                        (10, -10),
                        (*&self.config.font_family.as_str(), *&self.config.font_size)
                            .into_font()
                            .color(&BLACK),
                    );
            },
        ))?;

        root.present()?;
        Ok(())
    }
}
