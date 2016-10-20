#!/bin/sh

cd puyocore
RUSTFLAGS="-C target-cpu=native" cargo test
