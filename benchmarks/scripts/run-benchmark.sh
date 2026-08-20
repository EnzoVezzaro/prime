#!/bin/bash
# Prime Benchmark Runner
# Usage: ./benchmarks/scripts/run-benchmark.sh [--corpus pr|nightly] [--output path]
#
# Runs the Prime benchmark, validates the result, and optionally updates README.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
BENCH_DIR="$ROOT_DIR/benchmarks"
RESULT_FILE="$BENCH_DIR/results/latest.json"
CORPUS="pr"

while [[ $# -gt 0 ]]; do
    case $1 in
        --corpus)
            CORPUS="$2"
            shift 2
            ;;
        --output)
            RESULT_FILE="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1" >&2
            exit 1
            ;;
    esac
done

echo "=== Prime Benchmark ==="
echo "Corpus: $CORPUS"
echo "Output: $RESULT_FILE"
echo ""

# Step 1: Ensure release build
echo "Building release binary..."
cd "$ROOT_DIR/prime-rs"
cargo build --release --workspace 2>&1 | tail -3
echo ""

# Step 2: Run benchmark via CLI
echo "Running benchmark..."
cd "$ROOT_DIR"
"$ROOT_DIR/prime-rs/target/release/prime" -s /tmp/prime-bench-storage benchmark \
    --corpus "$CORPUS" \
    --output "$RESULT_FILE" \
    2>&1

# Step 3: Validate result
echo ""
echo "Validating result..."
python3 "$BENCH_DIR/scripts/validate-result.py" "$RESULT_FILE"

# Step 4: Update README
echo ""
echo "Updating README..."
python3 "$BENCH_DIR/scripts/update-readme.py" "$RESULT_FILE" "$ROOT_DIR/README.md"

echo ""
echo "=== Benchmark Complete ==="
echo "Result: $RESULT_FILE"
