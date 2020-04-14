FROM rust:alpine as builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM alpine:latest

COPY --from=builder /app/target/release/prime /prime

ENTRYPOINT ["/prime"]

