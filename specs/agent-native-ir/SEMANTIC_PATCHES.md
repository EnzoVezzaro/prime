# Semantic Patches & Validation

## Semantic Patch Engine (experimental)

### Patch model
```rust
struct SemanticPatch {
    id: PatchId,
    target: EntityId,
    operation: PatchOperation,
    precondition: Option<Precondition>,
    expected_state: Option<StateHash>,
    evidence: Vec<EvidenceRef>,
    validation: ValidationResult,
    affected_nodes: Vec<EntityId>,
    affected_tests: Vec<EntityId>,
}
```

### Operations
AddFunction, RemoveFunction, RenameSymbol, ChangeType, AddDependency, RemoveDependency, ModifyCall, AddContract, RemoveContract, ChangeEffect, MoveComponent, ChangeConfiguration, ModifyDataflow

### Validation layer
Checks executed after patch application:

invalid_dependency
type_mismatch
forbidden_effect
missing_capability
broken_invariant
invalid_architecture_boundary
missing_required_test
unexpected_external_side_effect

### Constraint engine
Constraint definitions stored as JSON:
{
  "max_function_complexity": 20,
  "required_test_coverage": 0.8,
  "forbidden_effects": ["IO"] for modules matching "lib/*",
  "architecture_boundaries": [...]
}

### Implementation steps
1. Define SemanticPatch structs in prime-core
2. Implement patch applier operating on KnowledgeGraph
3. Add precondition checker
4. Add validation runner
5. Project patches back to source via AST manipulation (experimental, read-only first)
6. Add CLI: prime patch apply --patch json

### Safety
- Patches are previewable
- Affected tests identified
- Validation results returned in envelope
- Source projection is best-effort, manual review recommended

## Evidence
Every semantic fact must have evidence:
- Source location
- Static analysis result
- Runtime observation
- Test result
- Commit info

Confidence derived from evidence quality.
