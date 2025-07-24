use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::Mutex;
use chrono::Local;
use std::sync::Arc;

type LogFile = Arc<Mutex<tokio::fs::File>>;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Serveur de journalisation démarré sur le port 8080...");

    // Fichier log partagé entre toutes les connexions
    let file = tokio::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs.txt")
        .await?;

    let log_file = Arc::new(Mutex::new(file));

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Client connecté : {}", addr);

        let log_file = Arc::clone(&log_file);

        // Lancer une tâche asynchrone pour gérer chaque client
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, log_file).await {
                eprintln!("Erreur avec le client {} : {}", addr, e);
            }
        });
    }
}

async fn handle_client(stream: TcpStream, log_file: LogFile) -> tokio::io::Result<()> {
    let reader = BufReader::new(stream);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();

        let entry = format!("{} {}\n", timestamp, line);

        let mut file = log_file.lock().await;
        file.write_all(entry.as_bytes()).await?;
    }

    Ok(())
}
