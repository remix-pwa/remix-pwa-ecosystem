#!/bin/bash

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
        eval "$COMMAND $value -- --features console_error_panic_hook"
    fi
  done
else
  for arg in "$@"
  do
    echo "Packing $arg. Hang on..."
    eval "$COMMAND $arg -- --features console_error_panic_hook"
  done
fi

exit 0
