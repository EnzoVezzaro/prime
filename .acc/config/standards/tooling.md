# tooling.md — Prime Research Tooling Standards

This standard defines tooling conventions for the Prime research project.

## Core Principles

1. **Evidence-based** — Tools must produce verifiable, reproducible results.
2. **Research-aware** — Understands SPECS/ structure and research terminology.
3. **Deterministic** — Same input → same output (critical for research reproducibility).
3. **Permissioned** — Explicit permission model (safe/moderate/dangerous).
4. **Extensible** — Plugin interface for research-specific tooling.

## Tool Detection Standards

### Research Project Detection

| File | Detected Capability |
|------|---------------------|
| `SPECS/` | Research specification directory |
| `init-promt.md` | Source research prompt |
| `.acc/` | ACC configuration |
| `AGENTS.md` | Project/research area contracts |
| `specs/` | Research specifications (legacy/alternative) |

### Language Projects (for analyzed codebases)

| File | Detected Capability |
|------|---------------------|
| `package.json` | Project type, scripts, dependencies |
| `Cargo.toml` | Project type, dependencies, targets |
| `go.mod` | Project type, dependencies |
| `pyproject.toml` | Project type, dependencies, tools config |
| `pom.xml` | Maven project, dependencies |

## Permission Defaults

### Safe (Always Enabled)
- `filesystem.read`, `filesystem.glob`, `filesystem.stat`
- `search.contracts`, `search.edges`, `search.code`
- `git.read` (status, log, diff, show)
- `graph`, `context`, `memory.read`
- `project.detect`

### Moderate (Configurable, Default: Auto-Approve)
- `filesystem.write`
- `shell.enabled` with `approval: "auto"`
- `git.write` (commit, add, branch)
- `tool.test`, `tool.lint`, `tool.typecheck`, `tool.build`, `tool.format`, `tool.audit`

### Dangerous (Default: Denied)
- `filesystem.delete`
- `network.enabled`
- `tool.install` (package installation)
- `git.push`
- `deploy`

## Research Tool Standards

### Research Tool Structure
```
tools/<name>/
├── tool.yaml          # Manifest (required)
├── index.py|rs|go|js  # Implementation (required)
├── README.md          # Documentation
├── test/              # Tool tests
└── validation/        # Research validation scripts
```

### Tool Manifest (`tool.yaml`)
```yaml
name: <unique-name>
version: "1.0.0"
description: "One-line description"
author: "Name <email>"
license: "MIT"

research_area: <speaks-area>  # e.g., "compression", "indexing", "retrieval"

capabilities:
  - name: <capability-name>
    description: "What this does for research"
    command: "<command-template>"
    permissions: ["filesystem.read", "shell"]  # subset of ACC permissions

inputs:
  - name: <input-name>
    type: <type>
    description: "What this tool needs"

outputs:
  - name: <output-name>
    type: <type>
    description: "What this tool produces"

validation:
  - type: "evidence_verification"
    description: "Verify cited sources support claims"
  - type: "comparison_accuracy"
    description: "Verify comparison tables are accurate"
```

### Tool Implementation
- MUST accept JSON input via stdin
- MUST emit JSON output via stdout
- MUST exit 0 on success, non-zero on failure
- MUST honor ACC project root boundary
- MUST NOT make network calls unless `network` permission granted
- MUST be deterministic (same input → same output)

## Tool Execution Standards

### `acc tool <name>` Contract
- Input: `--args` (array), `--scope` (path), `--json` flag
- Output: Structured JSON with `exit_code`, `stdout`, `stderr`, `duration_ms`
- Exit code: Tool's exit code, `2` = tool not found, `1` = permission denied

### `acc shell` Contract
- Input: Command string, `--cwd`, `--timeout`, `--env`, `--json`
- Output: Structured JSON with `exit_code`, `stdout`, `stderr`, `duration_ms`
- Sandbox: Restricted environment, project root boundary, resource limits

## Configuration Standards

### Minimal Config
```yaml
tools:
  auto_discover: true
```

### Research Recommended Config
```yaml
tools:
  auto_discover: true
  defaults:
    filesystem: true
    search: true
    shell: true
    git: true
    project: true
    context: true
    graph: true
    memory: true
    check: true
  detected:
    enabled: true
  plugins:
    enabled: true
  permissions:
    filesystem:
      read: true
      write: true
      glob: true
    shell:
      enabled: true
      approval: "auto"
    git:
      read: true
      write: true
    network:
      enabled: false
```

## Security Standards

1. **No auto-install** — ACC never installs tools without explicit user action.
2. **Path boundary** — All filesystem operations bounded to project root.
3. **No secrets** — Shell environment stripped of secrets by default.
4. **Resource limits** — Timeout, memory, CPU limits enforced.
5. **Audit trail** — All tool invocations logged with provenance.

## Research Tool Distribution

| Agent Role | Tool Set |
|------------|----------|
| Researcher | Full graph, full context, validation, search, analysis tools |
| Validator | Scoped context, validation tools, comparison tools, evidence verification |
| Architect | Full graph, full context, architecture analysis, dependency tools |

Workers receive `acc tools --json --scope <path>` capability manifest.