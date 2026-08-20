#!/usr/bin/env python3
"""
Update README.md benchmark section from a Prime Benchmark Result file.

Looks for <!-- PRIME_BENCHMARK_START --> and <!-- PRIME_BENCHMARK_END --> markers
and replaces the content between them.

Usage:
    python3 scripts/update-readme.py <result.json> [README.md]
"""

import json
import os
import re
import sys
import datetime


def load_result(path: str) -> dict:
    with open(path, "r") as f:
        return json.load(f)


def fmt_ms(val: float) -> str:
    """Format milliseconds for display."""
    if val < 1.0:
        return f"{val*1000:.0f} µs"
    elif val < 1000:
        return f"{val:.0f} ms"
    else:
        return f"{val/1000:.2f} s"


def fmt_bytes(val: int) -> str:
    if val < 1024:
        return f"{val} B"
    elif val < 1024 * 1024:
        return f"{val/1024:.1f} KB"
    else:
        return f"{val/(1024*1024):.1f} MB"


def fmt_us(val: float) -> str:
    if val < 1000:
        return f"{val:.0f} µs"
    else:
        return f"{val/1000:.1f} ms"


def generate_benchmark_section(data: dict) -> str:
    lines = []
    bm = data.get("benchmark", {})
    env = data.get("environment", {})
    benchmarks = data.get("benchmarks", [])
    corpus = data.get("corpus", [])
    integrity = data.get("integrity", {})

    # Compute aggregate metrics from per-repo benchmarks
    agg_derivation_time = 0.0
    agg_derivation_count = 0
    agg_artifact_bytes = 0
    agg_artifact_count = 0
    agg_artifact_ratio = 0.0
    agg_warm_median = 0.0
    agg_warm_p95 = 0.0
    agg_warm_count = 0
    agg_knowledge_total = 0
    agg_knowledge_correct = 0
    agg_knowledge_source_free_correct = 0
    
    # Retrieval metrics
    agg_entity_precision = 0.0
    agg_entity_recall = 0.0
    agg_entity_f1 = 0.0
    agg_rel_precision = 0.0
    agg_rel_recall = 0.0
    agg_rel_f1 = 0.0
    agg_mrr = 0.0
    agg_recall_at_1 = 0.0
    agg_recall_at_3 = 0.0
    agg_recall_at_5 = 0.0
    agg_recall_at_10 = 0.0
    agg_knowledge_count = 0

    for rb in benchmarks:
        if rb.get("status") == "complete":
            der = rb.get("derivation", {})
            if der.get("time_ms", 0) > 0:
                agg_derivation_time += der.get("time_ms", 0)
                agg_derivation_count += 1

            art = rb.get("artifact", {})
            if art.get("artifact_bytes", 0) > 0:
                agg_artifact_bytes += art.get("artifact_bytes", 0)
                agg_artifact_ratio += art.get("artifact_to_source_ratio", 0)
                agg_artifact_count += 1

            ret = rb.get("retrieval", {})
            warm = ret.get("warm", {})
            if warm.get("median", 0) > 0:
                agg_warm_median += warm.get("median", 0)
                agg_warm_p95 += warm.get("p95", 0)
                agg_warm_count += 1

            know = rb.get("knowledge", {})
            if know.get("total_questions", 0) > 0:
                agg_knowledge_total += know.get("total_questions", 0)
                agg_knowledge_correct += know.get("correct", 0)
                agg_knowledge_source_free_correct += know.get("source_free_correct", 0)
                agg_knowledge_count += 1
                
                # Retrieval metrics
                agg_entity_precision += know.get("entity_precision", 0.0)
                agg_entity_recall += know.get("entity_recall", 0.0)
                agg_entity_f1 += know.get("entity_f1", 0.0)
                agg_rel_precision += know.get("relationship_precision", 0.0)
                agg_rel_recall += know.get("relationship_recall", 0.0)
                agg_rel_f1 += know.get("relationship_f1", 0.0)
                agg_mrr += know.get("mrr", 0.0)
                agg_recall_at_1 += know.get("recall_at_1", 0.0)
                agg_recall_at_3 += know.get("recall_at_3", 0.0)
                agg_recall_at_5 += know.get("recall_at_5", 0.0)
                agg_recall_at_10 += know.get("recall_at_10", 0.0)

    lines.append("Latest benchmark:")
    lines.append("")
    lines.append("| Metric | Result |")
    lines.append("|--------|-------:|")

    # Derivation
    if agg_derivation_count > 0:
        mean_time = agg_derivation_time / agg_derivation_count
        lines.append(f"| Derivation | {fmt_ms(mean_time)} |")

    # Artifact
    if agg_artifact_count > 0:
        mean_artifact = agg_artifact_bytes / agg_artifact_count
        mean_ratio = agg_artifact_ratio / agg_artifact_count
        lines.append(f"| Artifact size | {fmt_bytes(mean_artifact)} |")
        lines.append(f"| Artifact/Source ratio | {mean_ratio:.3f} |")

    # Retrieval
    if agg_warm_count > 0:
        lines.append(f"| Retrieval p50 (warm) | {fmt_us(agg_warm_median / agg_warm_count)} |")
        lines.append(f"| Retrieval p95 (warm) | {fmt_us(agg_warm_p95 / agg_warm_count)} |")

    # Knowledge
    if agg_knowledge_total > 0:
        accuracy = agg_knowledge_correct / agg_knowledge_total
        source_free_accuracy = agg_knowledge_source_free_correct / agg_knowledge_total
        lines.append(f"| Accuracy | {accuracy*100:.1f}% |")
        lines.append(f"| Source-free accuracy | {source_free_accuracy*100:.1f}% |")
        
        if agg_knowledge_count > 0:
            lines.append(f"| Entity precision | {agg_entity_precision/agg_knowledge_count:.2f} |")
            lines.append(f"| Entity recall | {agg_entity_recall/agg_knowledge_count:.2f} |")
            lines.append(f"| Entity F1 | {agg_entity_f1/agg_knowledge_count:.2f} |")
            lines.append(f"| Relationship precision | {agg_rel_precision/agg_knowledge_count:.2f} |")
            lines.append(f"| Relationship recall | {agg_rel_recall/agg_knowledge_count:.2f} |")
            lines.append(f"| Relationship F1 | {agg_rel_f1/agg_knowledge_count:.2f} |")
            lines.append(f"| MRR | {agg_mrr/agg_knowledge_count:.2f} |")
            lines.append(f"| Recall@1 | {agg_recall_at_1/agg_knowledge_count:.1f}% |")
            lines.append(f"| Recall@3 | {agg_recall_at_3/agg_knowledge_count:.1f}% |")
            lines.append(f"| Recall@5 | {agg_recall_at_5/agg_knowledge_count:.1f}% |")
            lines.append(f"| Recall@10 | {agg_recall_at_10/agg_knowledge_count:.1f}% |")

    lines.append("")

    # Corpus
    if corpus:
        for repo in corpus:
            lines.append(f"Repository: `{repo['name']}` ({repo['language']}, {repo.get('size_category', 'unknown')})")
        lines.append("")

    # Integrity
    lines.append(f"Integrity: {'✅ Valid' if integrity.get('valid', False) else '❌ Invalid'}")
    lines.append(f"Repos: {integrity.get('repositories_completed', 0)}/{integrity.get('repositories_expected', 0)} completed")
    if integrity.get("warnings"):
        lines.append(f"Warnings: {', '.join(integrity['warnings'])}")
    lines.append("")

    # Commit info
    lines.append(f"Commit: `{bm.get('git_commit', 'unknown')}`")
    lines.append(f"Benchmark version: {bm.get('version', 'unknown')}")
    lines.append("")

    # Environment
    lines.append(f"Environment: {env.get('os', '?')} / {env.get('arch', '?')} / {env.get('cpu', '?')}")
    lines.append("")

    # Full result
    lines.append("Full machine-readable result: [`benchmarks/results/latest.json`](benchmarks/results/latest.json)")

    return "\n".join(lines)


def update_readme(readme_path: str, benchmark_section: str):
    with open(readme_path, "r") as f:
        content = f.read()

    start_marker = "<!-- PRIME_BENCHMARK_START -->"
    end_marker = "<!-- PRIME_BENCHMARK_END -->"

    if start_marker not in content:
        print(f"Warning: {start_marker} not found in {readme_path}", file=sys.stderr)
        print("Adding benchmark section at end of file.", file=sys.stderr)
        content += f"\n\n# Performance\n\n{start_marker}\n{benchmark_section}\n{end_marker}\n"
    else:
        pattern = re.compile(
            f"{re.escape(start_marker)}.*?{re.escape(end_marker)}",
            re.DOTALL,
        )
        replacement = f"{start_marker}\n{benchmark_section}\n{end_marker}"
        content = pattern.sub(replacement, content)

    with open(readme_path, "w") as f:
        f.write(content)

    print(f"Updated {readme_path} with benchmark results")


def main():
    if len(sys.argv) < 2:
        print(f"Usage: {sys.argv[0]} <result.json> [README.md]", file=sys.stderr)
        sys.exit(2)

    result_path = sys.argv[1]
    readme_path = sys.argv[2] if len(sys.argv) > 2 else os.path.join(
        os.path.dirname(SCRIPT_DIR), "..", "README.md"
    )

    data = load_result(result_path)
    section = generate_benchmark_section(data)
    update_readme(readme_path, section)


SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))

if __name__ == "__main__":
    main()
