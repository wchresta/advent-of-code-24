#!/usr/bin/env bash
DAY=$1

if ! (( $DAY > 0 )); then
    echo >&2 "Usage: $0 day"
    exit 1
fi

TARGET_FILE=src/bin/day$DAY.rs

if [[ -e $TARGET_FILE ]]; then
    echo >&2 "File $TARGET_FILE already exists"
    exit 1
fi

cat day.rs.tmpl | sed "s/%DAY%/$DAY" > "$TARGET_FILE"

echo "[[bin]]
name = "day$DAY"
path = "src/bin/day$DAY.rs"
" >> ./Cargo.toml
