# Retrieval Ground Truth Benchmark

This dataset contains manually verified retrieval queries for Prime.

## Format
JSON array of questions with fields: id, repo, query, intent, expected_entities, acceptable_entities, expected_relationships, expected_evidence, notes, verification_status

## Current status
Initial verified entries for httpx and bat. More entries pending verification.

## Validation
Run validation script to ensure entities exist.
