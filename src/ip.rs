use std::net::Ipv4Addr;

use num_enum::TryFromPrimitive;

pub const MTU: usize = 1350;

#[repr(u8)]
pub enum IpType {
    V4 = 0b0100,
    // V6,
}

impl TryFrom<u8> for IpType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let value = value >> 4;
        match value {
            v if v == Self::V4 as u8 => Ok(Self::V4),
            _ => Err(()),
        }
    }
}

#[derive(Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum ProtoType {
    ICMP = 1,
    IGMP = 2,
    TCP = 6,
    UDP = 17,
}

#[repr(packed)]
pub struct IPv4 {
    // 4+4
    pub version: u8,
    pub dsfield: u8,
    pub len: u16,
    pub id: u16,
    // 3+13
    pub flags: u16,
    pub ttl: u8,
    // (!)
    pub proto: u8,
    pub checksum: u16,
    // (!)
    pub src: u32,
    // (!)
    pub dst: u32,

    pub data: [u8; MTU - 20],
}

impl IPv4 {
    pub fn print(&self) {
        print!("ip.proto=");
        let proto = ProtoType::try_from(self.proto);
        match proto {
            Ok(v) => print!("{:?}", v),
            Err(_) => print!("UNKNOWN({})", self.proto),
        }

        let ip = Ipv4Addr::from_bits(u32::swap_bytes(self.src));
        print!(" ip.src={}", ip);

        let ip = Ipv4Addr::from_bits(u32::swap_bytes(self.dst));
        println!(" ip.dst={}", ip);
    }
}
