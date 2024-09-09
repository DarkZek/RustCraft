pub mod join_token;
pub mod open_session;

use std::fmt::Debug;
use bevy::app::App;
use bevy::log::info;
use bevy::prelude::{Plugin, Resource, trace, warn};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use tokio::runtime::{Builder, Runtime};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Non 200 status code returned")]
    Error(Value),
    #[error("HTTP error")]
    Reqwest(#[from] reqwest::Error)
}

pub struct ApiPlugin;

impl Plugin for ApiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ApiSystem::new());
    }
}

#[derive(Resource)]
pub struct ApiSystem {
    #[cfg(not(target_arch = "wasm32"))]
    runtime: Runtime
}

impl ApiSystem {
    pub fn new() -> ApiSystem {
        ApiSystem {
            #[cfg(not(target_arch = "wasm32"))]
            runtime: Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
        }
    }

    fn do_request<
        X: Serialize + Sized + Send + 'static,
        T: for<'a> Deserialize<'a> + Debug + Send + 'static
    >(
        &self,
        path: &str,
        body: X
    ) -> UnboundedReceiver<Result<T, ApiError>> {

        #[cfg(not(target_arch = "wasm32"))]
        let runtime = &self.runtime;

        #[cfg(target_arch = "wasm32")]
        let runtime = IoTaskPool::get();

        let (send, recv) = unbounded_channel();

        let url = format!("{}{}", env!("API_URL"), path);

        runtime.spawn(async move {
            info!("Starting HTTP request to {}", url);

            let body2 = body;

            let res = match reqwest::Client::new()
                .post(url)
                .json(&body2)
                .send()
                .await {
                Ok(v) => v,
                Err(e) => {
                    warn!("Failed to complete HTTP request");
                    send.send(Err(e.into())).unwrap();
                    return;
                }
            };

            if res.status() != 200 {
                let json = res.json::<Value>().await;
                match json {
                    Ok(v) => send.send(Err(ApiError::Error(v))).unwrap(),
                    Err(e) => {
                        warn!("Failed to get serde json response {:?}", e);
                        send.send(Err(e.into())).unwrap();
                    }
                }
                return
            }

            let json = match res.json::<T>().await {
                Ok(v) => v,
                Err(e) => {
                    warn!("Failed to parse HTTP JSON response {:?}", e);
                    send.send(Err(e.into())).unwrap();
                    return;
                }
            };

            trace!("Received HTTP response {:?}", json);

            send.send(Ok(json)).unwrap();
        });

        recv
    }
}