use std::io::prelude::*;
use std::net::TcpStream;
use std::net::ToSocketAddrs;

fn main() -> Result<(), std::io::Error> {
    let addr = "www.google.co.jp";
    let port = 80;
    let target = format!("{}:{}", addr, port).to_socket_addrs()?.next().unwrap();
    //let target = format!("{}:{}", addr, port);

    println!("{}", target);

    // pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<TcpStream>
    let mut response = [0u8; 4096];
    let mut stream = TcpStream::connect(target)?;

    println!("{}", stream.peer_addr()?);

    // fn write_all(&mut self, buf: &[u8]) -> Result<()>
    // fn read(&mut self, buf: &mut [u8]) -> Result<usize>
    stream.write_all(b"GET / HTTP/1.1\r\nHost: google.com\r\n\r\n")?;
    stream.read(&mut response)?;

    println!("{}", std::str::from_utf8(&response).unwrap());

    Ok(())
}
