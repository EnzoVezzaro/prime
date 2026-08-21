# testing.md — Prime Research Testing Standards

This standard defines testing conventions for the Prime research project.

## Test Organization

- **Research validation tests**: `tests/validation/` for validating research hypotheses.
- **Tooling tests**: `tests/tooling/` for research tooling and scripts.
- **Fixtures**: `tests/fixtures/` for sample repositories and test data.

## Test Categories

| Category | Command | Purpose |
|----------|---------|---------|
| Research validation | Custom scripts | Validate research hypotheses against evidence. |
| Tooling | Language-appropriate | Fast, isolated logic tests for research tools. |
| Determinism | Custom | Byte-identical output across runs for research tools. |
| Integration | `acc check` on Prime repo | Framework validates itself. |

## Test Requirements

### All Research Tooling

- Unit tests for all public functions in research tools.
- Edge cases: empty input, invalid input, boundary conditions.
- Error paths tested (not just happy path).

### Research Validation

- Tests that verify research claims against primary sources.
- Tests that compare alternative approaches (e.g., compression ratios).
- Tests that validate benchmark methodology.

### ACC Integration

- `acc check` must pass on Prime repo (dogfooding).
- `acc graph`, `acc context`, `acc inspect`, `acc impact` all run without panicking.
- `--json` output of every command parses and matches schema.

## CI Pipeline

```yaml
# Pseudocode for CI stages
stages:
  - lint:        # language-appropriate linters
  - test:        # research tooling tests, validation tests
  - dogfood:     # acc check on Prime repo
  - schema:      # validate JSON outputs against ACC schema
  - determinism: # run acc context twice, diff output
```

## Coverage Targets

- Research tooling: ≥ 80% line coverage.
- Critical paths (validation, comparison tools): ≥ 90%.
- New tooling code: ≥ 90% for modified files.

## Test Data

- Use real fixture repositories in `tests/fixtures/`.
- Fixtures cover: simple, nested, cyclic, multi-language, edge cases.
- Fixtures are versioned with the code; update when validation logic changes.

## Benchmark Results Convention

All benchmark and testing runs must produce results in `benchmarks/results/`.

### Output Files

Each benchmark run produces TWO files:

1. **`latest.json`** — always overwritten with the most recent result
2. **`result_<timestamp>.json`** — timestamped copy for historical tracking

The timestamp format is `YYYYMMDDTHHMMSS` (e.g., `result_20260821T204449.json`).

### File Structure

```
benchmarks/results/
├── latest.json                    # Always points to the most recent run
├── result_20260821T204449.json    # Historical result
├── result_20260822T101530.json    # Historical result
└── comparison.md                  # Optional: human-readable comparison
```

### When to Write Results

- Every `prime benchmark` run writes both `latest.json` and a timestamped copy.
- Every heavy testing session should commit timestamped results for tracking.
- `latest.json` is ephemeral (replaced each run); timestamped files are permanent.

### Comparison Tracking

After each significant change, compare the latest result against previous results:

```bash
# List all historical results
ls -la benchmarks/results/result_*.json

# Compare latest against a previous result
python3 -c "import json; ..." benchmarks/results/latest.json benchmarks/results/result_YYYYMMDDTHHMMSS.json
```

## Research-Specific Validation

- Evidence verification tests: verify that cited sources actually support claims.
- Comparison tests: verify comparison tables are accurate and up-to-date.
- Hypothesis tracking: tests that verify open questions are tracked and hypotheses are labeled.