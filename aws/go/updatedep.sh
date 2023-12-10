#!/usr/bin/env bash
set -xue

if ! [[ "$0" =~ updatedep.sh ]]; then
    echo "must be run from root"
    exit 255
fi

go get -u -v ./...
go mod tidy -v
