mod console_input;
mod input;
mod math_utils;
mod physics;
mod pid;
mod pid_processor;
mod visualizer;
mod web_controller;

use crate::web_controller::{get_image, update_input};
use actix_files::Files;
use actix_web::{web, App, HttpServer};
// fn main() -> anyhow::Result<()> {
//     let visualizer = Visualizer::new(VisualizerConfig::new(
//         "PID Response".to_string(),
//         "pid_response.png".to_string(),
//         2000,
//         1400,
//         30,
//     ));
//
//     let input = Input::set(1.5, 0.2);
//     let pid = PID::new(2.0, 13.0, 0.02);
//     let phx = Physics::cetus_pro(3.0, 0.0007);
//     println!("{:?}", phx);
//
//     let mut pid_processor = PIDProcessor::new(0.0, 0.0, 1.0, 0.01, pid, phx.clone());
//     let plot_data = pid_processor.process(&input);
//
//     visualizer.plot_response(&input, &plot_data, &phx.sim_time)?;
//
//     Ok(())
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // API endpoint for simulation input.
            .route("/api/input", web::post().to(update_input))
            // API endpoint for the PID response image.
            .route("/api/image", web::get().to(get_image))
            // Serve the Angular application (assuming it's built into "./angular-dist").
            .service(Files::new("/", "./angular-dist").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
