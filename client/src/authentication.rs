use std::collections::HashSet;
use bevy::prelude::Resource;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
#[cfg(target_arch = "wasm32")]
use web_sys::wasm_bindgen::prelude::wasm_bindgen;
use rc_shared::config;

// TODO: Fetch from api
static PUBLIC_KEY: &[u8] = config!("PUBLIC_JWT_KEY").as_bytes();

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

#[cfg(not(target_arch = "wasm32"))]
fn error(message: &str) {
    println!("{}", message)
}

#[derive(Resource)]
pub struct GameAuthentication {
    pub account_id: u64,
    pub username: String,
    pub refresh_token: String,
    pub session_token: String,
}

impl GameAuthentication {
    pub fn get() -> GameAuthentication {
        let Some(token) = get_token() else {
            error("No token provided.");
            std::process::exit(1);
        };

        let mut validation = Validation::new(Algorithm::RS512);
        let mut set = HashSet::new();
        set.insert(String::from("refresh"));
        validation.aud = Some(set);

        let key = DecodingKey::from_rsa_pem(PUBLIC_KEY).unwrap();

        let result = jsonwebtoken::decode::<serde_json::Value>(
            &token,
            &key,
            &validation
        );

        let token_data = match result {
            Err(e) => {
                error(&format!("Failed to validate token. {:?}", e));
                std::process::exit(1);
            }
            Ok(v) => v
        };

        GameAuthentication {
            username: token_data.claims.get("username").unwrap().as_str().unwrap().to_string(),
            account_id: token_data.claims.get("sub").unwrap().as_u64().unwrap(),
            refresh_token: token,
            session_token: String::new()
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn get_token() -> Option<String> {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

    local_storage.get_item("token").unwrap()
}

#[cfg(not(target_arch = "wasm32"))]
fn get_token() -> Option<String> {
    std::env::var("TOKEN").ok()
}