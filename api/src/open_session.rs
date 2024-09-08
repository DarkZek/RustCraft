use std::time::Duration;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use log::warn;
use crate::error::{AppError, ClientErr};
use crate::jwt::{jwt_sign, jwt_validate};

#[derive(Deserialize)]
pub struct OpenSessionBody {
    refresh_token: String
}

#[derive(Serialize)]
pub struct OpenSessionResponse {
    session_token: String
}

pub async fn open_session(
    Json(payload): Json<OpenSessionBody>,
) -> Result<(StatusCode, Json<OpenSessionResponse>), AppError> {

    let result = jwt_validate(
        payload.refresh_token,
        "refresh"
    );

    let claims = match result {
        Ok(v) => v,
        Err(e) => {
            warn!("Failed to verify refresh token. {:?}", e);
            return Err(ClientErr(format!("Failed to verify token")).into());
        }
    };

    let result = jwt_sign(
        "session",
        &claims.username,
        claims.sub,
        Duration::from_secs_f32(60.0)
    );

    let response = OpenSessionResponse {
        session_token: result
    };

    Ok((StatusCode::OK, Json(response)))
}