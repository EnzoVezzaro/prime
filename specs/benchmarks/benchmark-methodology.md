Benchmark methodology research:

- Benchmark design for Prime validation:
  - Dimensions: Language, Scale, Polyglot, Incremental, Agent task type
  - Metrics: Latency, Throughput, Precision, Recall, Token usage, Agent task success rate
  - Datasets: Representative codebases at multiple scales

- Benchmark dimensions:
  - Language: Single-language (TypeScript, Rust, Python, Go, Java) and polyglot (mixture)
  - Scale: 
    - Small: 5 files, ~50 entities, ~100 relationships
    - Medium: 500 files, ~500 entities, ~2K relationships
    - Large: 50K files, ~5K entities, ~20K relationships
    - Monorepo: 500K+ files, ~5M entities, ~50M relationships
  - Polyglot: Repository containing multiple languages (e.g., TypeScript frontend, Rust backend, Go services, Python scripts)
  - Incremental: Benchmark with file changes during measurement (incremental update performance)
  - Agent task type: "find symbol", "find references", "navigate relationship", "explore context", "search codebase"

- Benchmark metrics:
  - Latency: Time (ms) for agent query to return results (per query type)
  - Throughput: Queries processed per second (across all queries of a type)
  - Precision@k: Of top k retrieved results, what fraction are relevant to agent task
  - Recall@k: Of relevant results for agent task, what fraction were retrieved in top k
  - Token usage: Total tokens retrieved per agent task (critical metric from init-promt.md)
  - Agent task success rate: Does agent complete task with retrieved context? (yes/no/partial)
  - Reclamation rate: How much knowledge becomes stale/invalid per unit time (incremental benchmark)

- Dataset design:
  - Small dataset: Single small open-source project (e.g., minimal CLI tool, ~5 files, ~50 symbols)
  - Medium dataset: Medium organization project (e.g., web application, ~500 files, ~500 symbols)
  - Large dataset: Large codebase (e.g., framework or platform, ~50K files, ~5K symbols)
  - Monorepo simulation: Artificially large repository (combine multiple projects, ~500K+ files, ~5M symbols)
  - Polyglot dataset: Repository with multiple languages (TypeScript + Rust + Go + Python, ~200 files each language)
  - Purpose: Cover benchmark dimensions from small to monorepo scale

- Benchmark execution protocol:
  1. Prepare dataset (index codebase, build knowledge artifact)
  2. Define agent queries (set of queries per task type, per scale, per language)
  3. Run queries (measure latency, token usage, precision/recall, agent task success)
  4. (Optional) Apply incremental changes (single file change), re-run queries (measure incremental update + re-query latency)
  5. Record results (in benchmark dataset file for future comparison)
  - Purpose: Structured, repeatable benchmark execution

- Benchmark result analysis:
  - Compare across dimensions (language × scale × task type)
  - Identify scalability bottlenecks (where performance degrades as scale increases)
  - Correlate metrics (e.g., does token usage correlate with agent task success rate?)
  - Inform Prime design decisions (what to optimize, what tradeoffs to accept)
  - Purpose: Turn benchmark data into actionable research conclusions

- Benchmark tooling considerations:
  - Automated query generation (set of queries per task type, scale, language)
  - Result logging (structured format for analysis: JSON, CSV, database)
  - Comparison tooling (diff results across benchmark runs, identify improvements/degradations)
  - Visualization (charts showing performance vs. scale, precision/recall curves, token usage over time)
  - Purpose: Support repeated benchmark execution and analysis