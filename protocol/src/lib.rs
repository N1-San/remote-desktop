use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Frame { width: u32, height: u32, data: Vec<u8> },
    Input(InputEvent),
    AuthRequest { pin: String },
    AuthResult { success: bool },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum InputEvent {
    MouseMove { x: i32, y: i32 },
    MouseButton { button: MouseButton, pressed: bool },
    KeyPress { keycode: u32, pressed: bool },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

pub fn write_message<W: Write>(writer: &mut W, msg: &Message) -> io::Result<()> {
    let encoded = bincode::serialize(msg)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let len = encoded.len() as u32;
    writer.write_all(&len.to_be_bytes())?;
    writer.write_all(&encoded)?;
    Ok(())
}

pub fn read_message<R: Read>(reader: &mut R) -> io::Result<Message> {
    let mut len_buf = [0u8; 4];
    reader.read_exact(&mut len_buf)?;
    let len = u32::from_be_bytes(len_buf) as usize;

    let mut data = vec![0u8; len];
    reader.read_exact(&mut data)?;

    bincode::deserialize(&data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn roundtrip_auth_request() {
        let mut buf = Vec::new();
        let msg = Message::AuthRequest { pin: "1234".to_string() };
        write_message(&mut buf, &msg).unwrap();

        let mut cursor = Cursor::new(buf);
        let decoded = read_message(&mut cursor).unwrap();

        match decoded {
            Message::AuthRequest { pin } => assert_eq!(pin, "1234"),
            _ => panic!("wrong variant decoded"),
        }
    }

    #[test]
    fn roundtrip_input_event() {
        let mut buf = Vec::new();
        let msg = Message::Input(InputEvent::MouseMove { x: 100, y: 200 });
        write_message(&mut buf, &msg).unwrap();

        let mut cursor = Cursor::new(buf);
        let decoded = read_message(&mut cursor).unwrap();

        match decoded {
            Message::Input(InputEvent::MouseMove { x, y }) => {
                assert_eq!(x, 100);
                assert_eq!(y, 200);
            }
            _ => panic!("wrong variant decoded"),
        }
    }
}