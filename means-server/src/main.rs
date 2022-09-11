use std::collections::BTreeMap;
use std::io::{self, Error, ErrorKind};
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::ops::Bound::Included;
use std::thread;

#[derive(Debug)]
enum Message {
    Insert { ts: i32, price: i32 },
    Query { min_time: i32, max_time: i32 },
}

impl Message {
    fn from_slice(buffer: &[u8]) -> io::Result<Message> {
        if buffer.len() != 9 {
            return Err(Error::new(ErrorKind::Other, "Invalid message length"));
        }
        match buffer.get(0) {
            Some(b'I') => Ok(Message::Insert {
                ts: i32::from_be_bytes([buffer[1], buffer[2], buffer[3], buffer[4]]),
                price: i32::from_be_bytes([buffer[5], buffer[6], buffer[7], buffer[8]]),
            }),
            Some(b'Q') => Ok(Message::Query {
                min_time: i32::from_be_bytes([buffer[1], buffer[2], buffer[3], buffer[4]]),
                max_time: i32::from_be_bytes([buffer[5], buffer[6], buffer[7], buffer[8]]),
            }),
            _ => Err(Error::new(ErrorKind::Other, "Invalid message type")),
        }
    }
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let addr = stream.peer_addr()?;
    let mut map: BTreeMap<i32, i32> = BTreeMap::new();
    let mut buffer: Vec<u8> = vec![0u8; 9];
    println!("{addr}: connected");
    loop {
        if let Err(e) = stream.read_exact(&mut buffer) {
            let kind = e.kind();
            println!("{kind}: {e}");
            if kind == io::ErrorKind::UnexpectedEof {
                break;
            } else {
                continue;
            }
        }
        let msg = Message::from_slice(&buffer);
        if msg.is_err() {
            continue;
        }
        match msg.unwrap() {
            Message::Insert { ts, price } => {
                map.insert(ts, price);
            }
            Message::Query { min_time, max_time } => {
                let mut n = 0i64;
                let mut sum = 0i64;
                if min_time > max_time {
                    stream.write_all(b"\x00\x00\x00\x00")?;
                    continue;
                }
                for (&_key, &val) in map.range((Included(min_time), Included(max_time))) {
                    n += 1;
                    sum += val as i64;
                }
                let result = (sum / if n > 0 { n } else { 1 }) as i32;
                stream.write_all(&result.to_be_bytes())?
            }
        }
    }
    stream.shutdown(Shutdown::Both)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:1234")?;
    for client in listener.incoming() {
        if let Ok(client) = client {
            thread::spawn(move || {
                if let Err(e) = handle_client(client) {
                    println!("{e}");
                }
            });
        }
    }
    Ok(())
}
