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

cp day.rs.tmpl "$TARGET_FILE"
