#!/bin/sh

set -xe

cargo build --release
clear
perf stat -e task-clock,cycles,instructions,cache-references,cache-misses ./target/release/bfplan < test.json