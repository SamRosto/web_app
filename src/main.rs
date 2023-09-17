use actix_web::{App, HttpServer};
mod views;
mod to_do;
mod processes;
mod state;
mod json_serialization;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new()
            .configure(views::views_factory);
        app
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
 
