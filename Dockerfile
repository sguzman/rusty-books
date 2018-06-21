FROM ubuntu:latest

USER root
ENV USER root
WORKDIR /root

# Install package dependencies.
RUN apt-get update \
    && apt-get install -y \
    apt-utils \
    curl \
    gcc \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl https://sh.rustup.rs -sSf > /tmp/rustup-init.sh \
    && chmod +x /tmp/rustup-init.sh \
    && sh /tmp/rustup-init.sh -y \
    && rm -rf /tmp/rustup-init.sh
ENV PATH "~/.cargo/bin/:${PATH}"

ADD . rust
WORKDIR rust
RUN cargo build --release --verbose --color always

ENTRYPOINT ['./target/release/rustbooks']