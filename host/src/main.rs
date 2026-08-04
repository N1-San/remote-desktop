use protocol::{read_message, write_message, Message};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:7878")?;
    println!("Host listening on port 7878...");

    let (mut stream, addr) = listener.accept()?;
    println!("Client connected from {addr}");

    let expected_pin = "1234"; // hardcoded for now, will be randomly generated later

    match read_message(&mut stream)? {
        Message::AuthRequest { pin } => {
            let success = pin == expected_pin;
            println!("Auth attempt with pin '{pin}' -> success: {success}");
            write_message(&mut stream, &Message::AuthResult { success })?;
        }
        other => {
            println!("Unexpected message: {other:?}");
        }
    }

    Ok(())
}