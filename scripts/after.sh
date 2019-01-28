#!/usr/bin/env bash
set -x

BRANCH=$(git branch | grep -o master)
echo "Branch=$BRANCH"
if [ "$BRANCH" = "master" ]
  then
    echo "On master, performing release..."

    # publish to GitHub
    ls -lh target/release/rsui
    if [ "$ZIP_EXT" == "tgz" ]
    then
        tar -czvf rsui-$DIST.tar.gz target/release/rsui
    else
        zip -r rsui-$DIST.zip target/release/rsui
    fi
    /tmp/hub release create -a rsui-$DIST.zip $NEW_VER

    # publish crate
    cargo login $CRATESIO_TOKEN
    cargo package --allow-dirty
    cargo publish --allow-dirty
fi