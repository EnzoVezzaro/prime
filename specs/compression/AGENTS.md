# compression

## Purpose

Research on compression techniques applicable to codebase knowledge artifacts.

## Responsibilities

- Research integer compression (varints, SIMD-BP128, Stream VByte, PForDelta, Frame-of-Reference, Elias coding)
- Research graph compression (delta encoding, adjacency compression, WebGraph, succinct graphs)
- Research string compression (dictionary encoding, front coding, tries, FSTs)
- Research general compression (zstd, lz4, brotli, gzip, lzma)
- Analyze tradeoffs: compression ratio vs CPU cost vs random access vs decompression cost vs I/O reduction

## Ownership

Owner: research team

## Inputs

- Academic papers on compression algorithms
- Existing implementations (zstd, lz4, brotli, SIMD compression libraries)
- WebGraph, Zuckerli for graph compression
- Elias-Fano, Roaring bitmaps, Sux4J for succinct structures

## Outputs

- SPECS/compression/integer-compression.md
- SPECS/compression/graph-compression.md
- SPECS/compression/string-compression.md
- SPECS/compression/general-compression.md

## Dependencies

- SPECS/indexing/ (succinct structures, Elias-Fano, Roaring bitmaps)
- SPECS/storage/ (storage formats that integrate compression)
- SPECS/prior-art/ (WebGraph, CPG compression)

## Constraints

- Objective is NOT simply smallest file; objective is smallest useful representation with fastest retrieval
- Tradeoff analysis required: compression ratio vs CPU cost vs random access vs decompression cost vs I/O reduction
- Must support random access patterns for agent retrieval

## Architecture

Four research files covering different compression domains. Tradeoff analysis in comparison tables.

## Workflows

- See `.acc/config/workflows/research.md` for conducting compression research.
- See `.acc/config/workflows/feature.md` for adding a new compression technique.