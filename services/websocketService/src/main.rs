use warp::Filter;
use tokio::sync::broadcast;
use warp::ws::{Message, WebSocket, Ws};
use futures_util::{StreamExt, SinkExt};

#[tokio::main]
async fn main() {
    let broadcast_tx = broadcast::channel::<String>(100).0;

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::any().map(move || broadcast_tx.clone()))
        .map(|ws: Ws, broadcast_tx| {
            ws.on_upgrade(move |socket| client_connected(socket, broadcast_tx))
        });

    println!("WebSocket server started on ws://127.0.0.1:8080");

    warp::serve(ws_route)
        .run(([127, 0, 0, 1], 8080))
        .await;
}

async fn client_connected(ws: WebSocket, broadcast_tx: broadcast::Sender<String>) {
    let (mut ws_tx, mut ws_rx) = ws.split();
    let mut broadcast_rx = broadcast_tx.subscribe();

    tokio::spawn(async move {
        while let Ok(message) = broadcast_rx.recv().await {
            // Convert the String message to a Warp WebSocket Message
            let warp_message = Message::text(message);

            // Send the message to the client
            if ws_tx.send(warp_message).await.is_err() {
                break;
            }
        }
    });

    while let Some(Ok(message)) = ws_rx.next().await {
        if let Ok(text) = message.to_str() {
            println!("Received message: {}", text);
            let _ = broadcast_tx.send(text.to_string());
        }
    }
}

