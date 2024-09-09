use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedReceiver;
use crate::systems::api::{ApiError, ApiSystem};


#[derive(Serialize, Deserialize, Debug)]
pub struct OpenSessionBody {
    pub refresh_token: String
}

#[derive(Deserialize, Debug)]
pub struct OpenSessionResponse {
    pub session_token: String
}

impl ApiSystem {
    pub fn open_session(&self, refresh_token: String) -> UnboundedReceiver<Result<OpenSessionResponse, ApiError>> {
        self.do_request::<OpenSessionBody, OpenSessionResponse>(
            "/session",
            OpenSessionBody {
                refresh_token
            }
        )
    }
}