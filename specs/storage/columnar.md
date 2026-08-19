Columnar Storage research:

- Columnar storage: Storage format that organizes data by columns rather than rows:
  - Advantages: Column pruning (only read needed columns), compression (same type values together)
  - Disadvantages: Row-wise writes more expensive, complex for point lookups

- Columnar formats:
  - Parquet: Apache Parquet, columnar with compression (ZSTD, Snappy, GZIP), predicate pushdown, metadata-driven
  - ORC: Apache ORC, columnar with kind-specific compression, statistics
  - Feather: Fast columnar interchange (Arrow), minimal metadata, low latency
  - Avro: Row-based with columnar read optimization (file splits by type)
  - Feather/Arrow: In-memory columnar format, zero-copy integration

- Column pruning: Only reading required columns from a columnar store:
  - Usefulness: High (reduces I/O significantly for agent queries that need few symbols)
  - Example: Agent needs symbol names → only read name column, not body/references columns

- Predicate pushdown: Filtering data at storage layer before transfer:
  - Usefulness: High (agent queries like "find all functions named X" filtered at storage level)
  - Example: Filter by symbol type = "function" at read time, not after full retrieval

- Compression in columnar formats:
  - Dictionary encoding: Repeated strings stored once, references by ID
  - Bit packing: Bits-per-value encoding for integers
  - ZSTD/Snappy/GZIP: General-purpose compression on column chunks
  - Usefulness: High (columnar + compression = significant I/O reduction)

- Dictionary encoding: Replacing repeated values with integer IDs:
  - Benefit: Significant reduction when many repeated values (symbol names, types)
  - Overhead: ID mapping lookup needed
  - Usefulness: High for agent knowledge (many symbols share type names, etc.)

- Columnar vs row-oriented tradeoffs:
  - Columnar: Excellent for read-heavy, analytical queries (agent knowledge retrieval)
  - Row-oriented: Excellent for write-heavy, point lookup workloads (incremental updates)
  - Hybrid: Some systems support both (ClickHouse, DuckDB with materialized views)
  - Prime must decide based on dominant workload: agent retrieval vs incremental updates

- Columnar compressibility analysis:
  - Symbol names: Highly compressible (dictionary encoding, many repeated prefixes)
  - Type annotations: Moderately compressible (few type names repeated across many symbols)
  - Reference lists: Low compressibility (unique per symbol, variable length)
  - Body/source code: Low compressibility (without general compression)
  - Usefulness: Inform format design (optimize for most frequently accessed columns)

- Columnar storage for agent knowledge:
  - Agent typically needs: symbol names, types, locations, references to symbol
  - Columnar layout: separate columns for each attribute
  - Benefits: Agent retrieves only needed columns, significant I/O reduction
  - Tradeoff: Incremental update complexity (modifying one column doesn't rewrite entire row)

- Implementation considerations:
  - Column compression (ZSTD on each column chunk)
  - Metadata per column (statistics, min/max, null counts)
  - Column chunk sizing (typically 1MB-128MB per chunk)
  - Column order optimization (most frequently filtered/selected columns first)
  - Parquet/ORC integration for long-term storage

- Columnar vs custom binary for Prime:
  - Columnar: Best if agent workload dominates (retrieve specific knowledge without full artifact)
  - Custom binary: Best if incremental updates dominate or mixed workload
  - Hybrid approach: Columnar for read-optimized partitions, custom binary for write-optimized partitions