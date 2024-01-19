mod api;

use actix_web::{
    web::scope,
    App,
    HttpServer,
};
use actix_web_lab::web::spa;

use api::hello::hello;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(
                scope("/api")
                    .service(hello)
            )
            .service(
                spa()
                .index_file("./dist/index.html")
                .static_resources_mount("/")
                .static_resources_location("./dist/")
                .finish()
            )
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}

