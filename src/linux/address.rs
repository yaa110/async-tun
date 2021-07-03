use super::request::sockaddr;
use mac_address::MacAddress;
use std::convert::{TryFrom, TryInto};
use std::mem;
use std::net::Ipv4Addr;

pub trait Ipv4AddrExt {
    fn to_address(&self) -> sockaddr;
    fn from_address(sock: sockaddr) -> Self;
}

fn hton(octets: [u8; 4]) -> u32 {
    (octets[3] as u32) << 24 | (octets[2] as u32) << 16 | (octets[1] as u32) << 8 | octets[0] as u32
}

fn ntoh(number: u32) -> [u8; 4] {
    [
        (number & 0xff) as u8,
        (number >> 8 & 0xff) as u8,
        (number >> 16 & 0xff) as u8,
        (number >> 24 & 0xff) as u8,
    ]
}

impl Ipv4AddrExt for Ipv4Addr {
    fn to_address(&self) -> sockaddr {
        let mut addr: libc::sockaddr_in = unsafe { mem::zeroed() };
        addr.sin_family = libc::AF_INET as _;
        addr.sin_addr = libc::in_addr {
            s_addr: hton(self.octets()),
        };
        addr.sin_port = 0;
        unsafe { mem::transmute(addr) }
    }

    fn from_address(addr: sockaddr) -> Self {
        let sock: libc::sockaddr_in = unsafe { mem::transmute(addr) };
        ntoh(sock.sin_addr.s_addr).into()
    }
}

impl From<MacAddress> for sockaddr {
    fn from(mac_addr: MacAddress) -> Self {
        let mut addr: [std::os::raw::c_char; 14usize] = [0; 14];
        addr[..6].copy_from_slice(unsafe { &*(&mac_addr.bytes()[..] as *const _ as *const _) });
        Self {
            sa_family: libc::ARPHRD_ETHER,
            sa_data: addr,
        }
    }
}

#[derive(Debug, Clone)]
pub enum MacAddressConversionError {
    WrongType(std::os::raw::c_ushort),
    SliceConversion(std::array::TryFromSliceError),
}

impl From<std::array::TryFromSliceError> for MacAddressConversionError {
    fn from(error: std::array::TryFromSliceError) -> Self {
        Self::SliceConversion(error)
    }
}

impl std::fmt::Display for MacAddressConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongType(t) => write!(f, "Incorrect address family: {}", t),
            Self::SliceConversion(e) => write!(f, "Slice conversion failed: {}", e),
        }
    }
}

impl std::error::Error for MacAddressConversionError {}

impl TryFrom<sockaddr> for MacAddress {
    type Error = MacAddressConversionError;

    fn try_from(addr: sockaddr) -> Result<Self, Self::Error> {
        if addr.sa_family != libc::ARPHRD_ETHER {
            return Err(MacAddressConversionError::WrongType(addr.sa_family));
        }

        let addr: &[u8] = unsafe { &*(&addr.sa_data[..6] as *const _ as *const _) };
        Ok(MacAddress::new(addr.try_into()?))
    }
}
