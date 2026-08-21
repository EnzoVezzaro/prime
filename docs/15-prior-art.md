---
title: Prior Art
---

# Prior Art

Prime should build on existing research rather than pretending the field starts here.

## Code Intelligence Systems

### SCIP (Source Code Intelligence Protocol)
- **What**: Language-agnostic indexing protocol for symbols, definitions, references, implementations
- **Repo**: https://github.com/sourcegraph/scip
- **Relevance**: Cross-language symbol identity, standardized format
- **Gap**: Language-server oriented, not agent-optimized

### LSIF (Language Server Index Format)
- **What**: Persistent representation of language-server information
- **Repo**: https://github.com/microsoft/lsif-node
- **Relevance**: Graph representation, cross-reference persistence
- **Gap**: Tied to LSP, verbose, not compressed

### Code Property Graphs / Joern
- **What**: Rich program representation (AST + CFG + PDG + CG)
- **Repo**: https://github.com/joernio/joern
- **Relevance**: Multi-graph representation, query language
- **Gap**: Heavyweight, security-focused, not agent-native

### Tree-sitter
- **What**: Incremental, error-tolerant parsing for many languages
- **Repo**: https://github.com/tree-sitter/tree-sitter
- **Relevance**: Primary parsing frontend for Prime
- **Gap**: Only parsing, no semantic analysis

### Aider Repository Map
- **What**: Compact structural map for LLM context
- **Article**: https://aider.chat/2023/10/22/repomap.html
- **Relevance**: Proves compact maps help agents
- **Gap**: Heuristic, not typed, no provenance

### Graph-sitter
- **What**: Graph-based code analysis on Tree-sitter
- **Relevance**: Graph construction from Tree-sitter
- **Gap**: Early stage, single-language focus

## Information Retrieval & Search

### Vector Search / Embeddings
- **Systems**: Pinecone, Weaviate, Qdrant, Chroma
- **Relevance**: Semantic search for code
- **Gap**: No provenance, hallucinations, not exact

### Hybrid Retrieval (Lexical + Vector)
- **Systems**: Sourcegraph, GitHub Copilot, Cursor
- **Relevance**: Best of both worlds
- **Gap**: Black box, no structured output

### Learned Indexes
- **Papers**: "The Case for Learned Index Structures" (Kraska et al., 2018)
- **Relevance**: Replace B-trees with ML models
- **Gap**: Not yet production-ready for code

## Compression & Succinct Structures

### Grammar Compression
- **Papers**: "Grammar-Based Codes" (Kieffer & Yang, 2000)
- **Tools**: RePair, Sequitur
- **Relevance**: Pattern deduplication in graphs

### Succinct Data Structures
- **Book**: "Succinct Data Structures" (Navarro, 2016)
- **Techniques**: Rank/select, Elias-Fano, Roaring bitmaps
- **Relevance**: Space-efficient graph indexes

### WebGraph
- **Paper**: "The WebGraph Framework" (Boldi & Vigna, 2004)
- **Techniques**: Graph compression, node ordering
- **Relevance**: Billion-node graph compression

### Elias-Fano Encoding
- **Paper**: "Elias-Fano Compression" (Elias, 1974; Fano, 1971)
- **Relevance**: Monotonic sequences (entity IDs, offsets)

### Roaring Bitmaps
- **Paper**: "Roaring Bitmaps" (Chambers et al., 2016)
- **Repo**: https://github.com/RoaringBitmap/RoaringBitmap
- **Relevance**: Fast transitive closure, set operations

## Storage & Memory Mapping

### mmap / Page Cache
- **OS**: Linux (mmap), macOS (mmap), Windows (MapViewOfFile)
- **Behavior**: Demand paging, OS-managed caching
- **Relevance**: Zero-copy access for large artifacts

### Columnar Formats
- **Parquet**: Analytical, predicate pushdown
- **Arrow**: In-memory, SIMD-friendly
- **Relevance**: Compression, analytical queries

### Embedded Databases
- **SQLite**: Ubiquitous, SQL, WAL mode
- **LMDB**: Memory-mapped B+tree, ACID
- **RocksDB**: LSM-tree, high write throughput
- **Redb**: Pure Rust, memory-mapped

## Distributed Systems & P2P

### Content Addressing
- **IPFS**: Content-addressed, DHT, bitswap
- **Git**: Merkle DAG, content-addressed
- **CAS**: Content-addressed storage (Nix, Bazel)

### Merkle DAGs / CRDTs
- **CRDTs**: "Conflict-Free Replicated Data Types" (Shapiro et al., 2011)
- **Automerge**: JSON-like CRDT
- **Yjs**: Shared editing, YATA algorithm
- **Relevance**: Distributed knowledge merging

### P2P Protocols
- **libp2p**: Modular P2P stack (IPFS, Ethereum)
- **BitTorrent**: DHT, piece selection, choking
- **DAT/Hypercore**: Append-only logs, sparse replication

## Cryptography & Verification

### Transparency Logs
- **Certificate Transparency**: RFC 6962
- **Sigstore/ Cosign**: Supply chain signing
- **Rekor**: Transparency log for artifacts

### Searchable Encryption
- **SSE**: "Practical Searchable Symmetric Encryption" (Curtmola et al., 2006)
- **PEKS**: "Public Key Encryption with Keyword Search" (Boneh et al., 2004)

### Zero-Knowledge
- **ZK-SNARKs**: Groth16, PLONK, Halo2
- **ZK-STARKs**: Transparent, post-quantum
- **RISC Zero / SP1**: ZK-VM for general computation

## Agent Architectures

### Context Management
- **Aider**: Repo map + file contents
- **Cursor**: Embeddings + keyword + AST
- **Sourcegraph Cody**: LSIF + embeddings + LLM
- **AutoGPT**: Iterative tool use, memory

### Tool Use / MCP
- **MCP**: Model Context Protocol (Anthropic)
- **LangChain Tools**: Structured tool calling
- **Function Calling**: OpenAI, Anthropic native

## Academic Papers (Key References)

| Paper | Year | Relevance |
|-------|------|-----------|
| "Code Property Graphs" (Yamaguchi et al.) | 2014 | CPG foundation |
| "SCIP: A Language-Agnostic Indexing Protocol" | 2022 | Cross-language indexing |
| "LSIF: Language Server Index Format" | 2019 | LSP persistence |
| "The WebGraph Framework" | 2004 | Graph compression |
| "Roaring Bitmaps" | 2016 | Bitmap compression |
| "Elias-Fano Compression" | 1974/1971 | Monotonic encoding |
| "Succinct Data Structures" (Navarro) | 2016 | Space-efficient structures |
| "Information Bottleneck" | 1999 | Relevant information extraction |
| "Minimum Description Length" | 1978 | Model selection |
| "CRDTs" (Shapiro et al.) | 2011 | Distributed merging |

## What Prime Can Learn

| System | Lesson for Prime |
|--------|------------------|
| SCIP | Cross-language symbol identity |
| LSIF | Graph persistence format |
| CPG/Joern | Rich semantic relationships |
| Tree-sitter | Incremental parsing foundation |
| Aider | Compact maps work for agents |
| WebGraph | Graph compression at scale |
| Roaring/Elias-Fano | Space-efficient indexes |
| mmap | Zero-copy large artifact access |
| CRDTs | Distributed knowledge merge |
| IPFS/Git | Content addressing, Merkle DAGs |
| Sigstore | Supply chain trust |
| ZK/TEE | Verifiable derivation |

## Next

- [Introduction: What is Prime?](../01-what-is-prime.md)
- [Research: Agent Architecture](../research/agent-architecture.md)
- [Specification: Knowledge Model](../specification/knowledge-model.md)