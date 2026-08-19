Prime Research Project - Getting Started

This directory contains documentation for the Prime research project.

## Project Structure
The research is organized in the SPECS/ folder with the following main sections:

### 1. Read These First
- `SPECS/README.md` - Project overview and important constraints
- `SPECS/RESEARCH.md` - Central research document with objective, scope, methodology

### 2. Survey the Landscape (Prior Art)
- `SPECS/prior-art/` - Research of existing systems (SCIP, LSIF, CPG/Joern, Tree-sitter, Graph-sitter, agent indexers)
- Comparison tables in `SPECS/prior-art/comparison.md` - Detailed comparison of all major systems

### 3. Technical Domains
Explore these based on your interest:
- `SPECS/code-analysis/` - What can be derived from source code
- `SPECS/storage/` - Storage system options and tradeoffs
- `SPECS/compression/` - Compression techniques and tradeoffs
- `SPECS/indexing/` - Index structures for agent retrieval
- `SPECS/retrieval/` - How agents retrieve codebase knowledge
- `SPECS/systems/` - I/O, memory, caching, scalability characteristics
- `SPECS/languages/` - Language-agnostic representation research

### 4. Key Research Findings
- `SPECS/findings/` - Executive summary, key findings, technical findings, open problems, research gaps

### 5. Benchmarks and Datasets
- `SPECS/benchmarks/` - Existing benchmarks, methodology, dataset design considerations

### 6. References and Further Reading
- `SPECS/references/` - Academic papers, GitHub repositories, technical specifications, glossary

## Important Constraints
1. **Do NOT implement Prime** - This is a research repository. The deliverable is knowledge, not code.
2. **Do NOT claim product implementation** - Clearly distinguish research from future product.
3. **Avoid premature technology decisions** - Do not decide on SQLite, RocksDB, graph databases, etc. until research concludes why.
4. **Follow research principles** - Evidence over assumptions, primary sources first, no premature convergence, fair alternative research.

## First Steps
1. Start with `SPECS/README.md` and `SPECS/RESEARCH.md` to understand the project scope
2. Browse `SPECS/prior-art/` to understand existing systems you're researching against
3. Read `SPECS/compression/integer-compression.md` and `SPECS/compression/general-compression.md` for core compression research
4. Review `SPECS/prior-art/comparison.md` for comparison tables of all major systems
5. Check `SPECS/findings/` for key findings and open problems

## Research Workflow (from init-promt.md)
1. Create directory structure (done - see SPECS/ folder)
2. Create README.md (done)
3. Create RESEARCH.md (done)
4. Create research index (done - this directory structure)
5. Begin with prior art (in progress - prior-art/ folder populated)
6. Expand into technical domains (in progress - all other folders populated)
7. Maintain citations and source links (ongoing)
8. Maintain bibliography (ongoing)
9. Maintain glossary (in progress - references/glossary.md)
10. Maintain explicit list of open questions (in progress - findings/open-problems.md)

## Contact / Contributing
This is a research project, not a product. For questions about the research approach, refer to the documents in SPECS/ or the init-promt.md source document.