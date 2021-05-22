use std::mem;
use std::convert::TryInto;
use libc as c;

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


fn main() {
    let mut buf: [u8; 65536] = [0; 65536];

    unsafe {
        //
        let socket = c::socket(c::AF_INET, c::SOCK_RAW, c::IPPROTO_TCP);

        let mut servaddr = c::sockaddr_in {
            sin_family: c::AF_INET as u16,
            sin_port: 0u16.to_be(),
            sin_addr: c::in_addr {
                s_addr: u32::from_be_bytes([0, 0, 0, 0]).to_be()
            },
            sin_zero: mem::zeroed()
        };
    
        let mut addr_buf_sz: c::socklen_t = mem::size_of_val(&servaddr) as c::socklen_t;
    
        let _result = c::bind(socket, &servaddr as *const c::sockaddr_in as *const c::sockaddr, addr_buf_sz);

        let opt: c::c_int = 1;
        let opt_len = mem::size_of_val(&opt) as c::socklen_t;
        c::setsockopt(socket, c::IPPROTO_IP, c::IP_HDRINCL, &opt as *const _ as *const libc::c_void, opt_len);
    
        let addr_ptr = mem::transmute::<&mut c::sockaddr_in, &mut c::sockaddr>(&mut servaddr);

        loop {
            let nbytes = c::recvfrom(socket, buf.as_mut_ptr() as *mut c::c_void, buf.len(), 0, addr_ptr as &mut c::sockaddr, &mut addr_buf_sz);
            hex_dump((nbytes as isize).try_into().unwrap(), &buf);
            ip_dump(&buf);
        }
    }
}
