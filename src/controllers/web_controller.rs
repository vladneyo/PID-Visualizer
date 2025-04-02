use crate::controllers::pid_controller::PIDController;
use crate::logic::drone_models::DroneModels::CetusPro;
use crate::logic::input::Input;
use crate::logic::physics::Physics;
use crate::logic::pid::PID;
use actix_files::NamedFile;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::fs;
use timetrap::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputModel {
    pub time_window: f64,
    pub target: f64,
    pub time_response: f64,
    pub pid: PIDParams,
    pub effective_damping: f64,
    pub drone_model: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PIDParams {
    kp: f64,
    ki: f64,
    kd: f64,
}

pub async fn update_input(input: web::Json<InputModel>) -> impl Responder {
    trap_mem!("update_input", MemUnits::Kb, {
        let target = Input::set(input.target, input.time_response);
        let pid = PID::new(input.pid.kp, input.pid.ki, input.pid.kd);
        let phx = Physics::cetus_pro(input.time_window, input.effective_damping);

        match PIDController::update(target, pid, phx) {
            Some(_result) => {
                // Persist the input into a JSON file.
                let file_path = "./user_defaults.json";
                trap!("write to user_defaults.json", {
                    match serde_json::to_string(&input.into_inner()) {
                        Ok(json_str) => {
                            if let Err(e) = fs::write(file_path, json_str) {
                                println!("Error writing defaults to file: {}", e);
                            }
                        }
                        Err(e) => println!("Error serializing input: {}", e),
                    }
                });

                HttpResponse::Ok().json("Success")
            }
            None => HttpResponse::InternalServerError().json("Something went wrong"),
        }
    })
}

pub async fn get_image() -> actix_web::Result<NamedFile> {
    NamedFile::open("./pid_response.png").map_err(actix_web::error::ErrorInternalServerError)
}

pub async fn get_defaults() -> impl Responder {
    trap!("get_defaults", {
        println!("{:?}", "get_defaults called");
        let file_path = "./user_defaults.json";
        if let Ok(data) = fs::read_to_string(file_path) {
            if let Ok(default) = serde_json::from_str::<InputModel>(&data) {
                println!("default {:?}", default);
                return HttpResponse::Ok().json(default);
            } else {
                println!("Error deserializing defaults from file.");
            }
        }

        let d_input = Input::default();
        let d_pid = PID::default();
        let d_drone = CetusPro;
        let default: InputModel = InputModel {
            time_window: Physics::default().sim_time,
            target: d_input.target_value,
            time_response: d_input.acceptable_time,
            pid: PIDParams {
                kp: d_pid.kp,
                ki: d_pid.ki,
                kd: d_pid.kd,
            },
            effective_damping: 0.0004,
            drone_model: d_drone.to_string(),
        };
        HttpResponse::Ok().json(default)
    })
}
