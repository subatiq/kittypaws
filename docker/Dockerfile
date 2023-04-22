FROM rust as build

RUN apt update -qqy && apt -y install build-essential python3-dev

RUN mkdir -p /root/.kittypaws/plugins/{dropper,deathloop-sh,timeburglar}

WORKDIR /root/.kittypaws/plugins
RUN wget -P ./dropper/ https://github.com/subatiq/kittypaws-dropper/raw/master/main.py && wget -P ./deathloop-sh/ https://github.com/subatiq/kittypaws-deathloop/raw/master/run.sh && wget -P ./timeburglar/ https://github.com/subatiq/kittypaws-timeburglar/raw/main/main.py

WORKDIR /

COPY . .

RUN cargo build --release
RUN cp ./target/release/paws /usr/bin/

ENTRYPOINT paws config.yml
