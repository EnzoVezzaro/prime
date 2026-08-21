# Semgrep — Prior Art Analysis

**Confidence:** OBSERVATION (usage), HYPOTHESIS (optimization claims)
**Primary Sources:** Semgrep documentation, GitHub repository, academic papers
**Last Updated:** August 2026

## Executive Summary

Semgrep is a fast, open-source static analysis tool that combines pattern matching with semantic analysis. It uses simple patterns augmented with taint tracking for security-focused code analysis.

## Core Architecture

### Pattern Language

Semgrep's pattern language is intentionally simple:

```yaml
# Find hardcoded passwords
pattern: |
  password = "..."
  
# Find SQL injection (taint mode)
mode: taint
pattern: |
  $QUERY = f"SELECT ... {$INPUT} ..."
  $DB.execute($QUERY)
sources:
  - request.args
sinks:
  - $DB.execute
```

### Key Design Principles

1. **Simplicity over power:** Patterns are easy to write and read
2. **Fast execution:** No compilation step, direct pattern matching
3. **Language-agnostic:** Same patterns work across Python, JS, Go, Java, etc.
4. **Taint tracking:** Built-in support for data flow analysis

## Performance Characteristics

| Metric | Value | Notes |
|--------|-------|-------|
| Pattern match time | 1-10ms per file | Near-instant |
| Full scan time | 1-30s per 10K LOC | Much faster than CodeQL |
| Memory usage | 50-200MB | Lightweight |
| Rule compilation | <100ms | Pattern-based, no Datalog |

**OBSERVATION:** Semgrep prioritizes speed and simplicity over analytical depth.

## Comparison with Prime

| Dimension | Semgrep | Prime |
|-----------|---------|-------|
| **Approach** | Pattern matching + taint | Full semantic graph |
| **Query complexity** | Simple patterns | Complex traversals |
| **Speed** | Very fast | Fast (in-memory) |
| **Analysis depth** | Taint only | Multi-dimensional |
| **Agent use case** | Quick vulnerability checks | Deep code understanding |

## Key Findings for Prime

### ✅ Strengths to Adopt

1. **Fast pattern matching:** For simple queries (find function, find class), pattern matching is faster than full graph traversal.

2. **Taint tracking:** Semgrep's taint mode is useful for security-focused agent queries.

3. **Simplicity:** Patterns are easy for agents to generate and validate.

### ❌ Limitations to Avoid

1. **Limited query expressiveness:** Cannot express complex graph traversals.

2. **No incremental updates:** Full scan required for each change.

3. **No agent envelope:** No provenance or token-budget responses.

## Open Questions

- [ ] Can Semgrep's pattern matching be integrated into Prime for simple queries?
- [ ] Is Semgrep's taint tracking sufficient for agent code intelligence?
- [ ] Can Prime use Semgrep as a fast-path for simple queries?

## Confidence Assessment

| Claim | Confidence |
|-------|------------|
| Semgrep uses pattern-based matching | **FACT** (documentation) |
| Semgrep is faster than CodeQL | **FACT** (benchmark comparisons) |
| Semgrep supports taint tracking | **FACT** (documentation) |
| Pattern matching is sufficient for agents | **HYPOTHESIS** (requires validation) |
