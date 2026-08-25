# Semantic Query Layer Design

## Goals
Agent should not load entire graph. Targeted queries with token budgets.

## Core queries

### Entity retrieval
get_repository() -> Project summary
get_module(name) -> ModuleInfo + entities
get_function(qualified_name) -> EntityDetail with semantic metadata
get_symbol(name) -> List<EntityDetail>

### Relationship navigation
get_dependencies(entity, scope=direct|transitive, depth)
get_dependents(entity, scope, depth)
get_callers(entity)
get_callees(entity)
get_dataflow(entity) -> reads/writes/produces/consumes

### Semantic queries
get_effects(entity) -> Vec<Effect>
get_capabilities(entity) -> Vec<Capability>
get_constraints(entity) -> Vec<ConstraintViolation>
get_impact(entity) -> ImpactResult
get_tests(entity) -> Vec<EntityDetail>
get_evidence(entity) -> Vec<Evidence>
get_architecture(name?) -> ArchitectureResult

## API design

Extend ToolExecutor with semantic queries.

ToolRequest already has intent enum. Add new intents:
SemanticLookup, SemanticContext, Dataflow, Effects, Capabilities, Constraints, Evidence

Response envelope PrimeEnvelope<T> remains.

Context building with progressive disclosure:
1. First call returns target + summary
2. Agent can request expansion with token budget
3. System tracks ContextHandle { id, retrieved, used_tokens, remaining_tokens }

## Implementation plan
1. Extend prime-core::ToolIntent with semantic variants
2. Extend ToolExecutor::execute to handle new intents
3. Add semantic metadata to KnowledgeGraph building phase
4. Implement query functions in prime-query
5. Add MCP tool wrappers for new intents
6. CLI commands: prime query --type semantic, prime context --semantic

## Example usage
prime_context AuthService.login --semantic --depth 2 --token-budget 4096
Returns target with effects, capabilities, contracts, dependencies, callers, callees, tests, evidence.

## Token budgeting
Estimate tokens per entity:
base ~ 50 tokens
+ 10 per relation
+ 5 per effect
+ 20 per documentation chunk

Trim entities by confidence and relevance.
