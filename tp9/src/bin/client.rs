use futures::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tungstenite::protocol::Message;
use url::Url;

#[tokio::main]
async fn main() {
    let url = Url::parse("ws://127.0.0.1:9000").unwrap();

    println!("* Connexion à {}", url);
    let (ws_stream, _) = connect_async(url).await.expect("Échec connexion");

    let (mut write, mut read) = ws_stream.split();

    // envoi de messages texte
    write.send(Message::Text("Bonjour serveur!".to_string())).await.unwrap();
    write.send(Message::Text("Voici un autre message.".to_string())).await.unwrap();

    // envoi de message binaire
    write.send(Message::Binary(vec![1, 2, 3, 4])).await.unwrap();

    // lecture des réponses
    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(txt)) => println!("Réponse texte: {}", txt),
            Ok(Message::Binary(bin)) => println!("Réponse binaire: {:?}", bin),
            Ok(Message::Close(_)) => break,
            _ => {}
        }
    }
}