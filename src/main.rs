mod servers;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Invalid number of arguments: {}", args.len());
        return;
    }
    let domain = &args[1];

    let servers_response = servers::get_server(&domain);
    let server = match servers_response {
        Some(e) => e,
        None => {
            eprintln!("No server found for: {}", domain);
            return;
        }
    };
    
    let whois_response = get_response(&domain, &server);
    let whois = match whois_response {
        Some(e) => e,
        None => {
            eprintln!("Failed to get whois response");
            return;
        }
    };

    println!("Server: {}", server);
    println!("Domain: {}\n", domain);
    println!("{}", whois)
}

fn get_response(domain: &str, server: &str) -> Option<String> {
    let mut stream = TcpStream::connect(server).ok()?;
    stream.write_all(format!("{}\r\n", domain).as_bytes()).ok()?;
    let mut response = String::new();
    stream.read_to_string(&mut response).ok()?;

    if let Some(pos) = response.find("<<<") {
        Some(response[..pos+3].trim_end().to_string())
    } else {
        Some(response)
    }
}
