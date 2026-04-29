#!/bin/bash

if [[ -z $OS_AUTH_URL || -z $OS_USERNAME || -z $OS_PASSWORD || -z $OS_PROJECT_NAME || -z $OS_PROJECT_ID || -z $OS_USER_DOMAIN_NAME || -z $OS_PROJECT_DOMAIN_ID ]]; then
    echo "Some OpenStack environment variable is missing. Please source the admin-openrc.sh!"
    exit 1
fi

export APP_OPENSTACK__KEYSTONE_ENDPOINT="${OS_AUTH_URL}"
export APP_OPENSTACK__USERNAME="${OS_USERNAME}"
export APP_OPENSTACK__PASSWORD="${OS_PASSWORD}"
export APP_OPENSTACK__PROJECT="${OS_PROJECT_NAME}"
export APP_OPENSTACK__PROJECT_ID="${OS_PROJECT_ID}"
export APP_OPENSTACK__DOMAIN="${OS_USER_DOMAIN_NAME}"
export APP_OPENSTACK__DOMAIN_ID="${OS_PROJECT_DOMAIN_ID}"

cargo run \
    --bin avina-api \
    | bunyan
