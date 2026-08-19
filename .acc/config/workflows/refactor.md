# refactor.md — Refactor a Research Area

A reproducible procedure for refactoring a research area in the Prime
research repository.

## Steps

1. **Understand the current research structure.**
   - `acc graph --format mermaid` — visualize current state.
   - `acc context <dir> --depth 2` — deep context including dependent research areas.
   - Read `AGENTS.md` and `.acc-memory.md` for the research area and its dependents.

2. **Define the target research structure.** Document the desired end state
   in a design doc or directly in the target `AGENTS.md` (draft).

3. **Check constraints.**
   - `acc impact <dir>` — full blast radius across research areas.
   - `acc check` — ensure no pre-existing violations.
   - Verify no forbidden dependencies will be introduced.
   - Verify SPECS/ and docs/ layer separation maintained.

4. **Plan the migration.** Break into small, verifiable steps:
   - Each step should pass `acc check`.
   - Each step should maintain working research validation.
   - Use `acc discover` to find undeclared dependencies that need handling.

5. **Execute incrementally.** For each step:
   - Make the change.
   - Run validation tests.
   - `acc check`.
   - Update `AGENTS.md` if research structure changed.
   - Update SPECS/ files if research content moved/changed.
   - Commit.

6. **Final validation.**
   - `acc check` — clean.
   - `acc graph` — matches target research structure.
   - Full test suite passes (lint, validation, dogfood).

7. **Update memory.** Record decisions, trade-offs, and migration notes:
   - `acc memory add <dir> "Refactored <X> to <Y>. <Rationale>. <Trade-offs>."`

## Notes

- Prefer small, reversible steps over big bang rewrites.
- Keep `AGENTS.md` in sync with reality throughout.
- Keep SPECS/ files accurate — they are the research output.
- Consider feature flags for risky structural changes.
- Research hypotheses in affected areas must be re-evaluated after refactor.