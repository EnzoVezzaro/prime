---
title: Skills
---

# Skills

Prime integrates with agents via skills — portable capability packages that teach any agent how to use Prime.

## What is a Skill?

A skill is a self-contained package that:
- Declares the tool interface (MCP tools)
- Provides usage guidance for the agent
- Includes examples and best practices
- Specifies when to use each tool

Skills are agent-agnostic — the same skill works with Claude, Cursor, Codex, OpenCode, Gemini, and custom agents.

## Prime Skill Structure

```
skills/prime/
├── SKILL.md          # Skill manifest (tool declarations, guidance)
├── examples/         # Usage examples
│   ├── context.md
│   ├── search.md
│   ├── impact.md
│   └── architecture.md
├── prompts/          # Agent prompt templates
│   ├── system.md
│   └── tool-use.md
└── references/       # Reference cards
    ├── envelope.md
    ├── confidence.md
    └── escalation.md
```

## SKILL.md Manifest

```markdown
# Prime Skill

## Tools
- prime_search
- prime_lookup
- prime_context
- prime_relationships
- prime_dependencies
- prime_impact
- prime_architecture

## When to Use Prime
- Before reading any source file
- When exploring unfamiliar codebases
- For impact analysis before changes
- For architectural understanding

## Tool Selection Guide
| Question | Tool |
|----------|------|
| "Find X" | prime_search |
| "What is X?" | prime_lookup |
| "What calls X?" | prime_context |
| "What depends on X?" | prime_dependencies |
| "What breaks if I change X?" | prime_impact |
| "Show me the architecture" | prime_architecture |

## Envelope Handling
- Check `status` first
- If `source_required: true` → fetch source per `escalation_hints`
- Trust `confidence: exact` without verification
- Verify `confidence: inferred` in source

## Token Budgeting
- Default: 16384 tokens
- Reduce for simple queries
- Increase for architecture/impact
```

## Agent Integration

### Via MCP (Recommended)
```bash
# Install MCP server
cargo install prime-mcp

# Configure agent
# .cursor/mcp.json or equivalent
{
  "mcpServers": {
    "prime": {
      "command": "prime-mcp",
      "args": ["--storage", "/path/to/artifact"]
    }
  }
}
```

### Via CLI (Fallback)
```bash
# Direct CLI calls
prime query "AuthService.login" --type context --format json
```

## Best Practices for Agents

1. **Always start with Prime** — Before reading source, ask Prime
2. **Use the right tool** — Match question to tool
3. **Respect the envelope** — Check status, coverage, source_required
4. **Trust exact** — Don't re-verify exact confidence
5. **Verify inferred** — Fetch source for inferred facts
6. **Cache results** — Reuse within session
7. **Progressive disclosure** — Start shallow, go deeper

## Skill Distribution

- **GitHub**: skills/prime/ in Prime repo
- **npm**: `@prime/skill` package
- **MCP Registry**: Published to MCP registry
- **Agent marketplaces**: Cursor, Claude, etc.

## Next

- [MCP](./mcp.md)
- [Agent Tools](./agent-tools.md)
- [Structured Results](./structured-results.md)