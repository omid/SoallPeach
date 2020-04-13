FROM rust

RUN apt update && apt -y install cmake

WORKDIR /app
COPY . /app

RUN cargo build --release
RUN ls /app/target

ENTRYPOINT ["/app/target/release/prime"]