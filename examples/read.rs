use async_std::os::unix::io::AsRawFd;
use async_std::prelude::*;
use async_std::task;
use async_tun::result::Result;
use async_tun::{Kind, Params, Tun};

async fn async_main() -> Result<()> {
    let params = Params::new("", Kind::Tun, false);
    let mut tun = Tun::new(params).await?;

    println!("tun created, name: {}, fd: {}", tun.name(), tun.as_raw_fd());

    loop {
        let mut buf = [0u8; 1024];
        let n = tun.read(&mut buf).await?;
        println!("reading {} bytes: {:?}", n, &buf[..n]);
    }
}

fn main() -> Result<()> {
    task::block_on(async_main())
}
