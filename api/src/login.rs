use std::time::Duration;
use axum::http::StatusCode;
use axum::Json;
use log::warn;
use regex::Regex;
use serde::{Deserialize, Serialize};
use crate::error::{AppError, ClientErr};
use crate::jwt::jwt_sign;

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String
}
#[derive(Serialize)]
pub struct LoginResponse {
    refresh_token: String
}

pub async fn login(
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<LoginResponse>), AppError> {

    let regex = Regex::new(r"(?m)^[a-zA-Z0-9_.-]{3,16}$").unwrap();
    if !regex.is_match(&payload.username) {
        return Err(ClientErr(format!("Username '{}' failed requirements", &payload.username)).into())
    }

    let result = jwt_sign(
        "refresh",
        &payload.username,
        rand::random::<u64>(),
        Duration::from_days(300)
    );

    let response = LoginResponse {
        refresh_token: result
    };

    Ok((StatusCode::OK, Json(response)))
}