// bind udp server
// nc -lup 4444

use std::net::UdpSocket;

fn main() -> Result<(), std::io::Error> {
    let addr = "127.0.0.1";
    let port = 4444;
    let target = format!("{}:{}", addr, port);
    //let target = format!("{}:{}", addr, port).to_socket_addrs()?.next().unwrap();

    println!("[*] target: {}", target);

    // pub fn bind<A: ToSocketAddrs>(addr: A) -> Result<UdpSocket>
    let socket = UdpSocket::bind("127.0.0.1:5555")?;

    // pub fn send(&self, buf: &[u8]) -> Result<usize>
    // pub fn connect<A: ToSocketAddrs>(&self, addr: A) -> Result<()>

    // pub fn send_to<A: ToSocketAddrs>(&self, buf: &[u8], addr: A) -> Result<usize>
    let socket_size = socket.send_to(b"Hello World!\n", "127.0.0.1:4444")?;
    println!("{:?}", socket_size);

    Ok(())
}
