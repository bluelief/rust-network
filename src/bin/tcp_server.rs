use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn hex_dump(size: usize, source: &[u8]) {
    for x in source.iter().take(size) {
        print!("{:02x} ", x);
    }
    println!("");
}

fn handler(mut stream: TcpStream) -> Result<(), Error> {
    println!("[*] Connection from {}", stream.peer_addr().unwrap());
    let mut buf = [0u8; 1024];
    loop {
        let nbytes = stream.read(&mut buf)?;
        if nbytes == 0 {
            return Ok(());
        }
        print!("[*] Size: {} buf len: {} Message: {}", nbytes, &buf.len(), std::str::from_utf8(&buf).unwrap());
        hex_dump(nbytes, &buf);
        stream.write(&buf)?;
        stream.flush()?;
        buf = [0u8; 1024];
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
