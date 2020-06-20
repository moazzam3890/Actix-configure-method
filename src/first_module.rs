use actix_web::{web, HttpResponse};

pub fn first_mod(cfg: &mut web::ServiceConfig) {// make a function in first module
    cfg.service( //applied service method on cfg
        web::resource("/1stmod")//use of resource at path /1stmod
            .route(web::get().to(||HttpResponse::Ok().body("1st Module")))
            .route(web::head().to(||HttpResponse::MethodNotAllowed())),
    );
}