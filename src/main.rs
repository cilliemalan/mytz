use axum::{http::HeaderMap, routing::get, Router};

async fn root_handler() -> &'static str {
    "Hello, World!"
}

async fn tz_handler(headers: HeaderMap) -> String {
    for (key, value) in headers.iter() {
        println!("{}: {:?}", key, value);
    }

    let ip_addr = headers.get("X-Forwarded-For")
        .map(|s| s.to_str().ok().unwrap_or(""))
        .unwrap_or("");

    format!("Hello, {ip_addr}!")
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/tz", get(tz_handler));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3200".to_string());
    let listen = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(listen).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
