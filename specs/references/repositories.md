GitHub repositories research:

Key repositories for Prime research:

1. sourcegraph/scip (SCIP Code Intelligence Protocol)
   - Stars: 741, Forks: 74, Language: Go/Rust/TypeScript/Haskell
   - URL: https://github.com/sourcegraph/scip
   - License: Apache-2.0
   - Maturity: Production-grade (used in Sourcegraph code intelligence)
   - Maintainers: Sourcegraph company, active commits
   - Relevant components: scip.proto, indexer bindings, CLI tool
   - Prime relevance: Language-agnostic protocol, protobuf schema, indexer pattern

2. microsoft/lsif-node (LSIF Node Implementation)
   - Stars: ~1K (estimate), Forks: ~200 (estimate), Language: JavaScript/TypeScript
   - URL: https://github.com/microsoft/lsif-node
   - License: MIT
   - Maturity: Production-used (by VS Code, other tools)
   - Maintainers: Microsoft Open Source
   - Relevant components: LSIF output format, language server integration
   - Prime relevance: Standardized index format, vertex/edge model

3. joernio/joern (Code Property Graph Platform)
   - Stars: ~17K (estimate), Forks: ~3K (estimate), Language: Scala (core), frontends in Java/C++
   - URL: https://github.com/joernio/joern
   - License: Apache-2.0
   - Maturity: Production research platform (academic and commercial use)
   - Maintainers: CodeQL/Joern team, active community
   - Relevant components: CPG construction frontends, CPGQL query language, graph database
   - Prime relevance: CPG construction pipeline, graph query infrastructure

4. tree-sitter/tree-sitter (Parser Generator + Runtime)
   - Stars: ~22K, Forks: ~3.3K, Language: C (runtime), grammars in many languages
   - URL: https://github.com/tree-sitter/tree-sitter
   - License: MIT
   - Maturity: Production-used (Neovim, VS Code, many editors)
   - Maintainers: tree-sitter organization, community grammar maintainers
   - Relevant components: C11 runtime, parser generator, 50+ language grammars, incremental parsing
   - Prime relevance: Parser backend for multiple languages

5. ast-grep/ast-graph-sitter (Graph-based code grep/pattern matching)
   - Stars: ~6K (estimate), Forks: ~800 (estimate), Language: Rust
   - URL: https://github.com/ast-grep/ast-grep
   - License: Apache-2.0 (or similar OSS license)
   - Maturity: Early production (research/community use)
   - Maintainers: ast-grep open source team
   - Relevant components: AST-to-graph conversion, pattern query language
   - Prime relevance: Pattern-based code retrieval, graph-sitter research

6. microsoft/CodeQL (Semantic Code Analysis)
   - Stars: ~9K, Forks: ~1.5K, Language: QL (Query Language, based on JavaScript)
   - URL: https://github.com/microsoft/CodeQL
   - License: Apache-2.0
   - Maturity: Production (GitHub Code scanning)
   - Maintainers: Microsoft, GitHub security team
   - Relevant components: QL queries, database construction, taint analysis
   - Prime relevance: Semantic code analysis, data flow tracking

7. sourcegraph/sourcegraph (Code Intelligence Platform)
   - Stars: ~22K, Forks: ~3K, Language: Go, TypeScript, Python, Rust, etc.
   - URL: https://github.com/sourcegraph/sourcegraph
   - License: Apache-2.0
   - Maturity: Production (commercial product with free tier)
   - Maintainers: Sourcegraph company
   - Relevant components: Code navigation, symbol search, repository indexing, LSP integration
   - Prime relevance: Full code intelligence platform (comparison target, hybrid approach validation)

8. rust-analyzer/rust-analyzer (Rust Language Server)
   - Stars: ~13K, Forks: ~1K, Language: Rust
   - URL: https://github.com/rust-analyzer/rust-analyzer
   - License: Apache-2.0
   - Maturity: Production (de facto Rust language tooling)
   - Maintainers: rust-analyzer team, community contributors
   - Relevant components: Rust indexer (emits SCIP), semantic analysis, type resolution
   - Prime relevance: SCIP emitter, type inference model, incremental update handling

9. github/codeql-action (CodeQL GitHub Action)
   - Stars: ~2K, Forks: ~500, Language: JavaScript/TypeScript (action)
   - URL: https://github.com/github/codeql-action
   - License: Apache-2.0
   - Maturity: Production (GitHub Code scanning integration)
   - Maintainers: GitHub
   - Relevant components: CodeQL scanning, result presentation, automation
   - Prime relevance: Automated code analysis pipeline concept

10. prime-research/prime (Hypothetical - this research project)
    - Stars: N/A, Forks: N/A, Language: Research artifacts (Markdown, data formats)
    - URL: ./Prime (local research directory)
    - License: N/A (research project, no product implementation)
    - Maturity: Research phase (this is the current state)
    - Maintainers: Research agent (current session)
    - Relevant components: SPECS directory structure, research documents, comparison tables
    - Prime relevance: This document - the research repository itself

Repository license analysis:
- Apache-2.0: Most common (SCIP, Joern, LSIF-node, CodeQL, sourcegraph, rust-analyzer) - permissive, commercial-friendly
- MIT: Also common (tree-sitter, LMDB-related, ast-grep) - very permissive
- GPL/LGPL: Less common (some compiler tools), would require license compatibility analysis if used

Repository maturity assessment:
- Production-grade: Used in shipping products (Sourcegraph, CodeQL, rust-analyzer, tree-sitter in editors)
- Research/early production: Active development, community use, may have rough edges (Joern, LSIF-node, ast-grep)
- Library/tool: Supporting infrastructure (SCIP bindings, tree-sitter grammars)

Repository maintenance status:
- Active: Frequent commits, issue responses, release cycle (Sourcegraph, CodeQL, tree-sitter, rust-analyzer)
- Maintenance mode: Bug fixes, no new features (some older tools)
- Community-driven: Volunteer maintainers, variable response time (grammar collections, some indexers)

Prime reusable component identification:
- SCIP protocol: Reusable indexer protocol (language-agnostic knowledge extraction)
- LSIF format: Reusable index output format (standardized vertex/edge model)
- tree-sitter runtime: Reusable parser backend (C11, 50+ grammars, incremental)
- Joern CPG: Reusable graph construction + query pipeline (if adapting CPG approach)
- CodeQL QL: Reusable query language for code relationships (if adopting query-based retrieval)
- tree-sitter grammars: Reusable language parsers (50+ languages, incremental support)