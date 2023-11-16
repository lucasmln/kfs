FROM debian:bookworm

WORKDIR /app

RUN apt update && apt install -y gcc grub-common grub-pc-bin \
    binutils make nasm xorriso curl

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup default nightly
RUN rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu

CMD ["make"]
