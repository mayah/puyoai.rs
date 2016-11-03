#!/bin/sh

cd puyocore
RUSTFLAGS="-C target-cpu=native" RUST_BACKTRACE=1 cargo test
