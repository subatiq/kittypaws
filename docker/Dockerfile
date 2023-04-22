FROM rust as build

COPY . .

RUN apt update -qqy && apt -y install build-essential python3-dev

RUN cargo build --release

RUN cp ./target/release/paws /usr/bin/
