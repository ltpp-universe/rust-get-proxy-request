#!/bin/bash
docker run --rm -v "$(pwd):/tmp/test-code" ccr.ccs.tencentyun.com/linux_environment/cargo:1.0.0 /bin/bash -c "source ~/.bashrc;cd /tmp/test-code && cargo build --release --target=x86_64-unknown-linux-musl";
echo "Press Enter to continue...";
read -n 1;