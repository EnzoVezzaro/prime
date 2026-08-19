Rust language analysis:

- Parsing: Rust parser via tree-sitter (community grammar) or rust-frontend (rustc-based)
  - Language bindings: Rust (native), Go (go-tree-sitter), JavaScript (node-tree-sitter with Rust grammar)
  - Parser generation: Community-maintained tree-sitter Rust grammar
  - Incremental parsing: Supported (editor-like performance)
  - Error recovery: Rust's strict compiler aids error recovery
  - Usefulness to Prime: Reliable parsing for symbol extraction

- Symbols: Rust-specific symbol kinds
  - Function (fn), method (&self, &mut self, self), associated functions (::function)
  - Struct, enum, trait
  - Variable (let, const), constant (const), static
  - Lifetime markers ('a, 'b) - Rust-specific, annotate borrows
  - Usefulness: Prime extracts these as universal symbol kinds where possible

- Types: Rust type system features
  - Explicit type annotations (function parameters, return types, let bindings)
  - Type inference (implicit from usage, elided lifetimes in some positions)
  - Generics <T>, trait bounds: T: Trait, multiple bounds T: Trait1 + Trait2
  - Option<T>, Result<E, T> (error handling idioms)
  - Lifetime elision rules (compiler infers lifetimes in many positions)
  - Reference types (&T, &mut T), smart pointers (Box<T>, Option<T>, Vec<T>)
  - Usefulness: Prime extracts explicit types; inferred types marked with confidence level

- Traits: Rust's trait system
  - Trait definition: method signatures, default implementations
  - Trait implementation: impl Trait for Type
  - Trait bounds: generic parameter constraints
  - Usefulness: Prime universal model: TRAIT (maps from other languages' interfaces/abstract classes)

- Ownership/borrowing: Rust-specific concepts (affects symbol relationships)
  - Ownership: Each value has single owner
  - Borrowing: &T (immutable), &mut T (mutable) borrows
  - Lifes: Relationships between borrow and owner
  - Usefulness: Prime must represent these relationships (read/write references) in universal model

- Ownership → Universal model mapping:
  - Rust function with &self → Universal: CALLABLE (with mutability marker: "mutable" or "immutable")
  - Rust &mut T parameter → Universal: PARAMETER (with mutability: "mutable")
  - Rust &T parameter → Universal: PARAMETER (with mutability: "immutable")
  - Trait implementation → Universal: IMPLEMENTS (with trait kind marker)

- Memory safety: Rust's borrow checker enforces at compile time
  - Use-after-free prevented, data races prevented at compile time
  - Prime can represent these guarantees (confidence: high for Rust code)
  - Usefulness: Agent can rely on memory safety guarantees derived from Rust semantics

- Cargo packages: Rust's build/package system
  - Cargo.toml: Dependencies, package metadata, features flags
  - Usefulness: Prime extracts package kind, dependency information from Cargo.toml

- Rust-specific patterns:
  - Async/await: async fn, Future type, .await keyword
  - Macros: macro_rules!, derive macro, procedural macro
  - Usefulness: Prime recognizes macro patterns (may mark as "generated" or "metaprogramming")
  - Crate structure: lib.rs, main.rs, modular organization

- Rust → Universal vocabulary mapping examples:
  - Rust trait → Universal: TRAIT (with possible language tag: "rust")
  - Rust struct → Universal: STRUCT (with fields kind)
  - Rust enum → Universal: ENUM (with variants kind)
  - Rust fn → Universal: CALLABLE (with parameters/return type kind, mutability marker)
  - Rust impl block → Universal: IMPLEMENTS (with trait kind marker)