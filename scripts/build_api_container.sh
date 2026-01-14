#!/bin/bash

VERSION=1.0.1

docker build \
    --tag "gierens/avina:v${VERSION}" \
    --tag "gierens/avina:latest" \
    --file api/Dockerfile \
    .

if [ $? -eq 0 ]; then
    echo "Successfully built the container"
else
    echo "Failed to build the container"
    exit 1
fi

read -p "Publish container? " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    docker push "gierens/avina:v${VERSION}"
    docker push "gierens/avina:latest"
fi
