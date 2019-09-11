#!/bin/bash

mkdir build -p
cargo +nightly test --features "x86" --color always -- --nocapture 2>&1 | less -R
#cargo +nightly test --color always -- --nocapture 2>&1 | less -R
