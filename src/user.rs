use async_std::task;
use sqlx::{FromRow};
// use bcrypt::{hash};
use serde::{Serialize, Deserialize};
use serde_json::{json};
use actix_web::{http::header::ContentType,web, get, patch, delete, HttpResponse};
use crate::auth;

#[derive(Serialize)]
pub struct UserID {
    pub id: String,
}

#[derive(Serialize,Deserialize, FromRow)]
pub struct Userpassword {
    pub password: String,
}

#[derive(Debug,FromRow, Deserialize, Serialize,PartialEq, Eq)]
pub struct User {
    // pub id: Int4,
    pub lastname: String,
    pub firstname: String,
    pub emailadd: String,
    pub mobileno: String,
    pub username: String,
}

#[derive(Debug,FromRow,Deserialize, Serialize,PartialEq, Eq)]
pub struct Userlist {
    pub id: i32,
    pub lastname: String,
    pub firstname: String,
    pub emailadd: String,
    pub mobileno: String,
    pub username: String,
}

#[derive(Debug,FromRow, Deserialize, Serialize,PartialEq, Eq)]
pub struct Userupdate {
    pub lastname: String,
    pub firstname: String,
    pub mobileno: String,
    // pub created_at: chrono::NaiveDateTime,    
}

// find a User by its id `/users/{id}`
#[get("/users/{id}")]
pub async fn get(params: web::Path<String>) -> HttpResponse {
    let userid_result = task::block_on(auth::db::connect());

    let idno = UserID {
        id: params.to_string(),
    };    

    match userid_result {
        Err(_err) => {
            let msg1 = json!({"statuscode": 500,"message": "Cannot connect to MySQL database"});
            HttpResponse::Created().content_type(ContentType::json()).json(msg1)
        }        

        Ok(pool) => {
            let usr1 = sqlx::query_as::<_, User>("select lastname,firstname,emailadd,mobileno,username from users where id = ?")
            .bind(idno.id.to_string())
            .fetch_one(&pool)
            .await;
            if usr1.is_err() {
                let msg2 = json!({"statuscode": 500,"message": "Username not found..."});
                HttpResponse::Created().content_type(ContentType::json()).json(msg2)    
            } else {
                let usr2 = sqlx::query_as::<_, User>("select * from users where id = ?")
                .bind(idno.id.to_string())        
                .fetch_one(&pool)
                .await.unwrap();
                let msg3 = json!({"statuscode": 201,"lastname": usr2.lastname, "firstname": usr2.firstname, "emailadd": usr2.emailadd, "mobileno": usr2.mobileno, "username": usr2.username});
                HttpResponse::Created().content_type(ContentType::json()).json(msg3)    
            }
        }
    }
}

// list all User by its id `/users`
#[get("/users")]
pub async fn getallusers() -> HttpResponse {
    let userid_result = task::block_on(auth::db::connect());
    match userid_result {
        Err(_err) => {
            let msg1 = json!({"statuscode": 500,"message": "Cannot connect to MySQL database"});
            HttpResponse::Created().content_type(ContentType::json()).json(msg1)
        }        

        Ok(pool) => {
                let rows = sqlx::query_as::<_, Userlist>("SELECT id,lastname,firstname,emailadd,mobileno,username FROM users")
                .fetch_all(&pool)
                .await.unwrap();
                HttpResponse::Created().content_type(ContentType::json()).json(rows)    
        }
    }
}

// find a User by its id `/users/{id}` then update
#[patch("/users/{id}")]
pub async fn update(params: web::Path<String>, data: web::Json<Userupdate>) -> HttpResponse {
    let userid_result = task::block_on(auth::db::connect());
    
    let idno = UserID {
        id: params.to_string(),
    };        

    match userid_result {
        Err(_err) => {
            let msg1 = json!({"statuscode": 500,"message": "Cannot connect to MySQL database"});
            HttpResponse::Created().content_type(ContentType::json()).json(msg1)
        }        

        Ok(pool) => {
            let usr1 = sqlx::query_as::<_, Userupdate>("SELECT * FROM users WHERE id = ?")
            .bind(idno.id.to_string())
            .fetch_one(&pool)
            .await;
            if usr1.is_err() {
                let msg2 = json!({"statuscode": 500,"message": "User ID not found..."});
                HttpResponse::Created().content_type(ContentType::json()).json(msg2)    
            } else {
                // pub created_at: chrono::NaiveDateTime,

                let _ = sqlx::query("UPDATE users SET lastname=?, firstname=?, mobileno=? WHERE id = ?")
                .bind(data.lastname.to_string())
                .bind(data.firstname.to_string())
                .bind(data.mobileno.to_string())    
                .bind(idno.id.to_string())        
                .execute(&pool)
                .await.unwrap();
                let msg3 = json!({"statuscode": 201, "message": "Record(s) successfully updated."});
                HttpResponse::Created().content_type(ContentType::json()).json(msg3)    
            }
        }
    }
}

// find a User by its id `/users/{id}` then update password
#[patch("/users/updatepwd/{id}")]
pub async fn updatepassword(params: web::Path<String>, data: web::Json<Userpassword>) -> HttpResponse {
    let userid_result = task::block_on(auth::db::connect());
    
    let idno = UserID {
        id: params.to_string(),
    };        

    match userid_result {
        Err(_err) => {
            let msg1 = json!({"statuscode": 500,"message": "Cannot connect to MySQL database"});
            HttpResponse::Created().content_type(ContentType::json()).json(msg1)
        }        

        Ok(pool) => {
            let usr1 = sqlx::query_as::<_, Userpassword>("SELECT password FROM users WHERE id = ?")
            .bind(idno.id.to_string())
            .fetch_one(&pool)
            .await;
            if usr1.is_err() {
                let msg2 = json!({"statuscode": 500,"message": "User ID not found..."});
                HttpResponse::Created().content_type(ContentType::json()).json(msg2)    
            } else {
                let _ = sqlx::query("UPDATE users SET password=? WHERE id = ?")
                .bind(data.password.to_string())    
                .bind(idno.id.to_string())        
                .execute(&pool)
                .await.unwrap();
                let msg3 = json!({"statuscode": 201, "message": "Password updated successfully."});
                HttpResponse::Created().content_type(ContentType::json()).json(msg3)    
            }
        }
    }
}

// find a User by its id `/users/{id}` then delete
#[delete("/users/{id}")]
pub async fn delete(params: web::Path<String>) -> HttpResponse {
    let userid_result = task::block_on(auth::db::connect());

    let idno = UserID {
        id: params.to_string(),
    };    

    match userid_result {
        Err(_err) => {
            let msg11 = json!({"statuscode": 500,"message": "Cannot connect to MySQL database"});
            HttpResponse::Created().content_type(ContentType::json()).json(msg11)
        }        

        Ok(pool) => {
            let usr1 = sqlx::query_as::<_, Userupdate>("SELECT * FROM users WHERE id = ?")
            .bind(idno.id.to_string())
            .fetch_one(&pool)
            .await;
            if usr1.is_err() {
                let msg2 = json!({"statuscode": 500,"message": "User ID not found..."});
                HttpResponse::Created().content_type(ContentType::json()).json(msg2)    
            } else {
                let _ = sqlx::query("DELETE FROM users WHERE id = ?")
                .bind(idno.id.to_string())        
                .execute(&pool)
                .await.unwrap();
                let msg3 = json!({"statuscode": 201, "message": "Record(s) successfully deleted."});
                HttpResponse::Created().content_type(ContentType::json()).json(msg3)    
            }
        }
    }
}