#!/bin/bash -e

cargo build

version=$(grep ^version Cargo.toml | sed "s/\"//g" | awk '{print "v"$3}')

git tag "$version"
git push origin main --tags
