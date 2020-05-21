use std::io;

fn main() -> io::Result<()> {
    /*
     * Create Virtual Interface.
     */
    let nic = tun_tap::Iface::new("tun_0", tun_tap::Mode::Tun)?;
    // 1500 = MTU Usually Packet Size, 4 = Header Size

    // Flags [2 bytes]
    // Proto [2 bytes]
    let mut buf = [0u8; 1504];
    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        let flags = u16::from_be_bytes([buf[0], buf[1]]);
        let proto = u16::from_be_bytes([buf[2], buf[3]]);
        if proto != 0x0800 {
            // ignore ipv6
            continue
        }
        eprintln!(
            "read {} bytes (flags : {:x}, proto: {:x}): {:x?}",
            nbytes - 4,
            flags,
            proto,
            &buf[4..nbytes]
        );
    }
    Ok(())
}
