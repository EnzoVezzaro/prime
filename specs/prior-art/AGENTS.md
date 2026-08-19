# prior-art

## Purpose

Research and analysis of existing systems and approaches relevant to Prime.

## Responsibilities

- Research SCIP, LSIF, CPG/Joern, Tree-sitter, Graph-sitter
- Research agent-oriented code indexing systems
- Create and maintain comparison tables
- Document reusable components and licenses

## Ownership

Owner: research team

## Inputs

- Official repositories (sourcegraph/scip, microsoft/lsif-node, joernio/joern, tree-sitter/tree-sitter, ast-grep/ast-grep)
- Official documentation and specifications
- Academic papers on code graphs, program analysis, code indexing

## Outputs

- SPECS/prior-art/scip.md
- SPECS/prior-art/lsif.md
- SPECS/prior-art/cpg-joern.md
- SPECS/prior-art/tree-sitter.md
- SPECS/prior-art/graph-sitter.md
- SPECS/prior-art/agent-indexers.md
- SPECS/prior-art/comparison.md

## Dependencies

- SPECS/references/ (papers, repositories, specifications)
- SPECS/reusable-tools/ (parsers, analyzers, storage, search)

## Constraints

- Primary sources first: official docs, papers, specs, source code
- Do not rely solely on marketing/documentation; inspect implementations
- Research alternatives fairly: investigate multiple approaches per system
- Distinguish clearly: FACT, OBSERVATION, HYPOTHESIS, INFERENCE, OPEN QUESTION

## Architecture

Each prior art system has a dedicated research file. Comparison tables in comparison.md provide cross-system analysis.

## Workflows

- See `.acc/config/workflows/research.md` for conducting prior art research.
- See `.acc/config/workflows/feature.md` for adding a new prior art system.
- See `.acc/config/workflows/release.md` for research milestone checklists.