mod protocol;
use protocol::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    loop {
        let mut buf = [0u8; 256];
        let n = match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => break,
        };
        if let Some(msg) = ProtoMsg::deserialize(&buf[..n]) {
            match msg.op {
                OP_MSG => {
                    let text = String::from_utf8_lossy(&msg.payload);
                    println!("[Client] Message: {}", text);
                    let ack = ProtoMsg { op: OP_ACK, payload: b"OK".to_vec() };
                    let data = ack.serialize();
                    stream.write_all(&data).unwrap();
                },
                OP_QUIT => {
                    println!("[Client] Déconnexion demandée.");
                    break;
                },
                _ => {}
            }
        }
    }
    println!("Client déconnecté.");
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").expect("bind");
    println!("Serveur en écoute sur 7878");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            },
            Err(e) => eprintln!("Erreur connexion: {}", e),
        }
    }
} 