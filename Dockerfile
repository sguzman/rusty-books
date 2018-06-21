FROM alpine

WORKDIR /root

RUN apk update && apk add --no-cache rust cargo

ADD . rusty
WORKDIR crow
RUN cargo build --release --verbose --color always

ENTRYPOINT ['./target/release/rustbooks']