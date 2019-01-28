#!/usr/bin/env bash
git tag $NEW_VER
git push --tags

cargo login $CRATESIO_TOKEN
cargo package --allow-dirty
cargo publish --allow-dirty
