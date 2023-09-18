use actix_web::{App, HttpServer};
use actix_service::Service;

mod jwt;
mod views;
mod to_do;
mod processes;
mod state;
mod json_serialization;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new()
            .wrap_fn(|req, srv| {
                println!("{:?}", req);
                let future = srv.call(req);
                async {
                    let result = future.await?;
                    Ok(result)
                }
            })
            .configure(views::views_factory);
        app
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
 
