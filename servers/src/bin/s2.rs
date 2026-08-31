use axum::{
    routing::get,
    Router,
};

async fn root() -> &'static str {
    "8002 %1"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root));
    let listAener = tokio::net::TcpListener::bind("127.0.0.1:8002").await.unwrap();
    axum::serve(listener, app).await;
}
