Agent-oriented code indexing systems research:

Investigated systems and their research findings:

- Graph-sitter (ast-grep):
  - Repositories: https://github.com/ast-grep/ast-grep
  - What information they index: AST-derived graph patterns, symbol positions, relationship types
  - How they retrieve it: Pattern matching queries against AST/graph
  - Whether they use ASTs: Yes, core to their approach (ast-grep = AST grep)
  - Whether they use graphs: Yes, convert AST to graph for pattern queries
  - Whether they use embeddings: Not primarily (focuses on structural patterns)
  - How context is constructed: Through pattern matching results
  - Bottlenecks: Pattern matching complexity on large codebases, query performance
  - What information agents actually consume: Structured code patterns, symbol locations, relationship types
  - Key insight: Pattern-based retrieval is powerful for finding specific code patterns

- codebase-index systems:
  - Varied implementations (multiple open-source projects)
  - What information they index: Symbol tables, reference maps, dependency graphs
  - How they retrieve it: Hash-based lookups, index queries
  - Whether they use ASTs: Often as initial parsing step, then discard or transform
  - Whether they use graphs: Some use adjacency lists for relationships
  - Whether they use embeddings: Emerging, not universal
  - How context is constructed: Retrieval-based (fetch most relevant symbols)
  - Bottlenecks: Index size, update complexity for changes
  - What information agents actually consume: Depends on use case (navigation, completion, search)

- Sourcegraph systems:
  - Repository: https://github.com/sourcegraph/sourcegraph
  - What information they index: Full code intelligence (symbols, references, definitions, implementations, relationships)
  - How they retrieve it: Hybrid approach (database lookups + graph traversals + embeddings)
  - Whether they use ASTs: Yes, via language server integrations (LSP)
  - Whether they use graphs: Yes, code navigation graphs for relationship representation
  - Whether they use embeddings: Yes, for semantic search and similarity
  - How context is constructed: Agent-directed retrieval (ask for specific information, progressive disclosure)
  - Bottlenecks: Index maintenance across languages, language server coordination, keeping indexes updated
  - What information agents actually consume: Depends on agent task (navigation, completion, analysis, refactoring)
  - Key insight: Hybrid structural + semantic approach is most effective

- Cursor-style repository indexing:
  - What information they index: Code structure around cursor position, recent symbols, quick references
  - How they retrieve it: Context-aware retrieval based on current cursor position and task
  - Whether they use ASTs: Yes, for understanding code at cursor position
  - Whether they use graphs: Limited (focus on local context, not full codebase graph)
  - Whether they use embeddings: Increasingly yes (for semantic similarity)
  - How context is constructed: Progressive disclosure (start local, expand outward as needed)
  - Bottlenecks: Maintaining relevance as context expands, token limit management
  - What information agents actually consume: Minimal relevant context for current task

- Continue:
  - What information they index: Codebase structure, symbols, relationships, task-relevant context
  - How they retrieve it: Agent-directed with tool coordination
  - Whether they use ASTs: Yes, for initial code understanding
  - Whether they use graphs: Yes, for relationship mapping (varies by system)
  - Whether they use embeddings: Yes, for semantic search (increasingly common)
  - How context is constructed: Interactive agent loop with tool coordination
  - Bottlenecks: Context window limits, relevance ranking, coordinating multiple tools
  - What information agents actually consume: Task-relevant code context chunks

- Aider:
  - What information they index: Code relevant to pair programming task
  - How they retrieve it: Agent-directed, task-focused retrieval
  - Whether they use ASTs: Yes, for understanding code structure at point of use
  - Whether they use graphs: Partial (key relationships like calls, references for current task)
  - Whether they use embeddings: Yes, for semantic similarity and finding relevant code
  - How context is constructed: Minimal context for current editing task (typically < 10KB)
  - Bottlenecks: Token limits, relevance under severe token constraints, deciding what to surface
  - What information agents actually consume: Minimal relevant code slices (< 5KB typically)

- OpenHands:
  - What information they index: Broad codebase knowledge for agent tasks (configuration, symbols, references, relationships)
  - How they retrieve it: Hybrid (structural search + semantic search + tool results)
  - Whether they use ASTs: Yes, for initial code understanding and symbol extraction
  - Whether they use graphs: Yes, for relationship mapping (calls, references, dependencies)
  - Whether they use embeddings: Yes, for semantic search and similarity matching
  - How context is constructed: Agent-directed with tool coordination (multiple tools called sequentially)
  - Bottlenecks: Coordinating multiple tools within context window, relevance ranking across sources
  - What information agents actually consume: Task-relevant code context (varies by task, typically 10-50KB)

- SWE-agent:
  - What information they index: Code needed for software engineering tasks (fixes, features, understanding)
  - How they retrieve it: Task-specific retrieval based on problem description and codebase structure
  - Whether they use ASTs: Yes, for codebase understanding (symbol extraction, relationship mapping)
  - Whether they use graphs: Yes, for navigating code relationships (calls graph, dependency graph)
  - Whether they use embeddings: Yes, for semantic matching between problem description and code entities
  - How context is constructed: Problem→solution mapping (retrieve code that addresses the stated problem)
  - Bottlenecks: Mapping natural language tasks to code entities, finding minimal sufficient context
  - What information agents actually consume: Task-relevant code slices (typically 20-100KB depending on task)

Summary of research findings across all systems:

1. Information indexing trends:
   - Core: symbols, definitions, references, file structure
   - Growing: embeddings for semantic similarity
   - Increasing: graph structures for relationship representation

2. Retrieval patterns:
   - Two dominant: retrieval-based (pre-computed indexes) and query-based (graph patterns)
   - Most effective: hybrid approach (structural + semantic)
   - Agent-directed: retrieve based on agent's stated needs

3. AST usage:
   - Universal: all systems use ASTs as initial parsing step
   - Divergence: some stop at AST, others build richer structures

4. Graph usage:
   - Growing adoption for relationship representation
   - Excellent for: calls, references, data flow, control flow
   - Not all systems exploit graph query capabilities

5. Embedding usage:
   - Emerging trend across all systems
   - Used for: semantic search, similarity matching, clustering
   - Typically combined with structural information

6. Context construction:
   - Progressive disclosure pattern (start minimal, expand as needed)
   - Agent-directed: ask agent what they need, then retrieve
   - Task-focused: retrieve based on current agent task
   - Token efficiency: critical constraint across all systems

7. Bottlenecks common to all systems:
   - Index size and maintenance cost
   - Update complexity for incremental changes
   - Context window limits for agent reasoning
   - Token efficiency: maximizing usefulness per token
   - Balancing specificity vs generality of retrieved information
   - Coordinating multiple tools/sources within context constraints