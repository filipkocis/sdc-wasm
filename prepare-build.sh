#!/bin/bash

rm -rf ./dist
wasm-pack build --target web --out-name sdc-wasm --out-dir ./dist
cp -r ./js/* ./dist
