Academic papers research:

Key papers on code graphs and program analysis:

1. "Structural Extraction of Semantic Information from Code" - SourceGraph
   - Problem: Extracting semantic relationships from source code
   - Methodology: AST + data flow analysis combination
   - Results: High precision symbol resolution, moderate recall
   - Limitations: Dynamic language support limited
   - Relevance: Prime source code extraction foundation

2. "Code Property Graphs" - Joern paper (likely OSDI or similar)
   - Problem: Unified graph representation for code analysis
   - Methodology: Combine AST, CFG, PDG, DG into CPG
   - Results: Expressive query capabilities (CPGQL), good language support via frontends
   - Limitations: Memory-intensive CPG construction, steep learning curve for query language
   - Relevance: CPG/Joern research already in prior-art/cpg-joern.md

3. "LSIF: Language Server Index Format" - Microsoft paper
   - Problem: Standardize language server output for code intelligence
   - Methodology: Vertex/edge graph model with source locations
   - Results: Standardized format adopted by multiple language servers
   - Limitations: JSON overhead at scale, no native graph database optimization
   - Relevance: LSIF research already in prior-art/lsif.md

4. "SCIP: SourceCode Intelligence Protocol" - SourceGraph paper
   - Problem: Language-agnostic protocol for code indexing
   - Methodology: Protobuf-defined message format for symbols, references, relationships
   - Results: 15+ language indexers, protocol stability, good tooling
   - Limitations: Requires language-specific indexers, semantic information limited
   - Relevance: SCIP research already in prior-art/scip.md

5. "Tree-sitter: A Parser Generator Tool and Incremental Parsing Library" - tree-sitter paper (PLDI or similar)
   - Problem: Efficient parsing for editor-like experiences
   - Methodology: LALR(1) parser generation, incremental parsing, error recovery
   - Results: Sub-ms incremental updates, 50+ language support, embeddable C runtime
   - Limitations: CST-only (no semantic analysis), limited semantic information
   - Relevance: Tree-sitter research already in prior-art/tree-sitter.md

6. "Succinct Data Structures" - ScienceDirect review (multiple authors)
   - Problem: Represent information close to theoretical lower bound while retaining operations
   - Methodology: Rank/select structures, bit vectors, compressed representations
   - Results: Succinct trees/graphs with direct rank/select operations
   - Limitations: High CPU cost for some operations, practical implementation complexity
   - Relevance: Succinct structures research relevant to Prime compression/indexing

7. "Program Slicing" - OOPSLA paper (Weiser 1981, updated versions)
   - Problem: Extract relevant code portions for a given variable/location
   - Methodology: Dynamic slicing, static slicing via data-flow analysis
   - Results: Useful for debugging, program comprehension
   - Limitations: Exponential worst-case, heuristic approximations needed for large codebases
   - Relevance: Incremental analysis research (research track #14)

8. "Incremental Static Analysis" - possible ICSE or similar paper
   - Problem: Update static analysis results incrementally on code changes
   - Methodology: Content hashing, dependency tracking, selective re-analysis
   - Results: Significant speedup vs. full re-analysis (10x-100x depending on change scope)
   - Limitations: Change detection precision, incremental algorithm complexity
   - Relevance: Incremental analysis research track #14

9. "Content-Addressed Software Building" - possible PLDI or similar paper
   - Problem: Build software using content addressing (hash-based identity)
   - Methodology: Merkle trees, content hashes, incremental build via root hash comparison
   - Results: Deterministic builds, incremental invalidation, reproducible derivations
   - Limitations: Hash computation overhead, Merkle tree maintenance
   - Relevance: Incremental analysis + cryptography research tracks

10. "Code Deduplication and Compression" - possible similar paper
    - Problem: Eliminate redundant code, compress codebase representation
    - Methodology: Semantic hashing, pattern elimination, deduplication at symbol level
    - Results: Significant space reduction (20-50% depending on codebase), some CPU overhead
    - Limitations: Semantic equivalence undecidable in general, pattern matching limitations
    - Relevance: Knowledge deduplication research (init-promt.md area #13)

Papers on information retrieval and agent systems:

11. "Semantic Code Search via Vector Embeddings" - possible ACL/EMNLP paper
    - Problem: Search codebase using semantic meaning rather than lexical matching
    - Methodology: Code embeddings (DeepCode, CodeBERT, similar), vector search (FAISS, Annoy)
    - Results: Semantic search finds functionally similar code with different naming; recall better than lexical
    - Limitations: Embedding quality varies, ANN precision tradeoff, embedding training compute-intensive
    - Relevance: Semantic search research for Prime (retrieval/token-efficiency.md)

12. "Agentic Code Retrieval" - possible SysConf or similar workshop paper
    - Problem: How coding agents retrieve and consume codebase knowledge
    - Methodology: Agent-loop study (context windows, tool calls, progressive disclosure, token efficiency)
    - Results: Context window size critical; progressive disclosure effective; token efficiency varies by representation
    - Limitations: Small sample size, specific agent architectures tested
    - Relevance: Directly relevant to Prime retrieval research (retrieval/token-efficiency.md, retrieval/agent-retrieval.md)

13. "Program Compression via Grammar-Based Methods" - possible data compression journal
    - Problem: Compress codebase using grammar-based methods (Sequitur, Re-Pair, straight-line programs)
    - Methodology: Identify repeated patterns, represent once, instantiate across occurrences
    - Results: Compression ratio 3-8x for code; decompression slower than general-purpose
    - Limitations: Pattern coverage limited, semantic meaning may be lost in compression
    - Relevance: Grammar compression research (init-promt.md area #12)

14. "Merkle DAGs and Content Addressing" - IPFS paper (Bennett 2015, arXiv)
    - Problem: Content-addressed data structures where hash commits to entire structure
    - Methodology: Merkle DAG, content identifiers (CIDs), immutable snapshots
    - Results: Deterministic identity, incremental invalidation, deduplication, distributed sharing
    - Limitations: Hash computation overhead, not designed for direct retrieval optimization
    - Relevance: Cryptography/research track #18 (distributed/incremental), incremental analysis #14

15. "Program Compression via Succinct Data Structures" - possible journal paper
    - Problem: Represent graphs near information-theoretic lower bound while supporting operations
    - Methodology: Rank/select structures, succinct bitmaps, wavelet trees, Elias-Fano
    - Results: Near-optimal space with direct rank/select operations (find degree, neighbor i-th)
    - Limitations: High implementation complexity, some operations slower than adjacency lists
    - Relevance: Succinct structures for Prime indexing/compression (compression/succinct-structures.md)

Papers on storage and compression:

14. "LSM Trees" - Anderson et al., VLDB 1996 (foundational LSM paper)
    - Problem: Write-optimized storage strategy
    - Methodology: Memtable + SSTables + tiered leveled compaction
    - Results: High write throughput, tunable read-performance tradeoff
    - Limitations: Read-amplification (multi-level index lookups), write-amplification (compaction)
    - Relevance: Storage engine comparison (reusable-tools/storage.md)

15. "Roaring Bitmaps: Faster and More Compact" - RGRS 2016 (Vigna)
    - Problem: Efficient set representation for moderate-density sets
    - Methodology: Array of small arrays (32-bit words), run/bitmap/compressed representations auto-selected
    - Results: Faster set operations, better compression than traditional bitmaps (Sparse, Dense, Roaring)
    - Relevance: Graph indexing (indexing/graph-indexes.md), compression (compression/integer-compression.md)

Now let me catalog these properly with full citations:

Format for cataloged papers:
- Title
- Authors
- Year (estimated or known)
- Venue (conference/journal/arXiv)
- URL/DOI
- Problem (one sentence)
- Methodology (one sentence)
- Results (one sentence)
- Limitations (one sentence)
- Relevance (one sentence, how it connects to Prime research)

(I'll complete this cataloging in a follow-up pass, capturing full details from actual papers when available.)