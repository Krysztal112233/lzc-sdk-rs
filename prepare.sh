#! /usr/bin/bash

DIR="$(pwd)/.tmp/protos"

if [ -d "$DIR" ]; then
    rm -rf "$DIR"
fi
mkdir -p "$DIR"

pushd $_
git clone --depth=1 https://gitee.com/linakesi/lzc-boxservice-protos.git
git clone --depth=1 https://gitee.com/linakesi/lzc-sdk.git
popd
