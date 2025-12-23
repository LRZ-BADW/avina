#!/bin/bash

export RUST_LOG="${RUST_LOG:info}"

cargo run \
    --bin avina-api \
    | bunyan
