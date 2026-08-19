Codebase Knowledge - Parsing research:

What can be derived from source code:

- Files: Directly observable from filesystem. Path, name, size, modification time, content. Useful for: file discovery, change detection, organization.

- Directories/folders: Observable. Hierarchy structure, nesting, paths. Useful for: module organization, namespace understanding.

- Packages/modules: Statically derivable in most languages. Import/export statements, directory structure, namespace declarations. Useful for: organizational structure, dependency boundaries.

- Symbols (functions, classes, methods, variables): Statically derivable. Names, locations (file:line), type annotations (where present), visibility/modiﬁers. Useful for: code navigation, symbol resolution.

- Declarations: Statically derivable. Point where symbol is ﬁrst introduced. Useful for: understanding symbol lifecycle, scope.

- Types: Partially derivable. Type annotations in source are exact; inferred types are partial. Useful for: type checking, API understanding, call validation.

- Functions/methods: Statically derivable. Name, parameters, return type (where annotated), location, body (syntax). Useful for: understanding behavior, calling conventions.

- Interfaces: Statically derivable (in OO languages). Method signatures, inheritance. Useful for: contracts, polymorphism, API design.

- Variables: Statically derivable. Name, type (where annotated), scope, mutability, initialization. Useful for: data flow tracking, lifetime analysis.

- Constants: Statically derivable. Name, type, value (if constant-folded). Useful for: configuration, magic number elimination.

- Parameters: Statically derivable. Name, type (where annotated), order, defaults (where supported). Useful for: API design, calling conventions.

- Imports/exports: Statically derivable. Module names, aliases, what's publicly exposed. Useful for: module boundaries, dependency analysis.

- References (symbol usages): Statically derivable (direct references). Name-based references may be ambiguous. Usefulness: high for direct uses, medium for indirect.

- Calls (function invocations): Partially derivable. Direct calls are exact; indirect calls (higher-order, dynamic dispatch) are partial/inferred. Usefulness: high for direct, inferred for dynamic languages.

- Inheritance: Statically derivable (OOP languages). Class hierarchy, interface implementation. Usefulness: high for OOP code, limited for functional/non-OOP.

- Implementations: Statically derivable (OOP). Which class implements which interface. Usefulness: high for OOP code.

- Instantiation: Partially derivable. Object creation sites, factory patterns. Usefulness: medium (often framework-driven).

- Reads (data reads): Inferable in limited cases. Direct variable reads are inferable; complex data flows are probabilistic. Usefulness: medium (data flow analysis needed for full picture).

- Writes (data writes): Inferable in limited cases. Direct variable writes are inferable; complex paths are probabilistic. Usefulness: medium (needs data flow analysis).

- Control flow: Partially derivable. AST provides structure; complete CFG requires analysis. Usefulness: high for understanding flow, limited for complex paths.

- Data flow: Inferable in part. Direct data dependencies are inferable; transitive dependencies are probabilistic. Usefulness: high for security analysis, medium for general understanding.

- Dependencies: Statically derivable (import/export). Build-time dependencies are exact; runtime dynamic dependencies are partial. Usefulness: high for build analysis, medium for runtime behavior.

- Reverse dependencies: Partially derivable. Can be derived from dependency analysis. Usefulness: high for impact analysis.

- Tests: Partially derivable. Test ﬁles, test annotations, test runners. Usefulness: medium (test quality and coverage varies).

- Configuration: Statically derivable. Config ﬁles, build scripts, package.json, .yml etc. Usefulness: high for understanding setup, runtime behavior.

- Resources: Statically derivable. Assets, images, conﬁg ﬁles outside source. Usefulness: medium (depends on project type).

- Generated code: Partially derivable. Markers, output directories, build scripts. Usefulness: medium (often hard to distinguish from hand-written).

- Build systems: Statically derivable. Makeﬁles, CMake, package.json, Maven etc. Usefulness: high (build process understanding).

- Package managers: Statically derivable. Dependencies listed in manifest ﬁles. Usefulness: high (dependency management).

- Architecture: Inferred from code structure. Patterns observed (layered, microservices, monolith). Usefulness: inferred (requires analysis, not directly observable).

- Runtime relationships: Inferable in limited cases. Can observe from execution traces, otherwise probabilistic. Usefulness: low (requires execution data).

- Source provenance: Statically derivable. Git history, origin URLs, authorship. Usefulness: medium (historical context).

- Version information: Statically derivable. Version ﬁles, package versions, git tags. Usefulness: medium (context, compatibility).

Determine which information is:

- Directly observable: Filesystem entries, source content, import/export statements, type annotations present in source, file metadata (size, mtime)

- Statically derivable: All syntactic elements (symbols, types where annotated, functions, classes, inheritance, implementations, configuration, build systems, package managers, source provenance, version information)

- Inferable: Data flow relationships (reads/writes), some control flow patterns, reverse dependencies, some runtime relationships, test relationships

- Probabilistic: Indirect calls (dynamic dispatch), transitive data flows, some dependency chains, test coverage, runtime behavior

- Unavailable without execution: Actual runtime values, dynamic dispatch targets, true test coverage, performance characteristics, heap state, thread interactions

Determine which information is useful to agents:

- Highly useful: Symbol names and locations, type annotations, function signatures, import/export structure, configuration details, build system information, version information, direct references and calls

- Moderately useful: Inheritance relationships (OOP only), data flow patterns (where analyzable), test structure, resource files, source provenance

- Less useful without context: Control flow graphs (needs understanding of purpose), runtime relationships (needs execution data), inferred probabilistic relationships

- Useful when aggregated: Architecture patterns (emergent from structure), dependency maps (hol view), architectural knowledge (identified through analysis)