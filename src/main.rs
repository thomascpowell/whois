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

    let servers_res = servers::get_server(&domain);
    let server = match servers_res {
        Some(e) => e,
        None => {
            eprintln!("No server found for: {}", domain);
            return;
        }
    };

    println!("Server: {}", server);

    let mut stream = TcpStream::connect(server)
        .expect("Could not connect to WHOIS server");
    stream
        .write_all(format!("{}\r\n", domain).as_bytes())
        .expect("Failed to send query");

    let mut response = String::new();

    stream
        .read_to_string(&mut response)
        .expect("Failed to read response");
    if let Some(pos) = response.find("<<<") {
        println!("{}", &response[..pos+3].trim_end());
    } else {
        println!("{}", response);
    }
}

