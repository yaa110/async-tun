use super::request::ifreq;
use crate::linux::address::Ipv4AddrExt;
use crate::result::Result;
use std::net::Ipv4Addr;

nix::ioctl_write_int!(tunsetiff, b'T', 202);
nix::ioctl_write_int!(tunsetpersist, b'T', 203);
nix::ioctl_write_int!(tunsetowner, b'T', 204);
nix::ioctl_write_int!(tunsetgroup, b'T', 206);
nix::ioctl_write_ptr_bad!(siocsifmtu, libc::SIOCSIFMTU, ifreq);
nix::ioctl_write_ptr_bad!(siocsifflags, libc::SIOCSIFFLAGS, ifreq);
nix::ioctl_write_ptr_bad!(siocsifaddr, libc::SIOCSIFADDR, ifreq);
nix::ioctl_write_ptr_bad!(siocsifdstaddr, libc::SIOCSIFDSTADDR, ifreq);
nix::ioctl_read_bad!(siocgifmtu, libc::SIOCGIFMTU, ifreq);
nix::ioctl_read_bad!(siocgifflags, libc::SIOCGIFFLAGS, ifreq);
nix::ioctl_read_bad!(siocgifaddr, libc::SIOCGIFADDR, ifreq);
nix::ioctl_read_bad!(siocgifdstaddr, libc::SIOCGIFDSTADDR, ifreq);

#[derive(Clone)]
pub struct Interface {
    fd: i32,
    socket: i32,
    name: String,
}

impl Interface {
    pub fn new(fd: i32, name: &str, flags: i16) -> Result<Self> {
        let mut req = ifreq::new(name);
        req.ifr_ifru.ifru_flags = flags;
        unsafe { tunsetiff(fd, &req as *const _ as _) }?;
        Ok(Interface {
            fd,
            socket: unsafe { libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0) },
            name: req.name(),
        })
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn mtu(&self, mtu: Option<i32>) -> Result<i32> {
        let mut req = ifreq::new(self.name());
        if let Some(mtu) = mtu {
            req.ifr_ifru.ifru_mtu = mtu;
            unsafe { siocsifmtu(self.socket, &req) }?;
        } else {
            unsafe { siocgifmtu(self.socket, &mut req) }?;
        }
        Ok(unsafe { req.ifr_ifru.ifru_mtu })
    }

    pub fn address(&self, address: Option<Ipv4Addr>) -> Result<Ipv4Addr> {
        let mut req = ifreq::new(self.name());
        if let Some(address) = address {
            req.ifr_ifru.ifru_addr = address.to_address();
            unsafe { siocsifaddr(self.socket, &req) }?;
            return Ok(address);
        }
        unsafe { siocgifaddr(self.socket, &mut req) }?;
        Ok(unsafe { Ipv4Addr::from_address(req.ifr_ifru.ifru_addr) })
    }

    pub fn destination(&self, dst: Option<Ipv4Addr>) -> Result<Ipv4Addr> {
        let mut req = ifreq::new(self.name());
        if let Some(dst) = dst {
            req.ifr_ifru.ifru_dstaddr = dst.to_address();
            unsafe { siocsifdstaddr(self.socket, &req) }?;
            return Ok(dst);
        }
        unsafe { siocgifdstaddr(self.socket, &mut req) }?;
        Ok(unsafe { Ipv4Addr::from_address(req.ifr_ifru.ifru_dstaddr) })
    }

    pub fn flags(&self, flags: Option<i16>) -> Result<i16> {
        let mut req = ifreq::new(self.name());
        unsafe { siocgifflags(self.socket, &mut req) }?;
        if let Some(flags) = flags {
            unsafe { req.ifr_ifru.ifru_flags |= flags };
            unsafe { siocsifflags(self.socket, &req) }?;
        }
        Ok(unsafe { req.ifr_ifru.ifru_flags })
    }

    pub fn owner(&self, owner: i32) -> Result<()> {
        unsafe { tunsetowner(self.fd, owner as _) }?;
        Ok(())
    }

    pub fn group(&self, group: i32) -> Result<()> {
        unsafe { tunsetgroup(self.fd, group as _) }?;
        Ok(())
    }

    pub fn persist(&self) -> Result<()> {
        unsafe { tunsetpersist(self.fd, 1) }?;
        Ok(())
    }
}

impl Drop for Interface {
    fn drop(&mut self) {
        unsafe { libc::close(self.socket) };
    }
}
