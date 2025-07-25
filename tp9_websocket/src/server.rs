use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures::{StreamExt, SinkExt};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:9001";
    let listener = TcpListener::bind(addr).await.expect("bind");
    println!("Serveur WebSocket en écoute sur {}", addr);
    let (tx, _rx) = broadcast::channel::<String>(100);
    let tx = Arc::new(tx);
    loop {
        let (stream, _) = listener.accept().await.expect("accept");
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.expect("handshake");
            let (mut ws_sender, mut ws_receiver) = ws_stream.split();
            // Tâche d'envoi (reçoit du broadcast)
            let send_task = tokio::spawn(async move {
                while let Ok(msg) = rx.recv().await {
                    ws_sender.send(tokio_tungstenite::tungstenite::Message::Text(msg)).await.ok();
                }
            });
            // Tâche de réception (reçoit du client)
            while let Some(Ok(msg)) = ws_receiver.next().await {
                if let tokio_tungstenite::tungstenite::Message::Text(txt) = msg {
                    tx.send(txt).ok();
                }
            }
            send_task.abort();
            println!("Client déconnecté");
        });
    }
} 