LSIF (Language Server Index Format) research:

Repository: https://github.com/microsoft/lsif-node

Investigated topics:

- Why LSIF exists: LSIF was created to standardize the output of language servers so that tooling can index code consistently. Before LSIF, each language server implemented its own indexing format, making it difficult to build cross-language tools.

- Graph representation: LSIF uses a graph structure to represent code relationships. The graph consists of vertices (symbols, definitions, references) and edges (relationships between them).

- Vertices: LSIF vertices represent:
  - Symbol: A code entity like a function, class, method, variable
  - Definition: The primary location where a symbol is defined
  - Reference: A use/reference to a symbol
  - Document: The source file being indexed
  - Location: A position within a source file (line, character)

- Edges: LSIF edges represent relationships between vertices:
  - Definition edge: Connects a reference to its definition
  - Call edge: Connects a caller to a callee
  - Reference edge: Connects a symbol to its references
  - Containment edge: Connects a symbol to the document/file containing it

- Source locations: LSIF stores precise source locations for all vertices and edges. Each location includes:
  - URI of the source file
  - Line number
  - Character offset (start and end)

- Definitions: LSIF explicitly marks which symbols are definitions. This helps tools distinguish between a symbol declaration and its references.

- References: LSIF collects all references to definitions across a codebase. Each reference includes the source location where the reference occurs.

- Implementation: LSIF is implemented by language servers that support the format. The Microsoft TypeScript language server (tsserver) was one of the first to emit LSIF output. Other language servers have since added support.

- Persistence: LSIF output can be persisted to disk as JSON files. These files can be large because they contain complete index information for a codebase.

- Querying: LSIF data can be loaded by indexing tools (e.g., Sourcegraph, code-search tools). The data enables fast lookup of definitions, references, and symbol search.

- Language-server integration: LSIF is designed to be emitted by language servers as part of their normal operation. The format integrates with the Language Server Protocol (LSP).

- Limitations:
  - LSIF index files can be very large (full codebase index)
  - Requires language-server support for each language
  - Primarily focused on syntactic structure, not semantic analysis
  - Incremental updates may require re-indexing entire files
  - No native graph database optimization (raw JSON format)

- Lessons for Prime:
  - Standardized vertex/edge model for code representation
  - Source location tracking is essential for agent context
  - Definition/reference distinction is critical for agent reasoning
  - Language-server integration pattern can be reused
  - JSON serialization may not be optimal; binary formats (protobuf, FlatBuffers) could reduce size
  - Incremental indexing support needed for large codebases