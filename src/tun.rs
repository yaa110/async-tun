#[cfg(target_os = "linux")]
use crate::linux::interface::Interface;
#[cfg(target_os = "linux")]
use crate::linux::params::Params;
use crate::result::Result;
use async_std::fs::File;
use async_std::fs::OpenOptions;
#[cfg(target_os = "linux")]
use async_std::os::unix::io::{AsRawFd, FromRawFd};
use async_std::sync::Arc;
use std::net::Ipv4Addr;
use std::ops::{Deref, DerefMut};

/// Represents a Tun/Tap device. Use [`TunBuilder`](struct.TunBuilder.html) to create a new instance of [`Tun`](struct.Tun.html).
pub struct Tun {
    file: File,
    iface: Arc<Interface>,
}

impl Tun {
    #[cfg(target_os = "linux")]
    async fn alloc(params: Params, queues: usize) -> Result<(Vec<File>, Interface)> {
        let mut files = Vec::with_capacity(queues);
        for _ in 0..queues {
            files.push(
                OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open("/dev/net/tun")
                    .await?,
            );
        }
        let iface = Interface::new(
            files.iter().map(|file| file.as_raw_fd()).collect(),
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
        Ok((files, iface))
    }

    #[cfg(not(any(target_os = "linux")))]
    async fn alloc(params: Params) -> Result<Self> {
        unimplemented!()
    }

    /// Creates a new instance of Tun/Tap device.
    pub(crate) async fn new(params: Params) -> Result<Self> {
        let (files, iface) = Self::alloc(params, 1).await?;
        Ok(Self {
            file: files.into_iter().next().unwrap(),
            iface: Arc::new(iface),
        })
    }

    /// Creates a new instance of Tun/Tap device.
    #[cfg(target_os = "linux")]
    pub(crate) async fn new_mq(params: Params, queues: usize) -> Result<Vec<Self>> {
        let (files, iface) = Self::alloc(params, queues).await?;
        let mut tuns = Vec::with_capacity(queues);
        let iface = Arc::new(iface);
        for file in files.into_iter() {
            tuns.push(Self {
                file,
                iface: iface.clone(),
            })
        }
        Ok(tuns)
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
