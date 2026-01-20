#!/bin/bash

OWNER='gierens'
NAME='avina'
CRATE='avina-api'
FOLDER='api'

bash ./scripts/build_container.sh "${OWNER}" "${NAME}" "${CRATE}" "${FOLDER}"
