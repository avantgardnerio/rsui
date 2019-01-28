#!/bin/bash
BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [ "$BRANCH" = "master" ]
  then
    echo "On master, performing release..."
    PREV_VER=$(git describe --tags HEAD~1 | awk -F'[-.]' '{print $1"."$2"."$3}')
    echo "Previous version was $PREV_VER..."
    NEW_VER=$(echo $PREV_VER | awk -F'[-.]' '{print $1"."$2+1"."$3}')
    echo "Upgrading to $NEW_VER..."
    sed -i '' "s/version = \"[0-9\.]*\"/version = \"$NEW_VER\"/g" Cargo.toml
    git tag $NEW_VER
    git push --tags
fi
