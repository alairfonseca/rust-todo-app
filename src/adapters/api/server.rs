use actix_web::{ App, HttpServer };

use crate::config::app_state_factory;

use super::routes::*;

#[actix_web::main]
pub async fn setup() -> std::io::Result<()> {
    HttpServer::new(|| {
        println!("starting server...");

        let app = App::new()
            .data(app_state_factory())
            .configure(|config|
                routes_setup(config),
            );

        return app;
    })
    .bind("127.0.0.1:8080")?
    .workers(2)
    .run()
    .await
}
