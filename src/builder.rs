use super::params::Params;
use super::result::Result;
use super::tun::Tun;
use core::convert::From;
use libc::{IFF_NO_PI, IFF_TAP, IFF_TUN};

/// Represents a factory to build new instances of [`Tun`](struct.Tun.html).
pub struct TunBuilder<'a> {
    name: &'a str,
    is_tap: bool,
    packet_info: bool,
}

impl<'a> TunBuilder<'a> {
    /// Creates a new instance of [`TunBuilder`](struct.TunBuilder.html).
    pub fn new() -> Self {
        Self {
            name: "",
            is_tap: false,
            packet_info: true,
        }
    }

    /// Sets the name of device (max length: 16 characters), if it is empty, then device name is set by kernel. Default value is empty.
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = name;
        self
    }

    /// If `is_tap` is true, a TAP device is allocated, otherwise, a TUN device is created. Default value is `false`.
    pub fn tap(mut self, is_tap: bool) -> Self {
        self.is_tap = is_tap;
        self
    }

    /// If `packet_info` is false, then `IFF_NO_PI` flag is set. Default value is `true`.
    pub fn packet_info(mut self, packet_info: bool) -> Self {
        self.packet_info = packet_info;
        self
    }

    /// Builds a new instance of [`Tun`](struct.Tun.html).
    pub async fn try_build(self) -> Result<Tun> {
        Tun::new(self.into()).await
    }
}

impl<'a> From<TunBuilder<'a>> for Params {
    #[cfg(target_os = "linux")]
    fn from(builder: TunBuilder) -> Self {
        Params {
            name: if builder.name.is_empty() {
                None
            } else {
                Some(builder.name.into())
            },
            flags: {
                let mut flags = if builder.is_tap { IFF_TAP } else { IFF_TUN } as _;
                if !builder.packet_info {
                    flags |= IFF_NO_PI as i16;
                }
                flags
            },
        }
    }

    #[cfg(not(any(target_os = "linux")))]
    fn from(builder: TunBuilder) -> Self {
        unimplemented!()
    }
}
