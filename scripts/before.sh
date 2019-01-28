#!/bin/bash
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

export BRANCH=$(git branch | grep -o master)
echo "Branch=$BRANCH"
if [ "$BRANCH" = "master" ]
  then
    echo "On master, updating cargo version..."

    # github cli
    HUB_VER=2.8.3
    cd /tmp
    wget -q https://github.com/github/hub/releases/download/v$HUB_VER/hub-$DIST-amd64-$HUB_VER.$ZIP_EXT
    if [ "$ZIP_EXT" == "tgz" ]
    then
        tar -xfv hub-$DIST-amd64-$HUB_VER.$ZIP_EXT
    else
        unzip hub-$DIST-amd64-$HUB_VER.$ZIP_EXT
    fi
    cd -
    ls -l /tmp | grep hub
    ls -l /tmp/hub*/bin/
    mv /tmp/hub*/bin/hub .
    hub version

    # version
    PREV_VER=$(git describe --tags HEAD~1 | awk -F'[-.]' '{print $1"."$2"."$3}')
    echo "Previous version was $PREV_VER..."
    export NEW_VER=$(echo $PREV_VER | awk -F'[-.]' '{print $1"."$2+1"."$3}')
    echo "Upgrading to $NEW_VER..."
    sed -i.bak "s/version = \"[0-9\.]*\"/version = \"$NEW_VER\"/g" Cargo.toml
    cat Cargo.toml
fi
