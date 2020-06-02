use serde_derive::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct HealthResponse {
    pub message: String,
}
