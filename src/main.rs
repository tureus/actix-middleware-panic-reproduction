use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{self};
use actix_middleware_panic_reproduction::Authenticated;

#[tokio::main]
async fn main() {
    let app = actix_web::HttpServer::new(move || {
        let identity_service = IdentityService::new(
            CookieIdentityPolicy::new(&[0; 32])
                .name("auth-cookie")
                .secure(false), // TODO: should be true to force HTTPS all the time!
        );

        let authenticated = Authenticated::default();

        actix_web::App::new()
            .wrap(identity_service)
            .wrap(authenticated)
            .service(get_home)
    });

    app.bind("0.0.0.0:4000").unwrap().run().await.unwrap();
}

#[actix_web::get("/")]
async fn get_home() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().body("hi")
}