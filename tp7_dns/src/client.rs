use std::net::UdpSocket;
use std::io::{self, Write};

fn main() {
    println!("Client DNS simple");
    print!("Nom de domaine à résoudre : ");
    io::stdout().flush().unwrap();
    let mut domain = String::new();
    io::stdin().read_line(&mut domain).unwrap();
    let domain = domain.trim();

    // Construction du message DNS (header + question)
    let mut req = Vec::new();
    // Header (12 octets)
    req.extend(&[0x12, 0x34]); // ID
    req.extend(&[0x01, 0x00]); // Flags: standard query
    req.extend(&[0x00, 0x01]); // QDCOUNT: 1 question
    req.extend(&[0x00, 0x00]); // ANCOUNT
    req.extend(&[0x00, 0x00]); // NSCOUNT
    req.extend(&[0x00, 0x00]); // ARCOUNT
    // Question
    for label in domain.split('.') {
        req.push(label.len() as u8);
        req.extend(label.as_bytes());
    }
    req.push(0); // fin du nom
    req.extend(&[0x00, 0x01]); // QTYPE: A
    req.extend(&[0x00, 0x01]); // QCLASS: IN

    // Envoi de la requête au serveur DNS local (port 5353)
    let socket = UdpSocket::bind("0.0.0.0:0").expect("bind");
    socket.send_to(&req, "127.0.0.1:5353").expect("send_to");

    // Réception de la réponse
    let mut buf = [0u8; 512];
    let (len, _) = socket.recv_from(&mut buf).expect("recv_from");
    // Parsing simple de la réponse (on suppose une seule réponse A)
    // On saute header (12), question (variable)
    let mut pos = 12;
    // Skip QNAME
    while buf[pos] != 0 { pos += 1 + buf[pos] as usize; }
    pos += 1; // le 0 final
    pos += 4; // QTYPE + QCLASS
    // Réponse
    if buf.len() < pos + 16 {
        println!("Réponse DNS trop courte ou pas de réponse A");
        return;
    }
    pos += 2; // NAME (pointeur)
    pos += 2; // TYPE
    pos += 2; // CLASS
    pos += 4; // TTL
    let rdlength = ((buf[pos] as usize) << 8) | (buf[pos+1] as usize);
    pos += 2;
    if rdlength == 4 {
        let ip = format!("{}.{}.{}.{}", buf[pos], buf[pos+1], buf[pos+2], buf[pos+3]);
        println!("Adresse IP reçue : {}", ip);
    } else {
        println!("Pas d'adresse IPv4 dans la réponse");
    }
} 