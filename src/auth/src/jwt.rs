// use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use chrono::{ Utc, Duration};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Serialize, Deserialize};
// use actix_web::{web};
// use crate::web::JsonBody::Error;
// use std::fmt::Error;
// use actix_web::web::JsonBody::Error;
// use chrono::format::Item::Error;


const JWT_SECRET: &[u8] = b"secret";

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub email: String,
    pub exp: usize,
}

pub async fn generate_token( email: String) -> String {
    let exp: usize = (Utc::now() + Duration::hours(8)).timestamp() as usize;
    // let email: String = usrinfo.emailadd.to_string();
    // let claims: Claims = Claims{email,exp };
    let claims: Claims = Claims{email,exp };
    let header = Header::new(Algorithm::HS512);

    let token: String = encode(
        &header,
        &claims,
        &EncodingKey::from_secret(JWT_SECRET)
    ).unwrap();
    token
}

