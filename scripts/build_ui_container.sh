#!/bin/bash

OWNER='gierens'
NAME='avina-ui'
CRATE='avina-ui'
FOLDER='ui'

bash ./scripts/build_container.sh "${OWNER}" "${NAME}" "${CRATE}" "${FOLDER}"
