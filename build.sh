#!/bin/bash -e
# shellcheck disable=SC2162
# shellcheck disable=SC2001

version=$(grep ^version Cargo.toml | sed "s/\"//g" | awk '{print "v"$3}')

echo "Building version: $version..."
cargo build --release
echo "...done"

echo "Generating release notes..."
should_output=0
title=""
notes_file=$(mktemp "/tmp/$(basename "$0").XXXXXX") || exit 1

# Make a temp file
# Iterate over CHANGELOG.md
# If the line starts with ## $version, start there. Take that as the title
# skip to the next line and
while read line; do
  if [ "$(echo "$line" | grep -c "^## $version")" = "1" ] ; then
    title=$(echo "$line" | sed 's/## //')
    should_output=1
  elif [ "$should_output" = 1 ] && [ "$(echo "$line" | grep -c "^## v")" = "1" ] ; then
    should_output=0
  elif [ "$should_output" = 1 ] ; then
    echo "${line}" >> "$notes_file"
  fi
done<CHANGELOG.md

echo "...done"

echo "Generating release on GitHub"
gh release \
  create "$version" \
  --draft \
  --title "$title" \
  --notes-file "$notes_file" \
  --notes-start-tag "$version" \
  target/release/pwgen
echo "...done"
