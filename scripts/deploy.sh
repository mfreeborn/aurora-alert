#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

# This build script generates a single folder `dist` in the workspace 
# root which contains all the necessary files for serving the app on
# the raspberry pi. To complete the deployment, copy `./dist` to the
# raspberry pi and, from within the folder, run ther `server` binary.


# clean the workspace root from old artifacts
[[ -d ./dist ]] && rm -r dist/

pushd frontend
trunk build --release --public-url /assets/
cp -r dist/ ../
popd

pushd server
cross build --release --target armv7-unknown-linux-gnueabihf
cp -r configuration/ ../dist/
popd

cp target/armv7-unknown-linux-gnueabihf/release/server dist/
cp aurora-alert.service dist/

echo "Copy dist/ to the raspberry pi to continue deployment"
