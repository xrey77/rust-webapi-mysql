use actix_web::{App, HttpServer};
mod user;
use auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // println!("{}", auth::name(4));

    HttpServer::new(|| {
        App::new()
            .service(auth::register)
            .service(auth::userlogin)
            .service(user::get)
            .service(user::getallusers)
            .service(user::update)
            .service(user::updatepassword)
            .service(user::delete)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}