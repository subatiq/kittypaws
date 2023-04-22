#!/usr/bin/env bash

set -exu pipefail

apt update -qqy
apt -y install build-essential python3-dev sudo systemd systemd-timesyncd

mkdir -p /root/.kittypaws/plugins/{dropper,deathloop-sh,timeburglar}

wget -P /root/.kittypaws/plugins/dropper/ https://github.com/subatiq/kittypaws-dropper/raw/master/main.py
wget -P /root/.kittypaws/plugins/deathloop-sh/ https://github.com/subatiq/kittypaws-deathloop/raw/master/run.sh
wget -P /root/.kittypaws/plugins/timeburglar/ https://github.com/subatiq/kittypaws-timeburglar/raw/main/main.py

cargo build --release
cp ./target/release/paws /usr/bin/
paws config.yml &
