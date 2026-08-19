# feature.md — Add a New Research Area

A reproducible procedure for adding a new research area to the Prime
research repository.

## Who owns the ACC files?

- **Engine ON** — an always-on AI engine (`acc engine --watch`) is
  maintaining the ACC files. You do NOT need to run the ACC steps
  below; the engine reviews your changed code, updates knowledge, and
  reports drift in `ACC_WARN.md`. Just implement the research and
  check `ACC_WARN.md` before finishing.
- **Engine OFF** — you are exclusively responsible. Follow the steps
  below in every task; nobody else will keep the ACC files in sync.

## Steps

1. **Identify the research area.** Determine which SPECS/ directory
   will own the new research. A research area is a directory
   containing an `AGENTS.md` (e.g., `SPECS/compression/`, `SPECS/languages/`).

2. **Read the parent context.** Read the nearest ancestor `AGENTS.md`
   (usually `SPECS/AGENTS.md` or root `AGENTS.md`) to understand
   inheritable context (purpose, constraints, dependencies).

3. **Draft the local contract.**
   - Run `acc document <dir>` for a conservative template (stdout).
   - If useful, run `acc document <dir> --from-discovery` to pre-fill
     discovered dependencies and owners. Inferred fields are marked
     `<!-- inferred -->`; review them before promotion.
   - Review, edit, and commit `<dir>/AGENTS.md`. Inferred entries become
     declared only once they survive human review.

4. **Conduct the research.**
   - Follow the research principles in `init-promt.md` and `.acc/config/standards/architecture.md`.
   - Evidence over assumptions: cite primary sources.
   - Primary sources first: official docs, papers, specs, source code.
   - Do not prematurely converge: investigate alternatives.
   - Research alternatives fairly: multiple approaches per design area.
   - Distinguish clearly: FACT, OBSERVATION, HYPOTHESIS, INFERENCE, OPEN QUESTION.

5. **Update SPECS/ files.**
   - Add detailed research findings to appropriate files in the research area.
   - Update `SPECS/findings/` with key findings, technical findings, open problems, research gaps.
   - Update `SPECS/references/` with new papers, repositories, specifications.
   - Update `SPECS/benchmarks/` if new benchmarks or datasets are relevant.

6. **Validate.**
   - `acc check` — broken references, forbidden deps, duplicate ownership, stale docs.
   - `acc graph` — confirm relationships match intent.
   - `acc impact <dir>` — identify affected research areas.
   - Fix any `error`-level diagnostics before merging.

7. **Update memory.** If you learned something durable that isn't
   architectural (gotchas, tried-and-rejected, open questions), append
   to `<dir>/.acc-memory.md`:
   - `acc memory add <dir> "<text>"`, or
   - edit `.acc-memory.md` directly — it's plain Markdown.

8. **If interrupted or corrected by the human**, immediately record
   the reason under "Interrupts & Corrections" in `.acc-memory.md`:
   - `acc memory add <dir> "Interrupted because: <reason>. Corrected action: <what to do instead>"`
   - Do not repeat the same mistake in the same session.

## Notes

- `.acc-memory.md` is gitignored. Don't commit it.
- Never declare a dependency or owner based solely on an `acc discover`
  suggestion. Promote `Inferred` → `Declared` deliberately.
- The `acc` CLI is optional for this workflow; the fallback is reading
  `AGENTS.md`, research files, and `.acc-memory.md` directly.
- **No product design**: Document evidence, alternatives, tradeoffs, hypotheses, unresolved questions.