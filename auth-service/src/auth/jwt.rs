use jsonwebtoken::*;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

pub fn create(user_id: &str, role: &str, secret: &str) -> String {
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH).unwrap().as_secs() + 3600;

    encode(
        &Header::default(),
        &Claims {
            sub: user_id.into(),
            role: role.into(),
            exp: exp as usize,
        },
        &EncodingKey::from_secret(secret.as_ref()),
    ).unwrap()
}

pub fn decode(token: &str, secret: &str) -> Result<Claims, Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ).map(|d| d.claims)
}