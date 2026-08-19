# memory.md — Capture Durable Knowledge

A reproducible procedure for capturing knowledge discovered during research
that should remain available to future agents and sessions.

## What belongs in memory?

Memory is for useful knowledge that does not belong in an `AGENTS.md` contract
or a SPECS/ research file.

Good candidates include:

- **Gotchas** — unexpected behavior or non-obvious requirements in research tools or methodologies.
- **Tried and rejected** — research approaches that were tested and intentionally abandoned, including why.
- **Open questions** — unresolved issues worth revisiting.
- **Operational knowledge** — useful commands, procedures, or environment details for research.
- **Implementation discoveries** — facts learned while working with research tools that are useful to future work.
- **Hypothesis updates** — changes to confidence levels in research hypotheses based on new evidence.

Do not use memory for:

- Architecture or research area boundaries (belongs in AGENTS.md).
- Ownership or dependencies between research areas (belongs in AGENTS.md).
- Coding rules or project requirements (belongs in standards/).
- Stable constraints that should be enforced (belongs in AGENTS.md).
- Temporary task state.
- Information that belongs in SPECS/ research files.

If the information should constrain how future research is conducted, it probably
belongs in `AGENTS.md` or SPECS/, not memory.

## Steps

1. **Identify the scope.**

   Determine which research area the knowledge applies to.

   Memory should live at the narrowest useful scope:

   ```text
   SPECS/
   └── compression/
       ├── AGENTS.md
       ├── integer-compression.md
       └── .acc-memory.md
   ```

2. **Write the entry.**

   Use `acc memory add <dir> "<text>"` or edit `.acc-memory.md` directly.

   Format:
   ```markdown
   ## YYYY-MM-DDTHH:MM:SSZ

   Category: <gotcha|rejected|question|operational|discovery|hypothesis>
   Context: <which research area, which files>
   Detail: <the knowledge>
   ```

3. **Reference the memory** in relevant SPECS/ files or AGENTS.md
   with a comment: `<!-- see .acc-memory.md YYYY-MM-DDTHH:MM:SSZ -->`

## Research-Specific Memory Categories

- **Hypothesis Update**: When new evidence changes confidence in a research hypothesis.
- **Methodology Note**: When a research method is found to be more/less effective.
- **Tooling Gotcha**: When a research tool has a non-obvious behavior or limitation.
- **Source Verification**: When a cited source is found to be outdated or incorrect.
- **Comparison Update**: When a comparison table needs updating based on new data.