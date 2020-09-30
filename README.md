# Async TUN/TAP

[![Build](https://github.com/yaa110/async-tun/workflows/Build/badge.svg)](https://github.com/yaa110/async-tun/actions) [![crates.io](https://img.shields.io/crates/v/async-tun.svg)](https://crates.io/crates/async-tun) [![Documentation](https://img.shields.io/badge/docs-async--tun-blue.svg)](https://docs.rs/async-tun)

Asynchronous allocation of TUN/TAP devices in Rust using [`async-std`](https://crates.io/crates/async-std).

## Getting Started

- Create a tun device using `TunBuilder` and read from it in a loop:

```rust
use async_std::os::unix::io::AsRawFd;
use async_std::prelude::*;
use async_std::task;
use async_tun::result::Result;
use async_tun::TunBuilder;

async fn async_main() -> Result<()> {
    let mut tun = TunBuilder::new()
        .name("")            // If name is empty, then it is set by kernel.
        .tap(false)          // false (default): TUN, true: TAP.
        .packet_info(false)  // false: IFF_NO_PI, default is true.
        .try_build()
        .await?;

    println!("tun created, name: {}, fd: {}", tun.name(), tun.as_raw_fd());

    let mut buf = [0u8; 1024];
    loop {
        let n = tun.read(&mut buf).await?;
        println!("reading {} bytes: {:?}", n, &buf[..n]);
    }
}

fn main() -> Result<()> {
    task::block_on(async_main())
}
```

- Run the code using `sudo`:

```bash
➜  sudo -E /path/to/cargo run
```

- Set up the device:

```bash
➜  sudo ip link set dev <tun-name> up
➜  sudo ip a add 10.0.0.1/24 dev <tun-name>
```

- Ping to read packets:

```bash
➜  ping 10.0.0.2
```

- Display devices and analyze the network traffic:

```
➜  ip tuntap
➜  sudo tshark -i <tun-name>
```

## Supported Platforms

- [x] Linux
- [ ] FreeBSD
- [ ] OpenBSD
- [ ] NetBSD
- [ ] Android
- [ ] OSX
- [ ] iOS
- [ ] Solaris
- [ ] Windows
- [ ] QNX

## Supported Flags

- [x] Tun
- [x] Tap
- [x] No Packet Info
- [x] MTU
- [x] Owner
- [x] Group
- [x] Persistent
- [ ] Multi-Queue
