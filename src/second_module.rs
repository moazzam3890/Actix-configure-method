use actix_web::{web, HttpResponse};

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/2ndmod")
            .route(web::get().to(||HttpResponse::Ok().body("2nd Module")))
            .route(web::head().to(||HttpResponse::MethodNotAllowed())),
    );
}