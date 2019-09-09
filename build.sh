#!/bin/bash

cargo +nightly build --features "x86" --color always 2>&1 | less -R
