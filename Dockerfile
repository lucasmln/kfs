FROM debian:bookworm

WORKDIR /app

RUN apt update && apt install -y gcc grub-common grub-pc-bin \
    binutils make nasm xorriso

CMD ["make"]