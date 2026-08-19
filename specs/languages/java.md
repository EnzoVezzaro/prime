Java language analysis:

- Parsing: Java parser via tree-sitter (JavaCC grammar) or built-in AST (javac/compiler)
  - Language bindings: Java (native), others
  - Parser generation: tree-sitter Java grammar (community), or use compiler AST (more precise)
  - Incremental parsing: Limited (Java files typically not edited incrementally)
  - Error recovery: Java's verbosity and structure aids parsing
  - Usefulness: Compiler AST provides most precise symbol information

- Symbols: Java-specific symbol kinds
  - Class, interface, enum, annotation
  - Method, constructor, field (attribute)
  - Package (package declaration), import
  - Usefulness: Prime extracts these as universal symbol kinds

- Types: Java type system features
  - Explicit type annotations (abundant: return types, parameter types, variable declarations)
  - Generics <T> (Java 5+), type erasure at runtime
  - Class hierarchy: single inheritance (extends), multiple interface implementation (implements)
  - Access modifiers: public, protected, private, package-private (default)
  - Usefulness: Prime extracts explicit types; access modifiers map to visibility kind

- Classes/interfaces:
  - Class: fields, methods, inner classes, implements extends clauses
  - Interface: method signatures only (no implementation), extends clause
  - Abstract class: partial implementation, abstract methods
  - Usefulness: Prime universal model: CLASS (with parent/kind/abstract markers)

- Access modifiers → Visibility:
  - public → Universal: VISIBILITY (public)
  - protected → Universal: VISIBILITY (protected)
  - private → Universal: VISIBILITY (private)
  - package-private (default) → Universal: VISIBILITY (package-private/internal)

- Java → Universal vocabulary mapping examples:
  - Java class → Universal: CLASS (with parent/kind/abstract/modifiers markers)
  - Java interface → Universal: INTERFACE (with method signatures, modifiers markers)
  - Java method → Universal: CALLABLE (with parameters/return type, visibility/modifiers)
  - Java inheritance → Universal: INHERITS (with parent class kind, modifiers)