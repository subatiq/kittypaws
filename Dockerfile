FROM rust:1.67 as builder
RUN apt update && apt install -y python3.9-dev
WORKDIR /opt/app
COPY . .
RUN cargo install --path .


FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y python3.9-dev docker.io && rm -rf /var/lib/apt/lists/*
COPY --from=builder /opt/app/target/release/paws /opt/app/paws
ENTRYPOINT ["/opt/app/paws"]
