use std::time::Duration;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use log::warn;
use crate::error::{AppError, ClientErr};
use crate::jwt::{jwt_sign, jwt_validate};

#[derive(Deserialize)]
pub struct JoinServerBody {
    session_token: String
}

#[derive(Serialize)]
pub struct JoinServerResponse {
    join_token: String
}

pub async fn join_server(
    Json(payload): Json<JoinServerBody>,
) -> Result<(StatusCode, Json<JoinServerResponse>), AppError> {

    let result = jwt_validate(
        payload.session_token,
        "session"
    );

    let claims = match result {
        Ok(v) => v,
        Err(e) => {
            warn!("Failed to verify session token. {:?}", e);
            return Err(ClientErr(format!("Failed to verify token")).into());
        }
    };

    let result = jwt_sign(
        "join_server",
        &claims.username,
        claims.sub,
        Duration::from_secs_f32(15.0)
    );

    let response = JoinServerResponse {
        join_token: result
    };

    Ok((StatusCode::OK, Json(response)))
}