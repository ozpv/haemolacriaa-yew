mod api;

use actix_web::{web::scope, App, HttpServer};
use actix_web_lab::web::spa;

use api::hello::hello;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new().service(scope("/api").service(hello)).service(
            spa()
                .index_file("/var/www/htdocs/index.html")
                .static_resources_mount("/")
                .static_resources_location("/var/www/htdocs/")
                .finish(),
        )
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
