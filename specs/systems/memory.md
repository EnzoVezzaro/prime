Memory research:

- Memory usage patterns for codebase knowledge artifacts:
  - Artifact size: Total bytes representing indexed codebase knowledge
  - Working set: Frequently accessed portion of artifact (symbols, relationships agent currently exploring)
  - Total vs working set: Artifact may be 50MB-500MB+; working set typically 10KB-500KB per agent task
  - Usefulness: Distinguish artifact-scale vs. task-scale memory requirements

- Memory-mapped artifact:
  - Entire artifact mapped into process address space via mmap
  - Physical memory usage: Only pages actually touched (accessed) are resident
  - Virtual address space: Entire artifact appears available (64-bit: effectively unlimited)
  - Usefulness: Prime's primary memory strategy (zero-copy, page-fault-driven)
  - Prime relevance: Core memory strategy (init-promt.md: mmap research)

- Working set size:
  - Per-agent-task: Symbols + relationships currently being explored (typically 10KB-500KB)
  - Across-tasks: Cumulative across agent session (grows as agent explores, shrinks with cache eviction)
  - Usefulness: Distinguish per-task vs. cumulative memory needs
  - Prime relevance: Design for working set << artifact size (agent doesn't need entire artifact in memory)

- Cache hierarchy effects:
  - CPU L1 cache: ~32KB per core, fastest (1-cycle access)
  - CPU L2 cache: ~256KB-8MB per core, medium speed (10- cycle access)
  - CPU L3 cache: ~8-96MB shared, slower (40- cycle access)
  - Memory (RAM): ~10-100ns access latency
  - SSD/NVMe: ~25-100μs (random read) for page faults from mmap
  - Usefulness: Understand access latency at each level; optimize for hot working set in CPU caches
  - Prime relevance: Knowledge artifact structure can enhance cache locality (e.g., related symbols close in memory)

- Memory allocation overhead:
  - malloc/free: Syscall overhead, fragmentation, allocation metadata
  - mmap: No allocation overhead per access (page faults handle data retrieval)
  - Usefulness: mmap avoids per-access allocation overhead
  - Prime relevance: Prime should use mmap, avoid per-symbol allocations

- Memory fragmentation:
  - External fragmentation: Free memory scattered in small blocks, larger allocations impossible
  - Internal fragmentation: Allocated memory larger than requested (alignment, page granularity)
  - Usefulness: mmap reduces fragmentation (no small allocations from heap)
  - Prime relevance: Codebase knowledge artifact accessed via mmap; fragmentation minimal

- Prime memory design recommendations:
  1. mmap entire knowledge artifact (zero-copy, lazy page loading)
  2. Working set tracking: Track which pages/regions agent actively uses
  3. madvise(MADV_SEQUENTIAL/MADV_RANDOM): Tune kernel behavior based on access pattern
  4. NUMA awareness: Bind artifact mapping to local memory node on multi-socket systems
  5. Page cache utilization: Let OS manage page cache; hot symbols stay in cache across queries
  6. Avoid per-symbol allocations: Use offsets/pointers into mmap'd region instead
  6. Memory budgeting: Set upper memory limit (e.g., 1GB); beyond that, incremental page loading
  7. Cache-friendly artifact structure: Design binary format for good cache locality (related symbols close in memory)

- Incremental memory growth:
  - Agent session gradually accesses more of artifact (working set grows)
  - No need to load entire artifact upfront (mmap laziness)
  - Usefulness: Agent can start exploring immediately; memory grows naturally
  - Prime relevance: Supports agent loop (work iteratively, init-promt.md)

- Memory pressure signals:
  - Page fault rate increase: Working set growing, approaching memory limits
  - Process VM size growth: mmap'd region growing (but physical pages may not increase proportionally)
  - Cache miss rate increase: Working set exceeding CPU cache capacity
  - Usefulness: Trigger incremental compaction, compaction, or agent guidance (focus on smaller subset)
  - Prime relevance: Auto-balance memory usage vs. exploration depth

- Reusable memory management components:
  - mmap management (map, unmap, page fault handling)
  - Working set tracker (which pages/regions actively used)
  - madvise advisor (set access pattern advice to kernel)
  - NUMA affinity setter (bind to local memory node)
  - Page cache auditor (monitor cache hit rate, size)
  - Memory budget enforcer (enforce upper memory limit)