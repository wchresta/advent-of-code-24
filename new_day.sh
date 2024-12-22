#!/usr/bin/env bash
set -euo pipefail

day=$1
if [[ -z $day ]]; then
    echo >&2 "Usage: $0 <day>"
    exit 1
fi

inputFile="inputs/day${day}"
if [[ ! -e $inputFile ]]; then
    echo "Downloading input file"
    sessionToken=$(cat ~/.config/aoc/session_token)
    wget --no-cookies --header "Cookie: session=${sessionToken}" -O "$inputFile" https://adventofcode.com/2024/day/${day}/input
fi


binFile="src/bin/day${day}.rs"
if [[ ! -e $binFile ]]; then
    testInput=$(
        xmlstarlet fo -H <(curl https://adventofcode.com/2024/day/$day) 2>/dev/null \
        | xmlstarlet sel -t -v '//pre[1]/code[1]' | head -c -1 | tr '\n' '@' | sed 's/@/\\n/g')

    cat day.rs.tmpl | sed "s/%DAY%/$day/" | sed "s/%TESTINPUT%/$testInput/" > "$binFile"
fi

