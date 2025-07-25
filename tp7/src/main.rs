mod server;
mod client;

use std::io::{self, Write};

fn main() -> std::io::Result<()> {
    println!("Que voulez-vous faire ?");
    println!("1. Lancer le serveur DNS");
    println!("2. Lancer le client DNS");

    print!("Choix (1 ou 2) : ");
    io::stdout().flush()?;

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;

    match choice.trim() {
        "1" => server::run(),
        "2" => client::run(),
        _ => {
            println!("Choix invalide");
            Ok(())
        }
    }
}