#!/usr/bin/env zsh

set -e

export YEAR=$1
export DAY=$2

echo "compiling"
cargo build --color=always --all --all-targets

function get_input() {
  input_folder="inputs/$YEAR"
  input_file="$input_folder/$DAY"

  if [ ! -f "$input_file" ]; then
    mkdir -p "$input_folder"
    source ./.env
    curl -s "https://adventofcode.com/$YEAR/DAY/$((DAY))/input" -H "Cookie:$SESSION" > $input_file
  fi

  cat "$input_file"
}

echo "executing"
echo -e "\n\n======\n\n"

get_input | time target/debug/advent-of-code