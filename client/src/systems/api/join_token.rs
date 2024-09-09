use anyhow::Error;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedReceiver;
use crate::systems::api::{ApiError, ApiSystem};


#[derive(Serialize, Deserialize, Debug)]
pub struct GetJoinTokenBody {
    pub session_token: String
}

#[derive(Deserialize, Debug)]
pub struct GetJoinTokenResponse {
    pub join_token: String
}

impl ApiSystem {
    pub fn get_join_token(&self, session_token: String) -> UnboundedReceiver<Result<GetJoinTokenResponse, ApiError>> {
        self.do_request::<GetJoinTokenBody, GetJoinTokenResponse>(
            "/join",
            GetJoinTokenBody {
                session_token
            }
        )
    }
}