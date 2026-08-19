Technical specifications research:

SCIP protobuf schema (scip.proto):
- Message structure: Define symbol kind enum (function, method, class, variable, etc.)
- Relationship messages: Call, Reference, Implements, Inherits, etc.
- Location messages: file URI, line, character offset (start/end)
- Symbol message: name, kind, container, documentation, location, confidence
- Usefulness: Protocol schema as potential serialization format for Prime knowledge
- Adaptation: Prime could adopt protobuf schema (or subset) for cross-language consistency

LSIF vertex/edge specification:
- Vertex types: symbol, definition, reference, document, location
- Edge types: definition-of, call, reference-to, contained-in, etc.
- Location: URI + line + character offset (start/end)
- Usefulness: LSIF vertex/edge model as alternative knowledge representation
- Adaptation: Prime could adopt/revise vertex/edge types for its knowledge graph

Tree-sitter CST specification:
- Node type: Named capture group or anonymous subtree
- Properties: type, start-position (row, column), end-position (row, column), child nodes
- Usefulness: CST structure as intermediate parsing representation
- Adaptation: Prime could use CST as internal representation before universal knowledge extraction

Joern CPG specification:
- Graph layers: AST, CFG, PDG, DG (separate but interconnected layers)
- Node types per layer: AST nodes (functionDecl, classBody, etc.), CFG nodes (basicBlock, jump, etc.)
- Edge types per layer: control-flow, data-flow, calls, points-to, etc.
- Usefulness: Multi-layered graph as rich knowledge representation
- Adaptation: Prime could adopt multi-layered approach (if memory permits) or single-layer simplification

Inverted index specification:
- Term → posting list mapping
- Posting list entries: document ID, position(s), confidence, provenance
- Usefulness: Standard inverted index for lexical search
- Adaptation: Prime's inverted index includes confidence/provenance annotations

Vector embedding specification:
- Dense vector of N dimensions (typically 512-1024)
- Normalization: L2-normalized for cosine similarity calculation
- Storage: Array of floats per symbol/relationship
- Similarity metric: Cosine similarity, dot product
- Usefulness: Semantic search representation
- Adaptation: Prime could store symbol/type/relationship embeddings for semantic search

Merkle tree specification:
- Leaf nodes: Hash of data chunk (typically file content hash)
- Internal nodes: Hash of children's hashes (SHA-256 or similar)
- Root node: Repository root hash (commits to entire state)
- Usefulness: Content addressing, incremental invalidation, distributed sharing
- Adaptation: Prime Merkle tree for tracking codebase state and incremental updates

Incremental update specification:
- Content hash per file (or per knowledge entry)
- Change detection: Compare new hash to stored hash (O(1) per file)
- Incremental re-index: Re-process only changed files
- Merkle root update: New root hash reflects changed files
- Usefulness: Mechanistic specification for incremental analysis (research track #14)
- Adaptation: Prime incremental update algorithm specification

Confidence/provenance specification:
- Confidence levels: Exact (statically derivable from source), Inferred (reasonable deduction), Unknown (cannot determine)
- Provenance: Source evidence (static analysis, dynamic analysis, heuristic)
- Travel: Confidence/provenance travels with knowledge through invalidation/derivation cycles
- Usefulness: Agent can weigh reliability of knowledge (graceful degradation principle)
- Adaptation: Prime knowledge entries include confidence + provenance fields

Knowledge unit specification (atomic piece of universal knowledge):
- Fields: entity ID, relationship type, target entity ID, confidence, provenance, language tag, metadata
- Usefulness: Fixed schema for atomic knowledge units enables consistent indexing, retrieval, compression
- Adaptation: Prime's universal knowledge format specification

Token budget specification:
- Maximum tokens per agent query (configurable, depends on agent model's context window)
- Token cost per knowledge unit (estimated based on representation format)
- Token budget enforcement: Stop retrieval when budget exceeded (partial retrieval with confidence degradation)
- Usefulness: Critical for agent experience (init-promt.md central question)
- Adaptation: Prime query system enforces token budget during retrieval

Cross-language mapping specification:
- Universal vocabulary: Fixed set of knowledge kinds (INTERFACE, CALLABLE, CLASS, etc.)
- Language tag: Attach source language to each knowledge unit (enables filtering, differentiation)
- Mapping table: Language-specific constructs → universal knowledge kind + confidence level
- Usefulness: Language-agnostic knowledge representation (research track #15)
- Adaptation: Prime universal knowledge format with language tagging