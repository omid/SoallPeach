FROM rust as builder

RUN apt update && apt -y install cmake

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM frolvlad/alpine-glibc

COPY --from=builder /app/target/release/prime /prime

ENTRYPOINT ["/prime"]

