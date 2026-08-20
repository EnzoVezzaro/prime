# Contributing to Prime

Thank you for your interest in contributing to Prime! This document outlines the process for contributing to this research project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Types of Contributions](#types-of-contributions)
- [Development Workflow](#development-workflow)
- [Code Style](#code-style)
- [Testing](#testing)
- [Documentation](#documentation)
- [Submitting Changes](#submitting-changes)

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](https://www.contributor-covenant.org/version/2/1/code_of_conduct/). By participating, you are expected to uphold this code.

## Getting Started

### Prerequisites

- Rust 1.75+ (install via [rustup](https://rustup.rs/))
- Node.js 18+ (for package.json scripts)
- Docker & Docker Compose (for stack scripts)
- Git

### Setup

```bash
# Clone the repository
git clone https://github.com/EnzoVezzaro/prime.git
cd prime

# Build the project
cargo build --release --workspace

# Run the CLI
cargo run --bin prime -- --help
```

## Types of Contributions

We welcome several types of contributions:

### Research Contributions

- **Literature reviews** - Summaries of relevant papers with analysis
- **Comparative analyses** - Comparing approaches across systems
- **Benchmark results** - Performance measurements on real repositories
- **Failed approaches** - Documenting what didn't work and why
- **New ideas** - Novel approaches to research questions

### Code Contributions

- **Parser improvements** - Tree-sitter query enhancements
- **Storage optimizations** - Better compression, indexing, mmap
- **Query optimizations** - Faster retrieval, better context selection
- **Language support** - New Tree-sitter parsers, language analyzers
- **Bug fixes** - Correctness issues in parsing, indexing, querying

### Documentation

- **SPECS/ improvements** - More detailed research findings
- **Tutorials** - How-to guides for using Prime
- **API documentation** - Rustdoc improvements
- **Architecture diagrams** - Mermaid diagrams for architecture

### Infrastructure

- **CI/CD improvements** - GitHub Actions workflows
- **Benchmark automation** - Automated benchmark runs
- **Release automation** - Release tooling
- **Developer tooling** - Better dev experience

## Development Workflow

### Branching Strategy

- `main` - Stable research state
- `feature/*` - New research areas or features
- `fix/*` - Bug fixes
- `refactor/*` - Code restructuring
- `docs/*` - Documentation updates

### Commit Convention

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

Types:
- `feat:` New research area or feature
- `fix:` Bug fix
- `docs:` Documentation only
- `refactor:` Code restructuring
- `research:` Research findings or analysis
- `bench:` Benchmark changes
- `ci:` CI/CD changes
- `chore:` Maintenance

### Pull Request Process

1. Fork the repository
2. Create a feature branch from `main`
3. Make your changes
4. Ensure all checks pass:
   - `cargo check --workspace`
   - `cargo test --workspace`
   - `cargo fmt --all --check`
   - `cargo clippy --workspace -- -D warnings`
5. Update documentation if needed
6. Submit PR with clear description

### Review Process

- All PRs require at least one review
- Research changes require architect review
- Security changes require security review
- CI must pass before merge

## Code Style

### Rust

- Follow `rustfmt` (run `cargo fmt --all`)
- Pass `cargo clippy --workspace -- -D warnings`
- Use `#[derive(Serialize, Deserialize)]` for serializable types
- Prefer `anyhow::Result` for error handling
- Use `thiserror` for custom error types

### Markdown

- No inline comments unless explicitly asked
- Use `file_path:line_number` for code references
- Follow existing SPECS/ formatting conventions
- Distinguish FACT/OBSERVATION/HYPOTHESIS/INFERENCE/OPEN QUESTION

## Testing

```bash
# Run all tests
cargo test --workspace

# Run specific crate tests
cargo test -p prime-core
cargo test -p prime-parser
cargo test -p prime-index

# Run benchmarks
cargo bench --workspace

# Run with specific test
cargo test -p prime-parser test_parse
```

### Test Organization

- Unit tests: Co-located with source (`*_test.rs`)
- Integration tests: `tests/` directory at crate root
- Fixtures: `tests/fixtures/` for sample repositories
- Determinism tests: `tests/determinism.test.rs`

## Documentation

### SPECS/ Documentation

- Follow existing file structure
- Use evidence-based claims with citations
- Distinguish: FACT, OBSERVATION, HYPOTHESIS, INFERENCE, OPEN QUESTION
- Include comparison tables where alternatives exist
- Link to primary sources (official docs, papers, specs, source code)

### Code Documentation

- Public APIs must have doc comments
- Use `cargo doc --workspace --no-deps --open` to preview
- Update SPECS/ when architecture changes

## Submitting Changes

### Before Submitting

- [ ] All tests pass
- [ ] `cargo fmt --all --check` passes
- [ ] `cargo clippy --workspace -- -D warnings` passes
- [ ] Documentation updated if needed
- [ ] SPECS/ updated if research findings changed
- [ ] CHANGELOG.md updated (for user-facing changes)

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Research finding
- [ ] Bug fix
- [ ] Feature
- [ ] Documentation
- [ ] Refactor
- [ ] Benchmark
- [ ] Other

## Research Area (if applicable)
- [ ] Codebase Knowledge
- [ ] CPG/Joern
- [ ] SCIP
- [ ] LSIF
- [ ] Tree-sitter
- [ ] Agent Indexing
- [ ] Information Retrieval
- [ ] Storage
- [ ] Binary Formats
- [ ] Compression
- [ ] Succinct Structures
- [ ] Memory Mapping
- [ ] Large Scale
- [ ] Incremental Analysis
- [ ] Language Agnosticism
- [ ] Agent Context
- [ ] Reusable Tools
- [ ] Benchmarks
- [ ] Academic Research

## Testing
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Benchmarks run (if applicable)
- [ ] Manual testing done

## Checklist
- [ ] Tests pass
- [ ] Linting passes
- [ ] Formatting correct
- [ ] Documentation updated
- [ ] SPECS/ updated (if research)
- [ ] CHANGELOG.md updated (if user-facing)
```

## Recognition

Contributors will be recognized in:
- Release notes
- CONTRIBUTORS.md
- Release announcements

Thank you for contributing to Prime! 🚀