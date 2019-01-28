#!/bin/bash
NEXT_VER=$(git describe --tags | awk -F'[-.]' '{print $1"."$2+1"."$3}')
echo "Upgrading to $NEXT_VER..."
sed -i '' "s/version = \"[0-9\.]*\"/version = \"$NEXT_VER\"/g" Cargo.toml
