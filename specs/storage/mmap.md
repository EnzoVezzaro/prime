Memory Mapping research:

- mmap (memory-mapped files): Operating system mechanism that maps file content into process address space:
  - Uses virtual memory management (page tables)
  - Access via regular memory reads/writes (no syscalls per access after mapping)
  - Usefulness: High (efficient large file access, agent knowledge artifact access)

- Page cache: OS-managed cache of file content in physical memory:
  - Acts as buffer between disk and process
  - mmap uses page cache (or bypasses it, depending on OS/flags)
  - Usefulness: High (reduces I/O, improves performance)

- Page faults: Interrupts triggered when accessing unmapped or evicted pages:
  - Minor page fault: Page already in memory (file system cache)
  - Major page fault: Page must be read from disk
  - Usefulness: Understanding performance characteristics, optimization strategies

- Sequential vs random reads:
  - Sequential: Contiguous access pattern, prefetching effective
  - Random: Non-contiguous access, prefetching less effective, more page faults
  - Usefulness: Prime access patterns should be designed for workload

- SSD behavior: Solid-state drive performance characteristics:
  - Random read latency: ~25-100 microseconds
  - Sequential read throughput: ~500 MB/s - 3 GB/s
  - Endurance: Limited write cycles (but read-intensive workloads fine)
  - Usefulness: Prime deployed on SSD-aware systems

- NVMe behavior: Non-volatile memory express protocol for SSDs:
  - Lower latency than SATA/SAS SSDs (10-50 microseconds random read)
  - Higher throughput (3-7 GB/s)
  - Command queue depth optimization
  - Usefulness: Prime on NVMe storage gains performance

- Filesystem caching: OS-level cache of file content:
  - Least Recently Used (LRU) eviction policy commonly
  - Size limits (percentage of available memory)
  - Usefulness: Understanding effective access speed (cached vs uncached)

- Memory locality: CPU cache friendliness of access patterns:
  - Spatial locality: Accessing nearby memory addresses
  - Temporal locality: Re-accessing same memory addresses
  - Usefulness: Determines CPU cache hit rate, affects mmap performance

- CPU cache locality: L1/L2/L3 cache effects:
  - Small access patterns fit in L1 cache (fastest)
  - L2 cache medium, L3 larger shared
  - mmap effectiveness depends on working set size vs cache sizes
  - Usefulness: Prime working set design consideration

- NUMA (Non-Uniform Memory Access): Multi-socket server memory architecture:
  - Memory access time depends on processor-memory location
  - NUMA locality: Accessing local memory much faster than remote
  - Usefulness: Prime on multi-socket systems must consider NUMA

- Prefetching: Hardware/software hinting of upcoming memory accesses:
  - Hardware prefetcher: Detects sequential patterns automatically
  - Software prefetching: Explicit prefetch instructions
  - Usefulness: Optimizing mmap access patterns for hardware

- Zero-copy access: Accessing data without copying between kernel and user space:
  - mmap enables zero-copy file reading
  - Shared memory regions between processes
  - Usefulness: High (reduces allocation overhead, latency)

- Allocations: Memory allocation overhead avoided with mmap:
  - malloc/free syscalls not needed for mmap'd data
  - Fragmentation reduced (no small allocations from file mapping)
  - Usefulness: High (especially for large codebase knowledge artifacts)

- File size limits: Practical limits on mmap'd file size:
  - 32-bit systems: 2-4 GB address space limit
  - 64-bit systems: Effectively unlimited (virtual address space)
  - Usefulness: Prime must consider target deployment platform

- Advisory vs mandatory locking: mmap + file locking:
  - Advisory: Voluntary cooperation required
  - Mandatory: OS-enforced (rarely used, performance overhead)
  - Usefulness: Understanding concurrent access patterns

- Madvise (POSIX): Advice to kernel about expected usage pattern:
  - MADV_SEQUENTIAL: Expect sequential access (prefer readahead)
  - MADV_RANDOM: Expect random access (don't prefetch)
  - MADV_DONTNEED: No longer needed (release pages)
  - MADV_WILLNEED: Will need soon (prefetch)
  - Usefulness: Tuning kernel behavior for Prime access patterns

- mmap vs read/sread tradeoffs:
  - mmap advantages: Zero-copy, no allocation, page-fault-driven access
  - mmap disadvantages: Page fault overhead, NUMA considerations, file size limits (32-bit)
  - read/sread advantages: Predictable, no page faults, works on all platforms
  - read/sread disadvantages: Allocation overhead, copy required, syscall per read
  - Prime should benchmark both and choose based on workload profiling

- Security considerations: mmap security implications:
  - Memory protection (read-only, read-write, execute-none)
  - Address space layout randomization (ASLR) effects
  - Potential for information leakage via page cache
  - Usefulness: Prime must design security-aware mmap usage

- Prime mmap design recommendations:
  1. Map codebase knowledge artifact as mmap for zero-copy access
  2. Use MADV_RANDOM or MADV_SEQUENTIAL based on access pattern profiling
  3. Design binary format for deterministic field offsets (enables random access via mmap)
  4. Consider NUMA locality when allocating large mmap regions
  5. Use MADV_DONTNEED for unused portions (incremental loading)
  6. Page fault rate monitoring and adaptation
  6. madvise() calls between access pattern changes