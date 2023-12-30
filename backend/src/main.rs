use axum::{
    routing::get,
    response::IntoResponse,
    http::StatusCode,
    Router,
    Json,
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/test", get(handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8081").await.unwrap();
    
    axum::serve(listener, app).await.unwrap();
}

#[derive(serde::Serialize)]
struct Message {
    message: String,
}

async fn handler() -> impl IntoResponse {
    (StatusCode::OK, Json(Message {
        message: String::from("Works!"),
    }))
}
