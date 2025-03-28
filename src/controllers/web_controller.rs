use crate::controllers::pid_controller::PIDController;
use crate::logic::drone_models::DroneModels::CetusPro;
use crate::logic::input::Input;
use crate::logic::physics::Physics;
use crate::logic::pid::PID;
use actix_files::NamedFile;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputModel {
    pub target: f64,
    pub time_response: f64,
    pub pid: PIDParams,
    pub drone_model: String,
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
    let target = Input::set(input.target, input.time_response);
    let pid = PID::new(input.pid.kp, input.pid.ki, input.pid.kd);
    let phx = Physics::cetus_pro(3.0, 0.0007);

    // logic here
    match PIDController::update(target, pid, phx) {
        Some(result) => HttpResponse::Ok().json(input.into_inner()),
        None => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

// Endpoint to serve the generated image.
pub async fn get_image() -> actix_web::Result<NamedFile> {
    NamedFile::open("./pid_response.png").map_err(actix_web::error::ErrorInternalServerError)
}

pub async fn get_defaults() -> impl Responder {
    let d_input = Input::default();
    let d_pid = PID::default();
    let d_drone = CetusPro;
    let default: InputModel = InputModel {
        target: d_input.target_value,
        time_response: d_input.acceptable_time,
        pid: PIDParams {
            kp: d_pid.kp,
            ki: d_pid.ki,
            kd: d_pid.kd,
        },
        drone_model: d_drone.to_string(),
    };
    HttpResponse::Ok().json(default)
}
