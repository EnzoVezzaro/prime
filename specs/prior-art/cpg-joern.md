Code Property Graph (CPG) / Joern research:

Investigated topics:

- Code Purpose Graph (CPG) purpose: CPG is a graph representation of code designed for cross-language code analysis. It combines AST, CFG, PG, and data-flow information into a single graph structure.

- Data model: Joern's CPG uses a multi-layered graph structure with the following layers:
  - AST layer: Abstract Syntax Tree structure representing the syntactic structure of code
  - CFG layer: Control Flow Graph representing possible execution paths
  - PG layer: Points-to graph representing memory aliasing and pointer relationships
  - DFG layer: Data Flow Graph representing data dependencies between instructions

- Node model: CPG nodes represent program entities at different levels of abstraction:
  - AST nodes: FunctionDeclaration, ClassBody, IfStatement, ForLoop, VariableDeclaration, etc.
  - CFG nodes: BasicBlock, Jump, ConditionalBranch, Return statement
  - DFG nodes: Definition, Use, Parameter, Return value
  - Taint nodes: Source, Sink, Propagation point

- Edge model: CPG edges represent relationships between nodes:
  - Control flow edges: NextBasicBlock, BranchTo, FallThrough
  - Data flow edges: Uses, Defines, ParameterOf, ReturnsTo
  - Control dependency edges: Controls, IsControlledBy
  - Calls edges: Call, IsCalledBy
  - PKG/Points-to edges: Reads, Writes, MayAlias, DoesNotAlias

- Storage model: CPGs are stored in Joern's custom graph database, which is an in-memory graph database optimized for:
  - Random access to nodes and edges
  - Traversal of complex graph patterns
  - Incremental updates (adding/removing code elements)
  - Query execution via CPGQL (a Scala-based query language)

- Indexing: Joern indexes CPG nodes by type and relationship, enabling efficient graph traversals. Indexes are maintained for:
  - Node type lookups (all Functions, all Classes)
  - Edge type lookups (all calls, all data dependencies)
  - Property-based lookups (all nodes with name = "foo")

- Query model: Joern uses CPGQL, a Scala-based domain-specific query language for code analysis. CPGQL supports:
  - Core steps: filter, map, flatMap, groupBy
  - Custom steps: user-defined graph traversals
  - Augmentation directives: extend the graph with additional information
  - Graph patterns: match complex structures (e.g., "find all taint flows")

- Language support: Joern supports multiple languages through frontends:
  - C/C++: Eclipse CDT parser
  - Java: JavaParser
  - JavaScript: GraalVM
  - Python: JavaCC
  - x86/x64: Ghidra (binary analysis)
  - JVM Bytecode: Soot
  - Kotlin: IntelliJ PSI
  - PHP: PHP-Parser
  - Go: go.parser
  - Ruby: ANTLR
  - Swift: SwiftSyntax
  - C#: Roslyn

- Scalability: Joern can handle large codebases (hundreds of thousands of files) due to:
  - In-memory graph database with efficient indexing
  - Parallel parsing and indexing
  - Incremental updates (only re-index changed files)
  - Distributed analysis capabilities (Joern server)

- Performance: Joern's performance depends on:
  - Parsing speed (varies by language frontend)
  - Graph construction overhead
  - Query execution time (depends on graph complexity)
  - Memory usage (CPG is fully materialized in memory)

- Incremental updates: Joern supports incremental indexing through:
  - Content hashing (track file changes via hash)
  - Partial re-indexing (only re-process changed files)
  - Graph delta computation (compute differences between versions)

- Strengths:
  - Rich semantic information (combines multiple analysis layers)
  - Expressive query language (CPGQL)
  - Good language support via frontends
  - Incremental update support
  - Mature and active project

- Weaknesses:
  - CPG is fully materialized in memory (not storage-efficient)
  - Requires frontend for each language
  - Graph construction is resource-intensive
  - Query language (CPGQL) has learning curve
  - Not designed for distributed or networked access

- Reusable components:
  - CPG construction pipeline (parse → normalize → enrich → store)
  - Graph query infrastructure (traversal, pattern matching)
  - Incremental update mechanism (hash-based invalidation)
  - Frontend architecture (parser → AST → CPG transformation)

- Relevant source code:
  - Joern frontend implementations (cmd/scip, cmd/java, cmd/javascript, etc.)
  - Graph database implementation (graph package)
  - CPGQL interpreter and compiler
  - Incremental update logic (workspace package)