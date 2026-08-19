TypeScript language analysis:

- Parsing: Tree-sitter TypeScript grammar (official parser)
  - Language bindings: JavaScript/Node.js (node-tree-sitter), WebAssembly binding
  - Parser generation: Community grammars available (ts-tree-sitter, typescript-tree-sitter)
  - Incremental parsing: Supported (editor-like performance on keystrokes)
  - Error recovery: Grammar includes error recovery rules
  - Usefulness to Prime: Reliable parsing foundation for symbol extraction

- Symbols: TypeScript-specific symbol kinds
  - Interface, type alias, enum, namespace
  - Function (including overloads), method, constructor
  - Class, abstract class, implements clause
  - Variable (let, const, var), parameter, property
  - Export keyword: Makes symbol publicly accessible across modules
  - Usefulness: Prime extracts these as universal symbol kinds where possible

- Types: TypeScript type system features
  - Explicit type annotations (function parameters, return types, variable declarations)
  - Type inference (implicit types from assignment, return inference)
  - Generic types (<T>, generic interfaces, generic classes)
  - Union types (A | B), intersection types (A & B)
  - Optional types (?T), nullable types)
  - Readonly modifier, mutable vs immutable
  - Usefulness: Prime extracts explicit types; inferred types marked with confidence level

- Classes/interfaces:
  - Class: constructor, properties, methods, implements clause, extends clause
  - Interface: method signatures, extends clause, index signatures
  - Class vs interface distinction: Classes have implementation; interfaces are contracts
  - Usefulness: Prime universal model: INTERFACE kind (maps from both Class and Interface where possible)

- Modules/packages:
  - Module: file-level organization (export/import syntax)
  - Namespace: internal grouping (deprecated in modern TS, but still supported)
  - Package: external dependency (package.json name, version)
  - Usefulness: Prime module kind + namespace kind distinction

- Cross-language relationships:
  - TypeScript → JavaScript: Structural compatibility (JS is TS without type annotations)
  - TypeScript → Python: Map TypeScript interface → Python protocol/abc
  - TypeScript → Rust: Map TypeScript interface → Rust trait
  - Usefulness: Prime cross-language relationship mapping; TypeScript's strict types provide rich data for cross-language mapping

- Dynamic language considerations:
  - TypeScript is structurally typed (duck typing via structural subtyping)
  - Unlike Python (dynamic typing), TypeScript types are erased at runtime
  - Prime must distinguish: TypeScript-level types (compile-time) vs. runtime types (Python)
  - Usefulness: Agent needs both compile-time structure and runtime behavior knowledge

- Generated code:
  - TypeScript generates JavaScript (compilation output)
  - TypeScript may generate declaration files (.d.ts) for type information
  - Prime should treat .d.ts as source of type information (if .js is minified/obfuscated)
  - Usefulness: Prime can extract type information from declaration files even when source is unavailable

- TypeScript-specific patterns:
  - Async/await: Functions returning Promise, async marker
  - Decorators: Class property/Method decorators (experimental feature)
  - Generic constraints: extends constraint, implements constraint
  - Module augmentation: Declaring additional properties on existing types
  - Usefulness: Prime should recognize these patterns and represent them in universal model

- Type confidence/provenance:
  - Exact: Type annotation present in source (confidence: high)
  - Inferred: Type determined by TypeScript compiler (confidence: medium)
  - Unknown: Type cannot be determined (confidence: low, e.g., when only .js available without .d.ts)
  - Usefulness: Agent can weigh reliability of type knowledge

- TypeScript → Universal vocabulary mapping examples:
  - TypeScript interface → Universal: INTERFACE (with possible language tag: "typescript")
  - TypeScript class → Universal: CLASS (or TYPE if no inheritance)
  - TypeScript type alias → Universal: TYPE (with kind: "alias")
  - TypeScript enum → Universal: ENUM
  - TypeScript function → Universal: CALLABLE (with parameters/return kind)
  - TypeScript generic → Universal: GENERIC (with type parameters)