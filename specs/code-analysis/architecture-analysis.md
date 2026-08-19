Architecture analysis research:

- Codebase architecture: High-level organization and structural patterns of a software system.

- Architecture pattern detection: Identifying recurring structural organizations:
  - Layered architecture (presentation, business, data layers)
  - Microservices (small independently deployable services)
  - Monolith (single deployable unit)
  - Hexagonal/Clean (ports and adapters)
  - Event-driven (message-based communication)
  - Usefulness: High (summarizing codebase structure for agents)

- Layer detection: Identifying architectural layers:
  - Dependency structure matrix analysis
  - Package organization analysis
  - Usefulness: High (visualizing layer boundaries, identifying violations)

- Component identification: Grouping related functionality into components:
  - Cohesion analysis (how closely related elements are within a component)
  - Coupling analysis (how strongly components depend on each other)
  - Usefulness: High (modularization understanding)

- Package/module organization: How codebase arranges groupings:
  - By feature/functionality
  - By layer/concern
  - By domain/bounded context
  - Usefulness: High (understanding organizational decisions)

- Dependency structure matrix (DSM): Matrix-format visualization of dependencies:
  - Rows/columns represent components/packages
  - Cell indicates dependency relationship
  - Usefulness: Identifying circular dependencies, layer violations, reorganization opportunities

- Bounded context: Domain-driven design concept:
  - Clear boundaries within which a model applies
  - Usefulness: High (domain-driven design codebases)

- Service identification (microervices): Identifying potential service boundaries:
  - Bounded context mapping
  - Independence analysis (can one service change without affecting others?)
  - Usefulness: High (microervices migration assessment)

- Architecture style classification: Classifying codebase into known styles:
  - Pattern matching against known architectural styles
  - Usefulness: High (agent expects certain conventions based on style)

- Technical debt indicators: Architecture signals of debt:
  - High coupling, low cohesion
  - Deep inheritance hierarchies
  - God objects, massive classes
  - Circular dependencies
  - Usefulness: High (prioritizing refactoring)

- Architecture evolution patterns: How architectures change over time:
  - Monolith → microservices evolution
  - Layer addition/removal
  - Boundary shifting
  - Usefulness: Understanding codebase trajectory

- Architecture recovery: Recovering architectural knowledge from code:
  - Reverse engineering architectural views
  - Usefulness: High (legacy codebases, new team onboarding)

- Agent architecture queries supported:
  - "What layers exist in this codebase?"
  - "What components belong to which layer?"
  - "Are there dependency violations?"
  - "What is the bounded context mapping?"
  - "What is the suggested service decomposition?"

- Architecture representation formats:
  - Textual descriptions
  - Graph representations (components as nodes, dependencies as edges)
  - Matrix formats (DSM)
  - Usefulness: Different formats suit different agent queries

- Reusable components: Architecture analysis infrastructure:
  - DSM computation and visualization
  - Pattern matching against known architectures
  - Coupling/cohesion metrics
  - Boundary detection algorithms
  - Evolution pattern detection