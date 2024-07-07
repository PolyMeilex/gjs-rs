#!/bin/bash
set -x -e

gir -o .
gir -o . -m doc
rustdoc-stripper -m -d ./src/ -g -o ./vendor.md
cargo fmt
