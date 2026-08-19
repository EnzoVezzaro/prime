References research:

- Symbol references: Usages of a symbol definition within source code. Includes:
  - Direct references (textual occurrences of symbol name)
  - Indirect references (through dynamic dispatch, higher-order functions)
  - Usefulness: High for understanding symbol usage patterns

- Cross-file references: References to symbols defined in other files:
  - Import/path-based references
  - Usefulness: High for understanding codebase structure and dependencies

- Reference confidence: Confidence level that a reference correctly identifies the intended symbol:
  - High confidence: Direct textual reference, unambiguous context
  - Medium confidence: Reference with qualified name, partially ambiguous
  - Low confidence: Indirect reference, dynamic dispatch, high ambiguity
  - Usefulness: Agent should weigh confidence when consuming reference knowledge

- Definition vs reference asymmetry: Key research finding:
  - A symbol can have exactly one definition but many references
  - Reference tracking must travel with provenance/confidence
  - Prime should associate provenance/confidence with each reference

- Kinds of references:
  - Read references: Symbol is read/used
  - Write references: Symbol is assigned/written to
  - Pass-by-reference: Symbol passed as reference parameter
  - Usefulness: Distinguishing read vs write aids data flow analysis

- Reference provenance: Source evidence for each reference:
  - Static analysis (AST-based)
  - Dynamic analysis (execution trace)
  - Heuristic/inferred
  - Usefulness: Agent can filter based on provenance reliability

- Unresolved references: References without reachable definition:
  - External symbols (imported from other modules)
  - Removed/deleted symbols
  - Obfuscated/minified symbols
  - Usefulness: Agent must handle gracefully (confidence = unknown)

- Reference counting: Number of times a symbol is referenced:
  - Useful for: Understanding symbol importance, garbage collection hints, impact analysis
  - Prime relevance: Can inform agent prioritization of which symbols to surface first

- Cyclical references: Mutual references between symbols:
  - Common in circular dependencies, recursive types
  - Usefulness: High (detecting circular dependencies, understanding structure)

- Shadowed references: References obscured by local declarations:
  - Local variable shadows parameter/global
  - Usefulness: Agent should track shadowing relationships

- Higher-order references: Symbols passed as arguments or returned:
  - Common in functional programming, callbacks
  - Usefulness: High (represents advanced patterns agents should understand)

- Syntactic vs semantic references:
  - Syntactic: Textual occurrence in source
  - Semantic: Actual usage at runtime (may differ from syntactic)
  - Usefulness: Synthetic is statically available; semantic requires execution data

- Reference indexing structures: Efficient lookup of references:
  - Inverted index (symbol → list of references)
  - Forward index (reference → target symbol)
  - Usefulness: Enables fast reference navigation for agents