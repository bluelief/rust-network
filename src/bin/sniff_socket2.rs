use std::net::SocketAddr;
use socket2::{Socket, Domain, Protocol, Type};
use std::io::{Error};
use std::mem::MaybeUninit;
use std::slice;

fn hex_dump(size: usize, source: &[u8]) {
    for x in source.iter().take(size) {
        print!("{:02x} ", x);
    }
    println!("");
}

fn ip_dump(ip_header: &[u8]) {
    let ip_version = (ip_header[0] & 0b11110000) >> 4;
    let ip_ihl = (ip_header[0] & 0b00001111) * 4;
    println!("ip_version: {} ip_ihl: {}", ip_version, ip_ihl);

    let ip_tos = ip_header[1];
    let ip_len = (ip_header[2] << 4) + ip_header[3];
    println!("type of service: {} ip total length: {}", ip_tos, ip_len);

    let ip_id = (ip_header[4] << 4) + ip_header[5];
    let ip_flag = (((ip_header[6] << 4) + ip_header[7]) as u16 & 0b1110000000000000) >> 13;
    let ip_offset = ((ip_header[6] << 4) + ip_header[7]) as u16 & 0b0001111111111111;
    println!("id: {} flag: {} offset: {}", ip_id, ip_flag, ip_offset);

    let ip_ttl = ip_header[8];
    let ip_protocol = ip_header[9];
    let ip_sum = ((ip_header[10] << 4) + ip_header[11]) as u16;
    println!("Time to Live: {} Protocol: {} Checksum: {}", ip_ttl, ip_protocol, ip_sum);

    let ip_src = format!("{}.{}.{}.{}", ip_header[12], ip_header[13], ip_header[14], ip_header[15]);
    let ip_dst = format!("{}.{}.{}.{}", ip_header[16], ip_header[17], ip_header[18], ip_header[19]);
    println!("src: {} dst: {}", ip_src, ip_dst);
}

#[warn(unreachable_code)]
fn main() -> Result<(), Error> {
    let mut buf = [MaybeUninit::<u8>::uninit(); 65536];

    let socket = Socket::new_raw(Domain::IPV4, Type::RAW, Some(Protocol::TCP))?;

    let address: SocketAddr = "0.0.0.0:0".parse().unwrap();
    socket.bind(&address.into())?;
    // let nbytes: usize = match socket.recv(&mut buf) { //Some thing }
    loop {
        let (nbytes, _) = match socket.recv_from(&mut buf) {
            Ok(nbytes) => nbytes,
            Err(e) => return Err(e),
        };
        println!("{}", nbytes);
        let result = unsafe { slice::from_raw_parts(buf.as_mut_ptr() as *mut u8, nbytes) };
        println!("{:?}", result);
        hex_dump(nbytes, &result);
        ip_dump(&result);
    }
}