#!/bin/bash
rm -rf ./target;
docker run --rm -v "$(pwd):/tmp/cargo_build" ccr.ccs.tencentyun.com/linux_environment/cargo:1.0.0 /bin/bash -c "/tmp/cargo_build/sh/docker_start.sh";
mv -f ./target/x86_64-unknown-linux-gnu/release/rust-get-proxy-request ./;
echo "Press Enter to continue...";
read -n 1;
