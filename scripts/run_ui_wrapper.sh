#!/bin/bash

AVINA_API_URL="http://localhost:8000/api"

print_help ()
{
    echo "Usage: $0 [-p,--prod|-t,--test]"
    echo ""
    echo "  -p, --prod   Use production avina API."
    echo "  -t, --test   Use test system avina API."
    echo "  -h, --help   Display this help message."
    echo ""
}

if [ $# -gt 1 ]; then
    echo "Error: Wrong number of arguments" >&2
    print_help
    exit 1
fi

if [ $# = 1 ]; then
    if [ "$1" = "-p" ] || [ "$1" = "--prod" ]; then
        AVINA_API_URL="https://cc.lrz.de:1338/api"
    elif [ "$1" = "-t" ] || [ "$1" = "--test" ]; then
        AVINA_API_URL="https://tcc.cloud.mwn.de:1338/api"
    elif [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
        print_help
        exit 0
    else
        echo "Error: Unrecognized argument $1" >&2
        print_help
        exit 1
    fi
fi

export AVINA_API_URL

django-admin runserver \
    127.0.0.1:8888 \
    --pythonpath=./wrapper \
    --settings=app
