use axum::{
    routing::get,
    Router,
};

async fn root() -> &'static str {
    "8001 %0"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8001").await.unwrap();
    axum::serve(listener, app).await;
}
