use axum::extract::{ConnectInfo, Request};
use axum::http::header::{self, CONTENT_TYPE};
use axum::http::HeaderMap;
use axum::{
    http::header::SET_COOKIE,
    response::{Html, IntoResponse, Redirect},
};
use std::net::SocketAddr;

pub async fn api_handler(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request,
) -> impl IntoResponse {
    println!("------------------------------------");
    println!("Address: {:?}", addr);
    println!("Request Method: {}", req.method());
    println!("Request URI: {}", req.uri());
    for (key, value) in req.headers() {
        println!("{}: {}", key, value.to_str().unwrap());
    }
    let mut stre = String::new();
    if let Some(cookie_header) = req.headers().get("Cookie") {
        let formatted = format!("Cookies: {}", cookie_header.to_str().unwrap());
        stre.push_str(&formatted);
    }

    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        header::HeaderValue::from_static("text/html; charset=utf-8"),
    );
    let content = match req.uri().path() {
        "/api/route1" => {
            headers.insert(
                SET_COOKIE,
                header::HeaderValue::from_static("my_cookie=my_value"),
            );
            Html(format!(
                "/api {addr} Hello from Route 1!<br/><a href='/api'>Home</a> <br/>{stre}"
            ))
        }
        "/api/route2" => Html(format!(
            "/api {addr} Hello from Route 2!<br/><a href='/api'>Home</a> <br/>{stre}"
        )),
        "/api" => Html(format!(
            "/api {addr}<br/><a href='/api/route1'>route1</a><br/><a href='/api/route2'>route2</a><br/>{stre}"
        )),
        _ => {
            return Redirect::permanent("/api").into_response();
        }
    };
    (headers, content).into_response()
}
