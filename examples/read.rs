use async_std::os::unix::io::AsRawFd;
use async_std::prelude::*;
use async_std::task;
use async_tun::result::Result;
use async_tun::TunBuilder;
use std::net::Ipv4Addr;

async fn async_main() -> Result<()> {
    let mut tun = TunBuilder::new()
        .name("")
        .tap(false)
        .packet_info(false)
        .mtu(1350)
        .up()
        .address(Ipv4Addr::new(10, 0, 0, 1))
        .destination(Ipv4Addr::new(10, 1, 0, 1))
        .try_build()
        .await?;

    println!(
        "tun created\n\tname: {}\n\tfd: {}\n\tmtu: {}\n\tflags: {}\n\taddress: {}\n\tdestination: {}",
        tun.name(),
        tun.as_raw_fd(),
        tun.mtu().unwrap(),
        tun.flags().unwrap(),
        tun.address().unwrap(),
        tun.destination().unwrap(),
    );

    let mut buf = [0u8; 1024];
    loop {
        let n = tun.read(&mut buf).await?;
        println!("reading {} bytes: {:?}", n, &buf[..n]);
    }
}

fn main() -> Result<()> {
    task::block_on(async_main())
}
