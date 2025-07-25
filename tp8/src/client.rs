use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;

pub fn run() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:9000")?;
    println!("Connecté au serveur.");

    let mut reader = BufReader::new(stream.try_clone()?);
    let stdin = io::stdin();

    loop {
        print!("Entrez un message ('exit' pour quitter) : ");
        io::stdout().flush()?;
        let mut input = String::new();
        stdin.read_line(&mut input)?;

        let trimmed = input.trim();
        if trimmed.eq_ignore_ascii_case("exit") {
            break;
        }

        stream.write_all(input.as_bytes())?;

        let mut response = String::new();
        reader.read_line(&mut response)?;
        println!("Réponse du serveur: {}", response.trim());
    }

    Ok(())
}