# coding.md — Prime Research Project Coding Standards

This standard defines coding conventions for the Prime research project itself.
Note: Prime is a RESEARCH project, not a product. The "code" here refers to
any scripts, tooling, or infrastructure code used in the research process.

## General

- **Language**: Any language appropriate for the research task (Python for analysis, Rust/Go for performance-critical tools, TypeScript for tooling).
- **Style**: Follow the language's official style guide (rustfmt, prettier, gofmt, black).
- **Linting**: All code must pass linters appropriate to the language.
- **Tests**: Unit tests for research tooling; validation tests for research hypotheses.

## Research Tooling Standards

- **Determinism**: Research tools should produce deterministic output given the same input.
- **Reproducibility**: All research experiments should be reproducible from the SPECS/ documentation.
- **Version control**: Research scripts and tooling should be versioned with the research.
- **Documentation**: Research tools must be documented in SPECS/ with usage examples.

## Markdown Standards (for SPECS/ files)

- **No inline comments** unless explicitly asked.
- **Code references**: Use `file_path:line_number` pattern.
- **No proprietary schemas**: No YAML frontmatter or custom schemas in markdown.
- **Evidence-based**: All claims must have supporting evidence or be marked as HYPOTHESIS/INFERENCE/OPEN QUESTION.
- **Citations**: Use direct URLs/references for sources (official repos, papers, specs).

## Git Standards

- **Conventional commits**: `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`, `research:`.
- **Research commits**: Use `research:` prefix for commits adding research findings.
- **No direct pushes to main**; PRs required for research changes.
- **CI must pass** before merge (lint, tests, `acc check`).

## Documentation Standards

- **SPECS/**: Research specifications — evidence-based, hypotheses marked, open questions explicit.
- **docs/**: Operational docs — getting started, guidelines, contributing, navigation.
- **AGENTS.md**: Contract for each research area (Purpose, Responsibilities, Ownership, Inputs, Outputs, Dependencies, Constraints, Architecture, Workflows).

## Research-Specific

- **Hypothesis marking**: Any unvalidated claim must be marked `<!-- HYPOTHESIS -->`.
- **Inference marking**: Any deduced claim must be marked `<!-- INFERENCE -->`.
- **Open questions**: Explicitly listed in SPECS/findings/open-problems.md and research-gaps.md.
- **Evidence citation**: Direct URLs to official repositories, papers, specifications.