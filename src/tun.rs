use crate::interface::Interface;
#[cfg(target_os = "linux")]
use crate::linux::ifreq::{ifreq, tunsetiff};
use crate::params::Params;
use crate::result::Result;
use async_std::fs::File;
#[cfg(target_os = "linux")]
use async_std::fs::OpenOptions;
#[cfg(target_os = "linux")]
use async_std::os::unix::io::{AsRawFd, FromRawFd};
#[cfg(target_os = "linux")]
use std::ops::{Deref, DerefMut};

/// Represents a Tun/Tap device. Use [`TunBuilder`](struct.TunBuilder.html) to create a new instance of [`Tun`](struct.Tun.html).
pub struct Tun {
    file: File,
    name: String,
}

impl Tun {
    #[cfg(target_os = "linux")]
    async fn alloc(params: Params) -> Result<(File, String)> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/net/tun")
            .await?;
        let iface = ifreq::new(params)?;
        unsafe { tunsetiff(file.as_raw_fd(), &iface as *const _ as _) }?;
        Ok((file, iface.name()))
    }

    #[cfg(not(any(target_os = "linux")))]
    async fn alloc(params: Params) -> Result<Self> {
        unimplemented!()
    }

    /// Creates a new instance of Tun/Tap device.
    pub(super) async fn new(params: Params) -> Result<Self> {
        let (file, name) = Self::alloc(params).await?;
        Ok(Self { file, name })
    }

    /// Returns the name of Tun/Tap device.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl Clone for Tun {
    #[cfg(target_os = "linux")]
    fn clone(&self) -> Self {
        Self {
            file: unsafe { File::from_raw_fd(self.file.as_raw_fd()) },
            name: self.name.clone(),
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
