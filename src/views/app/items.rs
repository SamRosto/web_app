// Displays to-do items
use actix_web::HttpResponse;
use super::content_loader::read_file;

// To pass HttpResponse factory method should be defined in the `mod.rs`
pub async fn items() -> HttpResponse {
    let html_data = read_file("./templates/main.html");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_data)
}