Existing benchmarks research:

- Parsing benchmarks:
  - Geometric performance: Measure parsing throughput (files/second, lines/second)
  - Tree-sitter benchmarks: Typical ~10K-50K lines/minute incremental update rate
  - Purpose: Establish baseline parsing performance for Prime comparison
  - Relevant tools: tree-sitter benchmarks, custom parser benchmarks

- Code search benchmarks:
  - Query latency: Time to return results for code search queries
  - Precision/recall: Measure accuracy of search results (lexical, semantic, hybrid)
  - Dataset: Typical codebase subsets (1K-10K symbols queried)
  - Purpose: Establish baseline search performance for Prime comparison

- Static analysis benchmarks:
  - Analysis throughput: Time to perform full static analysis (data flow, control flow, type checking)
  - Scalability: How analysis time scales with codebase size (linear, O(n log n), O(n²))
  - Purpose: Establish baseline static analysis performance for Prime comparison

- Graph traversal benchmarks:
  - Query latency: Time for graph traversal queries (neighbor lookup, reachability, shortest path)
  - Graph size: Number of nodes (symbols) and edges (relationships) in benchmarked graph
  - Purpose: Establish baseline graph query performance for Prime comparison

- Indexing benchmarks:
  - Index construction time: Time to build index from source code
  - Index size: Disk/memory size of constructed index
  - Update latency: Time to apply incremental update (single file change)
  - Purpose: Establish baseline indexing performance for Prime comparison

- Storage benchmarks:
  - Read latency: Time to read knowledge entry/ies from storage backend
  - Write latency: Time to write knowledge entry/ies to storage backend
  - Scale: Benchmark at 1K, 10K, 100K, 1M entities
  - Purpose: Establish baseline storage performance for Prime comparison

- Compression benchmarks:
  - Compression ratio: Output size / input size (bytes)
  - Compression CPU time: Time to compress (decompress)
  - Decompression CPU time: Time to decompress
  - Random access: Time to decompress specific chunk without full decompress
  - Purpose: Establish baseline compression performance for Prime comparison

- Repository retrieval benchmarks:
  - End-to-end: Time from agent query to retrieved knowledge
  - Query types: "find symbol", "find references", "navigate relationship", "explore context"
  - Agent experience: Subjective assessment (agent task completion time, knowledge usefulness)
  - Purpose: Establish baseline agent retrieval performance for Prime comparison

- Coding agent benchmarks:
  - Task completion time: Time for agent to complete defined task (feature implementation, bug fix, understanding)
  - Token usage: Total tokens consumed during task completion
  - Knowledge quality: Subjective/ objective measure of knowledge usefulness for task
  - Purpose: Establish baseline coding agent performance for Prime comparison

- Benchmark methodology principles (from init-promt.md):
  - Do not run large experiments yet unless useful for validating a research question
  - Design benchmark methodology for future use (datasets, query types, metrics)
  - Test architecture against small, large, and polyglot repositories from the beginning
  - Purpose: Ensure Prime architecture validated across scales from beginning

- Benchmark design considerations:
  - Dimensions: Language (TypeScript, Rust, Python, Go, Java, polyglot), Scale (5 files, 500 files, 50K files, 5M entities), Polyglot (single-language, multi-language)
  - Metrics: Latency (query time), Throughput (queries/second), Precision@k, Recall@k, Token usage, Agent task success rate
  - Datasets: Representative codebases (small open-source project, medium monorepo subset, large monorepo simulation)
  - Purpose: Structured benchmark design for Prime validation