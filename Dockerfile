FROM rust:alpine

COPY . .

RUN apk add --no-cache python3

RUN cargo build
