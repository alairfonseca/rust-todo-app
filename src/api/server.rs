use actix_web::{ App, HttpServer, Responder, HttpResponse, get };

use super::routes::*;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("SAVERO PEGA NO BREU")
}

#[actix_web::main]
pub async fn setup() -> std::io::Result<()> {
    HttpServer::new(|| {
        println!("starting server...");

        let app = App::new()
            .configure(routes_factory);

        return app;
    })
    .bind("127.0.0.1:8080")?
    .workers(2)
    .run()
    .await
}
