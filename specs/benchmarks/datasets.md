Benchmark datasets research:

- Small dataset: Single small open-source project
  - Example: minimal CLI tool, simple library, toy project
  - Size: ~5 files, ~50 symbols, ~100 relationships
  - Languages: Typically single language (e.g., TypeScript or Python)
  - Purpose: Baseline benchmark; validate Prime architecture fundamentals; quick iteration
  - Indexing time: < 1 minute; Query latency: < 50ms; Full re-index on change: < 5 seconds

- Medium dataset: Medium organization project
  - Example: web application, API service, moderate-sized system
  - Size: ~500 files, ~500 symbols, ~2K relationships
  - Languages: Typically single language, possibly 2 (e.g., TypeScript + SQL config)
  - Purpose: Medium-scale validation; validate incremental update performance; agent usability testing
  - Indexing time: ~1-5 minutes; Query latency: < 100ms; Full re-index on change: < 30 seconds

- Large dataset: Large codebase
  - Example: framework, platform, extensive internal codebase
  - Size: ~50K files, ~5K symbols, ~20K relationships
  - Languages: Typically single language, possibly 2-3 (e.g., TypeScript + Python for scripts)
  - Purpose: Stress test scalability; validate partial loading/retrieval; performance at scale
  - Indexing time: ~30 minutes - 2 hours; Query latency: < 500ms (with mmap + chunked compression); Full re-index on change: < 5 minutes

- Monorepo simulation dataset
  - Artificially large repository combining multiple projects
  - Size: ~500K+ files, ~5M symbols, ~50M relationships (simulated, not physically 50M in memory)
  - Languages: Polyglot (TypeScript frontend, Rust backend, Go services, Python scripts, configuration)
  - Purpose: Ultimate scalability test; validate sharding/partitioning; distributed knowledge concepts
  - Indexing time: Hours to days (incremental incremental, not full re-index); Query latency: depends on shard layout
  - Partial loading: Only load relevant shard(s) for agent query; latency depends on shard size

- Polyglot dataset
  - Repository containing multiple programming languages simultaneously
  - Size: ~200 files per language (TypeScript, Rust, Go, Python, Java, configuration/languages)
  - Languages: TypeScript (frontend), Rust (backend), Go (services), Python (scripts/configuration), Java (legacy), YAML/JSON (infra)
  - Purpose: Validate language-agnosticism; test cross-language relationship tracking; evaluate language adapter pattern
  - Indexing time: ~30 minutes - 2 hours (per language adapter); Query latency: same as single-language (universal knowledge retrieval)
  - Cross-language queries: "find all implementations of this interface across languages"

- Dataset generation considerations:
  - Representative code: Use real open-source projects (with permission) or synthetic codebases designed to resemble real code
  - Language mix: Ensure datasets cover language features needed for Prime research (generics, inheritance, metaprogramming, etc.)
  - Relationship density: Vary (some symbols reference many others, most reference few) to test sparse graph handling
  - Change patterns: Simulate realistic change patterns (one file changes, one package changes, entire monorepo change)
  - Purpose: Benchmarks must valid Prime research questions, not just general performance

- Dataset availability:
  - Datasets should be open-source friendly (no proprietary code, or synthetic designed to resemble real code)
  - Version control: Datasets tracked with git (specific commit indexed)
  - Index artifacts: Index/storage artifacts included in dataset (for reproducibility (no re-index needed for re-runs))
  - Purpose: Benchmarks must be reproducible (same results given same dataset + benchmark parameters)