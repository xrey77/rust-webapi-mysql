use actix_files::Files;
use actix_web::{App, HttpServer, http::header};
mod user;
use auth;
use dotenv::dotenv;
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
                    .allowed_headers(vec![
                        header::CONTENT_TYPE,
                        header::AUTHORIZATION,
                        header::ACCEPT,
                    ])
                    .supports_credentials()
                    .max_age(3600)
            )
            // .route("/auth/register",web::post().to(auth::register))
            // .route("/auth/login",web::post().to(auth::userlogin))
            // .route("/user/{id}",web::get().to(user::get))
            // .route("/getusers",web::get().to(user::getallusers))
            // .route("/userupdate/{id}",web::patch().to(user::update))
            // .route("/userupdatepwd/updatepwd/{id}",web::patch().to(user::updatepassword))
            // .route("/userdelete/{id}",web::delete().to(user::delete))
            .service(auth::register)
            .service(auth::userlogin)
            .service(user::get)
            .service(user::getallusers)
            .service(user::update)
            .service(user::updatepassword)
            .service(user::delete)
            .service(
                Files::new("/","./client/build")
                .index_file("index.html")
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
