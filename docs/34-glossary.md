---
title: Glossary
---

# Glossary

## A

**Agent** — An AI system that operates on codebases: observes, reasons, retrieves, acts.

**Artifact** — The single logical knowledge representation produced by Prime from a codebase.

## C

**Confidence** — Four-level certainty: `exact`, `derived`, `inferred`, `unknown`.

**Content addressing** — Addressing data by its cryptographic hash rather than location.

## D

**Derivation** — The process of transforming source code into the Prime artifact.

**Dynamic dispatch** — Runtime polymorphism where the exact method is unknown statically.

## E

**Entity** — A symbol in the codebase (function, class, variable, module, etc.).

**Exact** — Highest confidence level: verified directly in source.

## G

**Graph** — The network of entities and relations in a codebase.

## I

**Inferred** — Confidence level: probabilistic heuristic (dynamic dispatch, reflection).

**Incremental** — Updating the artifact without full re-derivation.

## K

**Knowledge unit** — The smallest independently useful piece of codebase knowledge.

## L

**Language agnostic** — Working across programming languages with a universal model.

## M

**MCP** — Model Context Protocol; standard for agent-tool communication.

**Minimum knowledge unit** — Research term for the atomic unit of agent-useful knowledge.

## P

**Prime** — The project name. Also the derived artifact.

**Provenance** — Origin of a fact: `declared`, `discovered`, `inferred`, `memory`, `stored`.

## R

**Relation** — A typed edge between two entities (calls, defines, imports, etc.).

## S

**SCIP** — Source Code Intelligence Protocol; cross-language indexing standard.

**Source escalation** — When Prime cannot answer, targeted source retrieval.

**Succinct data structures** — Space-efficient structures with fast queries (rank/select, Elias-Fano).

## T

**Tree-sitter** — Incremental, error-tolerant parsing library for many languages.