Concurrency research:

- Concurrent agent queries: Multiple agents (or multiple queries from same agent) accessing Prime artifact simultaneously:
  - Usefulness: Modern agents often parallelize tasks; Prime must support concurrent access

- mmap concurrent access:
  - Read-only concurrent: Multiple processes/agents can read same mmap'd region simultaneously (no conflict)
  - Usefulness: Multiple agents can query artifact in parallel
  - Mechanism: OS page cache shares read-only mappings; copy-on-write for modifications
  - Prime relevance: Read-dominant Prime workload (agent retrieval)

- Read-write concurrency:
  - Readers-unblocked design (LMDB, MMAP with proper flags): Multiple readers, single writer, readers not blocked by writer
  - Usefulness: Allow incremental indexing/writing while agents query
  - Prime relevance: Support concurrent indexing and querying

- Prime artifact modification during querying:
  - Incremental updates: Adding/modifying symbols while agents query
  - Consistency model: eventual consistency (index slightly stale) vs. strong consistency (query blocks during write)
  - Usefulness: Incremental updates should not halt agent productivity
  - Prime relevance: Research question (incremental-analysis.md)

- Lock-free data structures for index access:
  - Usefulness: High concurrency without lock overhead
  - Examples: Atomic operations, lock-free queues, concurrent hash maps
  - Prime relevance: Index structures (symbol table, inverted index) can be lock-free for read-dominant workload

- Transactional access for incremental updates:
  - Begin transaction, make changes, commit (atomic across index structures)
  - Usefulness: Ensure index consistency during concurrent updates
  - Prime relevance: Incremental indexing may modify multiple index structures atomically

- Content-addressed concurrency:
  - Merkle DAG: Content addressing provides implicit consistency (hash changes reflect content changes)
  - Usefulness: Agents can detect staleness via root hash comparison (no lock needed for consistency check)
  - Prime relevance: Content addressing research area (distributed/incremental)

- Concurrent index structures:
  - Concurrent hash map: Multiple threads can insert/lookup simultaneously (sharding by key space)
  - Read-write lock: Separate read lock (multiple holders) from write lock (exclusive)
  - Usefulness: Scale concurrent queries across multiple threads/processes
  - Prime relevance: Index structures (symbol table, inverted index, relationship graph) must scale to concurrent agents

- Query parallelization:
  - Agent splits complex query into parallel sub-queries (e.g., search multiple modules simultaneously)
  - Usefulness: Faster query completion for broad searches
  - Result merging: Combine parallel results, deduplicate, re-rank
  - Prime relevance: Agent architecture research (not core Prime, but interaction pattern)

- Multi-process agent coordination:
  - Multiple agent processes collaborating on same codebase knowledge
  - Knowledge sharing: Via shared mmap region, message passing, or content-addressed exchange
  - Usefulness: Team of agents working together on large codebase
  - Prime relevance: Distributed Prime research (CRDTs, P2P research areas)

- Concurrent incremental indexing:
  - while agents query, indexer adds/modifies symbols, relationships
  - Incremental update algorithm: content hashing, delta computation, targeted index updates
  - Consistency: Agents see consistent (possibly slightly stale) view
  - Usefulness: Index stays current without halting agent queries
  - Prime relevance: Core incremental analysis challenge (incremental-analysis.md)

- Thread safety of knowledge entries:
  - Immutable knowledge entries: Once created, never modified (safe for concurrent read)
  - Versioned knowledge entries: Each modification creates new version; old versions remain readable
  - Usefulness: Immutable entries trivially thread-safe; versioned supports incremental updates
  - Prime relevance: Design Prime knowledge entries as immutable (append-only with new versions)

- Reusable concurrency components:
  - mmap concurrent read setup (flags, sharing)
  - Read-write lock implementation (or use OS-provided)
  - Lock-free index structures (concurrent hash map, atomic operations)
  - Transaction manager (begin/commit for atomic updates)
  - Content hash comparator (Merkle DAG root comparison for staleness)
  - Versioned knowledge entry store (immutable entries + version tracking)
  - Parallel query dispatcher (split complex query into parallel sub-queries)
  - Agent coordination interface (shared knowledge, message passing)