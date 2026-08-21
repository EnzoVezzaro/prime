---
title: Knowledge Model
---

# Knowledge Model

Prime's knowledge model defines the logical structure of the derived artifact.

## Core Types

### Entity
```typescript
interface Entity {
  id: EntityId;                    // Stable, content-addressed
  kind: SymbolKind;                // Function, Class, Interface, Module, ...
  name: string;                    // Simple name
  qualified_name: string;          // Globally unique
  file_id: EntityId;               // Containing file
  range: Range;                    // Source location
  language: Language;
  confidence: Confidence;
  signature?: string;              // For functions: "fn(a: Type) -> Ret"
  documentation?: string;
  children: EntityId[];            // Nested entities
  relations: Relation[];           // Direct relations
}
```

### Relation
```typescript
interface Relation {
  from: EntityId;
  to: EntityId;
  kind: RelationKind;
  confidence: Confidence;
  provenance: Provenance;
}
```

### SymbolKind
```
Function, Method, Constructor, Field, Property,
Class, Interface, Trait, Struct, Enum, TypeAlias,
Module, Namespace, Package, File,
Variable, Constant, Parameter, TypeParameter,
Macro, Attribute, Decorator, Annotation
```

### RelationKind
```
CALLS, CALLED_BY,
DEFINES, DEFINED_IN,
IMPORTS, IMPORTED_BY,
EXTENDS, IMPLEMENTS,
USES, USED_BY,
CONTAINS, CONTAINED_IN,
DEPENDS_ON, DEPENDENCY_OF,
RETURNS, MAY_THROW,
REFERENCES, REFERENCED_BY
```

## Provenance

```typescript
type Provenance =
  | { kind: 'declared'; source: 'AGENTS.md' }
  | { kind: 'discovered'; source: 'static_analysis' | 'imports' | 'calls' }
  | { kind: 'inferred'; source: 'heuristic' | 'dynamic_dispatch' | 'pattern'; reason: string }
  | { kind: 'memory'; source: '.acc-memory.md' }
  | { kind: 'stored'; source: 'prime_artifact' }
```

## Confidence

```typescript
type Confidence = 'exact' | 'derived' | 'inferred' | 'unknown';
```

## File
```typescript
interface File {
  id: EntityId;
  path: string;                    // Relative to project root
  language: Language;
  size: number;
  content_hash: ContentHash;
  entities: EntityId[];            // Entities defined in this file
}
```

## Module
```typescript
interface Module {
  id: EntityId;
  name: string;
  path: string;
  language: Language;
  files: EntityId[];
  parent?: EntityId;
  children: EntityId[];
  exports: EntityId[];             // Exported entity IDs
}
```

## Project
```typescript
interface Project {
  name: string;
  root_path: string;
  version: string;
  languages: Language[];
  file_count: number;
  entity_count: number;
  relation_count: number;
  created_at: number;              // Unix timestamp
  content_hash: ContentHash;       // Hash of all source for change detection
}
```

## KnowledgeGraph
```typescript
interface KnowledgeGraph {
  project: Project;
  entities: Map<EntityId, Entity>;
  relations: Relation[];
  files: Map<EntityId, File>;
  modules: Map<EntityId, Module>;
  // Inverse indexes (not serialized)
  name_index?: NameIndex;
  file_index?: FileIndex;
  relation_index?: RelationIndex;
}
```

## Next

- [Representation](./representation.md)
- [Artifact](./artifact.md)
- [Provenance](./provenance.md)