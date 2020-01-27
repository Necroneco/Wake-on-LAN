use eui48::MacAddress;
use std::io;
use std::net::{Ipv4Addr, ToSocketAddrs, UdpSocket};

pub struct MagicPacket {
    pub magic_bytes: [u8; 102],
}

const MAGIC_BYTES_HEADER: [u8; 6] = [0xFF; 6];

impl MagicPacket {
    pub fn new(mac_address: &[u8; 6]) -> MagicPacket {
        let mut magic_bytes = [0u8; 102];
        magic_bytes[0..6].clone_from_slice(&MAGIC_BYTES_HEADER);
        for i in 1..=16 {
            magic_bytes[(i * 6)..(i * 6 + 6)].clone_from_slice(mac_address);
        }
        MagicPacket { magic_bytes }
    }

    pub fn from_str(mac_address: &str) -> MagicPacket {
        MagicPacket::new(
            &MacAddress::parse_str(&mac_address)
                .expect("MAC Address Parse Error")
                .to_array(),
        )
    }

    pub fn send(&self) -> io::Result<()> {
        self.send_to((Ipv4Addr::new(255, 255, 255, 255), 9), (Ipv4Addr::new(0, 0, 0, 0), 0))
    }

    pub fn send_to(&self, to_addr: impl ToSocketAddrs, from_addr: impl ToSocketAddrs) -> io::Result<()> {
        let socket = UdpSocket::bind(from_addr)?;
        socket.set_broadcast(true)?;
        socket.send_to(&self.magic_bytes, to_addr)?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};

    #[test]
    fn magic_bytes() {
        let mut mac_address = [0u8; 6];
        thread_rng().fill(&mut mac_address);
        let mp = MagicPacket::new(&mac_address);
        assert_eq!(mp.magic_bytes[..6], [0xFF; 6]);
        for i in 1..=16 {
            assert_eq!(mp.magic_bytes[(i * 6)..(i * 6 + 6)], mac_address);
        }
    }
}
