use axum::{
    routing::get,
    Router,
};

async fn root_s1() -> &'static str { "s1 8001 %0\n" }
async fn root_s2() -> &'static str { "s2 8002 %1\n" }
async fn root_s3() -> &'static str { "s3 8003 %2\n" }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_s1 = Router::new().route("/", get(root_s1));
    let app_s2 = Router::new().route("/", get(root_s2));
    let app_s3 = Router::new().route("/", get(root_s3));

    let listener_s1 = tokio::net::TcpListener::bind("127.0.0.1:8001").await.unwrap();
    let listener_s2 = tokio::net::TcpListener::bind("127.0.0.1:8002").await.unwrap();
    let listener_s3 = tokio::net::TcpListener::bind("127.0.0.1:8003").await.unwrap();

    let s1 = axum::serve(listener_s1, app_s1);
    let s2 = axum::serve(listener_s2, app_s2);
    let s3 = axum::serve(listener_s3, app_s3);

    println!("starting servers on ports 8001, 8002, 8003");

    let _ = tokio::join!(s1, s2, s3);

    Ok(())
}
