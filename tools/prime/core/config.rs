//! Prime Config - Configuration for the Prime knowledge system

use crate::types::*;
use anyhow::Result;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Prime configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimeConfig {
    pub storage: StorageConfig,
    pub parser: ParserConfig,
    pub engine: EngineConfig,
    pub compression: CompressionConfig,
    pub language_analyzers: LanguageAnalyzerConfig,
    pub diagnostics: DiagnosticConfig,
    pub forbidden_deps: Vec<ForbiddenDep>,
    pub ownership: OwnershipConfig,
    pub engine_ai: EngineAIConfig,
    pub ai: AIConfig,
    pub multi_agent: MultiAgentConfig,
    pub context: ContextConfig,
    pub graph: GraphConfig,
    pub memory: MemoryConfig,
    pub discover: DiscoverConfig,
    pub tools: ToolsConfig,
}

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub path: PathBuf,
    pub compress: bool,
    pub compression_level: i32,
    pub use_mmap: bool,
    pub page_size: usize,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            path: PathBuf::from(".prime"),
            compress: true,
            compression_level: 3,
            use_mmap: true,
            page_size: 4096,
        }
    }
}

/// Parser configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserConfig {
    pub max_file_size: usize,
    pub excluded_patterns: Vec<String>,
    pub enable_incremental: bool,
    pub num_threads: usize,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            max_file_size: 1024 * 1024,
            excluded_patterns: vec![
                "target/**".to_string(),
                "node_modules/**".to_string(),
                ".git/**".to_string(),
                "dist/**".to_string(),
                "build/**".to_string(),
                "*.lock".to_string(),
                "**/*.min.js".to_string(),
                "**/*.min.css".to_string(),
            ],
            enable_incremental: true,
            num_threads: num_cpus::get(),
        }
    }
}

/// Engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    pub default_depth: usize,
    pub default_max_bytes: usize,
    pub default_include: Vec<String>,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            default_depth: 1,
            default_max_bytes: 65536,
            default_include: vec![
                "hierarchy".to_string(),
                "contract".to_string(),
                "dependencies".to_string(),
                "constraints".to_string(),
                "implementations".to_string(),
            ],
        }
    }
}

/// Compression configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionConfig {
    pub algorithm: String,
    pub level: i32,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            algorithm: "zstd".to_string(),
            level: 3,
        }
    }
}

/// Language analyzer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageAnalyzerConfig {
    pub rust: bool,
    pub typescript: bool,
    pub go: bool,
    pub python: bool,
    pub javascript: bool,
}

impl Default for LanguageAnalyzerConfig {
    fn default() -> Self {
        Self {
            rust: true,
            typescript: true,
            go: true,
            python: true,
            javascript: true,
        }
    }
}

/// Diagnostic configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticConfig {
    pub warn_only: Vec<String>,
}

impl Default for DiagnosticConfig {
    fn default() -> Self {
        Self {
            warn_only: vec![],
        }
    }
}

/// Forbidden dependency configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForbiddenDep {
    pub from: String,
    pub to: String,
}

impl Default for ForbiddenDep {
    fn default() -> Self {
        Self {
            from: String::new(),
            to: String::new(),
        }
    }
}

/// Ownership configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipConfig {
    pub strict: bool,
}

impl Default for OwnershipConfig {
    fn default() -> Self {
        Self {
            strict: false,
        }
    }
}

/// Engine AI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineAIConfig {
    pub enabled: bool,
    pub threshold: u8,
    pub max_iterations: u32,
    pub provider: Option<String>,
    pub model: Option<String>,
}

impl Default for EngineAIConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            threshold: 85,
            max_iterations: 3,
            provider: None,
            model: None,
        }
    }
}

/// AI provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub enabled: bool,
    pub default: Option<String>,
    pub providers: Vec<AIProvider>,
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            default: None,
            providers: Vec::new(),
        }
    }
}

/// AI Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIProvider {
    pub id: String,
    pub provider: String,
    pub model: String,
    pub api_key_env: String,
    pub base_url: Option<String>,
}

impl Default for AIProvider {
    fn default() -> Self {
        Self {
            id: String::new(),
            provider: String::new(),
            model: String::new(),
            api_key_env: String::new(),
            base_url: None,
        }
    }
}

/// Multi-agent orchestration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiAgentConfig {
    pub enabled: bool,
    pub max_concurrency: usize,
    pub max_depth: usize,
    pub task_timeout: u64,
    pub resource_limits: ResourceLimits,
    pub isolation_mode: String,
    pub conflict_policy: String,
}

impl Default for MultiAgentConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            max_concurrency: 4,
            max_depth: 1,
            task_timeout: 300,
            resource_limits: ResourceLimits::default(),
            isolation_mode: "git_worktree".to_string(),
            conflict_policy: "sequentialize".to_string(),
        }
    }
}

/// Resource limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub cpu_percent: u8,
    pub memory_mb: u64,
    pub token_budget: u64,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            cpu_percent: 80,
            memory_mb: 4096,
            token_budget: 1000000,
        }
    }
}

/// Context engine defaults
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextConfig {
    pub default_depth: usize,
    pub default_max_bytes: usize,
    pub default_include: Vec<String>,
}

impl Default for ContextConfig {
    fn default() -> Self {
        Self {
            default_depth: 1,
            default_max_bytes: 65536,
            default_include: vec![
                "hierarchy".to_string(),
                "contract".to_string(),
                "dependencies".to_string(),
                "constraints".to_string(),
                "implementations".to_string(),
            ],
        }
    }
}

/// Graph defaults
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphConfig {
    pub default_format: String,
    pub default_provenance: bool,
}

impl Default for GraphConfig {
    fn default() -> Self {
        Self {
            default_format: "json".to_string(),
            default_provenance: true,
        }
    }
}

/// Memory defaults
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub warn_bytes: usize,
    pub timestamp_format: String,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            warn_bytes: 65536,
            timestamp_format: "rfc3339".to_string(),
        }
    }
}

/// Discovery defaults
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoverConfig {
    pub default_kinds: Vec<String>,
}

impl Default for DiscoverConfig {
    fn default() -> Self {
        Self {
            default_kinds: vec![
                "missing-contract".to_string(),
                "missing-dependency".to_string(),
                "stale-dependency".to_string(),
                "unknown-owner".to_string(),
                "orphan-code".to_string(),
            ],
        }
    }
}

/// Tools configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsConfig {
    pub auto_discover: bool,
    pub plugins: PluginsConfig,
}

impl Default for ToolsConfig {
    fn default() -> Self {
        Self {
            auto_discover: true,
            plugins: PluginsConfig::default(),
        }
    }
}

/// Plugins configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginsConfig {
    pub enabled: bool,
    pub directory: String,
}

impl Default for PluginsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            directory: ".acc/config/tools".to_string(),
        }
    }
}

impl PrimeConfig {
    pub fn load(path: &std::path::Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let content = serde_yaml::to_string(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn default() -> Self {
        Self {
            storage: StorageConfig::default(),
            parser: ParserConfig::default(),
            engine: EngineConfig::default(),
            compression: CompressionConfig::default(),
            language_analyzers: LanguageAnalyzerConfig::default(),
            diagnostics: DiagnosticConfig::default(),
            forbidden_deps: Vec::new(),
            ownership: OwnershipConfig::default(),
            engine_ai: EngineAIConfig::default(),
            ai: AIConfig::default(),
            multi_agent: MultiAgentConfig::default(),
            context: ContextConfig::default(),
            graph: GraphConfig::default(),
            memory: MemoryConfig::default(),
            discover: DiscoverConfig::default(),
            tools: ToolsConfig::default(),
        }
    }
}