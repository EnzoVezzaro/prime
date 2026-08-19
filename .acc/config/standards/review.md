# review.md — Prime Research Review Guidelines

This standard defines the research review process for the Prime project.

## Review Checklist

### Research Quality

- [ ] No AI-generated summaries without clear attribution and labeling.
- [ ] All factual claims have supporting evidence or citation.
- [ ] Primary sources cited (official repos, papers, specs, source code).
- [ ] Claims distinguish: FACT, OBSERVATION, HYPOTHESIS, INFERENCE, OPEN QUESTION.
- [ ] No product design claims without research consensus.

### Research Principles

- [ ] No premature technology decisions (SQLite, RocksDB, graph DB, etc.).
- [ ] Alternatives considered and documented for major design areas.
- [ ] Tradeoffs recorded.
- [ ] Limitations identified.
- [ ] Reusable components recorded with licenses.
- [ ] Open questions explicitly listed.

### Architecture

- [ ] Changes respect SPECS/ folder structure (one file per research area).
- [ ] Dependencies between research areas declared in AGENTS.md.
- [ ] No product design claims in research files.
- [ ] SPECS/ and docs/ layers remain separate.

### Documentation

- [ ] SPECS/findings/ updated with key findings, open problems, research gaps.
- [ ] `.acc-memory.md` updated for durable knowledge (gotchas, decisions, hypotheses).
- [ ] Open questions explicitly listed in SPECS/findings/open-problems.md and research-gaps.md.
- [ ] `AGENTS.md` updated for new/modified research areas.
- [ ] Inferred content in SPECS/ marked `<!-- INFERENCE -->`.
- [ ] Hypotheses marked `<!-- HYPOTHESIS -->`.

### Formatting

- [ ] Follows existing SPECS/ file patterns and formatting.
- [ ] No inline comments unless asked.
- [ ] Code references use `file_path:line_number` pattern.
- [ ] No proprietary schemas (YAML frontmatter, etc.) in markdown.
- [ ] Evidence-based with direct URLs/references.

## Review Process

1. **Automated checks** run in CI: lint, tests, `acc check`, `acc graph`.
2. **Reviewer** assigned (see `.acc/config/agents/reviewer.md`).
3. **Architect review** required for architecture changes (see `architect.md`).
4. **Research review** required for research area changes (see `reviewer.md`).
5. **Approval** from at least one reviewer + architect if applicable.
6. **Merge** after CI passes and approvals granted.

## Review Etiquette

- Be constructive and specific.
- Reference `ACC0xx` codes for architectural issues.
- Distinguish between blocking (error) and non-blocking (warn/info) feedback.
- Distinguish between research quality issues and product design violations.
- Prefer questions over demands for subjective items.
- Approve when concerns are addressed; don't nitpick style if linting passes.