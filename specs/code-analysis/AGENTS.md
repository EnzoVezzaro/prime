# code-analysis

## Purpose

Research on what information can be derived from source code and how.

## Responsibilities

- Research parsing, AST/CST, symbols, type analysis
- Research references, call graphs, dependency analysis
- Research data flow, control flow, architecture analysis
- Document what is directly observable vs statically derivable vs inferable

## Ownership

Owner: research team

## Inputs

- Tree-sitter documentation and grammars
- Compiler APIs (rustc, tsserver, go/ast, Python ast, javac)
- Language Server Protocol specifications
- Academic papers on static analysis

## Outputs

- SPECS/code-analysis/parsing.md
- SPECS/code-analysis/ast-cst.md
- SPECS/code-analysis/symbols.md
- SPECS/code-analysis/type-analysis.md
- SPECS/code-analysis/references.md
- SPECS/code-analysis/call-graphs.md
- SPECS/code-analysis/dependency-analysis.md
- SPECS/code-analysis/data-flow.md
- SPECS/code-analysis/control-flow.md
- SPECS/code-analysis/architecture-analysis.md

## Dependencies

- SPECS/prior-art/ (Tree-sitter, CPG-Joern, LSIF, SCIP)
- SPECS/prior-art/cpg-joern.md
- SPECS/languages/ (language-specific analysis capabilities)

## Constraints

- Distinguish: directly observable, statically derivable, inferable, probabilistic, unavailable without execution
- Determine which information is useful to agents
- Evidence over assumptions for all claims

## Architecture

Each code analysis aspect has a dedicated research file. Language-specific capabilities documented in SPECS/languages/.

## Workflows

- See `.acc/config/workflows/research.md` for conducting code analysis research.
- See `.acc/config/workflows/feature.md` for adding a new analysis area.