ARG RUST_VERSION

FROM rust:${RUST_VERSION}-alpine AS builder

WORKDIR /app

USER root

RUN rustup target add x86_64-unknown-linux-musl
RUN apk add musl-dev

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl --offline

FROM gcr.io/distroless/static-debian12:nonroot AS production

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/stats-todos ./bin/stats-todos

ENTRYPOINT ["/app/bin/stats-todos"]


