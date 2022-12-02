#!/usr/bin/env zsh

set -e

export YEAR=$1
export DAY=$2

echo "compiling"
cargo build --color=always --all --all-targets --release

function get_input() {
  input_folder="inputs/$YEAR"
  input_file="$input_folder/$DAY"

  if [ ! -f "$input_file" ]; then
    mkdir -p "$input_folder"
    source ./.env
    >&2 echo "downloading input from https://adventofcode.com/$YEAR/day/$((DAY))/input"
    curl -s "https://adventofcode.com/$YEAR/day/$((DAY))/input" -H "Cookie:$SESSION" > $input_file
  fi

  cat "$input_file"
}

input=$(get_input)

echo "executing"
echo -e "\n=============================================================================================\n"

echo "$input" | time target/release/advent-of-code