use async_std::task;
use sqlx::{FromRow};
use bcrypt::{hash, verify};
use serde::{Serialize, Deserialize};
use serde_json::{json};
use actix_web::{http::header::ContentType,web, post, HttpResponse};
pub mod db;
pub mod jwt;

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use chrono::{ Utc, Duration};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub email: String,
    pub exp: usize,
}

#[derive(Debug,FromRow, Deserialize, Serialize,PartialEq, Eq)]
pub struct Userinfo {
    pub id: i32,
    pub lastname: String,
    pub firstname: String,
    pub emailadd: String,
    pub mobileno: String,
    pub username: String,
    pub profilepic: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Userdata {
    pub username: String,
    pub password: String
}

#[derive(Debug,FromRow, Deserialize, Serialize,PartialEq, Eq)]
pub struct Users {
    pub lastname: String,
    pub firstname: String,
    pub emailadd: String,
    pub mobileno: String,
    pub username: String,
    pub password: String
}

#[derive(Debug,FromRow, Deserialize, Serialize,PartialEq, Eq)]
pub struct Usersignin {
    pub username: String,
    pub password: String
}

// create a user registration
#[post("/auth/register")]
pub async fn register(user_req: web::Json<Users>) -> HttpResponse {
    let hashed_password = hash(user_req.password.to_string(), 10).unwrap();
    let result = task::block_on(db::connect());
    match result {
        Err(err) => {
            let err1 = json!({"statuscode": 201,"message": "May error..."});
            println!("Cannot connect to MySQL database [{}]", err.to_string());
            HttpResponse::Created().content_type(ContentType::json()).json(err1)
        }

        Ok(pool) => {

            println!("{}","Connected to MySQL database successfully.");
            let query_email_result = sqlx::query_as::<_, Users>("SELECT * FROM users WHERE emailadd = ?")
            .bind(user_req.emailadd.to_string())
            .fetch_all(&pool).await.unwrap();
            if query_email_result.len() == 1 {
                let err2 = json!({"statuscode": 500, "message": "Email Address is already taken."});
                HttpResponse::Created().content_type(ContentType::json()).json(err2)
            } else {

                let query_username_result = sqlx::query_as::<_, Users>("select lastname,firstname,emailadd,mobileno,username,password from users where username = ?")
                .bind(user_req.username.to_string())
                .fetch_all(&pool).await.unwrap();

                if query_username_result.len() == 1 {
                    let err3 = json!({"statuscode": 500, "message": "Username is already taken."});
                    HttpResponse::Created().content_type(ContentType::json()).json(err3)
                } else {

                    let query_lastname_firstname_result = sqlx::query_as::<_, Users>("select lastname,firstname,emailadd,mobileno,username,password from users where lastname = ? AND firstname = ?")
                    .bind(user_req.lastname.to_string()).bind(user_req.firstname.to_string())
                    .fetch_all(&pool).await.unwrap();
                    if query_lastname_firstname_result.len() == 1 {
                        let err4 = json!({"statuscode": 201, "message": "Lastname and Firstname is already taken."});
                        return HttpResponse::Created().content_type(ContentType::json()).json(err4)
                    } else {

                        let _ = sqlx::query(
                            "INSERT INTO users (
                                lastname,
                                firstname,
                                emailadd,
                                mobileno,
                                username,
                                password)
                            values (?, ?, ?, ?, ?, ?)")
                            .bind(&user_req.lastname)
                            .bind(&user_req.firstname)
                            .bind(&user_req.emailadd)
                            .bind(&user_req.mobileno)
                            .bind(&user_req.username)
                            .bind(&hashed_password)
                            .execute(&pool).await;
                        let msg = json!({"statuscode": 201, "message": "Registered successfully."});
                        HttpResponse::Created().content_type(ContentType::json()).json(msg)
                    }
                }
            }
        }
    }
}

// create a user login
#[post("/auth/login")]
pub async fn userlogin(user_req: web::Json<Usersignin>) -> HttpResponse {
    let loginresult = task::block_on(db::connect());
    match loginresult {
        Err(err) => {
            let msg1 = json!({"statuscode": 500,"message": "Cannot connect to MySQL database"});
            println!("Cannot connect to MySQL database [{}]", err.to_string());
            HttpResponse::Created().content_type(ContentType::json()).json(msg1)
        }

        Ok(pool) => {

            let usr1 = sqlx::query_as::<_, Usersignin>("select username,password from users where username = ?")
            .bind(user_req.username.to_string()).fetch_one(&pool).await;
            if usr1.is_err() {
                let msg2 = json!({"statuscode": 500,"message": "Username not found..."});
                HttpResponse::Created().content_type(ContentType::json()).json(msg2)
            } else {

                let usr2 = sqlx::query_as::<_, Usersignin>("select username,password from users where username = ?")
                .bind(user_req.username.to_string())
                .fetch_one(&pool)
                .await.unwrap();

                let hashed_password = usr2.password;

                let is_password_correct = verify(user_req.password.to_string(), &hashed_password).unwrap();
                if is_password_correct {

                    let usrinfo = sqlx::query_as::<_, Userinfo>("select id,lastname,firstname,emailadd,mobileno,username,profilepic from users where username = ?")
                    .bind(user_req.username.to_string())
                    .fetch_one(&pool)
                    .await.unwrap();

                    // GENERATE TOKEN 
                    let jwt_secret = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set.");
                    let exp: usize = (Utc::now() + Duration::hours(8)).timestamp() as usize;
                    let email: String = usrinfo.emailadd.to_string();
                    let claims: Claims = Claims{email,exp };
                    let header = Header::new(Algorithm::HS256);
                    let token: String = encode(
                        &header,
                        &claims,
                        &EncodingKey::from_secret(jwt_secret.as_str().as_ref())
                    ).unwrap();

                    let msg3 = json!({"statuscode": 201,"message": "Success...","token": token, "user": usrinfo});
                    HttpResponse::Created().content_type(ContentType::json()).json(msg3)
                } else {
                    let msg3 = json!({"statuscode": 500,"message": "Incorrect Password, try again..."});
                    HttpResponse::Created().content_type(ContentType::json()).json(msg3)
                }


            }
        }
    }
}
