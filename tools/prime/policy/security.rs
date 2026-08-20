//! Prime Security Policy

use crate::types::*;
use anyhow::Result;
use std::collections::HashMap;

/// Security policy for Prime
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    pub allowed_operations: Vec<AllowedOperation>,
    pub forbidden_operations: Vec<ForbiddenOperation>,
    pub max_token_budget: usize,
    pub max_results: usize,
    pub max_depth: usize,
    pub require_provenance: bool,
    pub audit_logging: bool,
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        Self {
            allowed_operations: vec![
                AllowedOperation::FindSymbol,
                AllowedOperation::FindSymbols,
                AllowedOperation::GetContext,
                AllowedOperation::GetSurrounding,
                AllowedOperation::GetCallers,
                AllowedOperation::GetCallees,
                AllowedOperation::GetDependencies,
                AllowedOperation::GetDependents,
                AllowedOperation::Search,
                AllowedOperation::ContextBuilder,
            ],
            forbidden_operations: vec![
                ForbiddenOperation::ExecuteCode,
                ForbiddenOperation::FileWrite,
                ForbiddenOperation::NetworkRequest,
                ForbiddenOperation::SystemCommand,
                ForbiddenOperation::EnvAccess,
            ],
            max_token_budget: 32768,
            max_results: 100,
            max_depth: 5,
            require_provenance: true,
            audit_logging: true,
        }
    }
}

/// Allowed operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AllowedOperation {
    FindSymbol,
    FindSymbols,
    GetContext,
    GetSurrounding,
    GetCallers,
    GetCallees,
    GetDependencies,
    GetDependents,
    Search,
    ContextBuilder,
}

/// Forbidden operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ForbiddenOperation {
    ExecuteCode,
    FileWrite,
    NetworkRequest,
    SystemCommand,
    EnvAccess,
}

/// Security manager for enforcing policies
pub struct SecurityManager {
    policy: SecurityPolicy,
    audit_log: Vec<AuditEntry>,
}

impl SecurityManager {
    pub fn new(policy: SecurityPolicy) -> Self {
        Self {
            policy,
            audit_log: Vec::new(),
        }
    }

    pub fn default() -> Self {
        Self::new(SecurityPolicy::default())
    }

    /// Check if an operation is allowed
    pub fn is_allowed(&self, operation: &AllowedOperation) -> bool {
        self.policy.allowed_operations.contains(operation)
    }

    /// Check if an operation is forbidden
    pub fn is_forbidden(&self, operation: &ForbiddenOperation) -> bool {
        self.policy.forbidden_operations.contains(operation)
    }

    /// Validate a query request
    pub fn validate_query(&self, opts: &crate::query::QueryOptions) -> Result<(), SecurityError> {
        if opts.token_budget > self.policy.max_token_budget {
            return Err(SecurityError::TokenBudgetExceeded(opts.token_budget));
        }

        if opts.max_results > self.policy.max_results {
            return Err(SecurityError::MaxResultsExceeded(opts.max_results));
        }

        if opts.max_depth > self.policy.max_depth {
            return Err(SecurityError::MaxDepthExceeded(opts.max_depth));
        }

        Ok(())
    }

    /// Validate token budget
    pub fn validate_token_budget(&self, budget: usize) -> Result<(), SecurityError> {
        if budget > self.policy.max_token_budget {
            return Err(SecurityError::TokenBudgetExceeded(budget));
        }
        Ok(())
    }

    /// Check if provenance is required
    pub fn require_provenance(&self) -> bool {
        self.policy.require_provenance
    }

    /// Log an audit entry
    pub fn audit(&mut self, entry: AuditEntry) {
        if self.policy.audit_logging {
            self.audit_log.push(entry);
        }
    }

    /// Get audit log
    pub fn audit_log(&self) -> &[AuditEntry] {
        &self.audit_log
    }
}

/// Security errors
#[derive(Debug, thiserror::Error)]
pub enum SecurityError {
    #[error("Token budget exceeded: {0} (max: {})", .0)]
    TokenBudgetExceeded(usize),

    #[error("Max results exceeded: {0} (max: {})", .0)]
    MaxResultsExceeded(usize),

    #[error("Max depth exceeded: {0} (max: {})", .0)]
    MaxDepthExceeded(usize),

    #[error("Operation not allowed: {0:?}")]
    OperationNotAllowed(AllowedOperation),

    #[error("Forbidden operation attempted: {0:?}")]
    ForbiddenOperation(ForbiddenOperation),
}

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub operation: String,
    pub entity_id: Option<u64>,
    pub agent: String,
    pub success: bool,
    pub error: Option<String>,
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
}

impl AuditEntry {
    pub fn new(operation: &str, agent: &str) -> Self {
        Self {
            timestamp: chrono::Utc::now(),
            operation: operation.to_string(),
            entity_id: None,
            agent: agent.to_string(),
            success: true,
            error: None,
            metadata: std::collections::HashMap::new(),
        }
    }

    pub fn with_entity_id(mut self, entity_id: u64) -> Self {
        self.entity_id = Some(entity_id);
        self
    }

    pub fn with_error(mut self, error: String) -> Self {
        self.success = false;
        self.error = Some(error);
        self
    }

    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Security policy builder
pub struct SecurityPolicyBuilder {
    policy: SecurityPolicy,
}

impl SecurityPolicyBuilder {
    pub fn new() -> Self {
        Self {
            policy: SecurityPolicy::default(),
        }
    }

    pub fn allow_operation(mut self, op: AllowedOperation) -> Self {
        if !self.policy.allowed_operations.contains(&op) {
            self.policy.allowed_operations.push(op);
        }
        self
    }

    pub fn forbid_operation(mut self, op: ForbiddenOperation) -> Self {
        if !self.policy.forbidden_operations.contains(&op) {
            self.policy.forbidden_operations.push(op);
        }
        self
    }

    pub fn max_token_budget(mut self, budget: usize) -> Self {
        self.policy.max_token_budget = budget;
        self
    }

    pub fn max_results(mut self, max: usize) -> Self {
        self.policy.max_results = max;
        self
    }

    pub fn max_depth(mut self, depth: usize) -> Self {
        self.policy.max_depth = depth;
        self
    }

    pub fn require_provenance(mut self, require: bool) -> Self {
        self.policy.require_provenance = require;
        self
    }

    pub fn audit_logging(mut self, enabled: bool) -> Self {
        self.policy.audit_logging = enabled;
        self
    }

    pub fn build(self) -> SecurityPolicy {
        self.policy
    }
}

impl Default for SecurityPolicyBuilder {
    fn default() -> Self {
        Self::new()
    }
}