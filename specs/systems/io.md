I/O research:

- I/O patterns for codebase knowledge artifacts:
  - Sequential reads: Reading consecutive sections of artifact (e.g., browsing symbols in order)
  - Random reads: Jumping between distant symbols/relationships (typical agent retrieval pattern)
  - Mixed patterns: Alternating sequential and random (common in agent loops)
  - Usefulness: Design Prime I/O around typical agent query patterns

- Bytes read: Primary I/O metric for agent retrieval performance:
  - Target: Minimize bytes read per agent query
  - Optimization: Chunked retrieval (only read needed knowledge portions)
  - mmap benefit: Pages touched = bytes read / page_size; zero-copy reduces explicit read bytes
  - Prime relevance: Core metric (init-promt.md: "measured in bytes read")

- Pages touched: Number of memory pages accessed during retrieval:
  - Page size: Typically 4KB (OS-dependent)
  - Target: Minimize pages touched per agent query
  - mmap effect: Pages touched depends on working set size vs. page granularity
  - Prime relevance: mmap-based retrieval measured in pages touched (init-promt.md)

- Sequential vs random reads:
  - Sequential: Contiguous access pattern, OS prefetching effective
    - Benefit: Fewer page faults, higher throughput
    - Usefulness: Prefetching, browsing consecutive symbols
  - Random: Non-contiguous access, prefetching less effective, more page faults
    - Usefulness: Typical agent retrieval (jump between unrelated symbols)
    - Optimization: Chunked compression + mmap (independent chunks)
  - Prime relevance: Agent patterns largely random; design for random-read optimization

- SSD behavior:
  - Random read latency: ~25-100 microseconds
  - Sequential read throughput: ~500 MB/s - 3 GB/s
  - Endurance: Read-intensive workloads fine (write endurance less concern)
  - Usefulness: Prime deployed on SSD or better (NVMe)
  - Optimization: 4KB alignment, avoid read-modify-write patterns

- NVMe behavior:
  - Random read latency: ~10-50 microseconds (faster than SSD)
  - Sequential read throughput: ~3-7 GB/s (faster than SSD)
  - Command queue depth: Optimization for parallel I/O requests
  - Usefulness: Prime on NVMe storage gains significant performance
  - Optimization: Parallel query requests, command queue awareness

- Filesystem caching:
  - OS-level page cache: Caches file content in physical memory
  - LRU eviction: least recently used pages evicted first
  - Size limits: Typically percentage of available memory
  - Usefulness: Hot knowledge artifact portions cached, reducing effective latency
  - Prime relevance: Agent query patterns may have hot symbols; cache benefits repeated queries

- Memory-mapped I/O (mmap) advantages:
  - Zero-copy: No data copying between kernel and user space
  - Page fault-driven: Pages loaded on demand (lazy)
  - Virtual address space: Entire artifact appears in process address space
  - Usefulness: High (reduces allocation overhead, latency, bytes transferred)
  - Prime relevance: Core I/O strategy (init-promt.md optimize for bytes read, pages touched)

- mmap disadvantages:
  - Page fault overhead: Each page fault has latency (though minor if page in cache)
  - NUMA considerations: Remote memory access slower than local
  - 32-bit address space limits: Limited to 2-4GB (64-bit essentially unlimited)
  - Security: Potential information leakage via page cache remnants
  - Usefulness: Must weigh against benefits for Prime deployment platform

- Prime I/O design recommendations:
  1. mmap artifact as primary I/O mechanism (zero-copy, page-fault-driven)
  2. Design binary format for deterministic field offsets (enables random access via mmap page faults)
  3. Chunk compression (zstd/lz4) with independent chunks (enables random access without full decompress)
  4. Profile access patterns (hot symbols, query types) and prefetch hot regions into page cache
  5. NUMA-aware allocation on multi-socket systems (local memory access)
  6. madvise() usage: MADV_SEQUENTIAL for pre read-ahead, MADV_RANDOM for random workloads
  7. Monitor page fault rate and adapt mmap parameters dynamically
  8. Target metrics: Minimize bytes read per agent query, minimize pages touched per retrieval

- I/O benchmark considerations:
  - Measure bytes read per typical agent query (find symbol, find references, navigate relationship)
  - Measure latency (page fault rate, total query time)
  - Compare mmap vs. explicit read/sread approaches
  - Compare compression strategies impact on bytes read vs. decompression cost
  - Prime relevance: Establish baseline, measure optimization impact