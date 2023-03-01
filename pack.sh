#!/bin/bash

# Script to package all WASM crates and get them ready for release

# Usage: ./pack.sh --all
# Usage: ./pack.sh <crate_name> # can also take multiple crates

COMMAND="wasm-pack --target web"

if [[ "$1" == "--all" ]]; then
  VALUES=("crates/client")
  for value in "${VALUES[@]}"
  do
    echo "Packing $value and getting it ready..."
    eval "$COMMAND $value"
  done
else
  for arg in "$@"
  do
    echo "Packing $arg. Hang on..."
    eval "$COMMAND $arg"
  done
fi
