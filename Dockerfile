FROM rust:latest

RUN apt-get update && apt-get -y install ca-certificates cmake musl-tools libssl-dev && rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo build --release

EXPOSE 9000

CMD ["cargo", "run", "--release"]