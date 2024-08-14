// use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Serialize, Deserialize};
use actix_web::{Result};
use chrono::{ Utc, Duration};
// use crate::web::JsonBody::Error;
// use std::fmt::Error;
// use actix_web::web::JsonBody::Error;
use chrono::format::Item::Error;


const JWT_SECRET: &[u8] = b"secret";

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
}

pub fn create_jwt(uid: &str, email: &str) -> Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: uid.to_owned(),
        email: email.to_string(),
        exp: expiration as usize,
    };

    // const JWT_SECRET =std::env::var("SECRET_KEY").is_ok();

    // println!("{}",JWT_SECRET.to_string());

    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| Error::JWTTokenCreationError)
}