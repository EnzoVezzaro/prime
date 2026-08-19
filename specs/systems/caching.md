Caching research:

- Caching strategies for codebase knowledge artifact:
  - Goal: Reduce latency for repeated agent queries by keeping hot knowledge in fast storage
  - Tradeoff: Memory overhead vs. query speedup, staleness management

- Page cache (OS-level):
  - Automatically caches file content read via mmap or explicit read/sread
  - LRU eviction: least recently used pages evicted first
  - Size limits: Typically percentage of available system memory
  - Usefulness: Hot symbols/relationships cached across agent queries
  - Prime relevance: Primary caching layer (mmap goes through page cache by default)

- Agent query cache:
  - Cache specific retrieval results (symbol metadata, reference lists) by query key
  - Usefulness: Repeated same query (e.g., "find references for AuthService") returns from cache
  - Cache key: Query parameters (symbol name, query type, confidence filters)
  - Cache entry: Retrieved knowledge + metadata (timestamp, source, provenance)
  - TTL (time-to-live): Invalidate cache entries after time period (reflects codebase changes)
  - Usefulness: Balance freshness vs. cache hit rate
  - Prime relevance: Agent loop benefits from repeated queries (navigation, exploration)

- Symbol metadata cache:
  - Cache symbol definition metadata (name, type, location) separately from full artifact
  - Usefulness: Agent frequently needs symbol metadata without full artifact retrieval
  - Invalidation: When symbol changes (file modification, re-indexing), invalidate entry
  - Prime relevance: Supports incremental analysis workflow (incremental-analysis.md)

- Relationship cache:
  - Cache adjacency lists (who calls X, who X calls) for frequently accessed symbols
  - Usefulness: Navigation queries benefit from cached relationship lookups
  - Incremental update: When codebase changes, update affected cached relationships
  - Prime relevance: Incremental update research area

- Cache coherence:
  - Ensuring cached knowledge reflects current codebase state
  - Strategies: Time-based invalidation (TTL), event-based invalidation (file change notifications), version-based (root hash comparison)
  - Usefulness: Agent receives stale knowledge if cache not invalidated properly
  - Prime relevance: Incremental analysis depends on accurate cached knowledge

- Warm cache strategies:
  - Pre-load hot symbols on agent startup (frequently navigated entry points)
  - Usefulness: Reduces initial query latency for new agent session
  - Hot symbol identification: Based on index access frequency, codebase structure (entry points, main modules)
  - Prime relevance: Agent experience (fast startup)

- Cache size strategies:
  - Fixed size (e.g., 10MB, 100MB): Simple, predictable memory usage
  - Adaptive size: Grow/shrink based on available memory, query patterns
  - Usefulness: Adaptive better for varying deployment environments (laptop vs. server)
  - Prime relevance: Deployable across device sizes (laptop to server)

- Cache eviction policies:
  - LRU (least recently used): Evict least recently accessed entries
  - LFU (least frequently used): Evict least frequently accessed entries
  - MRU (most recently used): Evict most recently accessed (less common)
  - Usefulness: LRU most common, LFU may better match agent query patterns (some symbols accessed repeatedly)
  - Prime relevance: Can configure based on deployment size/freshness requirements

- Read-ahead caching:
  - Predict next accesses and proactively load into cache
  - Patterns: Sequential symbol browsing, frequently followed relationship chains
  - Usefulness: Reduce latency for predictable access patterns
  - Prime relevance: Agent browsing patterns may be sequential (explore codebase in order)

- Write-back / write-through caching:
  - Write-through: Write knowledge to both cache and persistent store immediately
  - Write-back: Write to cache only, flush to persistent later (on eviction or trigger)
  - Usefulness: Write-back reduces write amplification, but risk of data loss if crash
  - Prime relevance: Prime is read-heavy (agent retrieval), write-back suitable (updates via incremental indexing)

- Caching for incremental analysis:
  - Cache incremental change detection results (content hashes, Merkle tree roots)
  - Cache invalidation patterns (which symbols change when file X modifications)
  - Usefulness: Speed up incremental analysis workflow (re-indexing only changed parts)
  - Prime relevance: Core to incremental analysis research area (incremental-analysis.md)

- Reusable caching components:
  - Page cache interface (leverage OS page cache, or custom implementation)
  - Agent query cache (key-value store for retrieval results)
  - Symbol metadata cache (symbol definition cache)
  - Relationship cache (cached adjacency lists)
  - Cache coherence manager (invalidation strategies)
  - Warm cache preloader (hot symbol identification and preloading)
  - Cache size manager (fixed/adaptive sizing)
  - Eviction policy selector (LRU/LFU/MRU configuration)
  - Read-ahead predictor (pattern-based proactive loading)