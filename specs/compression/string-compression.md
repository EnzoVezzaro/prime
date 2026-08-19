String Compression research:

- Dictionary encoding: Replacing repeated strings with integer IDs:
  - Benefit: Significant reduction when many identical or similar strings (symbol names, FQ names)
  - Overhead: ID mapping lookup, dictionary storage
  - Usefulness: High for codebase knowledge (many symbols share type names, module names, etc.)
  - Example: 1000 occurrences of "UserService" stored once + 1000 x 2-byte IDs vs 1000 x 12-byte strings

- String interning: Maintaining a pool of unique string instances:
  - Similar to dictionary encoding but at runtime/interpreter level
  - Usefulness: High (prevents duplicate string storage, enables pointer comparison)
  - Tradeoff: Requires intern pool management, memory for dictionary

- Front coding: Storing only the differing portion of strings, assuming common prefixes:
  - Usefulness: High for code (many symbols share prefixes: UserService, UserRepository, UserController)
  - Compression ratio: 3-5x for code-like string distributions
  - CPU cost: Low (prefix matching + store delta)
  - Random access: Requires reconstructing full string from front code + prefix
  - Prime relevance: Symbol names often share prefixes, good candidate

- Tries (prefix trees): Tree structure for storing string set:
  - Store strings as paths from root
  - Usefulness: Efficient prefix sharing, prefix-based search
  - Compression ratio: Good (shared prefixes stored once)
  - CPU cost: Low-Medium (tree traversal)
  - Random access: By prefix path
  - Prime relevance: Symbol name storage with prefix sharing

- FSTs (Finite State Transducers): DAG-based string representation:
  - More compact than tries for certain string distributions
  - Usefulness: Compact string representation, supports edit operations
  - Compression ratio: Better than tries for some distributions
  - CPU cost: Medium (FST traversal)
  - Random access: By string path
  - Prime relevance: Advanced string compression if FST suitable for code identifiers

- Suffix structures: Suffix trees, suffix arrays for string searching:
  - Usefulness: Pattern matching, not primarily compression
  - Can provide compression as side effect (suffix array + LCP)
  - Usefulness: Limited for pure compression, strong for search

- Front coding + dictionary encoding hybrid:
  - Front code the string (store common prefix once)
  - Dictionary encode the delta/remainder
  - Usefulness: Best of both worlds for code-like strings
  - Prime relevance: Strong candidate for symbol name compression

- General string compression:
  - zstd on strings: General-purpose compression applied to string data
    - Ratio: 3-10x+ depending on content
    - CPU: Medium
    - Random access: Requires full decompression (or chunked compression)
  - lz4 on strings: Fast general-purpose
    - Ratio: 2-4x
    - CPU: Very low
    - Random access: Chunked possible
  - brotli on strings: Optimized for text, good ratio
    - Ratio: 4-10x for text
    - CPU: Medium-High
    - Random access: Chunked possible

- Prime string compression recommendations:
  1. Dictionary encoding + front coding hybrid for symbol names (most common strings, shared prefixes)
  2. zstd chunked compression for larger text bodies (source code, comments)
  3. Intern common prefixes (module names, type names) at knowledge generation time
  4. Avoid general-purpose compression on frequently accessed fields (random access penalty)
  5. Chunked compression if random access needed on compressed data