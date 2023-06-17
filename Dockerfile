FROM rust:1.70-slim

WORKDIR /app

RUN apt update && apt install lld clang -y

COPY . .

RUN cargo build --release

ENTRYPOINT ["./target/release/convertify"]