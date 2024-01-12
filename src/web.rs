use std::{borrow::BorrowMut, ops::Deref, sync::Arc};

use axum::{
    body::Body,
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
};
use futures_channel::mpsc::unbounded;
use rust_embed::RustEmbed;
use serde::Deserialize;
use tokio_stream::wrappers::BroadcastStream;

use crate::camera::Camera;

static INDEX_HTML: &str = "index.html";

#[derive(RustEmbed)]
#[folder = "resources/"]
struct Assets;

pub async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == INDEX_HTML {
        return index_html().await;
    }

    match Assets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => {
            if path.contains('.') {
                return not_found().await;
            }

            index_html().await
        }
    }
}

async fn index_html() -> Response {
    match Assets::get(INDEX_HTML) {
        Some(content) => Html(content.data).into_response(),
        None => not_found().await,
    }
}

async fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "404").into_response()
}

pub async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(ws_control)
}

#[derive(Deserialize, Debug)]
struct WsPayloadData {
    x: f32,
    y: f32,
}

#[derive(Deserialize, Debug)]
struct WsPayload {
    event: String,
    data: Option<WsPayloadData>,
}

async fn ws_control(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(ref t) => {
                let payload = serde_json::from_slice::<WsPayload>(t.as_bytes());

                match payload {
                    Ok(payload) => {}
                    Err(e) => {
                        println!("{:?}", e);
                        socket
                            .send(Message::Text("Error".to_string()))
                            .await
                            .unwrap();
                    }
                }
            }
            _ => {}
        }
    }
}

pub async fn cam_feed(State(cam): State<Arc<Camera>>) -> Response {
    let camera = cam.clone();
    let rx = camera.subscribe();
    let s = BroadcastStream::new(rx);

    let body = Body::from_stream(s);

    Response::new(body)
}
