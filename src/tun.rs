#[cfg(target_os = "linux")]
use crate::linux::interface::Interface;
#[cfg(target_os = "linux")]
use crate::linux::params::Params;
use crate::result::Result;
use async_std::fs::File;
use async_std::fs::OpenOptions;
#[cfg(target_os = "linux")]
use async_std::os::unix::io::{AsRawFd, FromRawFd};
use std::net::Ipv4Addr;
use std::ops::{Deref, DerefMut};

/// Represents a Tun/Tap device. Use [`TunBuilder`](struct.TunBuilder.html) to create a new instance of [`Tun`](struct.Tun.html).
pub struct Tun {
    file: File,
    iface: Interface,
}

impl Tun {
    #[cfg(target_os = "linux")]
    async fn alloc(params: Params) -> Result<(File, Interface)> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/net/tun")
            .await?;
        let iface = Interface::new(
            file.as_raw_fd(),
            params.name.as_deref().unwrap_or_default(),
            params.flags,
        )?;
        if let Some(mtu) = params.mtu {
            iface.mtu(Some(mtu))?;
        }
        if let Some(owner) = params.owner {
            iface.owner(owner)?;
        }
        if let Some(group) = params.group {
            iface.group(group)?;
        }
        if let Some(address) = params.address {
            iface.address(Some(address))?;
        }
        if let Some(netmask) = params.netmask {
            iface.netmask(Some(netmask))?;
        }
        if let Some(destination) = params.destination {
            iface.destination(Some(destination))?;
        }
        if let Some(broadcast) = params.broadcast {
            iface.broadcast(Some(broadcast))?;
        }
        if params.persist {
            iface.persist()?;
        }
        if params.up {
            iface.flags(Some(libc::IFF_UP as i16 | libc::IFF_RUNNING as i16))?;
        }
        Ok((file, iface))
    }

    #[cfg(not(any(target_os = "linux")))]
    async fn alloc(params: Params) -> Result<Self> {
        unimplemented!()
    }

    /// Creates a new instance of Tun/Tap device.
    pub(crate) async fn new(params: Params) -> Result<Self> {
        let (file, iface) = Self::alloc(params).await?;
        Ok(Self { file, iface })
    }

    /// Returns the name of Tun/Tap device.
    pub fn name(&self) -> &str {
        self.iface.name()
    }

    /// Returns the value of MTU.
    pub fn mtu(&self) -> Result<i32> {
        self.iface.mtu(None)
    }

    /// Returns the IPv4 address of MTU.
    pub fn address(&self) -> Result<Ipv4Addr> {
        self.iface.address(None)
    }

    /// Returns the IPv4 destination address of MTU.
    pub fn destination(&self) -> Result<Ipv4Addr> {
        self.iface.destination(None)
    }

    /// Returns the IPv4 broadcast address of MTU.
    pub fn broadcast(&self) -> Result<Ipv4Addr> {
        self.iface.broadcast(None)
    }

    /// Returns the IPv4 netmask address of MTU.
    pub fn netmask(&self) -> Result<Ipv4Addr> {
        self.iface.netmask(None)
    }

    /// Returns the flags of MTU.
    pub fn flags(&self) -> Result<i16> {
        self.iface.flags(None)
    }
}

impl Clone for Tun {
    #[cfg(target_os = "linux")]
    fn clone(&self) -> Self {
        Self {
            file: unsafe { File::from_raw_fd(self.file.as_raw_fd()) },
            iface: self.iface.clone(),
        }
    }

    #[cfg(not(any(target_os = "linux")))]
    fn clone(&self) -> Self {
        unimplemented!()
    }
}

impl Deref for Tun {
    type Target = File;

    fn deref(&self) -> &Self::Target {
        &self.file
    }
}

impl DerefMut for Tun {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.file
    }
}
