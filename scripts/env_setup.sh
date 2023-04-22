#!/usr/bin/env bash

set -exuo pipefail

apt update -qqy
apt -y install build-essential python3-dev sudo systemd systemd-timesyncd

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > setup.sh
chmod +x setup.sh
./setup.sh -y
source "$HOME/.cargo/env"

rustup install stable
rustup default stable

mkdir -p /root/.kittypaws/plugins/{dropper,deathloop-sh,timeburglar}

wget -P /root/.kittypaws/plugins/dropper/ https://github.com/subatiq/kittypaws-dropper/raw/master/main.py
wget -P /root/.kittypaws/plugins/deathloop-sh/ https://github.com/subatiq/kittypaws-deathloop/raw/master/run.sh
wget -P /root/.kittypaws/plugins/timeburglar/ https://github.com/subatiq/kittypaws-timeburglar/raw/main/main.py

cargo build --release
cp ./target/release/paws /usr/bin/

mkdir -p /etc/docker/
echo -ne '{"metrics-addr" : "0.0.0.0:9323"}' > /etc/docker/daemon.json
systemctl daemon-reload
systemctl restart docker
