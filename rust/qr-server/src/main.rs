use std::process;
use axum::{
    extract::Query,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

#[derive(serde::Deserialize)]
struct TextParam {
    text: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/qrcode", get(handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:42069")
        .await
        .unwrap();
    println!("server {} listening on {}", process::id(), listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler(
    Query(param): Query<TextParam>
) -> Result<Response, (StatusCode, &'static str)> {
    if param.text.len() > 512 {
        return Err((
            StatusCode::BAD_REQUEST,
            "text must be <= 512 bytes"
        ));
    }
    match qr_lib::generate_qr_code(&param.text) {
        Ok(bytes) => Ok((
            [(header::CONTENT_TYPE, "image/png"),],
            bytes,
        ).into_response()),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to generate qr code"
        )),
    }
}
