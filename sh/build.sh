#!/bin/bash
RUSTFLAGS="-Ctarget-feature=+crt-static" cargo build --release;
echo "Press Enter to continue...";
read -n 1;