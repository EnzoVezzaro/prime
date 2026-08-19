Context Selection research:

- Context selection: Determining what codebase knowledge to retrieve for an agent given a query:
  - Core problem: Agent has limited context window (tokens); must maximize usefulness per token
  - Prime relevance: Central to "token efficiency" research area (init-promt.md)

- Context selection strategies:
  - Minimal context: Retrieve smallest sufficient representation for agent task
    - Usefulness: Maximizes token efficiency
    - Challenge: Determining "smallest sufficient" is undecidable in general
    - Heuristic: Retrieve core entity + immediate relationships
  - Progressive disclosure: Start with minimal context, expand as agent asks follow-up questions
    - Usefulness: Supports iterative agent exploration
    - Pattern: Query → retrieve minimal → agent asks follow-up → retrieve expanded
    - Prime relevance: Supports agent loop design (init-promt.md work iteratively principle)
  - Surrounding context: Retrieve symbol + surrounding entities (callers, callees, related symbols)
    - Usefulness: Balances token usage with agent usefulness
    - Typical size: Symbol + 3 callers + 3 callees + 5 related symbols (configurable)
    - Prime relevance: Default strategy for most agent queries
  - Importance-based: Retrieve symbols with highest centrality/importance in codebase graph
    - Usefulness: Agent gets most "valuable" knowledge first
    - Metric: Betweenness centrality, degree centrality, eigenvector centrality
    - Prime relevance: Can weight retrieval by graph importance measures
  - Recency-based: Retrieve recently accessed or modified symbols
    - Usefulness: Agent working on recent changes gets relevant knowledge
    - Mechanism: Timestamp on knowledge entries, LRU cache-like selection
    - Prime relevance: Incremental analysis workflow (incremental-analysis.md)

- Context selection algorithms:
  - Degree-based selection: Retrieve symbol + N most connected neighbors
    - N configurable (3, 5, 7, etc.)
    - Usefulness: Quick overview of symbol's most important relationships
    - Prime relevance: Simple, effective default
  - Hybrid importance/degree: Combine centrality measure with neighborhood size
    - Usefulness: Balance global importance with local connectivity
    - Prime relevance: More sophisticated, potentially better results
  - Query-type-specific: Different strategies for different query types
    - "Find symbol": Retrieve symbol metadata + location (minimal)
    - "Find references": Retrieve symbol + all direct references (expanded)
    - "Explore context": Retrieve surrounding relationships (surrounding context)
    - Prime relevance: Context selection adapts to query type

- Token budget management:
  - Allocate tokens per retrieval based on query type and agent state
  - Track cumulative token usage across agent loop iterations
  - Enforce budget hard limit (stop retrieval when budget exceeded)
  - Usefulness: Prevents agent context window overflow
  - Prime relevance: Critical infrastructure (agent has finite context window)

- Minimum information an agent needs (central research question from init-promt.md):
  - Symbol name and kind: What is this entity?
  - Location: Where is it defined? (file:line)
  - Type signature: What are parameters and return type?
  - Direct references: Where is it used immediately?
  - Related symbols: What other symbols is it connected to?
  - Confidence/provenance: How reliable is this knowledge?
  - Usefulness: Defines "minimum useful representation" from init-promt.md

- Context selection for cross-language support:
  - Language-qualified symbol identifiers
  - Confidence/provenance annotation travels with knowledge
  - Degradation gracefully: Level 3 knowledge (unavailable) marked as unknown rather than omitted
  - Usefulness: Agent can decide how to handle unsupported language features
  - Prime relevance: Core to language-agnosticism research

- Context selection evaluation metrics:
  - Precision@k: Of retrieved tokens, what fraction are useful for agent task
  - Recall@k: Of needed knowledge, what fraction was retrieved
  - Token efficiency: Usefulness per token (primary metric)
  - Agent task success rate: Does agent complete task with retrieved context?
  - Prime relevance: Measure research quality; determine optimal strategy

- Reusable context selection components:
  - Query type classifier (categorize agent query into known types)
  - Importance/centrality computation (graph metrics on Prime artifact)
  - Token budget calculator (estimate tokens for candidate retrieval set)
  - Selection strategy engine (choose strategy based on query type, agent state)
  - Progressive disclosure manager (control expand/contract of context)