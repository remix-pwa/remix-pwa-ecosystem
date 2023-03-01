#!/bin/bash

# Issue when running script without `--headless` mode.
# Skips testing all other crates after testing first crate
# `--headless` mode doesn't work well with web_sys either
# Fix needed

## wasm-pack test helper script

# Usage: ./test.sh --all # default (not-required)
# Usage: ./test.sh <crate_name> # can also take multiple crates
# Use the -f flag is this is your first time running the script to install dependencies

flag=false
COMMAND="wasm-pack.exe test --chrome"

while getopts ":f" opt
do
  case $opt in
    f)
      flag=true;;
    \?)
      echo "Invalid option: -$OPTARG" >&2
      exit 1
      ;;
    :)
      echo "Option -$OPTARG requires an argument." >&2
      exit 1
      ;;
  esac
done

shift $((OPTIND -1))

first_arg=$1

if [ $# -eq 0 ]; then
  first_arg="--all"
  echo "Running wasm tests for all crates..."
fi

# Install toml-cli & json for toml parsing
if [[ "$flag" == true ]]; then
  npm install -g toml-cli
  npm install -g json
fi

if [[ "$first_arg" == "--all" ]]; then
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

exit 0
