# Progressive Disclosure

## Concept
Return minimal relevant context first, allow expansion on demand.

## Levels
Level0: Entity summary
Level1: Entity + direct relations
Level2: Entity + 1-hop neighborhood
Level3: Entity + 2-hop neighborhood

## Token Budget
Respect opts.token_budget. Estimate tokens per entity.

## API
get_context_level(entity_id, level) -> QueryResult
expand_context(handle) -> next level

## Telemetry
Track bytes_retrieved, tokens_retrieved, expansion depth.

## Implementation Notes
ProgressiveContextBuilder already exists. Integrate with telemetry.
