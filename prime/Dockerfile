FROM rust

WORKDIR /app
COPY . /app

RUN cargo build --release

ENTRYPOINT ["/app/target/release/prime"]