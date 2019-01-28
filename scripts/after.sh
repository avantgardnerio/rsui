#!/usr/bin/env bash
set -x

# determine OS
export UNAME="$(uname -s)"
case "${UNAME}" in
    Linux*)     export DIST=linux ZIP_EXT=tgz;;
    Darwin*)    export DIST=darwin ZIP_EXT=tgz;;
    CYGWIN*)    export DIST=windows ZIP_EXT=zip;;
    MINGW*)     export DIST=windows ZIP_EXT=zip;;
    *)          export DIST="UNKNOWN:${UNAME}"
esac
echo DIST=${DIST} EXT=$ZIP_EXT

BRANCH=$(git branch | grep -o master)
echo "Branch=$BRANCH"
if [ "$BRANCH" = "master" ]
  then
    echo "On master, performing release..."

    NEW_VER=$(git describe --tags)

    # publish to GitHub
    ls -lh target/release/rsui
    if [ "$ZIP_EXT" == "tgz" ]
    then
        tar -czvf rsui-$DIST-$NEW_VER.tar.gz target/release/rsui
    else
        zip -r rsui-$DIST-$NEW_VER.zip target/release/rsui
    fi
    /tmp/hub release create -a rsui-$DIST-$NEW_VER.zip $NEW_VER

    # publish crate
    cargo login $CRATESIO_TOKEN
    cargo package --allow-dirty
    cargo publish --allow-dirty
fi