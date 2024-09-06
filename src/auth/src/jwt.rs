// Author : Reynald Marquez-Gragasin
// Email  : reynald89@icloud.com
// Module : Json Web Token Authorization with Token Expiration

use actix_web::{HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::collections::HashSet;
use crate::Claims;

pub fn authorizeuser(_req: HttpRequest) -> bool {
    let jwt_secret = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set.");
    let req_headers = _req.headers();
    if req_headers.get("Authorization").is_none() {
        return false;
    } else {

        let basic_auth_header = req_headers.get("Authorization");
        let basic_auth: Vec<String> = basic_auth_header.unwrap().to_str().unwrap().split("Bearer").map(|s| s.to_string()).collect();
        let token = basic_auth[1].trim();    
        let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
        // skip exp validation, which is on by default
        validation.required_spec_claims = HashSet::new();        
        // skip aud validation
        validation.validate_aud = false;        
        // decode token
        match decode::<Claims>(&token, &DecodingKey::from_secret(jwt_secret.as_ref()), &Validation::default()) {
            Ok(_res) => {
                
            },
            Err(_error) => {
                return false;
            }
        };


        return true;
    }
}