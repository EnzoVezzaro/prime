Prime Research Project - Contributing

This repository is a research project, not a product. Contributions are welcome in the form of research improvements, corrections, additional documentation, and guideline suggestions.

## How to Contribute

### 1. Research Improvements
- Corrections to existing documents (fact errors, unclear explanations)
- Additional citations or sources for claims
- New research areas or domains to investigate
- Improved comparison tables with more systems or dimensions
- Better analysis of tradeoffs or limitations

### 2. Documentation
- Clarifying existing documents
- Adding missing sections (e.g., more details on a research area)
- Creating index or overview documents
- Improving formatting and consistency
- Translating documentation if needed

### 3. Structure & Organization
- Suggesting better folder organization
- Recommending document splitting/merging
- Improving the index or navigation
- Adding cross-references between documents

## Contribution Process

### 1. Fork and Clone
- Fork this repository
- Clone your fork: `git clone https://github.com/your-username/prime.git`
- Add upstream remote: `git remote add upstream https://github.com/original/prime.git`

### 2. Create a Branch
- `git checkout -b feature/research-improvement`
- Or: `git checkout -b fix/documentation-clarity`
- Or: `git checkout -b docs/add-index`

### 3. Make Changes
- Edit markdown files in SPECS/ or docs/ as needed
- Follow existing style and conventions
- Add citations and sources for new claims
- Keep evidence-based approach (no AI-generated summaries without clear attribution)
- Update any comparison tables if adding new systems or dimensions

### 4. Commit Guidelines
- Use clear, descriptive commit messages
- Reference the document(s) you're modifying
- Example: `git commit -m "fix: correct SCIP protocol version in scip.md (line 23)"`
- Example: `git commit -m "docs: add getting-started overview (GETTING-STARTED.md)"`

### 5. Push and Pull Request
- `git push origin feature/research-improvement`
- Submit a Pull Request to the upstream repository
- Describe what research improvement or documentation change you're proposing
- Reference any relevant init-promt.md principles or other documents

## Research Quality Gates
Before contributions are merged, they should pass these checks:
- [ ] No AI-generated summaries without clear attribution and labeling
- [ ] All factual claims have supporting evidence or citation
- [ ] No product design claims without research consensus
- [ ] Consistent with init-promt.md principles (evidence over assumptions, primary sources first, no premature convergence, fair alternative research)
- [ ] Follow existing file patterns and formatting in SPECS/ and docs/
- [ ] No secrets or proprietary information added
- [ ] Clear distinction between research and product design

## What This Project Is NOT
- This is NOT a Prime product implementation
- This is NOT a codebase for building a graph engine or binary format
- This is NOT intended for production use
- This is a research repository investigating how to represent software repositories as compact, language-agnostic knowledge artifacts

## Project Contacts
For questions about the research approach or project structure, refer to:
- init-promt.md - The original prompt/document that started this research
- SPECS/README.md - Project overview and constraints
- SPECS/RESEARCH.md - Central research document

Thank you for contributing to serious technical research!