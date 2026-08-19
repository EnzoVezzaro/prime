Go language analysis:

- Parsing: Go parser via tree-sitter (community grammar) or go/ast (built-in)
  - Language bindings: Go (native), others
  - Parser generation: tree-sitter Go grammar (community)
  - Incremental parsing: Limited (Go files typically not edited incrementally like editor code)
  - Error recovery: Go's strict formatting (gofmt) aids parsing
  - Usefulness to Prime: Built-in go/ast sufficient for symbol extraction

- Symbols: Go-specific symbol kinds
  - Function (func), method (on receiver type), interface
  - Type (type definition), constant (const), variable (var)
  - Usefulness: Prime extracts these as universal symbol kinds

- Types: Go type system features
  - Explicit type annotations (function parameters, return types, variable declarations)
  - Type inference (short variable declaration :=, compiler infers type)
  - Interfaces (interface { Method1(), Method2() }), structural typing
  - Generics <T> (Go 1.18+), before: interface{} (empty interface)
  - Pointer types (*T), value types (T)
  - Usefulness: Prime extracts explicit types; inferred types marked with confidence level

- Interfaces: Go's interface system
  - Interface: implicit (struct satisfies interface by implementing methods, no explicit 'implements')
  - Interface composition: embedding interfaces interface { InnerInterface }
  - Usefulness: Prime universal model: INTERFACE (implicit satisfaction, different from explicit 'implements')

- Go concurrency: Goroutines, channels affect symbol relationships
  - Goroutine (go keyword), Channel (make(chan Type))
  - Usefulness: Prime marks goroutine/channel symbols as "runtime-semantic" (confidence: inferred)

- Go → Universal vocabulary mapping examples:
  - Go func → Universal: CALLABLE (with parameters/return type kind, confidence: annotated/inferred)
  - Go method → Universal: CALLABLE (with receiver kind, mutability marker)
  - Go interface → Universal: INTERFACE (implicit satisfaction, structural)
  - Go type definition → Universal: TYPE (with kind: "struct"/"interface"/"enum"/"alias")