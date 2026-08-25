# First Experiment Plan

## Hypothesis
AI coding agents perform better when given AIRL-like semantic representation instead of source-only context.

## Experimental setup
Use existing Prime benchmark repository set.
Keep model, task, repository, evaluation procedure constant.

## Conditions
A. Baseline: Existing Prime representation (source-oriented)
B. AST/structural representation
C. AIRL-inspired semantic representation
D. Semantic representation + targeted queries
E. Semantic representation + semantic patches

## Metrics

### Agent performance
- knowledge accuracy
- source-free accuracy
- task success
- localization accuracy
- edit correctness
- regression rate
- test success

### Efficiency
- input tokens
- output tokens
- total tokens
- context size
- number of retrieval operations
- number of files inspected
- tool calls
- latency

### Representation efficiency
- repository size
- AST size
- semantic graph size
- JSON size
- binary size
- agent serialization size
- token count

Derived metrics:
- tokens per repository fact
- tokens per dependency
- tokens per semantic relationship
- tokens per task

### Reliability
- hallucinated dependencies
- incorrect relationships
- unsupported semantic claims
- stale information
- incorrect edits

## Tasks
Select 10-20 representative tasks:
- Find definition
- Find callers
- Find dependents
- Impact analysis
- Architecture overview
- Edit function with contract preservation

## Procedure
1. Build knowledge graph for each repo
2. Generate representations A-E
3. Run agent with each representation
4. Record metrics
5. Statistical analysis

## Success criteria
Semantic representation improves task success by >10% OR reduces tokens by >20% with equal quality.

## Implementation
Create evaluation harness in prime-bench:
- benchmark_agent_performance
- Compare representations

Output comparison table.

## Risks
- Semantic metadata may be incomplete
- Tokenization differences may bias results
- Patch projection may be inaccurate

Mitigation: Use evidence-backed semantics only.
