General Compression research:

- zstd (Zstandard):
  - Development: Facebook (now open source)
  - Compression ratio: 2-10x+ (configurable via compression level)
  - CPU cost: Medium (level 1 fast, level 22+ very slow)
  - Decompression cost: Very low (SIMD-accelerated, often > 5GB/s)
  - I/O reduction: Very Good (especially for larger compression ratios)
  - Random access: With chunked decompression (decompress independent chunks)
  - Best for: General-purpose, high compression ratio, fast decompression
  - Prime relevance: Primary general compression algorithm; can chunk for random access

- lz4:
  - Development: Yann Collet, very fast
  - Compression ratio: 2-4x (limited, but very fast)
  - CPU cost: Very low (comparable to memcpy speed)
  - Decompression cost: Extremely low (often > 10GB/s)
  - I/O reduction: Good
  - Random access: With block-based format (independent blocks)
  - Best for: Real-time, streaming, scenarios where speed is critical
  - Prime relevance: Real-time indexing, incremental update scenarios

- brotli:
  - Development: Google (originally for Web)
  - Compression ratio: 4-10x+ for text, moderate for binary
  - CPU cost: Medium-High (optimized for compression ratio over speed)
  - Decompression cost: Medium (faster than zstd at similar ratios)
  - I/O reduction: Very Good for text-like data
  - Random access: With chunked decompression
  - Best for: Text-dominant data, web-style workloads
  - Prime relevance: Code/text-dominant knowledge artifacts

- gzip:
  - Development: GNU zip, very old, widespread
  - Compression ratio: 2-4x
  - CPU cost: Low
  - Decompression cost: Low
  - I/O reduction: Good
  - Random access: With gzip file format (original position, but limited)
  - Best for: Legacy, widespread support, moderate compression
  - Prime relevance: Compatibility, not optimal for Prime

- lzma/xz:
  - Development: LZMA SDK, 7-Zip integration
  - Compression ratio: 5-10x+ (highest of common algorithms)
  - CPU cost: High (slow compression)
  - Decompression cost: Medium (faster than compression)
  - I/O reduction: Very Good (highest ratio)
  - Random access: Limited (designed for sequential)
  - Best for: Maximum compression, archival
  - Prime relevance: Not optimal (slow compression, sequential bias)

- SIMD compression (SIMD-BP128, etc. from integer section):
  - Applies SIMD parallelism to general compression
  - Usefulness: Parallelizing compression/decompression
  - Prime relevance: Can parallelize zstd/lz4 decompression for agent queries

## Compression Technique Selection Matrix for Prime

| Technique | Ratio | CPU (Compress) | CPU (Decompress) | Random Access | I/O Reduction | Best Use Case |
|-----------|-------|----------------|------------------|---------------|---------------|---------------|
| **zstd** | Very Good (2-10x+) | Medium | Very Low | With chunking | Very Good | General purpose, primary candidate |
| **lz4** | Good (2-4x) | Very Low | Very Low | With block stabaling | Good | Real-time, incremental updates |
| **brotli** | Very Good (4-10x+) | Medium-High | Medium | With chunking | Very Good | Text-dominant code knowledge |
| **zlib/gzip** | Good (2-4x) | Low | Low | Limited | Good | Legacy, compatibility |
| **lzma/xz** | Very Good (5-10x+) | High | Medium | Limited | Very Good | Archival, not Prime-favored |
| **Chunked zstd** | Variable | Medium | Medium | Direct (per chunk) | Good | Prime: balance ratio/speed/access |

## Recommendations for Prime:

1. **zstd as primary**: Use zstd as the default general-purpose compression algorithm. Configurable compression levels allow trading ratio for speed. Chunked decompression enables random access.

2. **lz4 for incremental**: Use lz4 for scenarios where rapid incremental updates are needed (compression/decompression speed priority over ratio). Block-based format supports random access per block.

3. **brotli for text-dominant**: If knowledge artifact is text-dominant (symbol names, documentation, comments), brotli may provide better ratio than zstd with acceptable decompression speed.

4. **Chunked approach**: Prime should implement chunked compression (e.g., 64KB-1MB chunks) to enable random access without decompressing entire artifact. This is critical for agent retrieval (agent needs specific knowledge, not entire artifact).

5. **Compression level strategy**:
   - Level 1-3 (fast): For frequently updated knowledge, hot paths
   - Level 5-9 (balanced): For general-purpose knowledge artifact
   - Level 12+ (slow): For archival, rarely changed knowledge

6. **Compression integration points**:
   - On-disk artifact chunks
   - Network transfer chunks (agent retrieval)
   - Memory-to-disk persistence
   - Incremental update delta compression

7. **The overarching principle**: "Smallest useful representation with fastest retrieval" - Prime must balance compression ratio against the cost of decompression when agents need to retrieve specific knowledge without reading the entire artifact.