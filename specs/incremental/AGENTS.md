# incremental

## Purpose

Research on incremental analysis, invalidation, and snapshot mechanisms for codebase knowledge artifacts.

## Responsibilities

- Research incremental parsing (Tree-sitter, content hashing, Merkle trees)
- Research incremental indexing and dependency invalidation
- Research partial recompilation, persistent indexes, immutable snapshots
- Research content hashing, change detection, Merkle trees for incremental invalidation

## Ownership

Owner: research team

## Inputs

- Tree-sitter incremental parsing documentation
- Content hashing and Merkle tree literature
- Incremental compilation literature (Rust, Go, TypeScript incremental compilation)
- CRDT literature (for distributed incremental updates)

## Outputs

- SPECS/incremental/incremental-analysis.md
- SPECS/incremental/invalidation.md
- SPECS/incremental/snapshots.md

## Dependencies

- SPECS/storage/ (persistent indexes, immutable snapshots)
- SPECS/compression/ (incremental compression)
- SPECS/systems/ (concurrency for incremental updates)

## Constraints

- Determine what must be recomputed when: one file changes, one symbol changes, one dependency changes, entire package changes
- Support incremental derivation and incremental invalidation
- Support partial loading and partial retrieval
- Merkle trees for content addressing and incremental invalidation
- CRDTs for distributed Prime (research track, not core)

## Architecture

Three research files: incremental analysis, invalidation, snapshots. All support the incremental update pipeline.

## Workflows

- See `.acc/config/workflows/research.md` for conducting incremental research.
- See `.acc/config/workflows/feature.md` for adding a new incremental technique.