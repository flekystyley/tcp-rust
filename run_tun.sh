#!bin/bash
cargo b --release
sudo setcap cap_net_admin=eip target/release/tcp-rust
target/release/tcp-rust &
pid=$!
sudo ip addr add 192.168.0.1/24 dev tun_0
sudo ip link set up dev tun_0
trap "kill $pid" INT TERM
wait $pid
