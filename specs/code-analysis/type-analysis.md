Type analysis research:

- Type systems: The classification of values and expressions into categories with defined operations:
  - Static vs dynamic typing
  - Strong vs weak typing
  - Manifest vs inferred typing
  - Usefulness to agents: Understanding API contracts, enabling compile-time checks

- Type declarations: Explicit type annotations in source code:
  - Function parameter types
  - Return type annotations
  - Variable type declarations
  - Usefulness: High (exact type information, enables precise agent queries)

- Inferred types: Types determined by analysis rather than annotation:
  - Local variable inference (e.g., `var` in JS/C#, `auto` in C++)
  - Type inference in functional languages (Hindley-Milner)
  - Partial type inference in dynamic languages
  - Usefulness: Medium (provides type information where annotations absent)

- Generic types: Parameterized type families:
  - `List<T>`, `Array<T>`, `Option<T>`
  - Type variables, bounds, constraints
  - Usefulness: High (enables reusable component understanding)

- Subtyping: Relationship between types:
  - Nominal subtyping (explicit declaration: `class B extends A`)
  - Structural subtyping (structural compatibility)
  - Usefulness: High (understanding polymorphism, interface satisfaction)

- Union types: Types that can be one of several variants:
  - `string | number`, `None | str`
  - Usefulness: Medium (represents limited type possibilities)

- Intersection types: Types that satisfy multiple type constraints:
  - `A & B` (satisfies both A and B)
  - Usefulness: Lower (less common, language-dependent)

- Optional types: Types representing "value or null":
  - `T?`, `Optional<T>`, `?T`
  - Usefulness: High (critical for null-safety understanding)

- Union vs optional distinction: Important differentiation:
  - Optional: Explicitly represents possible absence
  - Union: Represents any of multiple types (may or may not include null)
  - Prime should distinguish these

- Abstract types/Interfaces: Type contracts without implementation:
  - Interface declarations, abstract classes
  - Method signatures without bodies
  - Usefulness: High (defines API contract, enables polymorphism understanding)

- Type aliases: Alternative names for existing types:
  - `type MyString = string`
  - Usefulness: Medium (readability, not semantic change)

- Enumerated types: Fixed set of named values:
  - `enum Color { Red, Green, Blue }`
  - Usefulness: Medium (represents limited choice set)

- Algebraic data types (ADTs): Sum and product types:
  - Sum types (variants/union): `Either<Left, Right>`
  - Product types (tuples/records): `(Int, String)`
  - Usefulness: High (functional language feature, represents complex data structures)

- Mutable vs immutable types:
  - Mutation markers, const correctness
  - Usefulness: Medium (understanding data flow, side effects)

- Higher-kinded types: Types that take type constructors as parameters:
  - `Functor<F>`, `Monad<M>`
  - Usefulness: Low (advanced functional concept, niche agent need)

- Type confidence/provenance: Same as symbols:
  - Exact (from source annotations)
  - Inferred (from analysis)
  - Unknown (cannot determine)
  - Agent should travel confidence with knowledge

- Erased types: Types removed at runtime (e.g., Java generics, TypeScript):
  - Runtime representation differs from compile-time
  - Usefulness: Low for runtime agents, high for static analysis

- Deprecated type annotations: Types marked as deprecated:
  - Usefulness: Medium (agents should prefer new types)

- Nested types: Types within types:
  - Generic parameters within generic classes
  - Usefulness: Medium (complexity increases nesting depth)

- Bottom type: Type representing "never" or "undefined":
  - `Void`, `Never`, `None`
  - Usefulness: Low (edge case)

- Type system completeness: How thoroughly a language's types cover possibilities:
  - Complete (covers all cases) vs incomplete (may have gaps)
  - Usefulness: High (agent can rely on type coverage for safety guarantees)