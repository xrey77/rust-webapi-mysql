use bcrypt::{hash};
use serde::{Serialize, Deserialize};
use serde_json::{json};
use actix_web::{http::header::ContentType,web, post, get, patch, delete, HttpResponse,Responder, Result};

#[derive(Debug, Deserialize, Serialize,PartialEq, Eq)]
pub struct Userlogin {
    pub username: String,
    pub password: String
}

// create a userlogin `/auth/login`
#[post("/users")]
pub async fn userlogin(user_req: web::Json<Userlogin>) -> HttpResponse {
    HttpResponse::Created()
        .content_type(ContentType::json())
        .json(user_req)
}
