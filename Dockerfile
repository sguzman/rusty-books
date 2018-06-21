FROM ubuntu

WORKDIR /root

RUN apt-get update && apt-get isntall -y rust cargo

ADD . rust
WORKDIR rust
RUN cargo build --release --verbose --color always

ENTRYPOINT ['./target/release/rustbooks']