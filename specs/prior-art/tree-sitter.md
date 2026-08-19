Tree-sitter research:

Repository: https://github.com/tree-sitter/tree-sitter

and: https://tree-sitter.github.io/tree-sitter/

Investigated topics:

- Parsing architecture: Tree-sitter is a parser generator tool and incremental parsing library. Key design decisions:
  - Uses context-free grammar (CFG) to parse programming languages
  - Generates a parser from grammar rules (written in a custom EBNF-like format)
  - Produces C code that can be compiled into an application
  - LALR(1) parser generation algorithm

- Concrete syntax trees (CST): Tree-sitter builds a concrete syntax tree that:
  - Retains all information from the source code (including comments, whitespace)
  - Is deterministic for a given input
  - Can be serialized and deserialized
  - Provides node type, start position, end position, and child relationships

- Incremental parsing: Tree-sitter's key feature - efficiently updates the syntax tree as source code is edited:
  - Uses a differential approach: only re-parses changed regions
  - Maintains parse state between edits
  - Can update in O(log n) or better for typical editor operations
  - Enables real-time syntax highlighting and error checking as you type

- Error recovery: Tree-sitter provides robust error handling:
  - Grammar can include error recovery rules
  - Parser can recover from syntax errors and continue parsing
  - Produces a partial CST even with errors
  - Useful for editor features (autocomplete, error highlighting) even with broken code

- Language support: Tree-sitter supports numerous languages through community-maintained grammars:
  - Official parsers: C, C++, CSS, HTML, Java, JavaScript, JSON, Markdown, etc.
  - Community parsers: 50+ languages including Go, Rust, Python, Ruby, etc.
  - Grammar format is reusable across languages

- Performance: Tree-sitter is designed for editor-like performance:
  - Parsing time: sub-millisecond for typical source files
  - Incremental update: typically < 1ms for editor keystrokes
  - Memory usage: low (parser is C11 library, no runtime dependencies)
  - Throughput: can parse on every keystroke in a text editor

- Memory usage: Very efficient:
  - Parser library is ~100KB compiled C code
  - No garbage collection required (C11)
  - Minimal runtime footprint
  - Can be embedded in any application

- Parsing large repositories: Tree-sitter excels at incremental parsing within a single file, but for whole-repository analysis:
  - Must parse each file individually
  - No built-in whole-repository indexing
  - Can be combined with other tools for repository-wide analysis
  - Performance scales with number of files × average file size

- Semantic limitations:
  - Tree-sitter produces a CST (concrete syntax tree), not an AST with semantic analysis
  - Does not resolve types, symbols, or references automatically
  - Does not understand language semantics (scope, types, inheritance)
  - Requires additional tools for semantic analysis

- Ecosystem:
  - Parser generator: tree-sitter CLI (generate parser from grammar)
  - Runtime library: tree-sitter C library (embeddable)
  - Language bindings: C#, Go, Haskell, Java, JavaScript, Python, Rust, Swift
  - Editor integrations: Neovim, Emacs, VS Code, Atom, Sublime Text
  - Tooling: tree-sitter-playground (online demo), tree-sitter-cli

- Bindings: Available for these languages (official):
  - C# (csharp-tree-sitter)
  - Go (go-tree-sitter)
  - Haskell (haskell-tree-sitter)
  - Java (JDK 22+)
  - JavaScript/Node.js (node-tree-sitter)
  - JavaScript/Wasm (binding_web)
  - Kotlin (kotlin-tree-sitter)
  - Python (py-tree-sitter)
  - Rust (binding_rust)
  - Swift (swift-tree-sitter)
  - Zig (zig-tree-sitter)

- Reusable components:
  - Parser generator framework (reusable for any language)
  - Incremental parsing library (reusable in editor/tool applications)
  - CST data structure (reusable representation)
  - C11 runtime library (embeddable in any application)
  - Grammar format (reusable across languages/Tools)

- Additional research value:
  - Used by major editors (Neovim's native parser, VS Code via extension)
  - Influenced by research on incremental parsing (Practical Algorithms for Incremental Software Development Environments)
  - Grammar development workflow and community practices