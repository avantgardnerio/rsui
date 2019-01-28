#!/usr/bin/env bash
cargo login $CRATESIO_TOKEN
cargo package --allow-dirty
cargo publish --allow-dirty
