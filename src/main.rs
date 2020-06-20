use actix_web::{App, web, HttpServer, HttpResponse};

mod first_module; //1st module import
mod second_module; //second module import

#[actix_rt::main] //to make main function asynchronous
async fn main() -> std::io::Result<()> { 
    HttpServer::new(||{ // create new http server
        App::new() //Create new app isntance of Actix
            .configure(first_module::first_mod) //Configuration of 1st module
            .service(web::scope("/api").configure(second_module::api)) //Configuration of second module with scope
            .route("/", web::get().to(||HttpResponse::Ok().body(//route defined for base path "/"
                "Nothing here | Please go to /1stmod OR /api/2ndmod")))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}