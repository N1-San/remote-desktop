mod capture;

use protocol::{read_message, write_message, Message};
use std::io;
use std::net::TcpListener;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:7878")?;
    println!("Host listening on port 7878...");
    println!("PIN for this session: 1234");

    let (mut stream, addr) = listener.accept()?;
    println!("Client connected from {addr}");

    let expected_pin = "1234";

    match read_message(&mut stream)? {
        Message::AuthRequest { pin } => {
            let success = pin == expected_pin;
            println!("Auth attempt with pin '{pin}' -> success: {success}");
            write_message(&mut stream, &Message::AuthResult { success })?;

            if success {
                println!("Starting frame stream (Ctrl+C to stop)...");
                loop {
                    let frame = capture::capture_primary_monitor()?;
                    let send_result = write_message(
                        &mut stream,
                        &Message::Frame {
                            width: frame.width,
                            height: frame.height,
                            data: frame.jpeg_data,
                        },
                    );

                    if let Err(e) = send_result {
                        println!("Client disconnected or send failed: {e}");
                        break;
                    }

                    std::thread::sleep(Duration::from_millis(100)); // ~10 fps to start
                }
            }
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