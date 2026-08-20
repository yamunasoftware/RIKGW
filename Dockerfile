FROM rust:1.97 AS builder
WORKDIR /main
COPY Cargo.toml Cargo.lock ./
COPY resources ./resources
COPY src ./src
RUN cargo build --release

FROM debian:trixie
WORKDIR /main
COPY --from=builder /main/target/release/rikgw /main/rikgw
ENTRYPOINT ["/main/rikgw"]