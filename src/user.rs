use async_std::task;
use sqlx::{FromRow};
use serde::{Serialize, Deserialize};
use serde_json::{json};
use actix_web::{http::header::ContentType,web, get, patch, delete,
    HttpResponse, HttpRequest, http::header::CONTENT_LENGTH};
use crate::auth;
use tokio::fs;
use tokio::io::AsyncWriteExt as _;
use actix_multipart::Multipart;
use futures_util::{ TryStreamExt as _ };
use mime::{ Mime, IMAGE_PNG, IMAGE_JPEG, IMAGE_GIF };
use uuid::Uuid;
use image::{ DynamicImage, imageops::FilterType };

#[derive(Serialize)]
pub struct UserID {
    pub id: String,
}

#[derive(Serialize,Deserialize, FromRow)]
pub struct Userpassword {
    pub password: String,
}

#[derive(Serialize,Deserialize, FromRow)]
pub struct Userprofilepic {
    pub profilepic: String,
}

#[derive(Debug,FromRow, Deserialize, Serialize,PartialEq, Eq)]
pub struct User {
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
}

// find a User by its id `/users/{id}`
#[get("/users/{id}")]
pub async fn get(params: web::Path<String>,req: HttpRequest) -> HttpResponse {

    if auth::jwt::authorizeuser(req) {
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
        
    } else {
        let msg = json!({"statuscode": 401,"message": "UnAuthorize Access.."});
        HttpResponse::Created().content_type(ContentType::json()).json(msg)
    }
}

// list all User by its id `/users`
#[get("/users")]
pub async fn getallusers(_req: HttpRequest) -> HttpResponse {

    if auth::jwt::authorizeuser(_req) {
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

    } else {
        let msg1x = json!({"statuscode": 401,"message": "UnAuthorized Access..."});
        return HttpResponse::Created().content_type(ContentType::json()).json(msg1x)    
    }
}

// find a User by its id `/users/{id}` then update
#[patch("/users/{id}")]
pub async fn update(params: web::Path<String>, data: web::Json<Userupdate>, req: HttpRequest) -> HttpResponse {
    if auth::jwt::authorizeuser(req) {
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
    } else {
        let msg1x = json!({"statuscode": 401,"message": "UnAuthorized Access..."});
        return HttpResponse::Created().content_type(ContentType::json()).json(msg1x)    
    }
}


// find a User by its id `/users/{id}` then update password
#[patch("/users/updatepwd/{id}")]
pub async fn updatepassword(params: web::Path<String>, data: web::Json<Userpassword>, req: HttpRequest) -> HttpResponse {
    if auth::jwt::authorizeuser(req) {
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
    } else {
        let msg1x = json!({"statuscode": 401,"message": "UnAuthorized Access..."});
        return HttpResponse::Created().content_type(ContentType::json()).json(msg1x)    
    }
}

// find a User by its id `/users/{id}` then delete
#[delete("/users/{id}")]
pub async fn delete(params: web::Path<String>, req: HttpRequest) -> HttpResponse {
    if auth::jwt::authorizeuser(req) {
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
    } else {
        let msg1x = json!({"statuscode": 401,"message": "UnAuthorized Access..."});
        return HttpResponse::Created().content_type(ContentType::json()).json(msg1x)    
    }
}

#[patch("/users/updateuserpic/{id}")]
pub async fn updateusepic(mut payload: Multipart,params: web::Path<String>, request: HttpRequest) -> HttpResponse {
    let xid: String = params.to_string();
    let newid = xid.parse::<i32>().unwrap();
    let content_length: usize = match request.headers().get(CONTENT_LENGTH) {
        Some(header_value) => header_value.to_str().unwrap_or("0").parse().unwrap(),
        None => "0".parse().unwrap(),
    };
    let max_file_count: usize = 3;
    let max_file_size: usize = 900_000;
    let legal_filetypes: [Mime; 3] = [IMAGE_PNG, IMAGE_JPEG, IMAGE_GIF];
    let mut current_count: usize = 0;
    let dir: &str = "./client/public/users/";

    if content_length > max_file_size { 
        let msg1a = json!({"statuscode": 500, "message": "Bad Request...."});
        return HttpResponse::Created().content_type(ContentType::json()).json(msg1a)    
    }
    loop {
        if current_count == max_file_count { break; }
        if let Ok(Some(mut field)) = payload.try_next().await {
            let filetype: Option<&Mime> = field.content_type();
            if filetype.is_none() { continue; }
            if !legal_filetypes.contains(&filetype.unwrap()) { continue; }

            let destination: String = format!(
                "{}{}-{}",
                dir,
                Uuid::new_v4(),
                field.content_disposition().get_filename().unwrap()
            );

            let mut saved_file: fs::File = fs::File::create(&destination).await.unwrap();
            while let Ok(Some(chunk)) = field.try_next().await {
                let _ = saved_file.write_all(&chunk).await.unwrap();
            }
            web::block(move || async move {
                let uploaded_img: DynamicImage = image::open(&destination).unwrap();
                let _ = fs::remove_file(&destination).await.unwrap();
                uploaded_img
                    .resize_exact(200, 200, FilterType::Gaussian)
                    .save(format!("{}{}.jpg", dir,newid)).unwrap();
            }).await.unwrap().await;
            
                // SAVE NEW FILENAME TO DATABASE
                let newfilename = format!("{}.jpg", newid);
                let userid_result = task::block_on(auth::db::connect());
                match userid_result {
                    Err(_err) => {
                        let msg1b = json!({"statuscode": 500,"message": "Unable to update profile picture.."});
                        return HttpResponse::Created().content_type(ContentType::json()).json(msg1b)                            
                    }

                    Ok(pool) => {

                        let _ = sqlx::query("UPDATE users SET profilepic=? WHERE id = ?")
                            .bind(newfilename)
                            .bind(newid.to_string())
                            .execute(&pool)
                            .await.unwrap();            
                    }
                }
                
        } else { break; }
        current_count += 1;
    }
    let msg1x = json!({"statuscode": 201,"message": "Profile picture has been changed.."});
    return HttpResponse::Created().content_type(ContentType::json()).json(msg1x)                                                

}
