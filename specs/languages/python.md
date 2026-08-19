Python language analysis:

- Parsing: Python parser via tree-sitter (community grammar) or ast module (built-in)
  - Language bindings: Python (native), JavaScript (node-tree-sitter), others
  - Parser generation: tree-sitter Python grammar (community)
  - Incremental parsing: Limited (Python rarely edited incrementally like editor code)
  - Error recovery: Python's indentation-dependent syntax requires careful error recovery
  - Usefulness to Prime: Built-in ast module sufficient for symbol extraction

- Symbols: Python-specific symbol kinds
  - Function (def), class (class), variable (assignment), parameter
  - Import, from-import (brings symbols into namespace)
  - Lambda (lambda keyword), closure (closes over enclosing scope)
  - Usefulness: Prime extracts these as universal symbol kinds

- Types: Python type system features
  - Type annotations (PEP 484+: function parameters, return types, variable declarations)
  - Type inference: Limited (type inference only where annotations present)
  - Optional types: Optional[T] (Union[T, None]), None keyword
  - Union types: Type[X | Y] (Python 3.10+ | syntax, older: Union[X, Y])
  - Generic types: TypeVar, Generic[BaseClass], from typing module
  - Duck typing: No type annotations required (runtime type based on behavior)
  - Usefulness: Prime extracts annotated types; unannotated marked as "unknown" confidence

- Classes/interfaces:
  - Class: class Body(ParentClass), methods, __init__, inheritance
  - Inheritance: Single inheritance (class A(B)), MRO (method resolution order)
  - Usefulness: Prime universal model: CLASS (with parent/kind markers)

- Dynamic features:
  - Dynamic dispatch: Method resolution at runtime (MRO)
  - Monkey patching: Modify class at runtime
  - Metaclasses: Custom class creation behavior
  - Usefulness: Prime marks these as "runtime-semantic" (confidence: inferred, not static)

- Module/packages:
  - Module: file-level organization (if __name__ == "__main__"), import structure
  - Package: directory with __init__.py, nested packages
  - Usefulness: Prime module kind + namespace kind distinction

- Cross-language relationships:
  - Python → JavaScript: Structural subtyping (both dynamic, duck typing)
  - Python → TypeScript: Map Python class → TypeScript interface (if type annotations present)
  - Python → Rust: Map Python class → Rust trait (if semantics align)
  - Usefulness: Prime cross-language relationship mapping; Python's dynamic nature limits static mapping

- Python → Universal vocabulary mapping examples:
  - Python function → Universal: CALLABLE (with parameters/return kind, confidence: annotated/inferred/unknown)
  - Python class → Universal: CLASS (with parent/kind markers, confidence: annotated/inferred/unknown)
  - Python import → Universal: IMPORT (with module name, confidence: static/dynamic)
  - Python class inheritance → Universal: INHERITS (with parent class kind)