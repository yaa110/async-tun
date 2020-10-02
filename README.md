# Async TUN/TAP

[![Build](https://github.com/yaa110/async-tun/workflows/Build/badge.svg)](https://github.com/yaa110/async-tun/actions) [![crates.io](https://img.shields.io/crates/v/async-tun.svg)](https://crates.io/crates/async-tun) [![Documentation](https://img.shields.io/badge/docs-async--tun-blue.svg)](https://docs.rs/async-tun) [![examples](https://img.shields.io/badge/examples-async--tun-blue.svg)](examples)

Asynchronous allocation of TUN/TAP devices in Rust using [`async-std`](https://crates.io/crates/async-std) and [`tokio`](https://crates.io/crates/tokio).

## Getting Started

- Create a tun device using `TunBuilder` and read from it in a loop:

```rust
use async_std::os::unix::io::AsRawFd;
use async_std::prelude::*;
use async_std::task;
use async_tun::result::Result;
use async_tun::TunBuilder;
use std::net::Ipv4Addr;

async fn async_main() -> Result<()> {
    let tun = TunBuilder::new()
        .name("")            // if name is empty, then it is set by kernel.
        .tap(false)          // false (default): TUN, true: TAP.
        .packet_info(false)  // false: IFF_NO_PI, default is true.
        .up()                // or set it up manually using `sudo ip link set <tun-name> up`.
        .try_build()         // or `.try_build_mq(queues)` for multi-queue support.
        .await?;

    println!("tun created, name: {}, fd: {}", tun.name(), tun.as_raw_fd());

    let mut reader = tun.reader();
    let mut buf = [0u8; 1024];
    loop {
        let n = reader.read(&mut buf).await?;
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

- Set the address of device (address and netmask could also be set using `TunBuilder`):

```bash
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
- [ ] Android
- [ ] OSX
- [ ] iOS
- [ ] Windows

## Examples

- [`read`](examples/read.rs): reading from tun using `async-std`.
- [`read-mq`](examples/read-mq.rs): reading from multi-queue tun using `async-std`.
- [`read-tokio`](examples/read-tokio.rs): reading from tun using `tokio`. It should be run with `tokio` feature:

```bash
➜  cargo run --example read-tokio --no-default-features --features tokio
```
