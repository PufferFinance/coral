ARG RUST_VERSION=1

FROM rust:${RUST_VERSION}-buster AS builder
WORKDIR /app

COPY . .

RUN cargo build --release

FROM rust:${RUST_VERSION}-buster AS runtime

WORKDIR /app

COPY --from=builder /app/target/release/coral-cli /app/coral-cli

CMD ["/app/coral-cli"]
