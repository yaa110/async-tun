#[cfg(target_os = "linux")]
mod linux {
    pub(crate) mod interface;
    pub(crate) mod params;
    pub(crate) mod request;
}

mod builder;
mod tun;

pub mod result;

pub use self::builder::TunBuilder;
pub use self::tun::Tun;
