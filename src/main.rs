mod console_input;
mod input;
mod math_utils;
mod physics;
mod pid;
mod pid_processor;
mod visualizer;
mod web_controller;

use crate::web_controller::{get_image, update_input};
use actix_cors::Cors;
use actix_files::Files;
use actix_web::http::header;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ui_path = "./ui/pid-visualizer/dist/pid-visualizer/browser";

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin() // or restrict to "http://localhost:4200"
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .route("/api/input", web::post().to(update_input))
            .route("/api/image", web::get().to(get_image))
            // Serve image and other endpoints as needed
            .service(Files::new("/", ui_path).index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
