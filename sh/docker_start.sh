#!/bin/bash
source ~/.bashrc;
cd /tmp/cargo_build;
RUSTFLAGS="-C target-feature=-crt-static" cargo build --release --target x86_64-unknown-linux-gnu;
