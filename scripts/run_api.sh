#!/bin/bash

RUST_LOG=info \
    cargo run \
    --bin avina-api \
    | bunyan
