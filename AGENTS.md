# prime

## Purpose

Prime is a research project investigating how software repositories can be transformed into extremely compact, language-agnostic knowledge representations optimized for machine analysis and AI-agent retrieval.

## Responsibilities

- Conduct evidence-based research on codebase knowledge representation
- Investigate prior art (SCIP, LSIF, CPG/Joern, Tree-sitter, agent indexers)
- Analyze compression, storage, indexing, retrieval tradeoffs
- Research language-agnostic semantic models
- Document findings with explicit confidence levels (FACT, OBSERVATION, HYPOTHESIS, INFERENCE, OPEN QUESTION)

## Ownership

Owner: research team

## Inputs

- Source code repositories (any language, any scale)
- Academic papers and technical specifications
- Open-source tools and libraries (parsers, analyzers, storage engines)

## Outputs

- SPECS/ research specifications (evidence-based, not product)
- docs/ operational documentation
- ACC configuration for agent context awareness

## Dependencies

- agents-code-context (ACC CLI for agent context)
- Tree-sitter parsers (language analysis)
- Language-specific analyzers (rust-analyzer, tsserver, etc.)

## Constraints

- Evidence over assumptions: never claim without evidence
- Primary sources first: official docs, papers, specs, source code
- No premature convergence: don't decide technologies until research establishes why
- Research alternatives fairly: investigate multiple approaches per design area
- No product design: document evidence, alternatives, tradeoffs, hypotheses, open questions
- Distinguish clearly: FACT, OBSERVATION, HYPOTHESIS, INFERENCE, OPEN QUESTION

## Architecture

The Prime research repository has two distinct layers:

1. **SPECS/** — Research specifications: evidence-based findings, hypotheses, open questions, NO product design
2. **docs/** — Operational documentation: getting started, guidelines, contributing, navigation

The ACC skill is configured to understand this repository structure and provide agent context for research navigation.

## Workflows

- See `.acc/config/workflows/feature.md` for adding a new research area.
- See `.acc/config/workflows/research.md` for conducting research.
- See `.acc/config/workflows/bugfix.md` for fixing tooling bugs.
- See `.acc/config/workflows/refactor.md` for restructuring research areas.
- See `.acc/config/workflows/release.md` for research milestone checklists.
- See `.acc/config/workflows/memory.md` for capturing durable knowledge.
- See `.acc/config/workflows/diagnostic.md` for diagnostic codes.
- See `.acc/config/workflows/security.md` for security-sensitive changes.