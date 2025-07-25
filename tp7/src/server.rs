use std::net::UdpSocket;
use std::collections::HashMap;

pub fn run() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8053")?;
    println!("Serveur DNS en écoute sur 127.0.0.1:8053");

    let mut dns_db = HashMap::new();
    dns_db.insert("example.com", "93.184.216.34");
    dns_db.insert("google.com", "142.250.186.14");
    dns_db.insert("rust-lang.org", "13.32.86.123");

    let mut buf = [0u8; 512];

    loop {
        let (size, src) = socket.recv_from(&mut buf)?;
        println!("Requête reçue de {}", src);

        let domain = parse_domain(&buf[12..size]).to_lowercase().trim().to_string();
        println!("Demande de résolution pour : {}", domain);

        let response = build_response(&buf, &domain, &dns_db);
        socket.send_to(&response, src)?;
    }
}

fn parse_domain(buf: &[u8]) -> String {
    let mut domain = String::new();
    let mut i = 0;

    while i < buf.len() && buf[i] != 0 {
        let len = buf[i] as usize;
        i += 1;
        if i + len > buf.len() {
            break;
        }
        let label = String::from_utf8_lossy(&buf[i..i + len]);
        if !domain.is_empty() {
            domain.push('.');
        }
        domain.push_str(&label);
        i += len;
    }

    domain
}

fn build_response(query: &[u8], domain: &str, dns_db: &HashMap<&str, &str>) -> Vec<u8> {
    let mut response = Vec::new();

    response.extend_from_slice(&query[0..2]); 
    response.extend_from_slice(&[0x81, 0x80]);
    response.extend_from_slice(&query[4..6]); 
    response.extend_from_slice(&[0x00, 0x01]); 
    response.extend_from_slice(&[0x00, 0x00]); 
    response.extend_from_slice(&[0x00, 0x00]); 

    let qname_len = domain_len(&query[12..]);
    let question_end = 12 + qname_len + 4; // nom + type + class
    response.extend_from_slice(&query[12..question_end]);

    response.push(0xc0); // pointer vers le nom (offset 12)
    response.push(0x0c);
    response.extend_from_slice(&[0x00, 0x01]); 
    response.extend_from_slice(&[0x00, 0x01]);
    response.extend_from_slice(&[0x00, 0x00, 0x00, 0x3c]); 
    response.extend_from_slice(&[0x00, 0x04]);

    let ip = dns_db.get(domain).unwrap_or(&"127.0.0.1");
    
    let octets: Vec<u8> = ip
        .split('.')
        .map(|s| s.parse::<u8>().unwrap())
        .collect();
    response.extend_from_slice(&octets);

    response
}

fn domain_len(buf: &[u8]) -> usize {
    let mut i = 0;
    while i < buf.len() && buf[i] != 0 {
        i += buf[i] as usize + 1;
    }
    i + 1
}