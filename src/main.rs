mod geolocation;
mod tz;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use axum::{http::HeaderMap, routing::get, Router};
use geolocation::{geolocate_timezone, geolocate_tz};

fn get_ip_address(headers: HeaderMap) -> String {
    let ip_addr = headers
        .get("CF-Connecting-IP")
        .or_else(|| headers.get("CF-Connecting-IPv6"))
        .or_else(|| headers.get("X-Forwarded-For"))
        .map(|s| s.to_str().ok().unwrap_or(""))
        .unwrap_or("");

    let ipv4_regex = regex::Regex::new(r"^(?:[0-9]{1,3}\.){3}[0-9]{1,3}$").unwrap();
    let ipv6_regex = regex::Regex::new(r"^([0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}$").unwrap();

    if !ipv4_regex.is_match(ip_addr) && !ipv6_regex.is_match(ip_addr) {
        return "".to_string();
    }

    return ip_addr.to_string();
}

#[tokio::main]
async fn main() {
    let geolocation_api_key = std::env::var("GEOLOCATION_API_KEY")
        .expect("GEOLOCATION_API_KEY environment variable not set");

    let app = Router::new()
        .route(
            "/",
            get({
                let geolocation_api_key = geolocation_api_key.clone();
                |headers: HeaderMap| async move {
                    geolocate_timezone(geolocation_api_key, get_ip_address(headers))
                        .await
                        .unwrap_or("".to_string())
                }
            }),
        )
        .route(
            "/tz",
            get({
                let geolocation_api_key = geolocation_api_key.clone();
                |headers: HeaderMap| async move {
                    geolocate_tz(geolocation_api_key, get_ip_address(headers))
                        .await
                        .unwrap_or("")
                }
            }),
        );

    let port = std::env::var("PORT").unwrap_or_else(|_| "3200".to_string());
    let listen = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(listen).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
