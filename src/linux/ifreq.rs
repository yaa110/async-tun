#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::interface::Interface;
use crate::params::Params;
use crate::result::Result;
use nix::ioctl_write_int;
use std::mem;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

ioctl_write_int!(tunsetiff, b'T', 202);

impl Interface for ifreq {
    fn new(params: Params) -> Result<Self> {
        let mut req: ifreq = unsafe { mem::zeroed() };
        req.ifr_ifru.ifru_flags = params.flags;
        if let Some(name) = params.name {
            let mut ifname: [i8; IFNAMSIZ as _] = [0; IFNAMSIZ as _];
            for (i, c) in name.as_bytes().iter().enumerate() {
                if i > ifname.len() - 1 {
                    break;
                }
                ifname[i] = *c as _;
            }
            req.ifr_ifrn.ifrn_name = ifname;
        }
        Ok(req)
    }

    fn name(&self) -> String {
        let mut name = String::new();
        for i in 0..IFNAMSIZ as _ {
            let c = unsafe { self.ifr_ifrn.ifrn_name }[i] as u8 as char;
            if c != '\0' {
                name.push(c)
            }
        }
        name
    }
}
