# findings

## Purpose

Research findings summary and synthesis for the Prime project.

## Responsibilities

- Maintain executive summary of all research findings
- Document key findings with evidence citations
- Track technical findings with confidence levels
- Maintain open problems and research gaps lists

## Ownership

Owner: research team

## Inputs

- All SPECS/ research area findings
- SPECS/prior-art/ comparison tables
- SPECS/benchmarks/ validation results

## Outputs

- SPECS/findings/executive-summary.md
- SPECS/findings/key-findings.md
- SPECS/findings/technical-findings.md
- SPECS/findings/open-problems.md
- SPECS/findings/research-gaps.md

## Dependencies

- All SPECS/ research areas

## Constraints

- All findings must distinguish: FACT, OBSERVATION, HYPOTHESIS, INFERENCE, OPEN QUESTION
- Evidence citations required for all FACT and OBSERVATION claims
- No product design claims in findings

## Architecture

Findings are synthesized from all SPECS/ research areas. Each finding file is maintained independently but cross-references are maintained via ACC graph derivation.

## Workflows

- See `.acc/config/workflows/feature.md` for adding a new finding.
- See `.acc/config/workflows/research.md` for conducting research.
- See `.acc/config/workflows/release.md` for research milestone checklists.