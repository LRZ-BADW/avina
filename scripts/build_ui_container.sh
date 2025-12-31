#!/bin/bash

VERSION=0.0.2

docker build \
    --tag "gierens/avina-ui:v${VERSION}" \
    --tag "gierens/avina-ui:latest" \
    --file ui/Dockerfile \
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
    docker push "gierens/avina-ui:v${VERSION}"
    docker push "gierens/avina-ui:latest"
fi
