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
        tar -xvf hub-$DIST-amd64-$HUB_VER.$ZIP_EXT
    else
        unzip hub-$DIST-amd64-$HUB_VER.$ZIP_EXT
    fi
    cd -
    mv /tmp/hub*/bin/hub /tmp/hub
    /tmp/hub version

    # version
    PREV_VER=$(git describe --tags HEAD~1 | awk -F'[-.]' '{print $1"."$2"."$3}')
    echo "Previous version was $PREV_VER..."
    export NEW_VER=$(echo $PREV_VER | awk -F'[-.]' '{print $1"."$2+1"."$3}')
    echo "Upgrading to $NEW_VER..."
    sed -i.bak "s/version = \"[0-9\.]*\"/version = \"$NEW_VER\"/g" Cargo.toml
    cat Cargo.toml
fi

# build
cargo build --release

# release
if [ "$BRANCH" = "master" ]
  then
    echo "On master, performing release..."

    # publish to GitHub
    ls -lh target/release/rsui
    if [ "$ZIP_EXT" == "tgz" ]
    then
        tar -czvf rsui-$DIST-$NEW_VER.$ZIP_EXT target/release/rsui
    else
        zip -r rsui-$DIST-$NEW_VER.$ZIP_EXT target/release/rsui
    fi
    /tmp/hub release create -m $NEW_VER -a rsui-$DIST-$NEW_VER.$ZIP_EXT $NEW_VER

    # publish crate
    cargo login $CRATESIO_TOKEN
    cargo package --allow-dirty
    cargo publish --allow-dirty
fi