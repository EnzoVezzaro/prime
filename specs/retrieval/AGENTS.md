# retrieval

## Purpose

Research on agent-oriented retrieval patterns, context selection, and token efficiency.

## Responsibilities

- Research information retrieval (inverted indexes, lexical/semantic/hybrid search, ranking, filtering)
- Research agent retrieval patterns (progressive disclosure, token efficiency, minimal sufficient context)
- Research context selection strategies (minimal, progressive, surrounding, importance-based)
- Research token efficiency (information density, structured vs text representation)

## Ownership

Owner: research team

## Inputs

- Academic papers on information retrieval, vector search, semantic search
- Agent architecture research (context windows, tool calls, progressive disclosure)
- Attention/transformer research (context caching, long-context degradation)
- Current agent systems (Cursor, Aider, OpenHands, SWE-agent, Continue)

## Outputs

- SPECS/retrieval/information-retrieval.md
- SPECS/retrieval/agent-retrieval.md
- SPECS/retrieval/context-selection.md
- SPECS/retrieval/token-efficiency.md

## Dependencies

- SPECS/indexing/ (search indexes, vector search)
- SPECS/compression/ (token efficiency via compression)
- SPECS/systems/ (context windows, attention research)

## Constraints

- Central research question: "What is the minimum information an agent needs to understand a code entity?"
- Token efficiency: maximize usefulness per agent context token
- Progressive disclosure: start minimal, expand as agent asks follow-ups
- Structured vs text representation tradeoffs

## Architecture

Four research files covering IR foundations, agent-specific retrieval, context selection, and token efficiency.

## Workflows

- See `.acc/config/workflows/research.md` for conducting retrieval research.
- See `.acc/config/workflows/feature.md` for adding a new retrieval pattern.