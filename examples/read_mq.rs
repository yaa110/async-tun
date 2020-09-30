use async_std::os::unix::io::AsRawFd;
use async_std::prelude::*;
use async_std::task;
use async_tun::result::Result;
use async_tun::TunBuilder;
use std::net::Ipv4Addr;

async fn async_main() -> Result<()> {
    let queues = 3;

    let tuns = TunBuilder::new()
        .name("")
        .tap(false)
        .packet_info(false)
        .mtu(1350)
        .up()
        .address(Ipv4Addr::new(10, 0, 0, 1))
        .destination(Ipv4Addr::new(10, 1, 0, 1))
        .broadcast(Ipv4Addr::BROADCAST)
        .netmask(Ipv4Addr::new(255, 255, 255, 0))
        .try_build_mq(queues)
        .await?;

    println!("--------------");
    println!("{} tuns created", queues);
    println!("--------------");

    println!(
        "┌ name: {}\n├ fd: {}, {}, {}\n├ mtu: {}\n├ flags: {}\n├ address: {}\n├ destination: {}\n├ broadcast: {}\n└ netmask: {}",
        tuns[0].name(),
        tuns[0].as_raw_fd(), tuns[1].as_raw_fd(), tuns[2].as_raw_fd(),
        tuns[0].mtu().unwrap(),
        tuns[0].flags().unwrap(),
        tuns[0].address().unwrap(),
        tuns[0].destination().unwrap(),
        tuns[0].broadcast().unwrap(),
        tuns[0].netmask().unwrap(),
    );

    println!("---------------------");
    println!("ping 10.1.0.2 to test");
    println!("---------------------");

    let mut tuns = tuns.into_iter();
    let mut tun0 = tuns.next().unwrap();
    let mut tun1 = tuns.next().unwrap();
    let mut tun2 = tuns.next().unwrap();

    task::spawn(async move {
        let mut buf = [0u8; 1024];
        loop {
            let n = tun0.read(&mut buf).await.unwrap();
            println!("reading {} bytes from tuns[0]: {:?}", n, &buf[..n]);
        }
    });

    task::spawn(async move {
        let mut buf = [0u8; 1024];
        loop {
            let n = tun1.read(&mut buf).await.unwrap();
            println!("reading {} bytes from tuns[1]: {:?}", n, &buf[..n]);
        }
    });

    let mut buf = [0u8; 1024];
    loop {
        let n = tun2.read(&mut buf).await.unwrap();
        println!("reading {} bytes from tuns[2]: {:?}", n, &buf[..n]);
    }
}

fn main() -> Result<()> {
    task::block_on(async_main())
}
