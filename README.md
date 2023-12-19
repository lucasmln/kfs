# kfs

![image of when the kernel just booted up](/screenshot/home.png)

## Installation

```
rustup update nightly
rustup default nightly
rustup target add i686-unknown-linux-gnu
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
cargo build
```

## Run
(qemu must be installed)  

```
cargo run
```

## Create an iso

```
make release
```
