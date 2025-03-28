use actix_files::NamedFile;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputModel {
    target: f64,
    time_response: f64,
    pid: PIDParams,
    drone_model: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PIDParams {
    kp: f64,
    ki: f64,
    kd: f64,
}

// Endpoint to update simulation settings.
pub async fn update_input(input: web::Json<InputModel>) -> impl Responder {
    println!("Received input: {:?}", input);
    // logic here
    HttpResponse::Ok().json(input.into_inner())
}

// Endpoint to serve the generated image.
pub async fn get_image() -> actix_web::Result<NamedFile> {
    NamedFile::open("pid_response.png").map_err(actix_web::error::ErrorInternalServerError)
}
