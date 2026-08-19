# languages

## Purpose

Research on language-agnostic semantic models and language-specific analysis capabilities.

## Responsibilities

- Research universal semantic models (common vocabulary across languages)
- Research language-specific analysis (TypeScript, Rust, Python, Go, Java, others)
- Research cross-language relationships and semantic normalization
- Define language capability model (parsing, symbols, references, types, calls, architecture, runtime behavior)

## Ownership

Owner: research team

## Inputs

- Tree-sitter grammars and parsers
- Language Server Protocol (LSP) and language servers (tsserver, rust-analyzer, pyright, gopls, jdtls)
- SCIP language coverage and indexers
- Compiler APIs (rustc, tsserver, go/ast, Python ast, javac)
- Academic papers on type systems, language interoperability

## Outputs

- SPECS/languages/language-agnostic-models.md
- SPECS/languages/typescript.md
- SPECS/languages/rust.md
- SPECS/languages/python.md
- SPECS/languages/go.md
- SPECS/languages/java.md
- SPECS/languages/other-languages.md

## Dependencies

- SPECS/prior-art/ (Tree-sitter, SCIP, LSP, CPG language support)
- SPECS/code-analysis/ (parsing, symbols, types, references)

## Constraints

- Do not force languages into artificial universal AST; each language frontend produces common semantic vocabulary
- Define capability model per language: parsing, symbols, references, types, calls, data-flow, control-flow, modules, packages, macros, runtime-semantics
- Graceful degradation: Level 1 (Parseable), Level 2 (Semantically analyzable), Level 3 (Knowledge derivable)
- Dynamic languages need special handling (Python, JavaScript, Ruby)
- Cross-language relationships must be preserved (TypeScript → HTTP API → Rust → gRPC → Go → database)

## Architecture

Universal semantic model with language-specific frontends. Seven research files: universal models + 6 language-specific + other languages.

## Workflows

- See `.acc/config/workflows/research.md` for conducting language research.
- See `.acc/config/workflows/feature.md` for adding a new language.