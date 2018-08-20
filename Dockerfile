FROM rust:1.28.0 as builder

ARG COMPONENT

WORKDIR /rust-invoker

COPY . .

RUN apt-get update

RUN apt-get install -qq musl-tools

RUN rustup target add x86_64-unknown-linux-musl

RUN cargo build --target x86_64-unknown-linux-musl --release

###########

FROM debian:wheezy-slim

# The following line forces the creation of a /tmp directory
WORKDIR /tmp

WORKDIR /

COPY --from=builder /rust-invoker/target/x86_64-unknown-linux-musl/release/rust_invoker /rust_invoker

ENTRYPOINT ["/rust_invoker"]