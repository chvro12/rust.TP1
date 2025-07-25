mod protocol;
use protocol::*;
use std::io::{self, Write, Read};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Connexion serveur");
    println!("Connecté au serveur. Tapez un message, ou 'quit' pour sortir.");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let msg = if input == "quit" {
            ProtoMsg { op: OP_QUIT, payload: vec![] }
        } else {
            ProtoMsg { op: OP_MSG, payload: input.as_bytes().to_vec() }
        };
        let data = msg.serialize();
        stream.write_all(&data).unwrap();
        if input == "quit" { break; }
        let mut resp = [0u8; 256];
        let n = stream.read(&mut resp).unwrap();
        if let Some(reply) = ProtoMsg::deserialize(&resp[..n]) {
            if reply.op == OP_ACK {
                println!("[Serveur] ACK: {}", String::from_utf8_lossy(&reply.payload));
            }
        }
    }
    println!("Déconnecté.");
} 