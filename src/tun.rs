#[cfg(target_os = "linux")]
use crate::linux::interface::Interface;
#[cfg(target_os = "linux")]
use crate::linux::params::Params;
use crate::result::Result;
use async_std::fs::File;
use async_std::fs::OpenOptions;
use async_std::io::{BufReader, BufWriter};
#[cfg(target_family = "unix")]
use async_std::os::unix::io::{AsRawFd, RawFd};
use async_std::sync::Arc;
use std::net::Ipv4Addr;

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
        let file = files.into_iter().next().unwrap();
        Ok(Self {
            file: file,
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

    /// Splits self to reader and writer pairs.
    pub fn split(&self) -> (BufReader<&File>, BufWriter<&File>) {
        (BufReader::new(&self.file), BufWriter::new(&self.file))
    }

    /// Returns a reader to read from tun.
    pub fn reader(&self) -> BufReader<&File> {
        BufReader::new(&self.file)
    }

    /// Returns a writer to write to tun.
    pub fn writer(&self) -> BufWriter<&File> {
        BufWriter::new(&self.file)
    }
}

#[cfg(target_family = "unix")]
impl AsRawFd for Tun {
    fn as_raw_fd(&self) -> RawFd {
        self.file.as_raw_fd()
    }
}
