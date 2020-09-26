use crate::kind::Kind;
use libc::{IFF_NO_PI, IFF_TAP, IFF_TUN};

/// Represents parameters for creating a new Tun/Tap device on Linux.
#[cfg(target_os = "linux")]
pub struct Params {
    pub name: Option<String>,
    pub flags: i16,
}

#[cfg(not(any(target_os = "linux")))]
struct Params;

#[cfg(target_os = "linux")]
impl Params {
    /// Creates a new instance of `Params`.
    /// `name` is the name of device (max length: 16 characters), if it is empty, then device name is set by kernel.
    /// `kind` specifies the type of device (Tun or Tap).
    /// if `packet_info` is false, then `IFF_NO_PI` is set.
    pub fn new(name: &str, kind: Kind, packet_info: bool) -> Self {
        Self {
            name: if name.is_empty() {
                None
            } else {
                Some(name.into())
            },
            flags: {
                let mut flags = if kind == Kind::Tun { IFF_TUN } else { IFF_TAP } as _;
                if !packet_info {
                    flags |= IFF_NO_PI as i16;
                }
                flags
            },
        }
    }
}
