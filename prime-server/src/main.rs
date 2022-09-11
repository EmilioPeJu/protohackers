use std::io;
use std::io::BufRead;
use std::io::Write;
use std::net::Shutdown;
use std::net::{TcpListener, TcpStream};
use std::thread;

use json::JsonValue;

fn is_prime(num: f64) -> bool {
    if num.fract() != 0.0 || num.abs() == 1.0 || num <= 0.0 {
        return false;
    }
    let inum: usize = num.abs() as usize;
    let limit = num.abs().sqrt() as usize;
    for i in 2..=limit {
        if inum % i == 0 {
            return false;
        }
    }
    true
}

fn handle_json(stream: &mut TcpStream, value: JsonValue) -> io::Result<()> {
    let method = &value["method"];
    if method == "isPrime" && value["number"].is_number() {
        let number: f64 = value["number"].as_f64().unwrap();
        let result = is_prime(number);
        let msg = format!("{{\"method\":\"isPrime\",\"prime\":{result}}}\n");
        stream.write_all(msg.as_bytes())?;
        Ok(())
    } else {
        stream.write_all("ERR\n".as_bytes())?;
        Err(io::Error::new(io::ErrorKind::Other, "Invalid format"))
    }
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let addr = stream.peer_addr().expect("No addr");
    println!("New client {addr}");
    let buf = io::BufReader::new(stream.try_clone()?);
    for line in buf.lines() {
        match json::parse(&line?) {
            Ok(parsed) => {
                if let Err(e) = handle_json(&mut stream, parsed) {
                    println!("{e}");
                    break;
                }
            }
            Err(e) => {
                println!("{e}");
                stream.write_all("ERR\n".as_bytes())?;
                break;
            }
        }
    }
    println!("Closing {addr}");
    stream.shutdown(Shutdown::Both).unwrap();
    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:1234")?;
    for stream in listener.incoming() {
        let stream = stream?;
        thread::spawn(move || {
            handle_client(stream).unwrap();
        });
    }
    Ok(())
}
