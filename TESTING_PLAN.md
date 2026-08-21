# Prime Heavy Testing Plan

**Created:** August 21, 2026
**Purpose:** Comprehensive testing of Prime implementation, benchmarking, and validation

## Result File Convention

Every benchmark run produces TWO files in `benchmarks/results/`:

1. **`latest.json`** — always overwritten with the most recent result
2. **`result_<timestamp>.json`** — timestamped copy for historical tracking

The timestamp format is `YYYYMMDDTHHMMSS` (e.g., `result_20260821T204449.json`).

After each testing phase, commit the timestamped results for historical tracking.

---

## Phase 1: Fix Critical Bugs (Priority: HIGH)

### 1.1 Fix JS Entity Extraction
**Problem:** express (JS) shows 0 entities despite having 182 files
**Root cause:** Tree-sitter query compilation errors in definition queries
**Action:**
1. Verify all definition queries compile without errors against their grammars
2. Test JS extraction on express repo directly
3. Run full benchmark to verify express now shows entities

**Success criteria:** express shows >100 entities

### 1.2 Fix "Impossible Pattern" Query Error
**Problem:** Persistent "Query error at 3:20. Impossible pattern:" error during build
**Root cause:** Unknown — persists even with minimal queries
**Action:**
1. Add debug logging to print the exact query string being compiled
2. Identify which language produces the error
3. Fix or suppress the error

**Success criteria:** Build completes without query errors

### 1.3 Fix Relationship F1 for All Repos
**Problem:** Only bat and httpx show non-zero Relationship F1 (0.095, 0.182)
**Root cause:** Benchmark evaluation doesn't properly extract relationships from Context tool
**Action:**
1. Trace the benchmark evaluation flow for gin and spdlog
2. Verify Context tool returns callers/callees for these repos
3. Fix relationship extraction in benchmark evaluation

**Success criteria:** All 5 repos show non-zero Relationship F1

---

## Phase 2: Improve Benchmark Quality (Priority: HIGH)

### 2.1 Create Repo-Specific Benchmark Questions
**Problem:** Current questions are generic ("main", "call invoke") and don't match repo content
**Action:**
1. Analyze each benchmark repo (bat, httpx, express, gin, spdlog)
2. Create 10-15 questions per repo with verified ground truth answers
3. Questions should cover: symbols, calls, imports, exports, dependencies, architecture
4. Each answer must include the exact qualified name or relationship

**Success criteria:** 50+ questions with verified ground truth

### 2.2 Fix Benchmark Evaluation Logic
**Problem:** entity_recall evaluation counts empty results as correct
**Action:**
1. Fix entity_recall: should only be correct if expected_entities is non-empty AND entities are found
2. Fix relationship_recall: should properly extract relationships from Context/Dependencies tools
3. Add debugging output to show what the benchmark actually finds vs. expects

**Success criteria:** Benchmark accuracy reflects actual extraction quality

### 2.3 Add Missing Metrics
**Problem:** Bytes retrieved, tokens exposed, source reads, tool calls not measured
**Action:**
1. Add byte count to search/context responses
2. Estimate token count from response size
3. Count source file reads during query
4. Count MCP tool calls per query

**Success criteria:** All spec metrics are measured

---

## Phase 3: Relationship Extraction Testing (Priority: HIGH)

### 3.1 Test Each Relationship Type
**Action:**
For each of the 8 relationship types, test extraction on a known codebase:

| Relationship | Test Repo | Expected Count | Test Method |
|--------------|-----------|----------------|-------------|
| References | bat | >50,000 | Search + verify sample |
| Calls | bat | >5,000 | Check call graph |
| FlowsTo | bat | >500 | Check return values |
| Exports | bat | >100 | Check pub items |
| Overrides | bat | >50 | Check method overrides |
| Instantiates | express | >100 | Check new/create calls |
| Imports | bat | >50 | Check use statements |
| TypeOf | bat | >5 | Check type annotations |

**Success criteria:** Each type extracts plausible relationships

### 3.2 Test Relationship Accuracy
**Action:**
1. For bat: manually verify 10 Calls relationships
2. For httpx: manually verify 10 Imports relationships
3. For express: manually verify 10 Exports relationships
4. Compare extracted relationships against source code

**Success criteria:** >80% of verified relationships are correct

### 3.3 Test Relationship Coverage
**Action:**
1. Pick 10 well-known functions across repos
2. For each function, check if Prime captures:
   - Who calls it (callers)
   - What it calls (callees)
   - What depends on it (dependents)
   - What it depends on (dependencies)
3. Compare against manual analysis

**Success criteria:** >60% coverage of known relationships

---

## Phase 4: Language Support Testing (Priority: MEDIUM)

### 4.1 Test Each Language Individually
**Action:**
For each of the 8 supported languages, create a test file and verify extraction:

**Rust:**
```rust
pub fn main() { let x = foo(); }
fn foo() -> u32 { 42 }
struct Config { name: String }
trait Drawable { fn draw(&self); }
enum Color { Red, Green, Blue }
```
Expected: 5+ entities, Calls, Exports, TypeOf

**Python:**
```python
def main(): x = foo()
def foo(): return 42
class Config: pass
```
Expected: 3+ entities, Calls

**JavaScript:**
```javascript
function main() { const x = foo(); }
function foo() { return 42; }
class Config {}
```
Expected: 3+ entities, Calls

**Go:**
```go
func main() { x := Foo() }
func Foo() int { return 42 }
type Config struct { Name string }
```
Expected: 3+ entities, Calls

**Java:**
```java
public class Main { public static void main(String[] args) { foo(); } }
static int foo() { return 42; }
class Config {}
```
Expected: 3+ entities, Calls

**TypeScript:**
```typescript
function main() { const x = foo(); }
function foo(): number { return 42; }
class Config {}
interface Drawable { draw(): void; }
```
Expected: 4+ entities, Calls

**C:**
```c
int main() { int x = foo(); return 0; }
int foo() { return 42; }
struct Config { char name[32]; };
```
Expected: 3+ entities, Calls

**C++:**
```cpp
int main() { int x = foo(); return 0; }
int foo() { return 42; }
class Config { public: std::string name; };
```
Expected: 3+ entities, Calls

**Success criteria:** All 8 languages extract entities correctly

### 4.2 Test Cross-Language Relationships
**Action:**
1. Create a test project with multiple languages
2. Verify that relationships across files/languages work
3. Test import resolution across modules

**Success criteria:** Cross-file relationships are extracted

---

## Phase 5: Performance Testing (Priority: MEDIUM)

### 5.1 Benchmark Derivation Speed
**Action:**
Test derivation on repos of increasing size:

| Repo | Files | LOC | Target Time |
|------|-------|-----|-------------|
| httpx | 98 | 23K | <0.5s |
| bat | 369 | 52K | <1s |
| express | 182 | 28K | <0.5s |
| gin | 111 | 23K | <0.5s |
| spdlog | 155 | 31K | <0.5s |

**Success criteria:** All repos derive in <2s

### 5.2 Benchmark Query Latency
**Action:**
Measure cold/warm query latency across different query types:

| Query Type | Cold Target | Warm Target |
|------------|-------------|-------------|
| Search | <500µs | <100µs |
| Lookup | <100µs | <50µs |
| Context | <100µs | <50µs |
| Dependencies | <100µs | <50µs |

**Success criteria:** All queries meet latency targets

### 5.3 Benchmark Incremental Updates
**Action:**
Test incremental update with varying change sizes:

| Change Size | Files Modified | Target Time |
|-------------|----------------|-------------|
| 1 file | 1 | <100ms |
| 10 files | 10 | <500ms |
| 1% | ~260 | <5s |
| 10% | ~2,600 | <30s |

**Success criteria:** Incremental updates scale linearly

---

## Phase 6: Agent Evaluation (Priority: LOW)

### 6.1 Test MCP Tools
**Action:**
Test each of the 7 MCP tools with real queries:

1. `prime_search` — search for "AuthService"
2. `prime_lookup` — lookup exact qualified name
3. `prime_context` — get context neighborhood
4. `prime_relationships` — get relationships across dimensions
5. `prime_dependencies` — get dependency graph
6. `prime_impact` — analyze impact of changes
7. `prime_architecture` — get architecture overview

**Success criteria:** All tools return valid PrimeEnvelope responses

### 6.2 Test Agent Workflows
**Action:**
Simulate agent queries and measure quality:

1. "Where is the authentication logic?" → search for auth
2. "What does this function call?" → context on function
3. "What would break if I change this?" → impact analysis
4. "What are the dependencies?" → dependency graph

**Success criteria:** Agent can answer questions using Prime alone

---

## Phase 7: Storage Testing (Priority: LOW)

### 7.1 Test Storage Formats
**Action:**
1. Build index with binary storage
2. Load index and verify data integrity
3. Test mmap vs. non-mmap access
4. Test compression ratios

**Success criteria:** Data integrity maintained across formats

### 7.2 Test Artifact Size
**Action:**
Measure artifact size for each repo:

| Repo | Source Size | Artifact Size | Ratio |
|------|-------------|---------------|-------|
| httpx | 1.2MB | <2MB | <2x |
| bat | 2.6MB | <5MB | <2x |
| express | 1.4MB | <3MB | <2x |

**Success criteria:** Artifact size <2x source size

---

## Testing Checklist

Before claiming "implementation complete", verify:

- [ ] All 8 languages extract entities without query errors
- [ ] All 8 relationship types extract plausible results
- [ ] Benchmark accuracy >10% for at least 3 repos
- [ ] Relationship F1 >0.30 for at least 2 repos
- [ ] All MCP tools return valid responses
- [ ] Derivation speed <2s for all benchmark repos
- [ ] Query latency <500µs cold, <100µs warm
- [ ] Incremental updates work for 1-100 files
- [ ] Artifact size <2x source size
- [ ] No panics or crashes during normal operation
