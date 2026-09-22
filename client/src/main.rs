mod display;

use display::FrameDisplay;
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
            println!("Waiting for stream...");

            let mut display: Option<FrameDisplay> = None;

            loop {
                match read_message(&mut stream) {
                    Ok(Message::Frame { width, height, data }) => {
                        if display.is_none() {
                            display = Some(FrameDisplay::new(width, height)?);
                        }
                        let d = display.as_mut().unwrap();
                        if !d.is_open() {
                            println!("Window closed by user, exiting.");
                            break;
                        }
                        d.render_frame(width, height, &data)?;
                    }
                    Ok(other) => {
                        println!("Unexpected message during stream: {other:?}");
                    }
                    Err(e) => {
                        println!("Host disconnected or read failed: {e}");
                        break;
                    }
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

    Ok(())
}