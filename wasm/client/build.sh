#!/bin/bash

cd "$(dirname "$0")"

mkdir -p public

CSS_FILE="$(realpath "$(pwd)/public/app.css")"
OUTPUT_CSS=$CSS_FILE wasm-pack build --no-typescript --dev --target no-modules --out-dir ../public
cp ../index.html ../public/
cp ./public/app.css ../public
rm -r public
