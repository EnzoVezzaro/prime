Control flow research:

- Control flow: Order in which individual statements, instructions, or function calls are executed or evaluated.

- Control flow graph (CFG): Directed graph representing control flow through a program:
  - Nodes: Basic blocks (sequential statements with single entry, single exit)
  - Edges: Possible flow of control between blocks
  - Usefulness: High (foundational program analysis structure)

- Basic block: Straight-line code sequence with no branches in or out:
  - Single entry point, single exit point
  - Usefulness: Granularity for CFG construction

- CFG construction algorithm:
  - Parse AST into basic blocks
  - Identify branch instructions (if, switch, goto, etc.)
  - Add edges based on branch targets
  - Add entry/exit nodes
  - Usefulness: Standard compilation/linker technique

- Reducible vs irreducible CFGs:
  - Reducible: Can be reduced using structured programming theorems (loops, branches)
  - Irreducible: Goto-heavy code, structured analysis difficult
  - Usefulness: Reducible more tractable for analysis

- Loop identification: Detecting loop structures in CFG:
  - Natural loops: Back edges from exit to entry
  - Usefulness: Enables loop-invariant code analysis, understanding iteration

- Cyclomatic complexity: Measure of CFG complexity (number of independent paths):
  - Formula: E − N + 2 (edges − nodes + 2)
  - Usefulness: High (predicts testing effort, complexity)

- Entry/exit nodes: Special CFG nodes:
  - Entry: Starting point of analysis
  - Exit: Terminating points (return, throw, fall off end)
  - Usefulness: CFG traversal bookkeeping

- Exception flow: Control flow through exception handling:
  - Try-catch-finally structures
  - Exception edges in CFG
  - Usefulness: High (modern codebases extensive use)

- Coroutine/concurrency flow: Non-linear control flow:
  - async/await, promises, futures
  - Thread spawn, join, message passing
  - Usefulness: High (modern asynchronous code)

- CFG pruning: Removing unnecessary edges/nodes:
  - Unreachable code removal
  - Dead code elimination
  - Usefulness: Simplifies analysis, reduces noise for agents

- Context-sensitive CFG: CFG augmented with calling context:
  - Distinguishes same function in different call contexts
  - Usefulness: Higher precision for agent queries

- CFG summarization: Abstracting fine-grained CFG for higher-level views:
  - Summarize loops as single nodes
  - Usefulness: Scalable analysis for large codebases

- CFG query patterns for agents:
  - "What paths can execution take from this point?"
  - "What code executes after this condition?"
  - "What is the loop structure here?"
  - Usefulness: Core agent reasoning about program behavior

- CFG vs call graph distinction:
  - CFG: Intra-procedural (within one function)
  - Call graph: Inter-procedural (across functions)
  - Usefulness: Both needed; CFG for intra-function behavior, call graph for inter-function

- Reusable components: Control flow infrastructure:
  - CFG construction from AST
  - Basic block identification
  - Loop detection algorithms
  - Complexity computation (cyclomatic)
  - Exception flow modeling
  - Summarization/abstraction techniques