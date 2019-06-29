#!/usr/bin/env bash

set -e

for arg in "$@"
do
    ./target/release/brainfrust $arg.b
    rustc -C debuginfo=0 -C opt-level=3 $arg.rs
    ./$arg
done
