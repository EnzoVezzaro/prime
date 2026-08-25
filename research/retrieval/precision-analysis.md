# Precision Analysis

Current problem: entity precision 1-5% with recall 21-100%.

Hypothesis: candidate over-generation + weak ranking.

Need measurements:
Precision@1/3/5/10
Recall@1/3/5/10
MRR
nDCG@10
Candidate recall/precision
Candidate→result ratio
Evidence density

Failure taxonomy:
WRONG_INTENT
WRONG_ENTITY
AMBIGUOUS_ENTITY
MISSING_CANDIDATE
WRONG_RANK
OVER_EXPANSION
RELATIONSHIP_NOISE
EVIDENCE_NOISE
DUPLICATE_RESULT
LANGUAGE_SPECIFIC_FAILURE
