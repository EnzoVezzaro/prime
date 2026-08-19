Agent Retrieval research:

- Agent retrieval: How coding agents retrieve codebase knowledge from the Prime artifact:
  - Two dominant patterns:
    1. Retrieval-based: Fetch pre-computed index entries (symbols, references, definitions)
    2. Query-based: Run graph queries or pattern matches against stored structure
  - Most effective: Hybrid approach (structural + semantic)
  - Prime relevance: Design Prime's retrieval around agent loop patterns

- Retrieval patterns for coding agents:
  - "Find symbol": Agent needs location and metadata for a specific symbol name
  - "Find references": Agent needs all places a symbol is used/referred
  - "Find implementations": Agent needs concrete implementations of an interface/abstract class
  - "Navigate relationship": Agent needs to follow a relationship (calls → called by → callee's calls)
  - "Explore context": Agent needs surrounding context for a symbol (callers, callees, related symbols)
  - "Search codebase": Agent needs to find symbols matching pattern/criteria
  - Prime relevance: Prime must support these core patterns efficiently

- Context selection strategies:
  - Minimal context: Retrieve smallest sufficient representation for agent task
  - Progressive disclosure: Start with minimal context, expand as agent asks follow-up questions
  - Surrounding context: Retrieve symbol + callers + callees + related symbols (fixed-size chunk)
  - Usefulness: Balances token usage with agent usefulness
  - Prime relevance: Core design constraint (token efficiency)

- Retrieval granularity:
  - Symbol-granular: Retrieve one symbol at a time (metadata + location)
  - Chunk-granular: Retrieve fixed-size chunks (symbol + neighbors + metadata)
  - File-granular: Retrieve entire source file (least granular, most tokens)
  - Usefulness: Smaller granularity = better token efficiency, but more requests needed
  - Prime relevance: Should support multiple granularities; default to chunk-granular

- Context selection algorithms:
  - Degree-based: Retrieve symbol + N most important neighbors (by centrality, frequency)
  - Recency-based: Retrieve recently accessed or modified symbols
  - Importance-based: Retrieve symbols with highest centrality in codebase graph
  - Usefulness: Different strategies for different agent tasks
  - Prime relevance: Should support configurable context selection strategy

- Token efficiency:
  - Tokens per retrieval: Number of tokens agent receives per query
  - Token cost: Agent's context window limited; every token consumes limited resource
  - Usefulness: Maximize usefulness per token (retrieve most relevant, minimal sufficient knowledge)
  - Prime relevance: Central research question (defined in init-promt.md)

- Minimum information an agent needs:
  - Symbol name and kind: What is this entity?
  - Location: Where is it defined? (file:line)
  - Type signature: What are parameters and return type?
  - Direct references: Where is it used immediately?
  - Related symbols: What other symbols is it connected to?
  - Usefulness: Defines "minimum useful representation" concept from init-promt.md

- Retrieval latency:
  - Target: Sub-100ms for single symbol retrieval (SSD + mmap + optimized index)
  - Multi-symbol: O(log n) per symbol with good index structure
  - Prime relevance: Agent experience depends on fast retrieval (interactive feel)

- Cached retrieval:
  - Cache hot symbols/relationships in fast memory (L1/L2 cache, SSD)
  - Usefulness: Repeated agent queries benefit from cache
  - Tradeoff: Memory overhead, staleness management
  - Prime relevance: Agent query patterns likely have hot symbols (entry points, frequently navigated)

- Failed retrieval handling:
  - Symbol not found: Graceful degradation, suggest similar symbols
  - Partial retrieval: Some knowledge retrieved, rest unavailable (with confidence annotation)
  - Usefulness: Agent can decide how to proceed with incomplete knowledge
  - Prime relevance: Graceful degradation principle from research principles (init-promt.md)

- Reachable retrieval strategies:
  - Single-hop: Retrieve immediate neighbors (callers, callees)
  - Multi-hop: Retrieve N-hop reachable symbols (transitive closure)
  - Usefulness: Single-hop for quick navigation, multi-hop for understanding impact
  - Prime relevance: Support both; multi-hop may require precomputed index or on-demand traversal

- Batch retrieval:
  - Retrieve multiple symbols/relationships in single operation
  - Usefulness: Fewer round trips, better agent experience
  - Implementation: Batch query API, bulk index lookups
  - Prime relevance: Agent may request multiple symbols at once (e.g., "all functions in this module")

- Retrieval privacy:
  - Query logging, data minimization (only retrieve what agent needs)
  - Usefulness: If Prime operates over sensitive repositories
  - Prime relevance: Research area (searchable encryption etc., but may be overkill for v1)

- Reusable agent retrieval components:
  - Query parser (natural language → structured query)
  - Index lookup engine (symbol, reference, relationship indexes)
  - Context selector (determine what to retrieve given query)
  - Token budget manager (ensure retrieval fits within agent context window)
  - Result formatter (format retrieved knowledge for agent consumption)