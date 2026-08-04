use protocol::{read_message, write_message, Message};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let host_addr = "127.0.0.1:7878"; // swap for the host's LAN IP once testing across machines
    let mut stream = TcpStream::connect(host_addr)?;
    println!("Connected to host at {host_addr}");

    let pin = "1234".to_string(); // later: read from stdin
    write_message(&mut stream, &Message::AuthRequest { pin })?;

    match read_message(&mut stream)? {
        Message::AuthResult { success } => {
            println!("Auth result: {success}");
        }
        other => {
            println!("Unexpected message: {other:?}");
        }
    }

    Ok(())
}