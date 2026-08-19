Agent-oriented code indexing research:

Investigated systems:

- Graph-sitter: https://github.com/ast-grep/ast-grep
  - What information they index: AST-based graph representations of code structure
  - How they retrieve it: Pattern-based querying using AST patterns
  - Whether they use ASTs: Yes, core to their approach
  - Whether they use graphs: Yes, converts AST to graph for querying
  - Whether they use embeddings: Not primarily (focuses on structural patterns)
  - How context is constructed: Through AST pattern matching and graph queries
  - Bottlenecks: Pattern matching complexity on large codebases
  - What information agents actually consume: Structured code patterns, symbol locations, relationship types

- codebase-index: Various codebase indexing systems
  - What information they index: Varies by implementation, typically symbols, references, dependencies
  - How they retrieve it: Index lookups, often via hash maps or databases
  - Whether they use ASTs: Often as initial parsing step
  - Whether they use graphs: Some use graph structures for relationships
  - Whether they use embeddings: Emerging trend for semantic search
  - How context is constructed: Retrieval-based (fetch most relevant symbols)
  - Bottlenecks: Index size, update complexity
  - What information agents actually consume: Depends on use case (navigation, completion, search)

- Sourcegraph systems:
  - What information they index: Full code intelligence (symbols, references, definitions, implementations)
  - How they retrieve it: Hybrid approach (database lookups + graph traversals)
  - Whether they use ASTs: Yes, via language server integrations
  - Whether they use graphs: Yes, code navigation graphs
  - Whether they use embeddings: Yes, for semantic search
  - How context is constructed: Agent-directed retrieval (ask for specific information)
  - Bottlenecks: Index maintenance, language server coordination
  - What information agents actually consume: Depends on agent task (navigation, completion, analysis)

- Cursor-style repository indexing:
  - What information they index: Code structure, symbol definitions, recent changes
  - How they retrieve it: Context-aware retrieval based on current cursor position
  - Whether they use ASTs: Yes, for understanding code at cursor position
  - Whether they use graphs: Limited (focus on local context)
  - Whether they use embeddings: Increasingly yes
  - How context is constructed: Progressive disclosure (start local, expand outward)
  - Bottlenecks: Maintaining relevance as context expands
  - What information agents actually consume: Minimal relevant context for current task

- Continue:
  - What information they index: Codebase structure, symbols, relationships
  - How they retrieve it: Retrieval-based with agent interaction
  - Whether they use ASTs: Yes
  - Whether they use graphs: Yes, for relationship representation
  - Whether they use embeddings: Under investigation
  - How context is constructed: Interactive agent loop
  - Bottlenecks: Context window limits, relevance ranking
  - What information agents actually consume: Agent-selected context chunks

- Aider:
  - What information they index: Code relevant to pair programming task
  - How they retrieve it: Agent-directed, task-focused retrieval
  - Whether they use ASTs: Yes, for understanding code structure
  - Whether they use graphs: Partial (key relationships)
  - Whether they use embeddings: Yes, for semantic similarity
  - How context is constructed: Minimal context for current editing task
  - Bottlenecks: Token limits, relevance under token constraints
  - What information agents actually consume: Minimal relevant code slices

- OpenHands:
  - What information they index: Broad codebase knowledge for agent tasks
  - How they retrieve it: Hybrid (structural + semantic search)
  - Whether they use ASTs: Yes, for initial code understanding
  - Whether they use graphs: Yes, for relationship mapping
  - Whether they use embeddings: Yes, for semantic search
  - How context is constructed: Agent-directed with tool coordination
  - Bottlenecks: Coordinating multiple tools within context window
  - What information agents actually consume: Task-relevant code context

- SWE-agent:
  - What information they index: Code needed for software engineering tasks (fixes, features)
  - How they retrieve it: Task-specific retrieval based on problem description
  - Whether they use ASTs: Yes, for codebase understanding
  - Whether they use graphs: Yes, for navigating code relationships
  - Whether they use embeddings: Yes, for semantic matching
  - How context is constructed: Problem→solution mapping
  - Bottlenecks: Mapping natural language tasks to code entities
  - What information agents actually consume: Task-relevant code slices

Key findings across all systems:

- What information they index:
  - Nearly all index some form of: symbols, definitions, references, relationships
  - Most also index: file structure, package/module organization, type information
  - Increasing number include: embeddings for semantic search, embeddings for similarity

- How they retrieve it:
  - Two dominant patterns:
    1. Retrieval-based: Fetch pre-computed index entries (symbols, references)
    2. Query-based: Run graph queries or pattern matches against stored structure
  - Most systems combine both approaches

- Whether they use ASTs:
  - Almost all use ASTs as the initial parsing step
  - ASTs provide the concrete syntax foundation
  - Beyond that, divergence: some stop at AST, others build graphs on top

- Whether they use graphs:
  - Growing number use graph structures for relationship representation
  - Graphs excel at representing: calls, references, data flow, control flow
  - AST-focused systems may not exploit graph query capabilities

- Whether they use embeddings:
  - Emerging trend across all systems
  - Used for: semantic search, similarity matching, clustering
  - Typically combined with structural information (hybrid search)

- How context is constructed:
  - Pattern: Progressive disclosure (start minimal, expand as needed)
  - Most systems use some form of context selection/ filtering
  - Agent-directed: Ask agent what they need, then retrieve
  - Task-focused: Retrieve based on current agent task

- What information agents actually consume:
  - Core needs: Symbol definitions, reference locations, relationship types
  - Increasing need: Semantic similarity, pattern matches
  - Token efficiency: Critical constraint (minimize tokens while maximizing usefulness)
  - Structural context: Understanding code organization without reading full files

Bottlenecks identified across systems:
- Index size and maintenance cost
- Update complexity for incremental changes
- Context window limits for agent reasoning
- Token efficiency: maximizing usefulness per token
- Balancing specificity vs generality of retrieved information