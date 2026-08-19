Reusable parsers research:

- Parser library reuse: Existing parser libraries that can be embedded in Prime:
  - tree-sitter C11 runtime library (embeddable, no runtime dependencies)
    - Language bindings: C#, Go, Haskell, Java, JavaScript, Python, Rust, Swift, Zig
    - Usefulness: Prime can use tree-sitter as parser backend for multiple languages
  - tree-sitter grammars: 50+ language grammars (community-maintained)
    - Usefulness: Parser for each language frontend
  - Compiler APIs: language-specific compiler frontends (e.g., rustc AST, javac AST, gcc AST)
    - More precise than tree-sitter (full semantic analysis available)
    - Usefulness: Prime can use compiler APIs for languages where tree-sitter insufficient
  - Language Server Protocol (LSP) clients: communicate with language servers (tsserver, pyright, rust-analyzer, etc.)
    - LSP provides: parsing, symbol extraction, type information, references, diagnostics
    - Usefulness: Prime can reuse LSP servers as knowledge extraction backends
  - custom parser combinations: For languages without good tooling, combine partial parsers

- Parser integration design for Prime:
  - Each language has a parser adapter that:
    1. Invokes appropriate parser (tree-sitter, compiler API, LSP)
    2. Extracts universal knowledge (symbols, types, relationships, confidence)
    2. Tags with language identifier and confidence levels
  - Prime's universal model remains language-agnostic; adapters handle language-specific details
  - Usefulness: Extensible design; new language = new adapter, doesn't weaken universal model

- tree-sitter as primary parser backend:
  - Advantages: C11 runtime (embeddable), 50+ grammars, incremental parsing, editor-proven
  - Disadvantages: CST-only (no semantic analysis), limited type inference, error recovery imperfect
  - Prime relevance: Primary parser backend for supported languages; supplement with compiler APIs where needed

- Compiler API integration:
  - Rust: rustc AST (full semantic information, ownership/lifetime data)
  - Java: javac/compiler AST (full type information, generics, annotations)
  - Go: go/ast (built-in, sufficient for symbol extraction)
  - Python: built-in ast module (sufficient for symbol extraction, limited type info)
  - Usefulness: Prime uses compiler APIs for languages where tree-sitter insufficient for required knowledge

- LSP client integration:
  - Communicate with language server via LSP protocol
  - Request: document symbols, type definitions, references, completions, hover information
  - Usefulness: Reuse established language server implementations (many languages supported)
  - Tradeoffs: Runtime dependency on language server process; latency per request; version skew

- Parser reuse evaluation criteria:
  - Language support: Does it cover target languages?
  - Knowledge richness: Does it provide symbols, types, relationships, confidence?
  - Incremental support: Does it support incremental updates (important for Prime)?
  - Embeddability: Can it be embedded (no heavy runtime) or requires server process?
  - Performance: Parsing speed, memory usage
  - Licensing: Permissive license for inclusion in research project
  - Prime relevance: Evaluate each candidate against these criteria

- Reusable parser components:
  - Parser adapter interface (uniform interface for all language parsers)
  - tree-sitter integration (C runtime + language grammars)
  - Compiler API wrappers (rustc, javac, go/ast, Python ast)
  - LSP client wrapper (generic LSP client, language-specific requests)
  - Knowledge extraction modules (extract symbols/types/relationships from parser output)
  - Confidence annotation module (exact/inferred/unknown per knowledge entry)