Data flow research:

- Data flow: Movement of data values through a program. Tracks how values produced at one point are consumed at another.

- Data flow analysis: Static analysis to determine possible set of values at each program point:
  - Forward analysis (data flows forward through control flow)
  - Backward analysis (data flows backward from uses to definitions)
  - Usefulness: High (understanding data dependencies, taint analysis)

- Def-use chains: Connection between a variable definition and its subsequent uses:
  - Direct def-use: Definition directly reaches use without intervening redefinition
  - Indirect def-use: Definition reaches use through complex paths
  - Usefulness: High (enabling precise data flow queries)

- Kill points: Program points where a variable's previous value is overwritten:
  - Usefulness: Identifying where old values are lost, tracking value provenance

- Data flow paths: Possible routes a value can take through program:
  - Single path vs multiple possible paths
  - Usefulness: Path sensitivity vs path insensitivity tradeoff

- Taint analysis: Specialized data flow tracking of attacker-controlled data:
  - Source: Entry point of untrusted data (user input, file read, network)
  - Sink: Exit point where data has security impact (SQL query, command execution, etc.)
  - Propagation: How data moves through program
  - Usefulness: High (security analysis, vulnerability discovery)

- Constant propagation: Determining when a variable has constant value:
  - Usefulness: Enabling optimizations, understanding possible values

- Inter-procedural data flow: Data flow spanning function boundaries:
  - Requires summary/callee modeling
  - Usefulness: Whole-program data flow analysis

- Flow-sensitive vs flow-insensitive analysis:
  - Flow-sensitive: Tracks data flow considering control flow order
  - Flow-insensitive: Ignores control flow, determines possible values overall
  - Usefulness: Flow-sensitive more precise but more expensive

- Flow-dependent vs flow-independent data flow:
  - Flow-dependent: Result depends on program execution path
  - Flow-independent: Result same regardless of path
  - Usefulness: Distinguishing aids analysis design choice

- Alias analysis: Determining when two references can point to same memory location:
  - Required for accurate data flow analysis
  - Usefulness: High (affects data flow precision)

- Points-to analysis: Determining possible target locations of pointers/references:
  - Flow-sensitive vs flow-insensitive
  - May-alias vs must-alias
  - Usefulness: High (enables precise data flow)

- Data flow precision tradeoffs:
  - More precise = more expensive, potentially less scalable
  - Less precise = faster but may have false positives/negatives
  - Usefulness: Prime must balance precision vs scalability

- Data flow for agent context: What agents need to know:
  - Which variables flow into which functions
  - Taint propagation for security-sensitive code
  - Constant values for understanding possible behaviors
  - Usefulness: Core agent knowledge requirements

- Data flow indexing structures:
  - Def-use chain indexes
  - Points-to indexes
  - Taint propagation indexes
  - Usefulness: Enables fast data flow queries for agents