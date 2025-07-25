use futures::{StreamExt, SinkExt};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tungstenite::protocol::Message;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:9000".to_string();
    let listener = TcpListener::bind(&addr).await.expect("Échec bind");

    println!("* Serveur WebSocket en écoute sur ws://{}", addr);

    while let Ok((stream, peer)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, peer));
    }
}

async fn handle_connection(stream: tokio::net::TcpStream, addr: SocketAddr) {
    println!("+ Nouveau client connecté: {}", addr);

    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("Erreur de handshake: {}", e);
            return;
        }
    };

    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(txt)) => {
                println!("message [{}] {}", addr, txt);
                let response = format!("Echo: {}", txt);
                write.send(Message::Text(response)).await.unwrap();
            }
            Ok(Message::Binary(bin)) => {
                println!("binaire [{}] Binaire reçu ({} octets)", addr, bin.len());
                write.send(Message::Binary(bin)).await.unwrap();
            }
            Ok(Message::Close(_)) => {
                println!("[{}] Connexion fermée", addr);
                break;
            }
            _ => {}
        }
    }

    println!("[{}] Déconnecté", addr);
}