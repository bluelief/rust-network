use std::mem;
use std::io;
use libc;

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

fn network_addr_to_sockaddr(
    storage: *mut libc::sockaddr_storage,
    proto: libc::c_int,
) -> usize {
    unsafe {
        let sll: *mut libc::sockaddr_ll = mem::transmute(storage);
        (*sll).sll_family = libc::AF_PACKET as libc::sa_family_t;
        (*sll).sll_addr = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0, 0];
        (*sll).sll_protocol = (proto as u16).to_be();
        (*sll).sll_halen = 6;
        (*sll).sll_ifindex = 4;
        mem::size_of::<libc::sockaddr_ll>()
    }
}

fn recv_from(
    socket: libc::c_int,
    buffer: &mut [u8],
    caddr: *mut libc::sockaddr_storage,
) -> io::Result<usize> {
    let mut caddrlen = mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
    let len = unsafe { libc::recvfrom(socket, buffer.as_ptr() as *mut libc::c_void, buffer.len() as libc::size_t, 0, caddr as *mut libc::sockaddr, &mut caddrlen) };

    if len < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(len as usize)
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut buf: [u8; 65536] = [0; 65536];

    unsafe {
        //
        let socket = libc::socket(libc::AF_PACKET, libc::SOCK_RAW, libc::ETH_P_ALL.to_be() as i32);
        if socket == -1 {
            println!("err");
            return Err(io::Error::last_os_error());
        }
        let mut addr: libc::sockaddr_storage = mem::zeroed();
        let len = network_addr_to_sockaddr(&mut addr, libc::ETH_P_ALL as i32);
    
        let send_addr = (&addr as *const libc::sockaddr_storage) as *const libc::sockaddr;
    
        libc::bind(socket, send_addr, len as libc::socklen_t);

        //let opt = "eth0";
        //let opt_len = mem::size_of_val(&opt) as c::socklen_t;
        //c::setsockopt(socket, c::SOL_PACKET, c::SO_BINDTODEVICE, &opt as *const _ as *const libc::c_void, opt_len);
    
        //let addr_ptr = mem::transmute::<&mut c::sockaddr_in, &mut c::sockaddr>(&mut servaddr);

        let mut caddr: libc::sockaddr_storage = mem::zeroed();
    
        //let addr_ptr = mem::transmute::<&mut libc::sockaddr_storage, &mut libc::sockaddr>(&mut caddr);
        
        //let nbytes = libc::recvfrom(socket, buf.as_mut_ptr() as *mut libc::c_void, buf.len() as libc::size_t, 0, caddr as *mut libc::sockaddr, &mut caddrlen);
        loop {
            let nbytes = recv_from(socket, &mut buf, &mut caddr);
            println!("{}", nbytes.as_ref().unwrap());
            hex_dump(nbytes.unwrap(), &buf);
            ip_dump(&buf[14..]);
        }
    }
}
