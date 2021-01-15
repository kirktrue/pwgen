#!/bin/bash -e

VERSION=v$(grep "^version = " Cargo.toml | cut -d\" -f 2)
KERNEL=$(echo $(uname -s) | tr '[:upper:]' '[:lower:]')

# This is just for compatibility with the Go version...
MACHINE=amd64

RELEASE_NAME=pwgen-$VERSION-$KERNEL-$MACHINE

cargo build --release
cp target/release/pwgen $RELEASE_NAME
