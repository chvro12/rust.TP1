use tokio_tungstenite::connect_async;
use futures::{SinkExt, StreamExt};
use tokio::io::{self, AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() {
    let url = url::Url::parse("ws://127.0.0.1:9001").unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("connect");
    let (mut write, mut read) = ws_stream.split();
    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();
    println!("Connecté au serveur WebSocket. Tapez un message, Ctrl+C pour quitter.");
    loop {
        tokio::select! {
            Some(Ok(line)) = lines.next_line() => {
                write.send(tokio_tungstenite::tungstenite::Message::Text(line)).await.ok();
            }
            Some(Ok(msg)) = read.next() => {
                if let tokio_tungstenite::tungstenite::Message::Text(txt) = msg {
                    println!("[Serveur] {}", txt);
                }
            }
        }
    }
} 