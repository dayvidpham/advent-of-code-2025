#!/usr/bin/env sh

TARGET="day2"

rustc -v ./src/lib/advent/main.rs \
    --out-dir ./build/lib \
    --crate-name advent \
    --crate-type=lib

rustc -v ./src/bin/${TARGET}.rs \
    -o ./build/${TARGET} \
    -L crate=./build/lib \
    --extern advent=./build/lib/libadvent.rlib &&
    ./build/${TARGET}
