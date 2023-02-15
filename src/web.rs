use futures_util::{SinkExt, StreamExt, TryFutureExt};
use serde_json::Value;
use std::sync::{atomic::Ordering, Arc};
use warp::ws::{Message, WebSocket};

use crate::car::Car;

pub async fn handle_socket(ws: WebSocket, car: Arc<Car>) {
    let (mut tx, mut rx) = ws.split();

    while let Some(result) = rx.next().await {
        let message = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error: {}", e);
                break;
            }
        };
        let msg = match message.to_str() {
            Ok(msg) => msg,
            Err(e) => {
                println!("Error converting message to string: {:?}", e);
                continue;
            }
        };
        match serde_json::from_str(msg) {
            Ok(data) => {
                if car.is_driving.load(Ordering::Relaxed) {
                    tx.send(Message::text("Already driving"))
                        .unwrap_or_else(|e| {
                            eprintln!("websocket send error: {:?}", e);
                        })
                        .await;
                    return;
                }

                handle_data(data, &car).await;
            }
            Err(e) => {
                println!("Non-JSON socket message: {:?}", e);
            }
        }
    }

    // If the client disconnects, stop driving
    if car.is_driving.load(Ordering::Relaxed) {
        car.stop();
        car.set_driving(false);
    }
}

async fn handle_data(data: Value, car: &Arc<Car>) {
    match data["event"].as_str() {
        Some("drive") => {
            if !car.is_driving.load(Ordering::Relaxed) {
                car.set_driving(true);
            }
            let x = data["x"].as_f64().unwrap();
            let y = data["y"].as_f64().unwrap();
            car.drive(x, y);
        }
        Some("stop") => {
            car.stop();
        }
        _ => {
            println!("Unknown event: {}", data["event"]);
        }
    };
}

pub async fn camera_feed() {}
