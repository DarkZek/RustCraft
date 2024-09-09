use std::collections::HashSet;
use bevy::prelude::warn;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

static PUBLIC_KEY: &[u8] = include_bytes!("../../../../jwt.public.pem");

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationResult {
    pub aud: String,
    pub sub: u64,
    pub username: String,
    pub exp: u64,
}

pub fn check_authorization(token: &str) -> Option<AuthorizationResult> {
    let mut validation = Validation::new(Algorithm::RS512);
    let mut set = HashSet::new();
    set.insert(String::from("join_server"));
    validation.aud = Some(set);

    let key = DecodingKey::from_rsa_pem(PUBLIC_KEY).unwrap();

    let result = jsonwebtoken::decode::<AuthorizationResult>(
        &token,
        &key,
        &validation
    );

    match result {
        Ok(v) => Some(v.claims),
        Err(e) => {
            warn!("Token validated failed. {:?}", e);
            None
        }
    }
}