Integer Compression research:

### Varints
- **Theory**: Variable-length encoding where each byte uses 7 bits for data and 1 bit to indicate if more bytes follow
- **Compression ratio**: 2-4x for typical integers (varies based on value distribution)
- **CPU cost**: Very low (simple byte-level operations)
- **Random access**: Possible but requires scanning from start or using lookup tables; not direct
- **Decompression cost**: None (encoding IS the representation)
- **I/O reduction**: Modest
- **Best for**: Small integers, values < 128 frequent, situations where decompression cost must be zero
- **Examples**: Protocol Buffers varint, Google's protobuf, LEB128 (used in DWARF, WASM)
- **Prime relevance**: Good for encoding symbol IDs, small integers in knowledge representation where random access is not required

### SIMD-BP128
- **Theory**: Block-based integer compression using SIMD instructions for parallel decompression
- **Compression ratio**: 3-8x depending on value distribution
- **CPU cost**: Low (SIMD parallelism reduces per-element cost)
- **Random access**: With correction data (bucket values at regular intervals); access time O(n/block_size)
- **Decompression cost**: Low (SIMD-accelerated)
- **I/O reduction**: Good
- **Best for**: Large arrays of integers where random access with some overhead is acceptable
- **Examples**: FastBit, various database implementations
- **Prime relevance**: Good for compressing large symbol ID arrays, reference counts, or other integer metadata where SIMD is available

### Stream VByte
- **Theory**: Variable-length encoding using 7 bits per byte plus 1 continuation bit, optimized for streaming
- **Compression ratio**: 3-5x for typical distributions
- **CPU cost**: Low (simple byte operations)
- **Random access**: Requires scanning from start or using indexed gaps; not direct
- **Decompression cost**: Low
- **I/O reduction**: Good
- **Best for**: Variable-length integers, streaming workloads
- **Examples**: Lucene, various search engine implementations
- **Prime relevance**: Good for compressing variable-size symbol identifiers or metadata where streaming access pattern fits

### PForDelta
- **Theory**: Packed with Forward-Delta encoding; groups integers into buckets with a base and deltas
- **Compression ratio**: 3-6x depending on distribution and bucket size
- **CPU cost**: Medium (bucket processing, delta encoding/decoding)
- **Random access**: With bucket-level access; requires reading bucket base + computing deltas
- **Decompression cost**: Medium
- **I/O reduction**: Good
- **Best for**: Batch processing, sorted or sorted-like data
- **Examples**: Databases (PostgreSQL, MySQL), indexing systems
- **Prime relevance**: Good for compressing sorted symbol IDs, reference lists, or other integer sequences with moderate spread

### Frame-of-Reference
- **Theory**: Subtracts a base value from each integer, then encodes deltas relative to the frame base
- **Compression ratio**: 3-5x for sorted sequences
- **CPU cost**: Low (subtraction + encoding)
- **Random access**: Requires knowing the frame base; access to element i requires base[i] + delta[i]
- **Decompression cost**: Low
- **I/O reduction**: Good
- **Best for**: Sorted or nearly-sorted integer sequences
- **Examples**: Used in various database and indexing systems
- **Prime relevance**: Good for compressed symbol ID lists, if IDs can be partially sorted or grouped

### Elias coding (Gamma, Delta, Omega)
- **Theory**: Prefix-free coding based on unary code length representation
- **Gamma**: Good for integers < 2^16, ratio ~3-4x
- **Delta**: Good for integers < 2^32, ratio ~4-6x  
- **Omega**: Good for very large integers, ratio improves with size
- **CPU cost**: Low (bit operations)
- **Random access**: Not direct (requires knowing prefix length)
- **Decompression cost**: Low
- **I/O reduction**: Good
- **Best for**: General-purpose integer compression where values vary widely
- **Examples**: Used in various compression schemes, coding theory
- **Prime relevance**: Good as component of hybrid compression, general-purpose integer encoding

## Compression Technique Selection Matrix for Prime

| Criterion | Varints | SIMD-BP128 | Stream VByte | PForDelta | Frame-of-Reference | Elias |
|-----------|---------|------------|--------------|-----------|-------------------|-------|
| **Small integers (< 2^7)** | Excellent | Good | Good | Fair | Fair | Fair |
| **Large dataset (> 1K integers)** | Poor | Good | Good | Good | Good | Good |
| **Random access required** | Direct | With buckets | Scanning | With buckets | Frame-based | No |
| **SIMD available** | N/A | Excellent | N/A | Good | Good | N/A |
| **Sorted data** | Poor | Good | Good | Excellent | Excellent | Fair |
| **Implementation complexity** | Very low | Medium | Low | Medium | Low | Low |
| **I/O reduction priority** | Low | Good | Good | Good | Good | Good |
| **CPU cost priority** | Lowest | Low | Lowest | Medium | Low | Low |

## Recommendations for Prime:

1. **Varints**: Use for encoding symbol IDs, small metadata integers where random access from anywhere in the stream is needed. Zero CPU cost for decompression.

2. **SIMD-BP128**: Use for large integer arrays (> 1K elements) where some random access overhead is acceptable and SIMD instructions are available. Good balance of compression ratio and speed.

3. **PForDelta**: Use for sorted or batch-processed integer sequences (e.g., reference lists, occurrence lists). Good compression with acceptable random access.

4. **Frame-of-Reference**: Use when symbol IDs can be grouped/sorted within frames. Good compression for structured data.

5. **Elias coding**: Use as component in hybrid approaches or general-purpose integer encoding where value range is unknown.

6. **Hybrid approach**: Prime should likely combine multiple techniques - varints for frequently accessed small integers, SIMD-BP128 or PForDelta for large batches, and potentially Frame-of-Reference for grouped data.

7. **The overarching principle**: "Smallest useful representation with fastest retrieval" - Prime must balance compression ratio against the cost of decompression when random access is required during agent queries.