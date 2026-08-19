Dependency analysis research:

- Dependency graph: Directed graph representing dependencies between code entities. Nodes can be files, modules, classes, functions; edges represent "uses/requires" relationships.

- Build-time dependencies: Dependencies needed for compilation:
  - Import statements, include directives, module declarations
  - Usefulness: High (determines build order, impact of changes)

- Runtime dependencies: Dependencies resolved during program execution:
  - Dynamic module loading, service discovery, configuration-driven
  - Usefulness: Medium (varies by runtime platform)

- Dependency direction: "A depends on B" means A requires B:
  - Edge direction: A → B
  - Usefulness: Standard convention for "depends on" direction

- Package dependencies: Dependencies at the package/library level:
  - Listed in manifest files (package.json, requirements.txt, Cargo.toml, pom.xml, build.gradle)
  - Usefulness: High (understanding library usage, version compatibility)

- Transitive dependencies: Dependencies of dependencies:
  - Second-order, third-order dependencies
  - Usefulness: High (full dependency tree reveals true scope)

- Dependency cycle: Circular dependency between entities:
  - Common in some architectures, problematic in others
  - Usefulness: Detection aids refactoring, understanding cyclic structures

- Soft vs hard dependencies:
  - Hard: Required for operation (missing → error/failure)
  - Soft: Optional, feature-gated, graceful degradation
  - Usefulness: Distinguishing aids impact analysis

- Dependency invalidation: When does a dependency become stale/broken:
  - Version range updates change compatibility
  - API removals or changes
  - Platform deprecations
  - Usefulness: High (dependency management, update planning)

- Dependency mapping: Visualization and analysis of dependency structure:
  - Dependency matrices, heat maps, tree maps
  - Usefulness: Identifying hotspots, circular dependencies, unnecessary dependencies

- Dependency reason: Why a dependency exists:
  - Required feature
  - Historical accident
  - Transitive pull (indirect dependency)
  - Usefulness: Understanding if dependency can be removed/replaced

- Dependency age: How long a dependency has been in use:
  - Usefulness: Old dependencies may be stable but have accumulated technical debt

- Dependency maintenance status: Active, inactive, deprecated, unmaintained:
  - Usefulness: High (risk assessment for dependency upgrades)

- Replacement feasibility: How easy it is to replace a dependency:
  - API compatibility, feature parity, learning curve
  - Usefulness: Informed decision-making on dependency management

- Dependency update strategy:
  - Semantic versioning compliance
  - Regular update schedule
  - Update testing pipeline
  - Usefulness: Preventing bit rot, security vulnerabilities

- Reusable components: Dependency analysis infrastructure:
  - Parsing manifest files for multiple formats
  - Transitive closure computation
  - Cycle detection algorithms
  - Compatibility checking against version ranges
  - Impact analysis when dependencies change