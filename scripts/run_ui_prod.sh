#!/bin/bash

echo "WARNING: running UI against production API."

dx serve \
    --cross-origin-policy \
    --package avina-ui
