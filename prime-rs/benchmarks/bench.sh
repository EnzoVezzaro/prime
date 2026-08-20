#!/bin/bash
# Prime Benchmark Runner
# Usage: ./bench.sh [options]
#   --language rust     Benchmark only Rust repos
#   --size small        Benchmark only small repos
#   --repo ripgrep      Benchmark only ripgrep
#   --clean             Remove all cloned repos and results
#   --json              Save results as JSON

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PRIME_DIR="$(dirname "$SCRIPT_DIR")"

# Ensure release build exists
if [ ! -f "$PRIME_DIR/target/release/prime" ]; then
    echo "Building release binary..."
    cd "$PRIME_DIR" && cargo build --release --workspace
fi

# Run benchmarks
python3 "$SCRIPT_DIR/bench.py" "$@"
