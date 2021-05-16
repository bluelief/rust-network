use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handler(mut stream: TcpStream) -> Result<(), Error> {
    println!("[*] Connection from {}", stream.peer_addr().unwrap());
    let mut buf = [0u8; 1024];
    loop {
        let nbytes = stream.read(&mut buf)?;
        if nbytes == 0 {
            return Ok(());
        }
        print!("[*] Message: {}", std::str::from_utf8(&buf).unwrap());
        stream.write(&buf)?;
        stream.flush()?;
    }
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:4444").expect("Cannot bind to port 4444.");
    println!("[*] Listening on port 4444...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handler(stream).unwrap_or_else(|error| eprintln!("[!] Error: {:?}", error));
                });
            },
            Err(e) => { println!("[!] Error: {}", e) }
        }
    }
}
