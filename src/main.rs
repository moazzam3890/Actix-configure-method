use actix_web::{App, web, HttpServer, HttpResponse};

mod first_module;
mod second_module;
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
            .configure(first_module::first_mod)
            .service(web::scope("/api").configure(second_module::api))
            .route("/", web::get().to(||HttpResponse::Ok().body(
                "Nothing here | Please go to /1stmod OR /api/2ndmod")))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}