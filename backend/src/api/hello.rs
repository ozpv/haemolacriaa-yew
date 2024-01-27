use actix_web::{get, web::Json};

#[get("/test")]
pub async fn hello() -> Json<String> {
    Json(String::from("under construction..."))
}
