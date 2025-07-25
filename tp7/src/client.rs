use std::io::{self, Write};
use std::net::UdpSocket;

pub fn run() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("127.0.0.1:8053")?;

    loop {
        println!("Domaines disponibles : example.com, google.com, rust-lang.org");
        print!("Entrez le nom de domaine à résoudre (ou 'exit' pour quitter) : ");
        io::stdout().flush()?; // pour afficher l'invite tout de suite

        let mut domain = String::new();
        io::stdin().read_line(&mut domain)?;
        let domain = domain.trim();

        if domain.eq_ignore_ascii_case("exit") {
            break;
        }

        if domain.is_empty() {
            continue;
        }

        let message = build_query(domain);
        socket.send(&message)?;

        let mut buf = [0u8; 1024];
        let len = socket.recv(&mut buf)?;

        parse_response(&buf[..len]);
    }

    Ok(())
}

fn build_query(domain: &str) -> Vec<u8> {
    let mut packet = Vec::new();

    packet.extend_from_slice(&[0x12, 0x34]);
    packet.extend_from_slice(&[0x01, 0x00]); 
    packet.extend_from_slice(&[0x00, 0x01]); 
    packet.extend_from_slice(&[0x00, 0x00]);
    packet.extend_from_slice(&[0x00, 0x00]);
    packet.extend_from_slice(&[0x00, 0x00]);
    
    for part in domain.split('.') {
        packet.push(part.len() as u8);
        packet.extend_from_slice(part.as_bytes());
    }
    packet.push(0); // fin du nom

    packet.extend_from_slice(&[0x00, 0x01]); 
    packet.extend_from_slice(&[0x00, 0x01]); 

    packet
}

fn parse_response(buf: &[u8]) {
    if buf.len() < 12 {
        println!("Réponse trop courte");
        return;
    }

    let qname_len = domain_len(&buf[12..]);

    let answer_start = 12 + qname_len + 4; // 4 = type (2) + class (2) de la question

    let ip_offset = answer_start + 12;

    if buf.len() >= ip_offset + 4 {
        let ip_bytes = &buf[ip_offset..ip_offset + 4];
        let ip = format!("{}.{}.{}.{}", ip_bytes[0], ip_bytes[1], ip_bytes[2], ip_bytes[3]);
        println!("Adresse IP retournée : {}", ip);
    } else {
        println!("Réponse DNS invalide ou trop courte");
    }
}


fn domain_len(buf: &[u8]) -> usize {
    let mut i = 0;
    while i < buf.len() && buf[i] != 0 {
        i += buf[i] as usize + 1;
    }
    i + 1
}
