// use mysql::*;
// use mysql::prelude::*;
// use bcrypt::{hash};
use serde::{Serialize, Deserialize};
// use serde_json::{Value, Map};
// use std::collections::HashMap;
// use std::hash::Hash;
use serde_json::{json};

use actix_web::{http::header::ContentType,web, get, patch, delete, HttpResponse,Responder, Result};
// use uuid::Uuid;


#[derive(Serialize)]
pub struct UserID {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize,PartialEq, Eq)]
pub struct User {
    pub lastname: String,
    pub firstname: String,
    pub emailadd: String,
    pub mobileno: String,
    pub username: String,
    pub password: String
    // pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}




// find a User by its id `/users/{id}`
#[get("/users/{id}")]
pub async fn get(params: web::Path<String>) -> Result<impl Responder> {
    let idno = UserID {
        id: params.to_string(),
    };    
    println!("ID {}", idno.id);
    Ok(web::Json(idno))
}


// find a User by its id `/users/{id}` then update
#[patch("/users/{id}")]
pub async fn update(params: web::Path<String>, data: web::Json<User>) -> Result<impl Responder> {
    let idno = UserID {
        id: params.to_string(),
    };    
    println!("ID {}", idno.id);

    println!("Lastname{}", data.lastname);
    println!("Firstname{}", data.firstname);
    
    let usrdata = json!({
        "statuscode": 200,
        "message": "Successfull..",
        "id": idno.id,
        "user": {"lastname": data.lastname, "firstname": data.firstname}
    });

    Ok(web::Json(usrdata))
}

// find a User by its id `/users/{id}` then delete
#[delete("/users/{id}")]
pub async fn delete(params: web::Path<String>) -> HttpResponse {
    let idno = UserID {
        id: params.to_string(),
    };    
    println!("ID {}", idno.id);

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(idno)

}


// list 50 last tweets `/tweets`
// #[get("/users")]
// pub async fn list() -> HttpResponse {
//     // TODO find the last 50 tweets and return them

//     let users = Users { results: vec![] };

//     HttpResponse::Ok()
//         .content_type(APPLICATION_JSON)
//         .json(users)
// }

