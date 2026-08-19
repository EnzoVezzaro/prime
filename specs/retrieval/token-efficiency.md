Token Efficiency research:

- Token efficiency: Maximizing usefulness per agent context token:
  - Central research question from init-promt.md: "What is the minimum information an agent needs to understand a code entity?"
  - Agent context window: Finite (varies by model, typically 4K-100K+ tokens)
  - Usefulness: Every token consumed reduces capacity for other knowledge

- Token cost model:
  - Each retrieved fact/consumes N tokens (depends on representation, formatting)
  - Agent reasoning quality degrades as context window fills (diminishing returns, attention dilution)
  - Usefulness: Model attend to all tokens; too many → reduced attention per token
  - Prime relevance: Design Prime representation for maximal information density

- Information density:
  - Ratio of useful information tokens to total retrieved tokens
  - High density: Every token contributes to agent understanding
  - Low density: Many tokens are redundant, filler, or noise
  - Usefulness: Design Prime for high information density (structured, relationship-dense vs. redundant text)
  - Comparison example from init-promt.md:
    - 100 KB of highly redundant text vs 12 KB of structured, relationship-dense knowledge
    - Second could be substantially more useful even with "same information"

- Minimum useful representation:
  - Concept from init-promt.md: "Prime attempts to eliminate source code from the agent's information path by deriving the smallest possible knowledge representation that can answer the agent's questions directly"
  - Goal: Smallest representation that still enables agent to answer questions
  - Tradeoff: Smaller = fewer tokens, but may lose information needed for agent tasks
  - Prime relevance: Core research objective; balance compression vs. agent capability

- Retrieval granularity vs token efficiency:
  - Symbol-granular: One symbol per retrieval (typically 50-200 tokens)
    - Pros: Precise, agent can request exactly what needed
    - Cons: More round trips if multiple symbols needed
  - Chunk-granular: Fixed-size chunk (symbol + neighbors + metadata, typically 200-500 tokens)
    - Pros: One retrieval often suffices for agent task
    - Cons: May include unnecessary information (token waste if agent only needs one entity)
  - File-granular: Entire source file retrieved (1K-50K+ tokens)
    - Pros: Complete context for file-level analysis
    - Cons: Typically too many tokens; agent cannot consume entire file
    - Usefulness: Only for specific use cases (full-file review)

- Compression for token efficiency:
  - Integer compression (varints, SIMD-BP128): Reduce token count for numeric metadata (symbol IDs, counts)
  - String compression (dictionary encoding, front coding): Reduce token count for symbol names, type names
  - General compression (zstd, lz4): Apply to stored knowledge artifact (tradeoff: decompression cost vs. token savings)
  - Hybrid compression: Different compression for different knowledge types (e.g., dictionary-encoded names, zstd-compressed bodies)
  - Prime relevance: Compression directly contributes to token efficiency (fewer tokens transferred)

- Structured vs text representation:
  - Structured (JSON, protobuf, custom binary): Compact field representation, explicit types, selective retrieval
    - Token efficiency: High (only needed fields retrieved; no redundant prose)
    - Agent consumption: Requires model to parse structured format (may need few-shot prompting)
  - Text (natural language prose, descriptions): Human-readable, but contains redundant connective language
    - Token efficiency: Lower (connective language, repetition, prose filler)
    - Agent consumption: Natural to model, but less efficient per information unit
  - Prime relevance: Structured likely more token-efficient; research needed to confirm

- Attention complexity:
  - Model attention scales with context window size (approximately O(n²) for full attention, though transformers use optimizations)
  - More tokens = higher computational cost for agent reasoning
  - Usefulness: Minimize tokens to reduce agent reasoning cost
  - Prime relevance: Token efficiency directly correlates with agent reasoning efficiency

- Keyword/keyphrase extraction:
  - Extract minimal set of keywords/phrases that convey entity meaning
  - Usefulness: Reduce tokens while preserving essential information
  - Example: Instead of "UserService class that handles user authentication and authorization", extract "UserService auth handles authZ"
  - Prime relevance: Represent knowledge using key phrases rather than full sentences where possible

- Progressive token disclosure:
  - Retrieve minimal initial context (e.g., 100 tokens)
  - Agent evaluates whether more needed; if so, retrieve additional context (e.g., next 200 tokens)
  - Usefulness: Agent only consumes tokens needed; avoids pre-loading entire knowledge
  - Prime relevance: Supports agent loop (work iteratively, init-promt.md)

- Token efficiency metrics:
  - Tokens per entity understood: Lower is better (fewer tokens for agent to understand one symbol)
  - Retrieval tokens per task: Total tokens retrieved during agent task completion
  - Information density: Useful information tokens / total retrieved tokens
  - Agent task success vs token budget: Does agent succeed within given context window?
  - Prime relevance: Measure research quality; compare different representation/formats

- Reusable token efficiency components:
  - Token counter/estimator (estimate token count for any knowledge representation)
  - Information density calculator (ratio of meaningful to total tokens)
  - Granularity selector (choose retrieval granularity based on token budget)
  - Compression selector (choose compression algorithm based on token savings vs. decompression cost)
  - Structured vs text formatter (convert knowledge to most token-efficient representation)