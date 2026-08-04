// use protocol::{read_message, write_message, Message};
// use std::net::TcpListener;

// fn main() -> std::io::Result<()> {
//     let listener = TcpListener::bind("0.0.0.0:7878")?;
//     println!("Host listening on port 7878...");

//     let (mut stream, addr) = listener.accept()?;
//     println!("Client connected from {addr}");

//     let expected_pin = "1234"; // hardcoded for now, will be randomly generated later

//     match read_message(&mut stream)? {
//         Message::AuthRequest { pin } => {
//             let success = pin == expected_pin;
//             println!("Auth attempt with pin '{pin}' -> success: {success}");
//             write_message(&mut stream, &Message::AuthResult { success })?;
//         }
//         other => {
//             println!("Unexpected message: {other:?}");
//         }
//     }

//     Ok(())
// }

// mod capture;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let frame = capture::capture_primary_monitor()?;
//     println!(
//         "Captured frame: {}x{}, JPEG size: {} bytes",
//         frame.width,
//         frame.height,
//         frame.jpeg_data.len()
//     );
//     Ok(())
// }

mod capture;
use xcap::Monitor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let monitors = Monitor::all()?;
    println!("Detected {} monitor(s):", monitors.len());
    for m in &monitors {
        println!(
            "  name={:?} primary={} width={} height={}",
            m.name(),
            m.is_primary(),
            m.width(),
            m.height()
        );
    }

    let frame = capture::capture_primary_monitor()?;
    println!(
        "Captured frame: {}x{}, JPEG size: {} bytes",
        frame.width,
        frame.height,
        frame.jpeg_data.len()
    );
    Ok(())
}