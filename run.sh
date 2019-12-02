set -e

day=$1

source ./.env

rustc -C opt-level=0 -C debuginfo=2 src/$day.rs -o build/$day
curl -s https://adventofcode.com/2019/day/$((day))/input -H "Cookie:$SESSION" | build/$day