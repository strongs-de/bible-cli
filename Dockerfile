FROM rust:1.60 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --bin bible-cli --all-features --path .

FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/bible-cli /usr/local/bin/bible-cli
WORKDIR /data
ENTRYPOINT ["bible-cli"]