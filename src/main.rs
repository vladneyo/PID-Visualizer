mod pid;
use pid::PID;
use plotters::prelude::*;

fn main() -> anyhow::Result<()> {
    // PID setup with manually set values
    let mut pid = PID::new(1.0, 0.1, 0.05);

    // Simulation params
    let setpoint = 1.0;     // Desired value
    let mut measurement = 0.0; // Initial value
    let dt = 0.01;          // Timestep
    let sim_time = 5.0;     // Total simulation time

    // Store values for plotting
    let mut data = vec![];

    for step in 0..(sim_time / dt) as usize {
        let control = pid.update(setpoint, measurement, dt);

        // Simple system response simulation (can later replace with realistic functions)
        measurement += control * dt;

        data.push((step as f64 * dt, measurement));
    }

    // Visualize the result
    plot_response(&data)?;

    Ok(())
}

fn plot_response(data: &[(f64, f64)]) -> anyhow::Result<()> {
    let root = BitMapBackend::new("pid_response.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("PID Response", ("sans-serif", 40))
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..5.0, 0.0..2.0)?;

    chart.configure_mesh()
        .x_desc("Time (s)")
        .y_desc("Measurement")
        .draw()?;

    chart.draw_series(LineSeries::new(data.iter().copied(), &RED))?;

    Ok(())
}