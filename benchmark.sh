#!/bin/sh

set -xe

cargo build --release
clear
perf stat -e task-clock,cycles,instructions,cache-references,cache-misses cargo run --bin bfplan --release < test.json