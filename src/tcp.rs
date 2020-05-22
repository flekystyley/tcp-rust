use std::io;

pub enum State {
    Closed,
    Listen,
    // SynRcvd,
    // Estab,
}

pub struct Connection {
    state: State,
}

/// State of the Send Sequence Space (RFC 793 S3.2 F4)
///
/// ```
///         1         2          3          4
///   ----------|----------|----------|----------
///            SND.UNA    SND.NXT    SND.UNA
///                                 +SND.WND
/// 1 - old sequence numbers which ehave been acknowledged
/// 2 - sequence numbers of unacknowledged data
/// 3 - sequence numbers allowed for new data transmission
/// 4 - future sequence numbers which are not yet allowed
/// ```
struct SendSequence {
    /// send unacknowledged
    una: usize,
    /// send next
    next: usize,
    /// send window
    window: usize,
    /// send urgent pointer
    up: bool,
    /// segment sequence number used for last window update
    wl1: usize,
    /// segment acknowledgment number used for last window update
    wl2: usize,
    /// initial send sequence number
    iss: usize,
}

struct ReceiveSequnece {
    /// receive next
    next: usize,
    /// receive window
    window: usize,
    /// receive urgent pointer
    up: bool,
    /// initial receive sequence number
    irs: usize,
}

impl Default for Connection {
    fn default() -> Self {
        Connection {
            state: State::Listen,
        }
    }
}

impl State {
    pub fn on_packet<'a>(
        &mut self,
        nic: &mut tun_tap::Iface,
        ip_header: etherparse::Ipv4HeaderSlice<'a>, 
        tcp_header: etherparse::TcpHeaderSlice<'a>, 
        data: &'a [u8]
    ) -> io::Result<usize>{
        let mut buf = [0u8; 1500];
        match *self {
            State::Closed => {
                return Ok(0);
            }
            State::Listen => {
                if !tcp_header.syn() {
                    return Ok(0);
                }

                // need to start establishing a connection
                let mut syn_ack = etherparse::TcpHeader::new(
                    tcp_header.destination_port(), 
                    tcp_header.source_port(),
                    unimplemented!(),
                    unimplemented!(),
                );
                syn_ack.syn = true; 
                syn_ack.ack = true;
                let mut ip = etherparse::Ipv4Header::new(
                    syn_ack.header_len(),
                    64,
                    etherparse::IpTrafficClass::Tcp, 
                    [
                        ip_header.destination()[0],
                        ip_header.destination()[1],
                        ip_header.destination()[2],                        
                        ip_header.destination()[3],                        
                    ],
                    [
                        ip_header.source()[0],
                        ip_header.source()[1],
                        ip_header.source()[2],                        
                        ip_header.source()[3],                        
                    ],
                );
                let unwritten = {
                    let mut unwritten = &mut buf[..];                
                    ip.write(&mut unwritten);
                    syn_ack.write(&mut unwritten);
                    unwritten.len()
                };
                nic.send(&buf[..unwritten]);
            }
        }
    }
}