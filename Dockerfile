FROM rust:alpine3.23 AS builder
WORKDIR /main
COPY Cargo.toml Cargo.lock ./
COPY resources ./resources
COPY src ./src
RUN cargo build --release

FROM alpine:3.23
RUN apk add --no-cache ca-certificates
COPY --from=builder /main/target/release/rikgw /usr/local/bin/rikgw
ENTRYPOINT ["/usr/local/bin/rikgw"]