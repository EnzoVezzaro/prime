# Ground Truth Benchmark for Retrieval Precision

## Repos
bat, httpx, express, gin, spdlog

## Query Template
| Query | Intent | Repo | Expected Entities | Notes |

## httpx examples
| AuthService.login | Lookup | httpx | AuthService.login method, AuthService class | Exact qualified name |
| who calls AuthService.login | Callers | httpx | UserController.authenticate, SessionManager.refresh | Call graph |
| AuthService | Search | httpx | AuthService class, AuthService.login, AuthService.logout | Top 5 |

## bat examples
| parse_file | Lookup | bat | bat::parser::parse_file | |
| who calls parse_file | Callers | bat | ... | |

## Metrics to compute
precision@5, recall@5, nDCG@5, MRR, candidate/result ratio, evidence density

## Status
Template created, needs manual verification per repo.
