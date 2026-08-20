//! Prime Evaluation Metrics

use crate::types::*;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Evaluation metrics for Prime
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationMetrics {
    pub prime_first_compliance: f64,
    pub queries_per_task: f64,
    pub source_accesses_per_task: f64,
    pub source_accesses_avoided: f64,
    pub avg_tokens_per_query: f64,
    pub avg_latency_ms: f64,
    pub task_success_rate: f64,
    pub answer_correctness: f64,
    pub escalation_rate: f64,
    pub source_access_avoidance: f64,
}

impl Default for EvaluationMetrics {
    fn default() -> Self {
        Self {
            prime_first_compliance: 0.0,
            queries_per_task: 0.0,
            source_accesses_per_task: 0.0,
            source_accesses_avoided: 0.0,
            avg_tokens_per_query: 0.0,
            avg_latency_ms: 0.0,
            task_success_rate: 0.0,
            answer_correctness: 0.0,
            escalation_rate: 0.0,
            source_access_avoidance: 0.0,
        }
    }
}

/// Evaluation runner
pub struct EvaluationRunner {
    results: Vec<TaskResult>,
}

impl EvaluationRunner {
    pub fn new() -> Self {
        Self { results: Vec::new() }
    }

    pub fn run_task(&mut self, task: TaskDefinition, agent: &mut dyn AgentEvaluator) -> TaskResult {
        let start = Instant::now();
        let result = agent.execute_task(&task);
        let duration = start.elapsed();

        let result = TaskResult {
            task_id: task.id,
            task_type: task.task_type,
            success: result.success,
            prime_queries: result.prime_queries,
            source_accesses: result.source_accesses,
            tokens_used: result.tokens_used,
            latency_ms: duration.as_millis() as u64,
            success: result.success,
            correctness: result.correctness,
            escalation_reasons: result.escalation_reasons,
        };

        self.results.push(result.clone());
        result
    }

    pub fn compute_metrics(&self) -> EvaluationMetrics {
        if self.results.is_empty() {
            return EvaluationMetrics::default();
        }

        let total = self.results.len() as f64;
        let successful = self.results.iter().filter(|r| r.success).count() as f64;
        let total_queries: usize = self.results.iter().map(|r| r.prime_queries).sum();
        let total_source_accesses: usize = self.results.iter().map(|r| r.source_accesses).sum();
        let total_tokens: usize = self.results.iter().map(|r| r.tokens_used).sum();
        let total_latency: u64 = self.results.iter().map(|r| r.latency_ms).sum();
        let total_escalations: usize = self.results.iter().map(|r| r.escalation_reasons.len()).sum();
        let source_avoided = self.results.iter()
            .filter(|r| r.source_accesses == 0)
            .count() as f64;

        EvaluationMetrics {
            prime_first_compliance: successful / total,
            queries_per_task: total_queries as f64 / total,
            source_accesses_per_task: total_source_accesses as f64 / total,
            source_accesses_avoided: source_avoided / total,
            avg_tokens_per_query: if total_queries > 0 { total_tokens as f64 / total_queries as f64 } else { 0.0 },
            avg_latency_ms: total_latency as f64 / total,
            task_success_rate: successful / total,
            answer_correctness: self.results.iter().map(|r| r.correctness).sum::<f64>() / total,
            escalation_rate: total_escalations as f64 / total_queries as f64,
            source_access_avoidance: source_avoided / total,
        }
    }

    pub fn results(&self) -> &[TaskResult] {
        &self.results
    }
}

impl Default for EvaluationRunner {
    fn default() -> Self {
        Self::new()
    }
}

/// Task definition for evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDefinition {
    pub id: String,
    pub task_type: TaskType,
    pub description: String,
    pub expected_answer: Option<String>,
    pub ground_truth: Option<String>,
    pub repository: String,
    pub symbols_of_interest: Vec<String>,
}

/// Task types for evaluation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskType {
    FindSymbol,
    FindReferences,
    FindCallers,
    FindCallees,
    FindDependencies,
    FindDependents,
    FindImplementations,
    FindImplementers,
    GetContext,
    GetSurroundingContext,
    Search,
    Architecture,
    ImpactAnalysis,
    TestDiscovery,
    Configuration,
}

/// Agent evaluator trait
pub trait AgentEvaluator {
    fn execute_task(&self, task: &TaskDefinition) -> TaskExecutionResult;
}

/// Task execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskExecutionResult {
    pub success: bool,
    pub prime_queries: usize,
    pub source_accesses: usize,
    pub tokens_used: usize,
    pub success: bool,
    pub correctness: f64,
    pub escalation_reasons: Vec<String>,
}

/// Task result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: String,
    pub task_type: TaskType,
    pub success: bool,
    pub prime_queries: usize,
    pub source_accesses: usize,
    pub tokens_used: usize,
    pub latency_ms: u64,
    pub success: bool,
    pub correctness: f64,
    pub escalation_reasons: Vec<String>,
}

/// Evaluation report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationReport {
    pub metrics: EvaluationMetrics,
    pub task_results: Vec<TaskResult>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub configuration: EvaluationConfig,
}

/// Evaluation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationConfig {
    pub tasks: Vec<TaskDefinition>,
    pub agent_config: String,
    pub repository: String,
    pub prime_version: String,
}

impl EvaluationRunner {
    pub fn generate_report(&self) -> EvaluationReport {
        EvaluationReport {
            metrics: self.compute_metrics(),
            task_results: self.results.clone(),
            timestamp: chrono::Utc::now(),
            configuration: EvaluationConfig {
                tasks: vec![],
                agent_config: String::new(),
                repository: String::new(),
                prime_version: env!("CARGO_PKG_VERSION").to_string(),
            },
        }
    }
}

impl Default for EvaluationRunner {
    fn default() -> Self {
        Self::new()
    }
}