---
title: Distributed Systems
---

# Distributed Systems Research

## Why Distributed?

- **Multi-repo workspaces** — Monorepos span multiple repositories
- **Team boundaries** — Different teams own different modules
- **CI/CD integration** — Artifacts produced in CI, consumed in IDE
- **Remote agents** — Agents running in cloud, not local
- **Knowledge sharing** — Reuse derived knowledge across orgs

## Architecture Patterns

### 1. Content-Addressed Storage (CAS)
- Blocks addressed by hash (blake3, sha256)
- Automatic deduplication
- Immutable, verifiable
- Sync via hash exchange

### 2. Merkle DAGs
- DAG of content-addressed blocks
- Efficient diff/sync (compare roots)
- Partial loading (fetch subtrees)
- Git-like but for knowledge artifacts

### 3. CRDTs for Knowledge Merging
- Conflict-free replicated data types
- Merge concurrent derivations
- Eventual consistency
- Semantic merge (not textual)

### 4. Peer-to-Peer Distribution
- No central server required
- BitTorrent-style or libp2p
- Direct agent-to-agent sync
- NAT traversal via relays

## Synchronization Strategies

| Strategy | Use Case | Consistency |
|----------|----------|-------------|
| Push on build | CI → Registry | Strong |
| Pull on demand | Agent → Registry | Eventual |
| Peer sync | Agent ↔ Agent | Eventual |
| Incremental | Watch → Delta push | Eventual |

## Partial Loading

Agents shouldn't load entire artifacts for large codebases:

```
Artifact (10GB)
    │
    ├── Index (100MB) ← always load
    ├── Module: auth (50MB) ← load on demand
    ├── Module: payments (200MB) ← load on demand
    └── Module: analytics (5GB) ← load on demand
```

### Loading Strategies

| Strategy | When |
|----------|------|
| Eager index | Always |
| Lazy module | On first query to module |
| Prefetch neighbors | After context query |
| Predictive | Based on agent task history |

## Consistency Models

| Model | Guarantees | Use Case |
|-------|------------|----------|
| Strong | Linearizable | CI artifact publishing |
| Sequential | Single-writer | Team-owned modules |
| Causal | Dependency-aware | Cross-team refs |
| Eventual | Convergent | Peer sync, caches |

## Security & Trust

- **Signed artifacts** — Ed25519 signatures on artifact root
- **Provenance chains** — Hash chain from source → artifact
- **Access control** — Capability-based (UCAN, JWT)
- **Privacy** — Encrypted blocks for private repos

## Open Questions

- CRDT design for graph-like knowledge (not just counters/sets)?
- Sync protocol: custom vs existing (libp2p, BitTorrent, rsync)?
- How to handle schema evolution in distributed setting?
- Garbage collection of unreferenced blocks?

## Next

- [Cryptography](./cryptography.md)
- [Prior Art](./prior-art.md)
- [Specification: Artifact](../specification/artifact.md)