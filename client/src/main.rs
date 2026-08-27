use protocol::{read_message, write_message, Message};
use std::io::{self, Write};
use std::net::TcpStream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("Enter host IP and port (e.g. 192.168.1.47:7878): ");
    io::stdout().flush()?;
    let mut host_addr = String::new();
    io::stdin().read_line(&mut host_addr)?;
    let host_addr = host_addr.trim();

    let mut stream = TcpStream::connect(host_addr)?;
    println!("Connected to host at {host_addr}");

    print!("Enter PIN: ");
    io::stdout().flush()?;
    let mut pin = String::new();
    io::stdin().read_line(&mut pin)?;
    let pin = pin.trim().to_string();

    write_message(&mut stream, &Message::AuthRequest { pin })?;

    match read_message(&mut stream)? {
        Message::AuthResult { success: true } => {
            println!("Auth result: true");
            println!("Waiting for frame...");

            match read_message(&mut stream)? {
                Message::Frame { width, height, data } => {
                    println!(
                        "Received frame: {}x{}, {} bytes",
                        width,
                        height,
                        data.len()
                    );
                }
                other => {
                    println!("Expected a Frame message, got: {other:?}");
                }
            }
        }
        Message::AuthResult { success: false } => {
            println!("Auth result: false — PIN rejected");
        }
        other => {
            println!("Unexpected message: {other:?}");
        }
    }

    println!("Press Enter to exit...");
    let mut _pause = String::new();
    io::stdin().read_line(&mut _pause)?;

    Ok(())
}