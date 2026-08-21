---
title: Cryptography
---

# Cryptography Research

## Why Cryptography?

Cryptography is not being researched because encryption is inherently faster.

It is being researched because **trust, provenance, identity, deduplication, and distributed knowledge verification** may materially improve the overall system.

## Cryptographic Primitives for Prime

### 1. Hashing (Content Addressing)
- **Algorithm**: BLAKE3 (fast, parallelizable, 256-bit)
- **Use**: Content addressing, Merkle DAGs, integrity
- **Alternative**: SHA-256 (standard, hardware accelerated)

### 2. Signatures (Provenance & Trust)
- **Algorithm**: Ed25519 (fast, small keys, deterministic)
- **Use**: Artifact signing, author attribution, CI attestation
- **Key management**: Short-lived keys, rotation, transparency logs

### 3. Commitments (Privacy-Preserving)
- **Pedersen commitments** — Hide values, allow verification
- **Use**: Private repo metadata, sensitive contracts

### 4. Searchable Encryption
- **SSE (Searchable Symmetric Encryption)** — Encrypted index, searchable
- **PEKS (Public Key Encryption with Keyword Search)** — Public search
- **Use**: Encrypted artifacts with queryable index

### 5. Zero-Knowledge Proofs
- **ZK-SNARKs / ZK-STARKs** — Prove knowledge without revealing
- **Use**: Prove artifact derived correctly without revealing source
- **Challenges**: Prover time, trusted setup, circuit complexity

### 6. Merkle Proofs
- **Merkle tree** — O(log n) inclusion proofs
- **Merkle DAG** — Partial artifact verification
- **Use**: Verify artifact subset without full download

## Threat Model

| Threat | Mitigation |
|--------|------------|
| Artifact tampering | Ed25519 signatures, Merkle proofs |
| Source leakage | Encrypted artifacts, searchable encryption |
| Provenance forgery | Signed transparency logs, key rotation |
| Supply chain | CI attestation, SLSA compliance |
| Privacy | Searchable encryption, ZK proofs |

## Key Management

| Component | Strategy |
|-----------|----------|
| Root keys | Offline, HSM, multi-party |
| CI keys | Short-lived (1hr), auto-rotated |
| Agent keys | Per-session, ephemeral |
| Transparency log | Append-only, public, auditable |

## Provenance Chain

```
Source code (git commit)
    │
    ▼
Derivation (tool version, config hash)
    │
    ▼
Artifact (content hash)
    │
    ▼
Signature (CI key)
    │
    ▼
Transparency log entry
```

Each link verifiable independently.

## Open Questions

- ZK proofs: practical for artifact derivation verification?
- Searchable encryption: performance overhead acceptable?
- Key rotation: how to handle revoked keys in CAS?
- Hardware acceleration: leverage CPU AES/SHA extensions?
- Post-quantum: when to migrate (ML-DSA, SLH-DSA)?

## Next

- [Prior Art](./prior-art.md)
- [Specification: Provenance](../specification/provenance.md)
- [Specification: Confidence](../specification/confidence.md)