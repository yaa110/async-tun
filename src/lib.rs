#[cfg(target_os = "linux")]
mod linux {
    pub mod address;
    pub mod interface;
    pub mod params;
    pub mod request;
}

mod builder;
mod tun;

pub mod address;
pub mod result;

pub use self::address::EthernetAddr;
pub use self::builder::TunBuilder;
pub use self::tun::Tun;
