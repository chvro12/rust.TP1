# TP7 : Client et Serveur DNS Simples

## Objectif

Implémenter un client DNS basique capable de résoudre des noms de domaine en adresses IP, et un serveur DNS simple qui répond à des requêtes pour quelques domaines prédéfinis. Ce TP permet de découvrir la programmation UDP, le format des messages DNS et la manipulation binaire en Rust.

## Structure du projet

```
tp7_dns/
├── src/
│   ├── client.rs      # Client DNS simple
│   ├── server.rs      # Serveur DNS simple
│   └── lib.rs         # (optionnel, pour factoriser du code)
├── Cargo.toml
```

## Lancer le serveur

```sh
cargo run --bin server
```

## Lancer le client

```sh
cargo run --bin client
```

## Concepts clés
- UdpSocket (UDP en Rust)
- Format des messages DNS (RFC 1035)
- Sérialisation/désérialisation binaire
- Résolution de noms

## Fonctionnalités attendues
- Le client construit une requête DNS, l’envoie au serveur, reçoit la réponse et affiche l’IP.
- Le serveur écoute sur un port UDP, parse les requêtes, et répond pour quelques domaines prédéfinis.

## Ressources utiles
- [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035)
- [UdpSocket Rust](https://doc.rust-lang.org/std/net/struct.UdpSocket.html)
- [DNS message format (schéma)](https://upload.wikimedia.org/wikipedia/commons/6/6a/DNS-message-format.png) 