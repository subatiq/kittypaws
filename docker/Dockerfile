FROM rust

COPY . .

RUN apt update -qqy && apt -y install build-essential python3-dev

RUN cargo build --release
