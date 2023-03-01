#!/bin/bash

# Script to package all WASM crates and get them ready for release

# Usage: ./pack.sh --all # default
# Usage: ./pack.sh <crate_name> # can also take multiple crates

COMMAND="wasm-pack.exe build --target web --scope remix-pwa"

# Install toml-cli & json for toml parsing
npm install -g toml-cli
npm install -g json

if [ "$1" == "--all" ] || [ "$1" == "" ]; then
  VALUES=($(toml < ./Cargo.toml | json workspace.members))
  for value in "${VALUES[@]}"
  do
    if [ "$value" != "[" ] && [ "$value" != "]" ]; then
        echo "Packing $value and getting it ready..."
        eval "$COMMAND $value"
    fi
  done
else
  for arg in "$@"
  do
    echo "Packing $arg. Hang on..."
    eval "$COMMAND $arg"
  done
fi
