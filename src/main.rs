use std::io;
use std::collections::HashMap;
use std::net::Ipv4Addr;

mod tcp;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Quad {
    src: (Ipv4Addr, u16),
    dst: (Ipv4Addr, u16),
}

fn main() -> io::Result<()> {
    let mut connections: HashMap<Quad, tcp::Connection> = Default::default();
    /*
     * Create Virtual Interface.
     */
    let mut nic = tun_tap::Iface::new("tun_0", tun_tap::Mode::Tun)?;
    // 1500 = MTU Usually Packet Size, 4 = Header Size

    // Flags [2 bytes]
    // Proto [2 bytes]
    let mut buf = [0u8; 1504];
    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        let _eth_flag = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);
        // 0x800 is ipv6
        if eth_proto != 0x0800 {
            // ignore ipv6
            continue
        }
        
        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(ip_header) => {
                let src = ip_header.source_addr();
                let dst = ip_header.destination_addr();
                let proto = ip_header.protocol();
                if proto != 0x06 {
                    // not tcp
                    continue
                }

                match etherparse::TcpHeaderSlice::from_slice(&buf[4 + ip_header.slice().len()..nbytes]) {
                    Ok(tcp_header) => {
                        let datai = 4 + ip_header.slice().len() + tcp_header.slice().len();
                        connections.entry(Quad {
                            src: (src, tcp_header.source_port()),
                            dst: (dst, tcp_header.destination_port()),
                        }).or_default().on_packet(&mut nic, ip_header, tcp_header, &buf[datai..nbytes])?;
                        // (srcip, srcport, dstip, dstport)
                    }
                    Err(e) => {
                        eprintln!("ignoring weird tcp packet {:?}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("ignoring weired packet {:?}", e);
            }
        }
    }
    Ok(())
}
