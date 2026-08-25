# Deterministic Ranking Model

Score = identity + intent + relationship + proximity + evidence + confidence - penalties

Signals:
+ exact qualified-name match
+ exact symbol match
+ exact path match
+ entity-type match
+ intent compatibility
+ relationship compatibility
+ direct relationship
+ semantic neighborhood proximity
+ definition over reference
+ production code over test
+ evidence availability
+ confidence

Penalties:
- unrelated entity type
- excessive graph distance
- test-only when production exists
- weak textual match
- ambiguous symbol
- low-confidence inferred relation
- duplicate semantic result

Weights configurable, not baked in.
Ranking must be inspectable.
