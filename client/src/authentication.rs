use std::collections::HashSet;
use bevy::prelude::Resource;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};

// TODO: Fetch from api
static PUBLIC_KEY: &[u8] = include_bytes!("../../jwt.public.pem");

#[derive(Resource)]
pub struct GameAuthentication {
    account_id: u64,
    username: String
}

impl GameAuthentication {
    pub fn get() -> GameAuthentication {
        let Some(token) = get_token() else {
            println!("No token provided.");
            std::process::exit(1);
        };

        let mut validation = Validation::new(Algorithm::RS512);
        let mut set = HashSet::new();
        set.insert(String::from("session"));
        validation.aud = Some(set);

        let key = DecodingKey::from_rsa_pem(PUBLIC_KEY).unwrap();

        let result = jsonwebtoken::decode::<serde_json::Value>(
            &token,
            &key,
            &validation
        );

        let token_data = match result {
            Err(e) => {
                println!("Failed to validate token. {:?}", e);
                std::process::exit(1);
            }
            Ok(v) => v
        };

        GameAuthentication {
            username: token_data.claims.get("username").unwrap().as_str().unwrap().to_string(),
            account_id: token_data.claims.get("sub").unwrap().as_u64().unwrap(),
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn get_token() -> Option<String> {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

    let value = local_storage.get_item("token").unwrap();

    value
}

#[cfg(not(target_arch = "wasm32"))]
fn get_token() -> Option<String> {
    std::env::var("TOKEN").ok()
}