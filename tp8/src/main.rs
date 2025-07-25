mod server;
mod client;

use std::io::{self, Write};

fn main() -> io::Result<()> {
    println!("Que voulez-vous faire ?");
    println!("1. Lancer le serveur");
    println!("2. Lancer le client");
    print!("Choix (1 ou 2) : ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    match input.trim() {
        "1" => server::run(),
        "2" => client::run(),
        _ => {
            println!("Choix invalide.");
            Ok(())
        }
    }
}