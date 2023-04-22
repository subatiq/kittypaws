FROM rust

COPY . .

RUN apt update -qqy && apt -y install build-essential python3

RUN cargo build --release
