#[cfg(target_os = "linux")]
mod linux {
    pub(crate) mod ifreq;
}

mod builder;
mod interface;
mod params;
mod tun;

pub mod result;

pub use self::builder::TunBuilder;
pub use self::tun::Tun;
