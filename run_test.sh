#!/bin/sh

cd field
RUSTFLAGS="-C target-cpu=native" cargo test
