#!/bin/sh
cargo afl build
cargo afl fuzz -i in -o out target/debug/dice