use actix_web::{App, HttpServer};
mod user;
use auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // println!("{}", auth::name(4));

    HttpServer::new(|| {
        App::new()
            .service(auth::register)
            .service(user::get)
            .service(user::update)
            .service(user::delete)
            .service(auth::userlogin)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}