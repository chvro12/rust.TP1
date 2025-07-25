# TP9 : Serveur et Client WebSocket en Rust

## Objectif

Implémenter un serveur et un client WebSocket en Rust pour permettre une communication bidirectionnelle temps réel (chat multi-clients).

## Structure du projet

```
tp9_websocket/
├── src/
│   ├── server.rs   # Serveur WebSocket multi-clients
│   ├── client.rs   # Client WebSocket interactif
├── Cargo.toml
```

## Lancer le serveur

```sh
cd tp9_websocket
cargo run --bin server
```

## Lancer le client

```sh
cd tp9_websocket
cargo run --bin client
```

## Fonctionnalités
- Le serveur accepte plusieurs connexions WebSocket et diffuse les messages à tous les clients connectés.
- Le client permet d'envoyer des messages et d'afficher ceux reçus du serveur.

## Concepts clés
- WebSocket (protocole full-duplex)
- tokio-tungstenite (crate Rust pour WebSocket asynchrone)
- Programmation asynchrone avec Tokio
- Diffusion de messages (broadcast)

## Pour aller plus loin
- Gérer les messages binaires
- Ajouter des pseudos ou une authentification
- Créer une interface graphique ou web pour le client 