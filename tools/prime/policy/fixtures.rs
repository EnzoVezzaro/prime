//! Agent Task Fixtures for Prime Evaluation

use crate::types::*;
use serde::{Deserialize, Serialize};

/// Standard agent task fixtures for Prime evaluation
pub mod fixtures {
    use super::*;

    /// Get all standard task fixtures
    pub fn all_fixtures() -> Vec<TaskFixture> {
        vec![
            fixture_find_symbol(),
            fixture_find_references(),
            fixture_find_callers(),
            fixture_find_callees(),
            fixture_find_dependencies(),
            fixture_find_dependents(),
            fixture_find_implementations(),
            fixture_find_implementers(),
            fixture_get_context(),
            fixture_get_surrounding(),
            fixture_search(),
            fixture_architecture(),
            fixture_impact_analysis(),
            fixture_test_discovery(),
            fixture_configuration(),
            fixture_cross_language(),
            fixture_large_scale(),
            fixture_incremental(),
            fixture_polyglot(),
            fixture_generated_code(),
        ]
    }

    fn fixture_find_symbol() -> TaskFixture {
        TaskFixture {
            id: "find_symbol_basic".to_string(),
            task_type: TaskType::FindSymbol,
            description: "Find a symbol by exact qualified name".to_string(),
            query: "AuthService.login".to_string(),
            expected_entities: vec!["AuthService.login".to_string()],
            expected_relations: vec!["Calls".to_string(), "References".to_string()],
            expected_confidence: Confidence::Exact,
            repository: "test-project".to_string(),
            tags: vec!["navigation".to_string(), "exact_match".to_string()],
        }
    }

    fn fixture_find_references() -> TaskFixture {
        TaskFixture {
            id: "find_references".to_string(),
            task_type: TaskType::FindReferences,
            description: "Find all references to a symbol".to_string(),
            query: "UserRepository.findByEmail".to_string(),
            expected_entities: vec!["AuthService.login".to_string(), "AuthController.login".to_string()],
            expected_relations: vec!["References".to_string()],
            expected_confidence: Confidence::Exact,
            repository: "test-project".to_string(),
            tags: vec!["relationships".to_string(), "reverse_lookup".to_string()],
        }
    }

    fn fixture_find_callers() -> TaskFixture {
        TaskFixture {
            id: "find_callers".to_string(),
            task_type: TaskType::FindCallers,
            description: "Find all callers of a function".to_string(),
            query: "UserRepository.findByEmail".to_string(),
            expected_entities: vec!["AuthService.login".to_string(), "PasswordResetService.requestReset".to_string()],
            expected_relations: vec!["Calls".to_string()],
            expected_confidence: Confidence::Exact,
            repository: "test-project".to_string(),
            tags: vec!["call_graph".to_string(), "reverse_call_graph".to_string()],
        }
    }

    fn fixture_find_callees() -> TaskFixture {
        TaskFixture {
            id: "find_callees".to_string(),
            task_type: TaskType::FindCallees,
            description: "Find all functions called by a function".to_string(),
            query: "AuthService.login".to_string(),
            expected_entities: vec!["UserRepository.findByEmail".to_string(), "PasswordVerifier.verify".to_string(), "SessionStore.create".to_string()],
            expected_relations: vec!["Calls".to_string()],
            expected_confidence: Confidence::Exact,
            repository: "test-project".to_string(),
            tags: vec!["call_graph".to_string(), "forward_call_graph".to_string()],
        }
    }

    fn fixture_find_dependencies() -> TaskFixture {
        TaskFixture {
            id: "find_dependencies".to_string(),
            task_type: TaskType::FindDependencies,
            description: "Find all dependencies of a module".to_string(),
            query: "AuthService".to_string(),
            expected_entities: vec!["UserRepository".to_string(), "PasswordVerifier".to_string(), "SessionStore".to_string(), "Config".to_string()],
            expected_relations: vec!["DependsOn".to_string(), "Imports".to_string()],
            expected_confidence: Confidence::Exact,
            repository: "test-project".to_string(),
            tags: vec!["dependencies".to_string(), "architecture".to_string()],
        }
    }

    fn fixture_find_dependents() -> TaskFixture {
        TaskFixture {
            id: "find_dependents".to_string(),
            task_type: TaskType::FindDependents,
            description: "Find all modules that depend on a given module".to_string(),
            query: "UserRepository".to_string(),
            expected_entities: vec!["AuthService".to_string(), "UserProfileService".to_string(), "AdminUserService".to_string()],
            expected_relations: vec!["DependsOn".to_string()],
            expected_confidence: Confidence::Exact,
            repository: "test-project".to_string(),
            tags: vec!["dependents".to_string(), "impact_analysis".to_string()],
        }
    }

    fn fixture_find_implementations() -> TaskFixture {
        TaskFixture {
            id: "find_implementations".to_string(),
            task_type: TaskType::FindImplementations,
            description: "Find all implementations of an interface".to_string(),
            query: "UserRepository".to_string(),
            expected_entities: vec!["PostgresUserRepository".to_string(), "InMemoryUserRepository".to_string(), "MockUserRepository".to_string()],
            expected_relations: vec!["Implements".to_string()],
            expected_confidence: Confidence::Exact,
            repository: "test-project".to_string(),
            tags: vec!["implementations".to_string(), "polymorphism".to_string()],
        }
    }

    fn fixture_find_implementers() -> TaskFixture {
        TaskFixture {
            id: "find_implementers".to_string(),
            task_type: TaskType::FindImplementers,
            description: "Find all types implementing a trait".to_string(),
            query: "PaymentProvider".to_string(),
            expected_entities: vec!["StripePaymentProvider".to_string(), "PayPalPaymentProvider".to_string(), "MockPaymentProvider".to_string()],
            expected_relations: vec!["Implements".to_string()],
            expected_confidence: Confidence::Exact,
            repository: "test-project".to_string(),
            tags: vec!["implementers".to_string(), "polymorphism".to_string()],
        }
    }

    fn fixture_get_context() -> TaskFixture {
        TaskFixture {
            id: "get_context_basic".to_string(),
            task_type: TaskType::GetContext,
            description: "Get minimal context for understanding a symbol".to_string(),
            query: "AuthService.login".to_string(),
            expected_entities: vec!["AuthService.login".to_string()],
            expected_relations: vec!["Calls".to_string(), "References".to_string(), "DependsOn".to_string()],
            expected_confidence: Confidence::Exact,
            repository: "test-project".to_string(),
            tags: vec!["context".to_string(), "minimal".to_string()],
        }
    }

    fn fixture_get_surrounding() -> TaskFixture {
        TaskFixture {
            id: "get_surrounding".to_string(),
            task_type: TaskType::GetSurrounding,
            description: "Get surrounding context for a symbol".to_string(),
            query: "AuthService.login".to_string(),
            expected_entities: vec![
                "AuthService.login".to_string(),
                "UserRepository.findByEmail".to_string(),
                "PasswordVerifier.verify".to_string(),
                "SessionStore.create".to_string(),
                "AuthController.login".to_string(),
                "AuthController.register".to_string(),
            ],
            expected_relations: vec!["Calls".to_string(), "References".to_string(), "DependsOn".to_string(), "CalledBy".to_string(), "ReferencedBy".to_string()],
            expected_confidence: Confidence::Exact,
            repository: "test-project".to_string(),
            tags: vec!["surrounding".to_string(), "context".to_string()],
        }
    }

    fn fixture_search() -> TaskFixture {
        TaskFixture {
            id: "search_keyword".to_string(),
            task_type: TaskType::Search,
            description: "Search for symbols by keyword".to_string(),
            query: "authentication".to_string(),
            expected_entities: vec![
                "AuthService".to_string(),
                "AuthController".to_string(),
                "AuthMiddleware".to_string(),
                "AuthenticationService".to_string(),
            ],
            expected_relations: vec![],
            expected_confidence: Confidence::High,
            repository: "test-project".to_string(),
            tags: vec!["search".to_string(), "keyword".to_string()],
        }
    }

    fn fixture_architecture() -> TaskFixture {
        TaskFixture {
            id: "architecture_module".to_string(),
            task_type: TaskType::Architecture,
            description: "Understand module architecture and boundaries".to_string(),
            query: "AuthModule".to_string(),
            expected_entities: vec![
                "AuthModule".to_string(),
                "AuthService".to_string(),
                "AuthController".to_string(),
                "AuthMiddleware".to_string(),
                "AuthService.login".to_string(),
                "AuthController.login".to_string(),
            ],
            expected_relations: vec!["Contains".to_string(), "DependsOn".to_string(), "Imports".to_string()],
            expected_confidence: Confidence::Exact,
            repository: "test-project".to_string(),
            tags: vec!["architecture".to_string(), "module_boundaries".to_string()],
        }
    }

    fn fixture_impact_analysis() -> TaskFixture {
        TaskFixture {
            id: "impact_analysis".to_string(),
            task_type: TaskType::ImpactAnalysis,
            description: "Analyze impact of changing a symbol".to_string(),
            query: "UserRepository.findByEmail".to_string(),
            expected_entities: vec![
                "AuthService.login".to_string(),
                "AuthController.login".to_string(),
                "PasswordResetService.requestReset".to_string(),
                "UserProfileService.getByEmail".to_string(),
            ],
            expected_relations: vec!["Calls".to_string(), "References".to_string()],
            expected_confidence: Confidence::High,
            repository: "test-project".to_string(),
            tags: vec!["impact_analysis".to_string(), "refactoring".to_string()],
        }
    }

    fn fixture_test_discovery() -> TaskFixture {
        TaskFixture {
            id: "test_discovery".to_string(),
            task_type: TaskType::TestDiscovery,
            description: "Find tests for a symbol".to_string(),
            query: "AuthService.login".to_string(),
            expected_entities: vec![
                "AuthService.login_test".to_string(),
                "AuthService.login_invalid_credentials_test".to_string(),
                "AuthService.login_expired_token_test".to_string(),
                "AuthController.login_test".to_string(),
            ],
            expected_relations: vec!["Tests".to_string()],
            expected_confidence: Confidence::High,
            repository: "test-project".to_string(),
            tags: vec!["testing".to_string(), "test_discovery".to_string()],
        }
    }

    fn fixture_configuration() -> TaskFixture {
        TaskFixture {
            id: "configuration".to_string(),
            task_type: TaskType::Configuration,
            description: "Find configuration affecting a symbol".to_string(),
            query: "AuthService.login".to_string(),
            expected_entities: vec![
                "AuthConfig".to_string(),
                "AuthConfig.jwt_secret".to_string(),
                "AuthConfig.token_expiry".to_string(),
                "AuthConfig.max_login_attempts".to_string(),
            ],
            expected_relations: vec!["Configures".to_string()],
            expected_confidence: Confidence::Exact,
            repository: "test-project".to_string(),
            tags: vec!["configuration".to_string(), "config".to_string()],
        }
    }

    fn fixture_cross_language() -> TaskFixture {
        TaskFixture {
            id: "cross_language".to_string(),
            task_type: TaskType::Search,
            description: "Cross-language symbol resolution".to_string(),
            query: "UserService".to_string(),
            expected_entities: vec![
                "UserService".to_string(),
                "user_service.rs".to_string(),
                "user_service.py".to_string(),
                "UserService.java".to_string(),
                "UserService.go".to_string(),
            ],
            expected_relations: vec!["Implements".to_string(), "References".to_string()],
            expected_confidence: Confidence::High,
            repository: "polyglot-project".to_string(),
            tags: vec!["polyglot".to_string(), "cross_language".to_string()],
        }
    }

    fn fixture_large_scale() -> TaskFixture {
        TaskFixture {
            id: "large_scale".to_string(),
            task_type: TaskType::Search,
            description: "Search in large monorepo".to_string(),
            query: "service".to_string(),
            expected_entities: vec![
                "UserService".to_string(),
                "AuthService".to_string(),
                "PaymentService".to_string(),
                "NotificationService".to_string(),
                "OrderService".to_string(),
                "InventoryService".to_string(),
            ],
            expected_relations: vec![],
            expected_confidence: Confidence::High,
            repository: "large-monorepo".to_string(),
            tags: vec!["large_scale".to_string(), "monorepo".to_string()],
        }
    }

    fn fixture_incremental() -> TaskFixture {
        TaskFixture {
            id: "incremental_update".to_string(),
            task_type: TaskType::Search,
            description: "Incremental update after code change".to_string(),
            query: "AuthService.login".to_string(),
            expected_entities: vec!["AuthService.login".to_string()],
            expected_relations: vec!["Calls".to_string(), "References".to_string()],
            expected_confidence: Confidence::Exact,
            repository: "test-project".to_string(),
            tags: vec!["incremental".to_string(), "incremental_update".to_string()],
        }
    }

    fn fixture_polyglot() -> TaskFixture {
        TaskFixture {
            id: "polyglot_repository".to_string(),
            task_type: TaskType::Architecture,
            description: "Understand polyglot repository architecture".to_string(),
            query: "polyglot-project".to_string(),
            expected_entities: vec![
                "frontend/".to_string(),
                "backend/".to_string(),
                "shared/".to_string(),
                "api/".to_string(),
            ],
            expected_relations: vec!["DependsOn".to_string(), "Imports".to_string(), "Exports".to_string()],
            expected_confidence: Confidence::High,
            repository: "polyglot-monorepo".to_string(),
            tags: vec!["polyglot".to_string(), "monorepo".to_string()],
        }
    }

    fn fixture_generated_code() -> TaskFixture {
        TaskFixture {
            id: "generated_code".to_string(),
            task_type: TaskType::Search,
            description: "Handle generated code appropriately".to_string(),
            query: "UserService".to_string(),
            expected_entities: vec!["UserService".to_string(), "UserService.generated.rs".to_string()],
            expected_relations: vec!["GeneratedBy".to_string()],
            expected_confidence: Confidence::Inferred,
            repository: "generated-code-project".to_string(),
            tags: vec!["generated".to_string(), "code_generation".to_string()],
        }
    }
}

/// Task fixture for evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskFixture {
    pub id: String,
    pub task_type: TaskType,
    pub description: String,
    pub query: String,
    pub expected_entities: Vec<String>,
    pub expected_relations: Vec<String>,
    pub expected_confidence: Confidence,
    pub repository: String,
    pub tags: Vec<String>,
}

/// Get fixture by ID
pub fn get_fixture(id: &str) -> Option<TaskFixture> {
    all_fixtures().into_iter().find(|f| f.id == id)
}

/// Get fixtures by task type
pub fn fixtures_by_type(task_type: TaskType) -> Vec<TaskFixture> {
    all_fixtures().into_iter().filter(|f| f.task_type == task_type).collect()
}

/// Get fixtures by tag
pub fn fixtures_by_tag(tag: &str) -> Vec<TaskFixture> {
    all_fixtures().into_iter().filter(|f| f.tags.contains(&tag.to_string())).collect()
}

/// Get fixtures by repository
pub fn fixtures_by_repo(repo: &str) -> Vec<TaskFixture> {
    all_fixtures().into_iter().filter(|f| f.repository == repo).collect()
}

/// Validate fixture against actual Prime results
pub fn validate_fixture(fixture: &TaskFixture, results: &crate::query::QueryResult) -> ValidationResult {
    let mut result = ValidationResult {
        fixture_id: fixture.id.clone(),
        passed: true,
        entity_matches: Vec::new(),
        relation_matches: Vec::new(),
        confidence_match: false,
        issues: Vec::new(),
    };

    // Check entity matches
    for expected in &fixture.expected_entities {
        let found = results.entity.qualified_name.contains(expected);
        result.entity_matches.push(EntityMatch {
            expected: expected.clone(),
            found,
            matched: found,
        });
        if !found {
            result.passed = false;
            result.issues.push(format!("Expected entity '{}' not found", expected));
        }
    }

    // Check relation matches
    for expected in &fixture.expected_relations {
        let found = results.relations.iter().any(|r| r.kind.to_string() == *expected);
        result.relation_matches.push(RelationMatch {
            expected: expected.clone(),
            found,
            matched: found,
        });
        if !found {
            result.passed = false;
            result.issues.push(format!("Expected relation '{}' not found", expected));
        }
    }

    // Check confidence
    result.confidence_match = results.entity.confidence >= fixture.expected_confidence;
    if !result.confidence_match {
        result.passed = false;
        result.issues.push(format!("Confidence {} below expected {}", results.entity.confidence, fixture.expected_confidence));
    }

    result
}

/// Validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub fixture_id: String,
    pub passed: bool,
    pub entity_matches: Vec<EntityMatch>,
    pub relation_matches: Vec<RelationMatch>,
    pub confidence_match: bool,
    pub issues: Vec<String>,
}

/// Entity match result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityMatch {
    pub expected: String,
    pub found: bool,
    pub matched: bool,
}

/// Relation match result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationMatch {
    pub expected: String,
    pub found: bool,
    pub matched: bool,
}

/// Run all fixtures against Prime
pub fn run_all_fixtures(engine: &crate::query::QueryEngine) -> Vec<ValidationResult> {
    let mut validation_results = Vec::new();
    for fixture in all_fixtures() {
        let opts = crate::query::QueryOptions::for_agent();
        let results = match fixture.task_type {
            crate::types::TaskType::FindSymbol => {
                vec![engine.find_by_name(&fixture.query, &crate::query::QueryOptions::for_agent()).first().cloned()]
            }
            crate::types::TaskType::FindReferences => engine.find_by_name(&fixture.query, &crate::query::QueryOptions::for_agent()),
            crate::types::TaskType::FindCallers => {
                if let Some(id) = engine.graph.find_by_qualified(&fixture.query) {
                    engine.callers(entity_id).into_iter().map(|id| engine.graph.entities.get(&id).unwrap().clone()).collect()
                } else { vec![] }
            }
            crate::types::TaskType::FindCallees => {
                if let Some(id) = engine.graph.find_by_qualified(&fixture.query) {
                    engine.callees(entity_id).into_iter().map(|id| engine.graph.entities.get(&id).unwrap().clone()).collect()
                } else { vec![] }
            }
            crate::types::TaskType::FindDependencies => {
                if let Some(id) = engine.graph.find_by_qualified(&fixture.query) {
                    graph.dependencies(entity_id).into_iter().map(|id| graph.entities.get(&id).unwrap().clone()).collect()
                } else { vec![] }
            }
            crate::types::TaskType::FindDependents => {
                if let Some(id) = engine.graph.find_by_qualified(&fixture.query) {
                    graph.dependents(entity_id).into_iter().map(|id| graph.entities.get(&id).unwrap().clone()).collect()
                } else { vec![] }
            }
            crate::types::TaskType::FindImplementations => {
                if let Some(id) = engine.graph.find_by_qualified(&fixture.query) {
                    engine.graph.entities.values().filter(|e| e.kind == SymbolKind::Implements).cloned().collect()
                } else { vec![] }
            }
            crate::types::TaskType::FindImplementers => {
                if let Some(id) = engine.graph.find_by_qualified(&fixture.query) {
                    // Would need implementation
                    vec![]
                } else { vec![] }
            }
            crate::types::TaskType::GetContext => {
                if let Some(id) = engine.graph.find_by_qualified(&fixture.query) {
                    engine.get_context(entity_id, &crate::query::QueryOptions::for_agent()).map(|r| vec![r.entity]).unwrap_or_default()
                } else { vec![] }
            }
            crate::types::TaskType::GetSurrounding => {
                if let Some(id) = engine.graph.find_by_qualified(&fixture.query) {
                    engine.get_surrounding_context(entity_id, &crate::query::QueryOptions::for_agent())
                } else { vec![] }
            }
            crate::types::TaskType::Search => engine.search(&fixture.query, &crate::query::QueryOptions::for_agent()),
            crate::types::TaskType::Architecture => {
                if let Some(id) = engine.graph.find_by_qualified(&fixture.query) {
                    // Would need architecture query
                    vec![]
                } else { vec![] }
            }
            crate::types::TaskType::ImpactAnalysis => {
                if let Some(id) = engine.graph.find_by_qualified(&fixture.query) {
                    engine.get_surrounding_context(entity_id, &crate::query::QueryOptions::for_agent())
                } else { vec![] }
            }
            crate::types::TaskType::TestDiscovery => {
                if let Some(id) = engine.graph.find_by_qualified(&fixture.query) {
                    engine.graph.entities.values().filter(|e| e.kind == SymbolKind::Test && e.qualified_name.contains(&fixture.query)).cloned().collect()
                } else { vec![] }
            }
            crate::types::TaskType::Configuration => {
                if let Some(id) = engine.graph.find_by_qualified(&fixture.query) {
                    engine.graph.entities.values().filter(|e| e.kind == SymbolKind::Configuration && e.qualified_name.contains(&fixture.query)).cloned().collect()
                } else { vec![] }
            }
            _ => vec![],
        };

        let query_result = crate::query::QueryResult {
            entity: results.first().cloned().unwrap_or_default(),
            relations: vec![],
            score: 1.0,
        };

        let validation = validate_fixture(&fixture, &query_result);
        validation_results.push(validation);
    }

    validation_results
}
}