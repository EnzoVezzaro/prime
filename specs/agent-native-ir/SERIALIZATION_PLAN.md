# Multiple Representations & Serialization Plan

## Views

### 1. Source representation
Existing. Tree-sitter AST + source text. Used for projection.

### 2. Structural representation
Symbol tree JSON:
{
  "module": "...",
  "entities": [{"id", "kind", "name", "qualified_name", "children"}],
  "relations": [{"from","to","kind"}]
}

### 3. Semantic graph representation
Extended KnowledgeGraph with semantic metadata:
- EntitySemantic extension
- Effect nodes
- Capability nodes
- Evidence links
Stored as JSON for debugging, binary for storage.

### 4. Compact agent-facing serialization
Goal: maximize semantic density per token.

Candidates to evaluate:
A. Minimal JSON
{"e":42,"k":"fn","n":"login","i":["User"],"o":["Token"],"ef":["IO"]}

B. Line-oriented
E 42 fn login i User o Token ef IO calls 43 reads db.users

C. S-expression
(entity 42 fn login (inputs User) (outputs Token) (effects IO) ...)

D. Custom compact semantic notation
fn#42 login<User->Token> [IO] -> 43

Measure tokens using model's tokenizer.

Recommend starting with line-oriented + field abbreviation.

### 5. Binary/canonical
Canonical format requirements:
- Deterministic node ordering by ID
- Sorted fields alphabetically
- No whitespace
- Content-addressed hash

Current prime-index uses zstd/lz4 + FlatBuffers/bincode. Keep for storage.

Add semantic IR binary format: FlatBuffers schema:
table SemanticEntity { base: Entity; inputs: [TypeRef]; outputs: [TypeRef]; effects: [Effect]; evidence: [EvidenceRef]; }

Canonical serialization for hashing.

Do NOT feed raw binary to LLM.

## Token efficiency measurement
Metrics:
- tokens per repository fact
- tokens per dependency
- tokens per semantic relationship
- tokens per task

Tools needed:
- Token counter using same tokenizer as Prime's target model
- Script to generate each serialization variant for sample repo
- Compare semantic density

## Implementation steps
1. Define SemanticEntity struct extending Entity
2. Add serialization module prime-index::semantic_serialization
3. Implement line-oriented serializer
4. Implement binary FlatBuffers schema
5. Add CLI export --format semantic|agent|binary
6. Benchmark token counts
