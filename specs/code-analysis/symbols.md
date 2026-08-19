Symbols research:

- Symbol identity: The unique identification of a code entity within a codebase. Identity typically combines:
  - Symbol name (identifier)
  - Kind (function, class, variable, method, etc.)
  - Container (namespace, class, module, file)
  - Language-specific qualifiers

- Symbol resolution: The process of determining which symbol definition a reference refers to. Involves:
  - Scope analysis (lexical, dynamic)
  - Import/path resolution
  - Language-specific rules (e.g., Python LEGB rule: Local, Enclosing, Global, Built-in)

- Symbol metadata: Additional information attached to symbols that aids agent reasoning:
  - Documentation comments/docstrings
  - Deprecation warnings
  - Default values
  - Visibility modifiers (public, protected, private)
  - Async marker
  - Mutability markers

- Naming conventions: Patterns in symbol names that convey meaning:
  - CamelCase vs snake_case conventions
  - Prefix/suffix conventions (e.g., "is", "has", "manager", "service")
  - Hungarian notation (language-dependent)
  - Prime should distinguish convention from semantics

- Symbol visibility: Who can access this symbol:
  - Public/private/protected (OOP languages)
  - Internal/exported (modules/packages)
  - Package-private (Java)
  - Usefulness to agents: High (understanding API surface area)

- Symbol lifecycle: Birth and death of symbols:
  - Declaration vs definition distinction
  - Shadowing/hiding rules
  - Deprecation and removal
  - Usefulness: Medium (understanding code evolution)

- Symbol overloading: Multiple symbols with same name but different signatures:
  - Common in functional languages
  - Common in OOP with different parameter types
  - Usefulness: High (disambiguation needed for agent queries)

- Deprecated symbols: Marked-for-removal symbols:
  - Explicit deprecation annotations
  - Usefulness: Medium (agents should prefer non-deprecated alternatives)

- Experimental symbols: Feature-flagged symbols:
  - Conditional compilation, feature flags
  - Usefulness: Low (context-dependent)

- Reserved symbols: Language-defined symbols:
  - Keywords, built-in names
  - Usefulness: Low (expected knowledge)

- Fully qualified names: Complete path to symbol:
  - Namespace.Class.Method format
  - Usefulness: High (enables cross-file reference without ambiguity)

- Short vs fully qualified: Tradeoff between brevity and ambiguity:
  - Short names: Compact but may be ambiguous
  - Fully qualified: Unambiguous but verbose
  - Prime should support both with context-dependent selection

- Hashed symbol identities: Content-addressed symbol IDs:
  - Hash of name + container + language
  - Usefulness: Enables deduplication, content addressing, distributed sharing
  - Prime relevance: High (connects to Merkle DAG / content addressing research)

- Symbol confidence/provenance: Confidence level associated with symbol knowledge:
  - Exact (statically derivable from source)
  - Inferred (reasonable deduction)
  - Unknown (cannot determine)
  - Usefulness: Agent can weigh reliability of knowledge

- Hashed vs named identity: Tradeoff analysis:
  - Named: Human-readable, debugging-friendly, may vary across refactoring
  - Hashed: Stable across refactoring, supports content addressing, not human-readable
  - Prime should support both with conversion capabilities