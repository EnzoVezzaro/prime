# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project structure with Rust workspace (6 crates)
- Tree-sitter based multi-language parser (8 languages)
- Compact binary storage with zstd/lz4 compression
- mmap-based zero-copy access
- Agent-optimized query API with progressive context
- CLI with 15+ commands (build, query, stats, check, deps, etc.)
- ACC skill integration with 4 agent profiles
- Comprehensive SPECS/ research specification (68 files)
- Security policy and contributing guidelines

### Changed
- N/A

### Fixed
- N/A

## [0.1.0] - 2024-08-19

### Added
- Initial Prime research project structure
- SPECS/ research specification with 68 markdown files
- docs/ operational documentation (5 files)
- ACC skill configuration (4 agents, 5 standards, 8 workflows)
- Rust workspace with 6 crates (prime-core, prime-parser, prime-index, prime-query, prime-cli, prime-bench)
- Tree-sitter based multi-language parser (8 languages)
- Compact binary storage with zstd/lz4 compression
- mmap-based zero-copy access
- Agent-optimized query API with progressive context
- CLI with 15+ commands
- Benchmarks for parsing, indexing, queries, storage
- ACC skill integration
- Comprehensive SPECS/ research specification
- Security policy and contributing guidelines
- Package.json with npm-style scripts
- Docker support with docker-compose
- GitHub Actions CI/CD pipeline
- Dependabot configuration

### Changed
- N/A

### Fixed
- N/A

## Template for Future Releases

## [X.Y.Z] - YYYY-MM-DD

### Added
- New features

### Changed
- Changes in existing functionality

### Deprecated
- Soon-to-be removed features

### Removed
- Removed features

### Fixed
- Bug fixes

### Security
- Security fixes