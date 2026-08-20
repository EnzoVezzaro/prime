#!/usr/bin/env python3
"""
Validate a Prime Benchmark Result file against the canonical schema.

Usage:
    python3 scripts/validate-result.py results/latest.json

Exit codes:
    0 - valid
    1 - invalid
    2 - file not found
"""

import json
import sys
import os

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
SCHEMA_PATH = os.path.join(SCRIPT_DIR, "..", "schemas", "prime-benchmark-result.schema.json")


def load_json(path: str) -> dict:
    with open(path, "r") as f:
        return json.load(f)


def validate(data: dict) -> list[str]:
    errors = []

    # Required top-level fields
    required = ["schema", "schema_version", "benchmark", "prime", "environment", "corpus", "benchmarks", "bmf", "integrity", "status"]
    for field in required:
        if field not in data:
            errors.append(f"Missing required field: {field}")

    if errors:
        return errors

    # Schema identifier
    if data["schema"] != "prime-benchmark-result":
        errors.append(f"Invalid schema identifier: {data['schema']}")

    # Status
    valid_statuses = {"complete", "partial", "not_run", "failed", "skipped"}
    if data["status"] not in valid_statuses:
        errors.append(f"Invalid status: {data['status']}")
    if data["status"] != "complete" and "reason" not in data:
        errors.append(f"Non-complete status '{data['status']}' requires 'reason' field")

    # Benchmark section
    bm = data.get("benchmark", {})
    for field in ["name", "version", "timestamp", "git_commit", "dirty"]:
        if field not in bm:
            errors.append(f"Missing benchmark.{field}")

    commit = bm.get("git_commit", "")
    if len(commit) < 7 or len(commit) > 40:
        errors.append(f"Invalid benchmark.git_commit length: {len(commit)}")

    # Prime section
    prime = data.get("prime", {})
    for field in ["version", "git_commit"]:
        if field not in prime:
            errors.append(f"Missing prime.{field}")

    # Environment section
    env = data.get("environment", {})
    for field in ["os", "arch", "cpu", "cpu_cores", "memory_bytes", "runtime", "runtime_version"]:
        if field not in env:
            errors.append(f"Missing environment.{field}")
    if env.get("cpu_cores", 0) < 1:
        errors.append("environment.cpu_cores must be >= 1")
    if env.get("memory_bytes", 0) < 0:
        errors.append("environment.memory_bytes must be >= 0")

    # Corpus
    corpus = data.get("corpus", [])
    if not isinstance(corpus, list):
        errors.append("corpus must be an array")
    else:
        for i, repo in enumerate(corpus):
            for field in ["name", "language", "commit", "files", "source_bytes", "lines_of_code"]:
                if field not in repo:
                    errors.append(f"corpus[{i}] missing field: {field}")

    # Benchmarks section
    benchmarks = data.get("benchmarks", [])
    if not isinstance(benchmarks, list):
        errors.append("benchmarks must be an array")

    # BMF
    bmf = data.get("bmf", {})
    if not isinstance(bmf, dict):
        errors.append("bmf must be an object")

    # Integrity section
    integrity = data.get("integrity", {})
    if not isinstance(integrity, dict):
        errors.append("integrity must be an object")
    else:
        for field in ["valid", "repositories_expected", "repositories_completed", "repositories_failed", "repositories_skipped", "metrics_valid", "warnings", "errors"]:
            if field not in integrity:
                errors.append(f"Missing integrity.{field}")

    # Check benchmark status values (benchmarks is an array of repo benchmarks)
    for i, repo_bench in enumerate(benchmarks):
        if isinstance(repo_bench, dict):
            if "status" in repo_bench:
                if repo_bench["status"] not in valid_statuses:
                    errors.append(f"benchmarks[{i}].status invalid: {repo_bench['status']}")
            # Check sub-sections (no status field in sub-sections anymore)
            for section_name in ["derivation", "artifact", "retrieval", "knowledge", "source_savings"]:
                if section_name in repo_bench:
                    section = repo_bench[section_name]
                    if isinstance(section, dict) and "status" in section:
                        if section["status"] not in valid_statuses:
                            errors.append(f"benchmarks[{i}].{section_name}.status invalid: {section['status']}")

    # No fake data check: if status is complete, at least one repo must have derivation timing
    if data["status"] == "complete":
        has_valid_derivation = False
        for repo_bench in benchmarks:
            if repo_bench.get("derivation", {}).get("time_ms", 0) > 0:
                has_valid_derivation = True
                break
        if not has_valid_derivation:
            errors.append("Complete benchmark must have at least one repo with positive derivation time")

        # Check integrity
        if not data.get("integrity", {}).get("valid", False):
            errors.append("Status is 'complete' but integrity.valid is false")

    return errors


def main():
    if len(sys.argv) < 2:
        print(f"Usage: {sys.argv[0]} <result.json>", file=sys.stderr)
        sys.exit(2)

    path = sys.argv[1]
    if not os.path.exists(path):
        print(f"File not found: {path}", file=sys.stderr)
        sys.exit(2)

    try:
        data = load_json(path)
    except json.JSONDecodeError as e:
        print(f"Invalid JSON: {e}", file=sys.stderr)
        sys.exit(1)

    errors = validate(data)
    if errors:
        print(f"Validation FAILED with {len(errors)} error(s):", file=sys.stderr)
        for err in errors:
            print(f"  - {err}", file=sys.stderr)
        sys.exit(1)
    else:
        print(f"Validation PASSED: {path}")
        print(f"  Schema: {data.get('schema')} v{data.get('schema_version')}")
        print(f"  Status: {data.get('status')}")
        print(f"  Benchmark: {data.get('benchmark', {}).get('name')} v{data.get('benchmark', {}).get('version')}")
        print(f"  Commit: {data.get('benchmark', {}).get('git_commit')}")
        print(f"  Corpus: {len(data.get('corpus', []))} repositories")
        sys.exit(0)


if __name__ == "__main__":
    main()
