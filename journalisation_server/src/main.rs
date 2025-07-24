use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::Mutex;
use chrono::Utc;
use std::sync::Arc;
use std::fs::OpenOptions;
use std::io::Write;

// Chemin du fichier de log
const LOG_FILE: &str = "logs/server.log";

#[tokio::main]
async fn main() {
    // Mutex partagé pour l'accès concurrent au fichier log
    let log_mutex = Arc::new(Mutex::new(()));

    // Démarre le serveur TCP sur le port 9000
    let listener = TcpListener::bind("0.0.0.0:9000").await.expect("Erreur bind port 9000");
    println!("Serveur de journalisation en écoute sur le port 9000...");

    loop {
        // Accepte une nouvelle connexion
        let (stream, addr) = listener.accept().await.expect("Erreur accept");
        println!("Nouveau client : {}", addr);
        let log_mutex = Arc::clone(&log_mutex);
        // Spawn une tâche asynchrone pour chaque client
        tokio::spawn(async move {
            handle_client(stream, log_mutex).await;
        });
    }
}

async fn handle_client(stream: TcpStream, log_mutex: Arc<Mutex<()>>) {
    let peer = stream.peer_addr().ok();
    let reader = BufReader::new(stream);
    let mut lines = reader.lines();
    while let Ok(Some(line)) = lines.next_line().await {
        let now = Utc::now();
        let log_entry = format!("[{}]  {}\n", now.to_rfc3339(), line);
        // Accès exclusif au fichier log
        let _guard = log_mutex.lock().await;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(LOG_FILE)
            .expect("Impossible d'ouvrir le fichier log");
        file.write_all(log_entry.as_bytes()).expect("Erreur écriture log");
        drop(_guard);
        println!("Log reçu de {:?} : {}", peer, line);
    }
    println!("Client {:?} déconnecté.", peer);
}
