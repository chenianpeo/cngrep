#!/usr/bin/env bash
set -e

echo "===================="
echo "CNGREP Smoke Test"
echo "===================="
echo

# 1. basic commands: normal file search
echo "[1] base search - content.txt"
cargo run -- t content.txt

echo
echo "[2] grep search - content.txt"
cargo run -- grep content.txt

# 2. bigfile search
echo
echo "[3] big file search - bigfile.txt"
cargo run -- rust bigfile_40m.txt

# 3. empty file
echo
echo "[4] empty file test"
cargo run -- grep empty.txt

# 4. file not exist
echo
echo "[5] file not exist"
cargo run -- t not_exist.txt

echo
echo "===================="
echo "Smoke Test Finished Ok"
echo "===================="