SCIP (SourceCode Intelligence Protocol) research:

Repository: https://github.com/sourcegraph/scip

Investigated topics:

- Symbol identity: SCIP provides a language-agnostic way to identify symbols across different programming languages. Uses a combination of name, kind, and container to uniquely identify symbols.

- Symbol indexing: SCIP indexes symbols during the parsing phase. The index enables fast lookups for Go-to-definition, Find references, and Find implementations.

- Definitions: SCIP defines "definition" as the primary symbol origin. A symbol can have exactly one definition but multiple references.

- References: SCIP tracks all references to a symbol definition. References include information about where the symbol is used, with file, line, and column positions.

- Implementations: SCIP distinguishes between interface definitions and concrete implementations. This is particularly important for object-oriented and generic types.

- Relationships: SCIP represents relationships between symbols through the protocol's message format. Key relationships include:
  - CALLS: A function calling another function
  - IMPLEMENTS: A class/interface implementing another interface
  - INHERITS: A class inheriting from another class
  - REFERENCES: A symbol referencing another symbol

- Cross-language representation: SCIP is designed to be language-agnostic. The SCIP protobuf schema defines a common vocabulary that can represent symbols from different languages. The schema uses numeric kind identifiers that map to language-specific meanings.

- Serialization: SCIP uses Protocol Buffers (protobuf) for serialization. The scip.proto file defines the schema for all SCIP messages. Protobuf provides binary serialization that is both compact and evolvable.

- Index generation: SCIP can be emitted by language indexers (e.g., scip-typescript, scip-java, rust-analyzer). The indexer parses source code and emits SCIP messages describing the code structure.

- Storage: SCIP indexes can be stored as:
  - SCIP CLI can emit JSON or Protobuf binary format
  - Can be stored in databases (SQLite, etc.) for querying
  - SCIP workspaces can be persisted and loaded incrementally

- Incremental behavior: SCIP supports incremental indexing. The protocol includes messages for:
  - Symbol refresh (update existing symbol)
  - Symbol delete (remove existing symbol)
  - Partial indexing (only changed files)

- Limitations:
  - SCIP focuses on syntactic and structural information, not semantic semantics
  - Some language features may not map cleanly to the common vocabulary
  - Full type information may be lost in cross-language translation
  - Requires language-specific indexers for each supported language

- Reusable concepts for Prime:
  - Language-agnostic symbol vocabulary (protobuf-based)
  - Incremental update support (add/delete/refresh operations)
  - Standardized relationship types (calls, references, implements)
  - Protobuf schema as a potential serialization format
  - Indexer architecture pattern (parse → emit protocol messages)