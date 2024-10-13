use std::collections::HashSet;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use jsonwebtoken::errors::Error;
use serde::{Deserialize, Serialize};
use crate::{PRIVATE_KEY, PUBLIC_KEY};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,
    pub sub: u64,
    pub username: String,
    pub exp: u64,
}

pub fn jwt_sign(audience: &str, username: &str, subject: u64, validity: Duration) -> String {
    let now = SystemTime::now();

    let expires = now + validity;

    let header = Header::new(Algorithm::RS512);
    let claims = Claims {
        aud: audience.to_string(),
        sub: subject,
        username: username.to_string(),
        exp: expires.duration_since(UNIX_EPOCH).unwrap().as_secs(),
    };

    let key = EncodingKey::from_rsa_pem(PRIVATE_KEY.get().unwrap()).unwrap();

    jsonwebtoken::encode(
        &header,
        &claims,
        &key
    ).unwrap()
}

pub fn jwt_validate(token: String, audience: &str) -> Result<Claims, Error> {
    let mut validation = Validation::new(Algorithm::RS512);
    let mut set = HashSet::new();
    set.insert(String::from(audience));
    validation.aud = Some(set);

    let key = DecodingKey::from_rsa_pem(PUBLIC_KEY.get().unwrap()).unwrap();

    let result = jsonwebtoken::decode::<Claims>(
        &token,
        &key,
        &validation
    );

    result.map(|v| v.claims)
}