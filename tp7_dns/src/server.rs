use std::net::UdpSocket;
use std::collections::HashMap;

fn main() {
    println!("Serveur DNS simple en écoute sur 0.0.0.0:5353");
    let socket = UdpSocket::bind("0.0.0.0:5353").expect("bind");
    // Table de correspondance nom -> IP
    let records = HashMap::from([
        ("example.com", [1,2,3,4]),
        ("rust-lang.org", [185,199,110,153]),
    ]);
    let mut buf = [0u8; 512];
    loop {
        let (len, src) = socket.recv_from(&mut buf).expect("recv_from");
        println!("Requête reçue de {} ({} octets)", src, len);
        // Parse le nom demandé
        let mut pos = 12; // après header
        let mut labels = Vec::new();
        while buf[pos] != 0 {
            let l = buf[pos] as usize;
            pos += 1;
            let label = std::str::from_utf8(&buf[pos..pos+l]).unwrap_or("");
            labels.push(label);
            pos += l;
        }
        pos += 1; // 0 final
        let qtype = (buf[pos] as u16) << 8 | (buf[pos+1] as u16);
        pos += 2;
        let _qclass = (buf[pos] as u16) << 8 | (buf[pos+1] as u16);
        pos += 2;
        let name = labels.join(".");
        println!("Nom demandé : {} (QTYPE={})", name, qtype);
        // Si le nom est connu et QTYPE = A (1)
        if let Some(ip) = records.get(name.as_str()) {
            // Construire la réponse DNS
            let mut resp = Vec::new();
            // Copie header, change flags (QR=1, RCODE=0)
            resp.extend(&buf[0..2]); // ID
            resp.extend(&[0x81, 0x80]); // Flags: réponse standard
            resp.extend(&[0x00, 0x01]); // QDCOUNT
            resp.extend(&[0x00, 0x01]); // ANCOUNT
            resp.extend(&[0x00, 0x00]); // NSCOUNT
            resp.extend(&[0x00, 0x00]); // ARCOUNT
            // Question (copie brute)
            resp.extend(&buf[12..pos]);
            // Réponse
            resp.push(0xC0); resp.push(0x0C); // NAME (pointeur vers QNAME)
            resp.extend(&[0x00, 0x01]); // TYPE: A
            resp.extend(&[0x00, 0x01]); // CLASS: IN
            resp.extend(&[0x00, 0x00, 0x00, 0x3C]); // TTL: 60s
            resp.extend(&[0x00, 0x04]); // RDLENGTH: 4
            resp.extend(ip); // RDATA: IPv4
            socket.send_to(&resp, src).expect("send_to");
            println!("Réponse envoyée : {}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3]);
        } else {
            println!("Nom inconnu ou type non supporté");
        }
    }
} 