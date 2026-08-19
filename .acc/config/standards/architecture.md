# architecture.md — Prime Research Project Architecture Standard

This standard is referenced by `AGENTS.md` files across the Prime research repo.
It defines the project's architecture expectations and the rules that
govern how research areas relate.

## Hard Invariant

```text
ACC-enhanced  =  Repository  +  AGENTS.md  +  .acc/
```

Removing `.acc/` or the `acc` CLI MUST leave a valid `AGENTS.md`
repository. This invariant is load-bearing for every design decision.

## Research Repository Structure

The Prime research repository has two distinct layers:

```
Prime/
├── SPECS/           # Research specifications (evidence-based, not product)
│   ├── README.md
│   ├── RESEARCH.md
│   ├── findings/    # Executive summary, key findings, technical findings, open problems, research gaps
│   ├── prior-art/   # SCIP, LSIF, CPG/Joern, Tree-sitter, Graph-sitter, agent indexers, comparison
│   ├── code-analysis/
│   ├── storage/
│   ├── compression/
│   ├── indexing/
│   ├── retrieval/
│   ├── systems/
│   ├── languages/
│   ├── incremental/
│   ├── reusable-tools/
│   ├── benchmarks/
│   └── references/
└── docs/            # Operational documentation
    ├── README.md
    ├── GETTING-STARTED.md
    ├── GUIDELINES.md
    ├── CONTRIBUTING.md
    └── SPECS-GUIDE.md
```

These are separate layers:
- **SPECS/** — Research specifications: evidence-based, hypotheses, open questions, NO product design
- **docs/** — Operational documentation: how to use this research, guidelines, contributing

## Truth Categorization (from init-promt.md)

| Kind | Authority | Source |
|------|-----------|--------|
| FACT | Authoritative | Verified evidence, official sources, benchmarks |
| OBSERVATION | High | Direct inspection, reproducible measurements |
| HYPOTHESIS | None | Research proposal, requires validation |
| INFERENCE | Low | Deduced from evidence, marked as such |
| OPEN QUESTION | None | Explicitly unknown, needs research |

FACT wins over OBSERVATION when they disagree; the disagreement becomes a diagnostic.
HYPOTHESIS and INFERENCE are never asserted as architecture.
OPEN QUESTIONs are explicitly tracked and never resolved without evidence.

See `init-promt.md` for the complete research principles.

## Research vs Product Separation

- **Research (SPECS/)**: Investigating what is technically possible, what exists, what works, what doesn't, why, what should be reused/avoided, what remains unsolved, what architecture evidence supports, what experiments are necessary.
- **Product (future)**: The eventual Prime product will be designed FROM this research.
- **Do NOT decide**: Prime's final graph model, binary format, API, storage engine, compression algorithm, programming language, architecture — unless research explicitly reaches that conclusion.

## Stability Contracts

- `ACC0xx` diagnostic codes: stable forever. No renumbering.
- JSON `schema_version`: breaking changes require a major bump.
- CLI flag names: stable post-1.0; renaming is forbidden.

## Security

- No code execution (no `npm scripts`, `Makefiles`, build scripts).
- No network calls by ACC.
- Symlinks escaping the project root are not followed.
- Paths escaping the project root are refused (ACC080).

## Dependency Rules

- All dependencies between research areas MUST be declared in `AGENTS.md` using canonical paths.
- Discovered but undeclared dependencies surface as `ACC022` warnings.
- Forbidden dependencies are enforced via `.acc/config/config.yaml`.
- Circular dependencies are warned (`ACC014`) but not forbidden.

## Ownership Rules

- Every research area MUST have a declared owner (researcher or team).
- Ownership is exclusive: one owner per research area.
- Unowned dependency targets emit `ACC031` warnings.
- Duplicate ownership emits `ACC030` errors.

## Constraint Rules

- Constraints are declared invariants in `AGENTS.md`.
- Constraints are plain text; ACC surfaces them but does not enforce.
- Constraints apply to the declaring research area and its subtree.
- Constraints are surfaced in `acc context` and `acc impact`.

## Research Principles (from init-promt.md)

1. **Evidence over assumptions**: Never write "X is faster" without evidence.
2. **Primary sources first**: Prefer official documentation, academic papers, technical specifications, original repositories, engineering papers, benchmark results, source code.
3. **Do not prematurely converge**: Do not decide on technologies until research establishes why.
4. **Research alternatives fairly**: For every major design area, investigate multiple approaches.
5. **No product design yet**: Document evidence, alternatives, tradeoffs, hypotheses, unresolved questions.