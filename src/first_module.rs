use actix_web::{web, HttpResponse};

pub fn first_mod(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/1stmod")
            .route(web::get().to(||HttpResponse::Ok().body("1st Module")))
            .route(web::head().to(||HttpResponse::MethodNotAllowed())),
    );
}