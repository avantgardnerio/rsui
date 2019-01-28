#!/usr/bin/env bash
set -x

BRANCH=$(git branch | grep -o master)
echo "Branch=$BRANCH"
if [ "$BRANCH" = "master" ]
  then
    echo "On master, performing release..."

    # determine OS
    UNAME="$(uname -s)"
    case "${UNAME}" in
        Linux*)     DIST=linux ZIP_EXT=tgz;;
        Darwin*)    DIST=darwin ZIP_EXT=tgz;;
        CYGWIN*)    DIST=windows ZIP_EXT=zip;;
        MINGW*)     DIST=windows ZIP_EXT=zip;;
        *)          DIST="UNKNOWN:${UNAME}"
    esac
    echo DIST=${DIST} EXT=$ZIP_EXT

    # github cli
    HUB_VER=2.8.3
    cd /tmp
    wget -q https://github.com/github/hub/releases/download/v$HUB_VER/hub-$DIST-amd64-$HUB_VER.$ZIP_EXT
    if [ "$ZIP_EXT" == "tgz" ]
    then
        tar -xf hub-$DIST-amd64-$HUB_VER.$ZIP_EXT
    else
        unzip hub-$DIST-amd64-$HUB_VER.$ZIP_EXT
    fi
    cd -
    mv /tmp/hub*/bin/hub .
    hub version

    # publish to GitHub
    ls -lh target/release/rsui
    if [ "$ZIP_EXT" == "tgz" ]
    then
        tar -czvf rsui-$DIST.tar.gz target/release/rsui
    else
        zip -r rsui-$DIST.zip target/release/rsui
    fi
    hub release create -a rsui-$DIST.zip $NEW_VER

    # publish crate
    cargo login $CRATESIO_TOKEN
    cargo package --allow-dirty
    cargo publish --allow-dirty
fi