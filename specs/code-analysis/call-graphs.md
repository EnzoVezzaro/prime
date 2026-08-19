Call graphs research:

- Call graph: Directed graph representing function invocation relationships across a codebase. Nodes are functions/methods; edges represent calls.

- Call graph construction:
  - Direct calls: Textually obvious function invocations (e.g., `foo()`)
  - Indirect calls: Through variables, higher-order functions, dynamic dispatch
  - Usefulness: Direct calls are exact; indirect calls are partial/inferred

- Call graph types:
  - Caller-callee graph: Who calls whom
  - Call chain graph: Sequences of invocations (n-depth)
  - Call cycle graph: Cycles in calling patterns (recursion, mutual recursion)
  - Usefulness: Different types serve different agent queries

- Construction algorithm phases:
  - Parse: Extract function definitions and invocations
  - Build: Create nodes and edges based on textual analysis
  - Resolve: Attempt to determine target of indirect calls
  - Prune: Remove unreachable or unnecessary edges
  - Usefulness: Structured process for scalable construction

- Sparsity: Call graphs are typically sparse (each function calls few others):
  - Usefulness: Efficient storage, fast traversal
  - Prime relevance: Sparse representation advantageous

- Concurrency calls: Function calls within asynchronous/context-switching contexts:
  - async/await patterns, thread spawning, future/promise patterns
  - Usefulness: High (modern codebases extensive use)

- Method call graphs (OOP):
  - Include virtual method dispatch, interface calls
  - May include dynamic dispatch targets (partial)
  - Usefulness: High (OOP codebase understanding)

- Recursive calls: A function calling itself:
  - Direct recursion: Textually obvious
  - Indirect recursion: Through intermediate functions
  - Usefulness: Understanding recursion depth, stack usage

- Tail call optimization: Calls in tail position that can be optimized:
  - Usefulness: Understanding stack usage, recursion elimination

- Call graph granularity:
  - Function-level: One node per function
  - Basic block level: Multiple nodes per function
  - Usefulness: Function-level is standard; basic block provides more precision

- Inter-procedural analysis: Analysis spanning function boundaries:
  - Required for accurate call graph
  - Usefulness: Essential for whole-codebase understanding

- Call graph mutability: How call graph changes with code modifications:
  - Adding a call: Add edge
  - Removing a call: Remove edge
  - Changing call target: Modify edge
  - Usefulness: Incremental update planning

- Call graph uncertainty: Edges with confidence levels:
  - High confidence: Direct call, unambiguous target
  - Medium confidence: Indirect call with limited resolution
  - Low confidence: Dynamic dispatch, unresolved target
  - Usefulness: Agent can filter based on uncertainty

- Call graph slicing: Extracting relevant subgraph for a query:
  - Call slice: All functions that can reach a target
  - Usefulness: Focused agent queries (e.g., "what calls this function?")

- Inter-procedural constant propagation: Determining constant values across call boundaries:
  - Affects call graph edges (conditional calls)
  - Usefulness: Pruning unreachable edges

- Call graph vs dependency graph: Distinction:
  - Call graph: Function invocation relationships
  - Dependency graph: Data and control dependencies (broader)
  - Usefulness: Different agent queries need different graphs

- Reusable components: Call graph infrastructure:
  - Construction pipeline (parse → resolve → build → prune)
  - Incremental update mechanism
  - Query interface (who calls, who called by, call chains)
  - Uncertainty/confidence annotation