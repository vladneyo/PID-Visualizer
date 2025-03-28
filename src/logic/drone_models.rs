use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Serialize, Deserialize, Display)]
pub enum DroneModels {
    #[strum(serialize = "cetus_pro")]
    CetusPro,
}
