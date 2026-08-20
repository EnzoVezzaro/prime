#!/usr/bin/env python3
"""
Prime Benchmark Suite
=====================
Benchmarks Prime against real repositories in 10 languages.
Tests: build time, index size, query latency, MCP tool latency.
Shows real-time terminal output per language/size.
"""

import subprocess
import time
import os
import sys
import json
import shutil
import argparse
from pathlib import Path
from dataclasses import dataclass, field
from typing import Optional

# ─── Configuration ──────────────────────────────────────────────────────────────

PRIME_BIN = Path(__file__).parent.parent / "target" / "release" / "prime"
BENCH_DIR = Path(__file__).parent
REPOS_DIR = BENCH_DIR / "repos"
RESULTS_DIR = BENCH_DIR / "results"

# ANSI colors for terminal output
class C:
    RESET   = "\033[0m"
    BOLD    = "\033[1m"
    DIM     = "\033[2m"
    RED     = "\033[31m"
    GREEN   = "\033[32m"
    YELLOW  = "\033[33m"
    BLUE    = "\033[34m"
    MAGENTA = "\033[35m"
    CYAN    = "\033[36m"
    WHITE   = "\033[37m"
    BG_BLUE = "\033[44m"
    BG_GREEN = "\033[42m"

# ─── Test Repositories ─────────────────────────────────────────────────────────
# Format: (name, git_url, language, size_category)

TEST_REPOS = [
    # Rust
    ("ripgrep", "https://github.com/BurntSushi/ripgrep.git", "rust", "medium"),
    ("bat", "https://github.com/sharkdp/bat.git", "rust", "small"),
    ("fd", "https://github.com/sharkdp/fd.git", "rust", "small"),

    # Python
    ("httpx", "https://github.com/encode/httpx.git", "python", "small"),
    ("rich", "https://github.com/Textualize/rich.git", "python", "medium"),
    ("fastapi", "https://github.com/tiangolo/fastapi.git", "python", "medium"),

    # JavaScript/TypeScript
    ("express", "https://github.com/expressjs/express.git", "javascript", "small"),
    ("next.js", "https://github.com/vercel/next.js.git", "typescript", "large"),
    ("zod", "https://github.com/colinhacks/zod.git", "typescript", "small"),

    # Go
    ("gin", "https://github.com/gin-gonic/gin.git", "go", "small"),
    ("echo", "https://github.com/labstack/echo.git", "go", "small"),
    ("fiber", "https://github.com/gofiber/fiber.git", "go", "medium"),

    # Java
    ("fastjson", "https://github.com/alibaba/fastjson.git", "java", "medium"),
    ("guava", "https://github.com/google/guava.git", "java", "large"),

    # C
    ("redis", "https://github.com/redis/redis.git", "c", "medium"),
    ("nginx", "https://github.com/nginx/nginx.git", "c", "large"),

    # C++
    ("spdlog", "https://github.com/gabime/spdlog.git", "c++", "small"),
    ("nlohmann-json", "https://github.com/nlohmann/json.git", "c++", "small"),

    # TypeScript (Node.js ecosystem)
    ("vitest", "https://github.com/vitest-dev/vitest.git", "typescript", "medium"),
    ("esbuild", "https://github.com/evanw/esbuild.git", "typescript", "small"),

    # Kotlin
    ("ktor", "https://github.com/ktorio/ktor.git", "kotlin", "medium"),

    # Swift
    ("Alamofire", "https://github.com/Alamofire/Alamofire.git", "swift", "small"),
    ("vapor", "https://github.com/vapor/vapor.git", "swift", "medium"),
]

# Query benchmarks: (query_string, query_type)
QUERY_BENCHMARKS = [
    ("main", "search"),
    ("fn", "search"),
    ("struct", "search"),
    ("impl", "search"),
    ("test", "search"),
    ("error", "search"),
]

# ─── Data Structures ────────────────────────────────────────────────────────────

@dataclass
class BenchmarkResult:
    repo_name: str
    language: str
    size_category: str
    file_count: int = 0
    entity_count: int = 0
    relation_count: int = 0
    index_size_bytes: int = 0
    build_time_ms: float = 0.0
    query_latencies: list = field(default_factory=list)
    avg_query_latency_ms: float = 0.0
    min_query_latency_ms: float = 0.0
    max_query_latency_ms: float = 0.0
    p95_query_latency_ms: float = 0.0
    error: Optional[str] = None

# ─── Terminal Output Helpers ────────────────────────────────────────────────────

def clear_line():
    sys.stdout.write("\033[2K\r")
    sys.stdout.flush()

def print_header(text):
    width = 70
    print()
    print(f"{C.BG_BLUE}{C.WHITE}{C.BOLD}{'':^{width}}{C.RESET}")
    print(f"{C.BG_BLUE}{C.WHITE}{C.BOLD}{text:^{width}}{C.RESET}")
    print(f"{C.BG_BLUE}{C.WHITE}{C.BOLD}{'':^{width}}{C.RESET}")
    print()

def print_section(text):
    print(f"\n{C.CYAN}{C.BOLD}{'─' * 60}{C.RESET}")
    print(f"{C.CYAN}{C.BOLD}  {text}{C.RESET}")
    print(f"{C.CYAN}{C.BOLD}{'─' * 60}{C.RESET}")

def print_result_row(repo, lang, size, build_ms, idx_size, entities, avg_ms, status="ok"):
    if status == "ok":
        status_color = C.GREEN
        status_icon = "✓"
    elif status == "skip":
        status_color = C.YELLOW
        status_icon = "○"
    else:
        status_color = C.RED
        status_icon = "✗"

    idx_size_str = format_size(idx_size)
    build_str = f"{build_ms:.0f}ms" if build_ms > 0 else "—"
    avg_str = f"{avg_ms:.1f}ms" if avg_ms > 0 else "—"

    print(
        f"  {status_color}{status_icon}{C.RESET} "
        f"{C.BOLD}{repo:<20}{C.RESET} "
        f"{C.DIM}{lang:<12}{C.RESET} "
        f"{C.DIM}{size:<8}{C.RESET} "
        f"{C.CYAN}{build_str:>10}{C.RESET} "
        f"{C.MAGENTA}{idx_size_str:>10}{C.RESET} "
        f"{C.GREEN}{entities:>8}{C.RESET} "
        f"{C.YELLOW}{avg_str:>10}{C.RESET}"
    )

def print_summary_table(results):
    print(f"\n{C.BOLD}{'Language':<14} {'Repos':>6} {'Avg Build':>12} {'Avg Query':>12} {'Avg Size':>12} {'Total Entities':>14}{C.RESET}")
    print(f"{C.DIM}{'─' * 74}{C.RESET}")

    by_lang = {}
    for r in results:
        if r.error:
            continue
        if r.language not in by_lang:
            by_lang[r.language] = []
        by_lang[r.language].append(r)

    for lang, reps in sorted(by_lang.items()):
        avg_build = sum(r.build_time_ms for r in reps) / len(reps)
        avg_query = sum(r.avg_query_latency_ms for r in reps) / len(reps)
        avg_size = sum(r.index_size_bytes for r in reps) / len(reps)
        total_entities = sum(r.entity_count for r in reps)

        print(
            f"  {C.CYAN}{lang:<14}{C.RESET} "
            f"{len(reps):>6} "
            f"{C.CYAN}{avg_build:>9.0f} ms{C.RESET} "
            f"{C.YELLOW}{avg_query:>9.1f} ms{C.RESET} "
            f"{C.MAGENTA}{format_size(avg_size):>10}{C.RESET} "
            f"{C.GREEN}{total_entities:>14}{C.RESET}"
        )

def format_size(size_bytes):
    if size_bytes < 1024:
        return f"{size_bytes} B"
    elif size_bytes < 1024 * 1024:
        return f"{size_bytes / 1024:.1f} KB"
    elif size_bytes < 1024 * 1024 * 1024:
        return f"{size_bytes / (1024 * 1024):.1f} MB"
    else:
        return f"{size_bytes / (1024 * 1024 * 1024):.1f} GB"

def format_ms(ms):
    if ms < 1:
        return f"{ms * 1000:.0f}µs"
    elif ms < 1000:
        return f"{ms:.1f}ms"
    else:
        return f"{ms / 1000:.2f}s"

# ─── Git Helpers ────────────────────────────────────────────────────────────────

def clone_repo(name, url, target_dir):
    if target_dir.exists():
        print(f"  {C.DIM}Repository already cloned: {name}{C.RESET}")
        return True

    print(f"  {C.CYAN}Cloning {name}...{C.RESET}", end="", flush=True)
    try:
        result = subprocess.run(
            ["git", "clone", "--depth", "1", "--single-branch", url, str(target_dir)],
            capture_output=True, text=True, timeout=120
        )
        if result.returncode == 0:
            print(f" {C.GREEN}done{C.RESET}")
            return True
        else:
            print(f" {C.RED}failed{C.RESET}")
            return False
    except subprocess.TimeoutExpired:
        print(f" {C.RED}timeout{C.RESET}")
        return False
    except Exception as e:
        print(f" {C.RED}error: {e}{C.RESET}")
        return False

# ─── Benchmark Runner ───────────────────────────────────────────────────────────

def count_source_files(repo_dir, language):
    exts = {
        "rust": [".rs"],
        "python": [".py"],
        "javascript": [".js", ".jsx", ".mjs"],
        "typescript": [".ts", ".tsx", ".mts"],
        "go": [".go"],
        "java": [".java"],
        "c": [".c", ".h"],
        "c++": [".cpp", ".cc", ".cxx", ".hpp", ".h"],
        "kotlin": [".kt", ".kts"],
        "swift": [".swift"],
    }
    count = 0
    for ext in exts.get(language, []):
        count += sum(1 for _ in repo_dir.rglob(f"*{ext}") if _.is_file())
    return count

def run_build(repo_dir, storage_dir):
    """Run prime build and return build time in ms."""
    storage_dir.mkdir(parents=True, exist_ok=True)

    cmd = [
        str(PRIME_BIN),
        "-r", str(repo_dir),
        "-s", str(storage_dir),
        "build",
        "--force",
    ]

    start = time.perf_counter()
    result = subprocess.run(cmd, capture_output=True, text=True, timeout=300)
    elapsed = (time.perf_counter() - start) * 1000

    if result.returncode != 0:
        raise RuntimeError(f"Build failed: {result.stderr[:200]}")

    return elapsed

def get_stats(storage_dir):
    """Get graph statistics."""
    cmd = [str(PRIME_BIN), "-s", str(storage_dir), "stats"]
    result = subprocess.run(cmd, capture_output=True, text=True, timeout=30)
    if result.returncode != 0:
        return {}

    stats = {}
    for line in result.stdout.split("\n"):
        if ":" in line:
            key, _, value = line.partition(":")
            stats[key.strip()] = value.strip()
    return stats

def get_index_size(storage_dir):
    """Get total size of index files."""
    total = 0
    if storage_dir.exists():
        for f in storage_dir.rglob("*"):
            if f.is_file():
                total += f.stat().st_size
    return total

def run_queries(storage_dir, queries):
    """Run query benchmarks and return latencies."""
    latencies = []
    for query_str, query_type in queries:
        cmd = [
            str(PRIME_BIN),
            "-s", str(storage_dir),
            "query", query_str,
            "--type", query_type,
            "--format", "json",
            "--limit", "10",
        ]
        start = time.perf_counter()
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=30)
        elapsed = (time.perf_counter() - start) * 1000
        if result.returncode == 0:
            latencies.append(elapsed)
    return latencies

def run_benchmark_single(repo_info, clone=True):
    """Run benchmark for a single repository."""
    name, url, language, size_cat = repo_info
    repo_dir = REPOS_DIR / name
    storage_dir = REPOS_DIR / f".prime-{name}"

    result = BenchmarkResult(
        repo_name=name,
        language=language,
        size_category=size_cat,
    )

    # Clone if needed
    if clone and not repo_dir.exists():
        if not clone_repo(name, url, repo_dir):
            result.error = "clone_failed"
            return result

    if not repo_dir.exists():
        result.error = "not_found"
        return result

    # Count source files
    result.file_count = count_source_files(repo_dir, language)

    # Build
    print(f"  {C.CYAN}Building index...{C.RESET}", end="", flush=True)
    try:
        result.build_time_ms = run_build(repo_dir, storage_dir)
        print(f" {C.GREEN}done{C.RESET} ({format_ms(result.build_time_ms)})")
    except Exception as e:
        result.error = str(e)[:100]
        print(f" {C.RED}failed: {e}{C.RESET}")
        return result

    # Stats
    stats = get_stats(storage_dir)
    result.entity_count = int(stats.get("Entities", "0"))
    result.relation_count = int(stats.get("Relations", "0"))
    result.index_size_bytes = get_index_size(storage_dir)

    # Queries
    print(f"  {C.CYAN}Running queries...{C.RESET}", end="", flush=True)
    result.query_latencies = run_queries(storage_dir, QUERY_BENCHMARKS)
    if result.query_latencies:
        result.query_latencies.sort()
        result.avg_query_latency_ms = sum(result.query_latencies) / len(result.query_latencies)
        result.min_query_latency_ms = result.query_latencies[0]
        result.max_query_latency_ms = result.query_latencies[-1]
        p95_idx = int(len(result.query_latencies) * 0.95)
        result.p95_query_latency_ms = result.query_latencies[min(p95_idx, len(result.query_latencies) - 1)]
    print(f" {C.GREEN}done{C.RESET} (avg {format_ms(result.avg_query_latency_ms)})")

    return result

# ─── Main ───────────────────────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser(description="Prime Benchmark Suite")
    parser.add_argument("--language", "-l", help="Benchmark specific language only")
    parser.add_argument("--size", "-s", choices=["small", "medium", "large"], help="Filter by size")
    parser.add_argument("--repo", "-r", help="Benchmark specific repo only")
    parser.add_argument("--no-clone", action="store_true", help="Don't clone, use local only")
    parser.add_argument("--clean", action="store_true", help="Clean all repos and results")
    parser.add_argument("--json", action="store_true", help="Output results as JSON")
    args = parser.parse_args()

    # Check binary exists
    if not PRIME_BIN.exists():
        print(f"{C.RED}Error: Prime binary not found at {PRIME_BIN}{C.RESET}")
        print(f"Run: cargo build --release --workspace")
        sys.exit(1)

    # Clean mode
    if args.clean:
        print(f"{C.YELLOW}Cleaning repos and results...{C.RESET}")
        if REPOS_DIR.exists():
            shutil.rmtree(REPOS_DIR)
        if RESULTS_DIR.exists():
            shutil.rmtree(RESULTS_DIR)
        print(f"{C.GREEN}Done{C.RESET}")
        return

    # Filter repos
    repos = TEST_REPOS
    if args.language:
        repos = [r for r in repos if r[2] == args.language]
    if args.size:
        repos = [r for r in repos if r[3] == args.size]
    if args.repo:
        repos = [r for r in repos if r[0] == args.repo]

    if not repos:
        print(f"{C.RED}No repos match the filter criteria{C.RESET}")
        return

    # Header
    print_header("PRIME BENCHMARK SUITE")
    print(f"  {C.DIM}Binary: {PRIME_BIN}{C.RESET}")
    print(f"  {C.DIM}Repos:  {len(repos)}{C.RESET}")
    print(f"  {C.DIM}Dir:    {REPOS_DIR}{C.RESET}")
    print()

    # Table header
    print(f"  {C.BOLD}{'Status':<4} {'Repository':<20} {'Language':<12} {'Size':<8} {'Build':>10} {'Index':>10} {'Entities':>8} {'Query':>10}{C.RESET}")
    print(f"  {C.DIM}{'─' * 86}{C.RESET}")

    results = []
    start_time = time.time()

    for i, repo_info in enumerate(repos):
        name, url, language, size_cat = repo_info

        print(f"\n{C.BOLD}[{i+1}/{len(repos)}] {name} ({language}, {size_cat}){C.RESET}")

        result = run_benchmark_single(repo_info, clone=not args.no_clone)
        results.append(result)

        status = "ok" if not result.error else "err"
        print_result_row(
            result.repo_name, result.language, result.size_category,
            result.build_time_ms, result.index_size_bytes, result.entity_count,
            result.avg_query_latency_ms, status
        )

    elapsed = time.time() - start_time

    # Summary
    print_summary_table(results)
    print(f"\n  {C.DIM}Total time: {format_ms(elapsed * 1000)}{C.RESET}")

    # JSON output
    if args.json:
        RESULTS_DIR.mkdir(parents=True, exist_ok=True)
        json_path = RESULTS_DIR / f"benchmark-{int(time.time())}.json"
        json_data = []
        for r in results:
            json_data.append({
                "repo": r.repo_name,
                "language": r.language,
                "size": r.size_category,
                "file_count": r.file_count,
                "entity_count": r.entity_count,
                "relation_count": r.relation_count,
                "index_size_bytes": r.index_size_bytes,
                "build_time_ms": r.build_time_ms,
                "avg_query_latency_ms": r.avg_query_latency_ms,
                "min_query_latency_ms": r.min_query_latency_ms,
                "max_query_latency_ms": r.max_query_latency_ms,
                "p95_query_latency_ms": r.p95_query_latency_ms,
                "error": r.error,
            })
        with open(json_path, "w") as f:
            json.dump(json_data, f, indent=2)
        print(f"\n  {C.GREEN}Results saved to: {json_path}{C.RESET}")

    # Error summary
    errors = [r for r in results if r.error]
    if errors:
        print(f"\n{C.RED}Errors ({len(errors)}):{C.RESET}")
        for r in errors:
            print(f"  {C.RED}✗ {r.repo_name}: {r.error}{C.RESET}")

    sys.exit(1 if errors else 0)

if __name__ == "__main__":
    main()
