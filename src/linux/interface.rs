use super::request::ifreq;
use crate::result::Result;

nix::ioctl_write_int!(tunsetiff, b'T', 202);
nix::ioctl_write_int!(tunsetowner, b'T', 204);
nix::ioctl_write_int!(tunsetgroup, b'T', 206);
nix::ioctl_write_ptr_bad!(siocsifmtu, libc::SIOCSIFMTU, ifreq);
nix::ioctl_read_bad!(siocgifmtu, libc::SIOCGIFMTU, ifreq);

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

    pub fn mtu(&self) -> Result<i32> {
        let mut req = ifreq::new(self.name());
        unsafe { siocgifmtu(self.socket, &mut req) }?;
        Ok(unsafe { req.ifr_ifru.ifru_mtu })
    }

    pub fn owner(&self, owner: i32) -> Result<()> {
        unsafe { tunsetowner(self.fd, owner as _) }?;
        Ok(())
    }

    pub fn group(&self, group: i32) -> Result<()> {
        unsafe { tunsetgroup(self.fd, group as _) }?;
        Ok(())
    }

    pub fn set_mtu(&mut self, mtu: i32) -> Result<()> {
        let mut req = ifreq::new(self.name());
        req.ifr_ifru.ifru_mtu = mtu;
        unsafe { siocsifmtu(self.socket, &req) }?;
        Ok(())
    }
}
