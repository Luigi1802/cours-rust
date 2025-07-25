use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

pub fn run() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9000")?;
    println!("Serveur en écoute sur 127.0.0.1:9000");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Client connecté: {}", stream.peer_addr()?);
                std::thread::spawn(move || handle_client(stream));
            }
            Err(e) => eprintln!("Erreur de connexion: {}", e),
        }
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    loop {
        let mut message = String::new();
        match reader.read_line(&mut message) {
            Ok(0) => {
                println!("Client déconnecté.");
                break;
            }
            Ok(_) => {
                println!("Message reçu: {}", message.trim());
                if let Err(e) = writeln!(stream, "[Serveur]: Message bien reçu") {
                    eprintln!("Erreur d'envoi: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Erreur de lecture: {}", e);
                break;
            }
        }
    }
}