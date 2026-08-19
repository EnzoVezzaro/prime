# references

## Purpose

Research references: academic papers, repositories, specifications, and glossary for the Prime project.

## Responsibilities

- Maintain catalog of academic papers with full citations (title, authors, year, venue, URL/DOI, problem, methodology, results, limitations, relevance)
- Maintain catalog of relevant GitHub repositories with license, maturity, maintenance status
- Maintain catalog of technical specifications (SCIP, LSIF, CPG, Tree-sitter, etc.)
- Maintain glossary of key terms used throughout the research

## Ownership

Owner: research team

## Inputs

- Academic databases (arXiv, Google Scholar, conference proceedings)
- GitHub repositories for all prior art systems
- Official specifications (SCIP protobuf, LSIF spec, CPG spec, Tree-sitter grammar spec)
- Research findings from all SPECS/ areas

## Outputs

- SPECS/references/papers.md
- SPECS/references/repositories.md
- SPECS/references/specifications.md
- SPECS/references/glossary.md

## Dependencies

- All SPECS/ research areas (for citing relevant papers and repos)

## Constraints

- Use direct URLs/references in documents
- Primary sources first: official docs, papers, specs, source code
- For important papers: title, authors, year, venue, URL/DOI, problem, methodology, results, limitations, relevance
- For repositories: GitHub URL, license, language, maturity, maintenance status, performance characteristics, reusable libraries
- Glossary must define all key terms used across SPECS/

## Architecture

Four reference files: papers, repositories, specifications, glossary. Cross-referenced from all SPECS/ research files.

## Workflows

- See `.acc/config/workflows/research.md` for maintaining references.
- See `.acc/config/workflows/feature.md` for adding a new reference category.