# indexing

## Purpose

Research on index structures for codebase knowledge artifacts and agent retrieval.

## Responsibilities

- Research symbol indexes (hash tables, inverted indexes, inverted indexes with confidence)
- Research graph indexes (adjacency lists, CSR/CSC, Roaring bitmaps, succinct structures)
- Research search indexes (inverted, vector/ANN, hybrid, structural)
- Research succinct structures (rank/select, Elias-Fano, Roaring bitmaps, FSTs, wavelet trees)

## Ownership

Owner: research team

## Inputs

- Academic papers on succinct data structures (rank/select, Elias-Fano, Roaring bitmaps)
- Sux4J, Roaring bitmap implementations
- FAISS, Annoy, ScaNN for vector search
- WebGraph, Zuckerli for graph compression

## Outputs

- SPECS/indexing/symbol-indexes.md
- SPECS/indexing/graph-indexes.md
- SPECS/indexing/search-indexes.md
- SPECS/indexing/succinct-structures.md

## Dependencies

- SPECS/compression/ (integer compression, succinct structures)
- SPECS/storage/ (storage formats that integrate indexes)
- SPECS/retrieval/ (search indexes for agent retrieval)

## Constraints

- Determine which index structures materially improve Prime's storage efficiency
- Support random access for agent retrieval
- Tradeoff: index size vs query speed vs construction cost

## Architecture

Four research files covering different index domains. Symbol indexes for direct lookup, graph indexes for relationship traversal, search indexes for query, succinct structures for space efficiency.

## Workflows

- See `.acc/config/workflows/research.md` for conducting indexing research.
- See `.acc/config/workflows/feature.md` for adding a new index structure.