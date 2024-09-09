use bevy::log::info;
use bevy::prelude::{Event, EventReader, NextState, Res, ResMut};
use bevy::tasks::{IoTaskPool, TaskPool};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use tokio::runtime::Builder;
use tokio::sync::mpsc::unbounded_channel;
use rc_networking::client::NetworkingClient;
use crate::state::AppState;
use crate::systems::networking::NetworkingSystem;

#[derive(Event)]
pub struct ConnectToServerIntent {
    pub(crate) address: Url
}

pub struct PendingServerConnection {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetJoinTokenBody {
    session_token: String
}

#[derive(Deserialize, Debug)]
pub struct GetJoinTokenResponse {
    join_token: String
}

/// Connects to the local server instance
pub fn accept_server_connection_intent(
    mut intent: EventReader<ConnectToServerIntent>,
    mut client: ResMut<NetworkingClient>,
    networking_system: Res<NetworkingSystem>,
    mut app_state: ResMut<NextState<AppState>>,
) {

    let entry = intent.read().next();

    let Some(intent) = entry else {
        return;
    };

    info!("Connecting to server on {}", &intent.address);

    app_state.set(AppState::Connecting);

    // Fetch token
    let pool = IoTaskPool::get();
    // let (send, recv) = unbounded_channel();

    pool.spawn(async {

        let runtime = Builder::new_current_thread().build().unwrap();

        runtime.spawn(async {
            let body = GetJoinTokenBody {
                session_token: "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzUxMiJ9.eyJhdWQiOiJzZXNzaW9uIiwic3ViIjoxMzM2MDE2NDEyODkzMDIzNzUyMSwidXNlcm5hbWUiOiJTaW1wbHlKcGsiLCJleHAiOjE3MjU4NjE3Mzd9.3QvsDX9cD3MivCHAedQgVKDhlvLTdIjaoT3mMGhNJtPN7bOyHZaOvzM5HjPCXX1jgmSg9uNpmPtWoH1s2ngsH80p-a0-IS1dlr-PIpdLmvhjX3a0Ng0KG4mbRrwNBmCg1LnffztSb09TXS7Sprv9vpWUwGNQFLBkC21ISbDAxJRfrn0VJYGVIv2pdT9tjan5OlLYdU3pmI0z4hVbORU0VYHBoAjCoJedsg7Ymovy-V0wvRG-ZAoq87qjvuJWteFg6NwcoPYBfO8kact225j-o19YoUd5xLL0Y1XOjWvU9lhrNghX-L1kBs6h0RdcW9X00DeCljgscvWvqfMdP-N7qZg5bohyBi7LWTY367rVWTSKoYyn4uH_Q6C8oIZZde2fgIaR9p9oiwJjYdY8echr5xz8U2mVNF1vUEl3--m7ZGLQY7sLZNEerYO93tk1kW-GDnFt3AobA5kHeYL2IP4UnFbgegNsVewFeh98kcvect3ZocYQHmBsL23gkMraGcMY".to_string()
            };

            let res = reqwest::Client::new()
                .get(format!("{}/join", env!("API_URL")))
                .json(&body)
                .send()
                .await
                .unwrap();

            let json = res.json::<GetJoinTokenResponse>().await.unwrap();

            info!("join token {}", json.join_token);
        });
    }).detach();

    // TODO: Fetch from api
    let join_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzUxMiJ9.eyJhdWQiOiJqb2luX3NlcnZlciIsInN1YiI6MTMzNjAxNjQxMjg5MzAyMzc1MjEsInVzZXJuYW1lIjoiU2ltcGx5SnBrIiwiZXhwIjoxNzI3MzYxNjg4fQ.bwc5WDcivILWi-CIDa7wLEBPK0N5KeLNdu0KgJk4dAoITvB45GFP4iXW3J76JRSemoCY_MV0g7o0g14JC_HiTEZ5O6oDJYL617wQUBP4sWViV10gTBSpWLUUt3RX8YiH4CVI__sKPAB7W5Ck45zjV-J4hnxh-gup11S-bQMDb7n2qe2C5GOjUeeOkUZ5JKENGsdBqMh4K55jToUK8cClJzPmcnNWvm7HDNZNPvl3bDFEpguUede0IXPCnj8x3lSpS0J8Jg_8O8Ay0XaaAdCKI8bW0p-yn-BsPugPIE5F5kNFSx14ikJVez95sW0c_YhFnTLLQiI0A0_skYoiVMtjV1V5stZq4t14JqU4sdeVx2NuT-i6CYBukSdhXl-Y5eFbKm2sOiXlCmyof1mgP_c5oghPrPj7iZ1HC2SuIoCk8rfY7gGkGkTZt4bA1blPs7fsBeaUnLKNOScFv2OkfUdNXeuopyAkqGHHr3cul672vzK1uv_K7oIGFPQZsglgY1hV";

    client.connect(intent.address.clone(), join_token.to_string());
}

pub fn connect_to_server() {

}