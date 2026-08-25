# Prime Retrieval Architecture Map – Current

## Entry points

- ToolExecutor.execute → ToolIntent
- QueryEngine methods:
  - find_by_qualified
  - find_by_name
  - find_by_prefix
  - search
  - get_context
  - get_surrounding_context
  - get_relations

## Pipeline as implemented

query string
  ↓
ToolIntent selection (from MCP)
  ↓
QueryEngine.search / find_by_qualified / get_context
  ↓
Candidate generation = scan all entities or name index
  ↓
Lexical scoring in search
  ↓
Confidence filter + max_results truncation
  ↓
Result serialization via PrimeEnvelope

No explicit stages:
- intent classification – none
- entity resolution – exact qualified name or name list
- hard filtering – only confidence
- ranking – lexical score only, hardcoded 1.0 for context
- graph expansion – all incoming/outgoing, unranked
- evidence selection – none, provenance empty
- progressive disclosure – token budget in ProgressiveContextBuilder but not enforced

## Data flows

KnowledgeGraph
- entities HashMap<EntityId, Entity>
- relations Vec<Relation>
- name_index, relation_index

QueryEngine holds Arc<KnowledgeGraph>. No separate candidate pool.

## Gaps vs target

Target pipeline:
query → intent → candidate generation → entity resolution → filtering → ranking → graph expansion → evidence selection → serialization

Current missing:
intent classifier
candidate pool abstraction
entity resolver with disambiguation
hard filter stage
ranker with multiple signals
intent-aware expansion policy
evidence selection
explainability

## Instrumentation needed

- candidate count
- filtered count
- result count
- per-candidate scores
- rejection reasons
- intent detected
