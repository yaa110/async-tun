#[cfg(target_os = "linux")]
mod linux {
    pub(crate) mod ifreq;
}

mod interface;
mod kind;
mod params;
mod tun;

pub mod result;

pub use self::kind::Kind;
pub use self::params::Params;
pub use self::tun::Tun;
