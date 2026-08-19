# release.md — Research Milestone Checklist

Stable contract checklist before publishing a research milestone. The
hard rules below MUST pass — they are load-bearing for downstream agents
and consumers of this research.

## Stability (blocking)

- [ ] No `ACC0xx` diagnostic code was renumbered, renamed, or removed.
- [ ] No JSON field was removed or had its type changed without a
      `schema_version` major bump.
- [ ] No CLI flag was renamed or had its meaning changed.
- [ ] Diagnostic code severities were not changed; new `warn_only`
      overrides are per-project, not global.

## Compatibility (blocking)

- [ ] Removing `.acc/` from a test repo leaves a valid `AGENTS.md`
      repository (the hard invariant).
- [ ] No ACC operation executes code, build scripts, or network calls.
- [ ] `acc init` preserves any existing `AGENTS.md` and `.agents/` content.
- [ ] `.acc-memory.md` remains gitignored by `acc init`.

## Determinism (blocking)

- [ ] `acc context <path> --json` produces byte-identical output across
      runs with the same repo state and flags.
- [ ] JSON object keys are sorted; arrays are sorted per spec.
- [ ] No timestamps, random IDs, or locale-dependent formatting in JSON.

## Research Quality (blocking)

- [ ] No AI-generated summaries without clear attribution and labeling.
- [ ] All factual claims have supporting evidence or citation.
- [ ] Primary sources cited (official repos, papers, specs, source code).
- [ ] Claims distinguish: FACT, OBSERVATION, HYPOTHESIS, INFERENCE, OPEN QUESTION.
- [ ] No product design claims without research consensus.
- [ ] No premature technology decisions without research rationale.
- [ ] Alternatives considered and documented for major design areas.
- [ ] Tradeoffs recorded.
- [ ] Limitations identified.
- [ ] Reusable components recorded with licenses.
- [ ] Open questions explicitly listed in SPECS/findings/.

## Dogfooding (blocking)

- [ ] `acc check` on the Prime repo itself passes.
- [ ] `acc graph`, `acc context`, `acc inspect`, `acc impact`,
      `acc dependencies`, `acc dependents`, `acc search`, `acc discover`
      all run on the Prime repo without panicking.
- [ ] `--json` output of every command parses and matches the schema.

## Documentation

- [ ] SPECS/ reflects the current research state.
- [ ] docs/ reflects the current operational guidance.
- [ ] SPECS/findings/ has executive summary, key findings, technical findings, open problems, research gaps.
- [ ] SPECS/references/ has updated papers, repositories, specifications, glossary.
- [ ] SPECS/benchmarks/ has methodology and datasets defined.