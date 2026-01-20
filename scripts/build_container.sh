#!/bin/bash

if [ ! $# -eq 4 ]; then
    echo "Wrong number of arguments supplied."
    echo "Usage: $0 OWNER NAME CRATE FOLDER"
    exit 1
fi

OWNER=$1
NAME=$2
CRATE=$3
FOLDER=$4

VERSION=$(cargo info "${CRATE}" | grep ^version: | awk '{print $2}')

SPEC="${OWNER}/${NAME}"
FULLSPEC="${SPEC}:v${VERSION}"

docker build \
    --tag "${FULLSPEC}" \
    --tag "${SPEC}:latest" \
    --file "${FOLDER}/Dockerfile" \
    .

if [ $? -eq 0 ]; then
    echo "Successfully built the container: ${FULLSPEC}"
else
    echo "Failed to build the container: ${FULLSPEC}"
    exit 1
fi

read -p "Publish container ${FULLSPEC}? " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    docker push "${FULLSPEC}"
    docker push "${SPEC}:latest"
fi
