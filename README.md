# Async TUN/TAP

[![Build](https://github.com/yaa110/async-tun/workflows/Build/badge.svg)](https://github.com/yaa110/async-tun/actions) [![crates.io](https://img.shields.io/crates/v/async-tun.svg)](https://crates.io/crates/async-tun) [![Documentation](https://img.shields.io/badge/docs-async--tun-blue.svg)](https://docs.rs/async-tun)

Asynchronous allocation of TUN/TAP devices in Rust using [`async-std`](https://crates.io/crates/async-std).

## Getting Started

- Create a tun device and read from it in a loop:

```rust
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
- [ ] Multi-Queue
- [ ] Owner
- [ ] Group
- [ ] MTU
- [ ] Persistent
