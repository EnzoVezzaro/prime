# Retrieval Diagnostic Mode Design

## Goal
Expose why Prime returns what it returns.

CLI: `prime query --explain "<query>"` or `prime query --debug "<query>"`

## Output Structure

QUERY
INTENT
RESOLVED ENTITIES
CANDIDATES
FILTERED CANDIDATES
RANKED RESULTS
SELECTED EVIDENCE
EXPANDED RELATIONSHIPS

## Per-candidate metadata

entity_id
qualified_name
file
entity_type
match_reason
retrieval_source
score
name_score
path_score
namespace_score
relationship_score
semantic_score
confidence
graph_distance
evidence_available
why_selected / why_rejected

## Pipeline stages to trace

QUERY → QUERY CLASSIFICATION → ENTITY RESOLUTION → CANDIDATE GENERATION → LEXICAL MATCH → SYMBOL MATCH → PATH/MODULE MATCH → RELATIONSHIP EXPANSION → GRAPH RANKING → EVIDENCE FILTERING → FINAL RESULTS

## Implementation notes

Add `ExplainOptions` to QueryOptions.
Add `ExplainResult` struct.
Instrument QueryEngine.search, find_by_name, get_context, etc.
Log candidate pool size, filtered size, result size.
No code changes yet – design only.
