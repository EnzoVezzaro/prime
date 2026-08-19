Other languages analysis:

- Language coverage beyond TypeScript, Rust, Python, Go, Java:
  - C/C++: Eclipse CDT parser, tree-sitter C/C++ grammars
  - JavaScript/Node.js: tree-sitter JavaScript (official), extensive ecosystem
  - CSS/HTML: tree-sitter CSS, tree-sitter HTML (web markup languages)
  - Ruby: tree-sitter Ruby (community grammar)
  - PHP: tree-sitter PHP (community grammar)
  - Swift: tree-sitter Swift (community grammar) or SwiftCompiler API
  - Kotlin: tree-sitter Kotlin (community grammar) or KotlinCompiler API
  - C#: tree-sitter C# (community grammar) or Roslyn (Microsoft compiler platform)
  - Rust: already covered, but also GCC/clang-based parsing
  - Haskell: tree-sitter Haskell (community grammar) or GHC API
  - OCaml: tree-sitter OCaml (community grammar) or OCamlCompiler API
  - Scala: tree-sitter Scala (community grammar) or ScalaCompiler API

- Common challenges across languages:
  - Macros/metaprogramming: Code that generates/transforms other code at parse time
    - Usefulness: Prime marks symbols from macro-expanded code; may have reduced fidelity
  - Dynamicity: Languages where structure changes at runtime (Python, JavaScript)
    - Usefulness: Prime distinguishes static (compile-time) vs. dynamic (runtime) knowledge
  - Deprecated/removed features: Languages evolve; old features may remain in codebase
    - Usefulness: Prime marks confidence based on feature deprecation status
  - External dependencies: Languages relying on external runtimes/libraries
    - Usefulness: Prime extracts import/dependency information

- Cross-language mapping principles:
  - Structural vs. nominal typing: Map structural compatibility where possible (C#/TypeScript/Java Go interfaces)
  - Runtime vs. compile-time: Distinguish static knowledge (type annotations) from dynamic (behavior at runtime)
  - Abstraction level: Match abstraction levels (class → class, interface → trait, function → callable)
  - Language tags: Attach language tag to universal knowledge (enables agent to filter by language)

- Language capability model (from init-promt.md):
  - Each language adapter advertises capabilities:
    - parsing: exact (parse tree available), partial (limited parsing)
    - symbols: exact (symbol kinds known), partial (some kinds unknown)
    - references: exact (reference tracking available), partial (some references unresolvable)
    - types: partial (some type information available), none (no type system)
    - calls: partial (some call patterns resolvable), none (dynamic dispatch dominant)
    - architecture: inferred (high-level patterns recognizable), unavailable
    - runtime behavior: unavailable (requires execution data)

- Language adapter pattern:
  - Each language has adapter that extracts knowledge in language-specific way
  - Adapter produces universal knowledge (common vocabulary) + language tags + confidence levels
  - Prime consumes universal knowledge, ignores language-specific details (or tags them)
  - Usefulness: Extensible design; new language = new adapter, doesn't weaken universal model

- Reusable language adapter components:
  - Parser integration (tree-sitter, compiler API, built-in AST)
  - Symbol extraction (language-specific kinds → universal vocabulary)
  - Type extraction (language-specific types → universal type model + confidence)
  - Relationship extraction (calls, references, inheritance → universal relationship model)
  - Capability advertisement (parsing, symbols, references, types, calls, architecture, runtime behavior)
  - Confidence/provenance annotation (exact/inferred/unknown per knowledge entry)