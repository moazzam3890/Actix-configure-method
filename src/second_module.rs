use actix_web::{web, HttpResponse};

pub fn api(cfg: &mut web::ServiceConfig) {// make a function in second module
    cfg.service( //applied service method on cfg
        web::resource("/2ndmod") //use of resource at path /api/2ndmod
            .route(web::get().to(||HttpResponse::Ok().body("2nd Module")))
            .route(web::head().to(||HttpResponse::MethodNotAllowed())),
    );
}