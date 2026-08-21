//! Multi-language parser using Tree-sitter

mod analyzer;
mod extractor;
mod language_config;
mod walker;

pub use analyzer::*;
pub use extractor::*;
pub use language_config::*;
pub use walker::*;

use prime_core::{Language, EntityId, ContentHash};
use std::path::Path;
use std::sync::Arc;

/// Parse result for a single file
#[derive(Debug, Clone)]
pub struct ParseResult {
    pub file_path: String,
    pub language: Language,
    pub content_hash: ContentHash,
    pub entities: Vec<ExtractedEntity>,
    pub relations: Vec<ExtractedRelation>,
    pub parse_errors: Vec<ParseError>,
}

/// An entity extracted from source
#[derive(Debug, Clone)]
pub struct ExtractedEntity {
    pub kind: prime_core::SymbolKind,
    pub name: String,
    pub qualified_name: String,
    pub range: prime_core::Range,
    pub signature: Option<String>,
    pub documentation: Option<String>,
    pub confidence: prime_core::Confidence,
}

/// A relation extracted from source
#[derive(Debug, Clone)]
pub struct ExtractedRelation {
    pub from_name: String,
    pub from_qualified: String,
    pub to_name: String,
    pub to_qualified: String,
    pub kind: prime_core::RelationKind,
    pub confidence: prime_core::Confidence,
}

/// Parse error
#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub range: Option<prime_core::Range>,
}

/// Parser configuration
#[derive(Debug, Clone)]
pub struct ParserConfig {
    pub max_file_size: usize,
    pub excluded_patterns: Vec<String>,
    pub enable_incremental: bool,
    pub num_threads: usize,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            max_file_size: 1024 * 1024, // 1MB
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

/// Main parser entry point
pub struct Parser {
    config: ParserConfig,
    language_configs: Arc<LanguageConfigRegistry>,
}

impl Parser {
    pub fn new(config: ParserConfig) -> anyhow::Result<Self> {
        let language_configs = Arc::new(LanguageConfigRegistry::new()?);
        Ok(Self { config, language_configs })
    }

    /// Parse a single file
    pub fn parse_file(&self, path: &Path) -> anyhow::Result<ParseResult> {
        let content = std::fs::read(path)?;
        if content.len() > self.config.max_file_size {
            return Ok(ParseResult {
                file_path: path.display().to_string(),
                language: Language::Unknown,
                content_hash: prime_core::ContentHash::from_bytes(&content),
                entities: Vec::new(),
                relations: Vec::new(),
                parse_errors: vec![ParseError {
                    message: format!("File too large: {} bytes", content.len()),
                    range: None,
                }],
            });
        }

        let language = self.detect_language(path);
        let content_hash = prime_core::ContentHash::from_bytes(&content);

        if language == Language::Unknown {
            return Ok(ParseResult {
                file_path: path.display().to_string(),
                language,
                content_hash,
                entities: Vec::new(),
                relations: Vec::new(),
                parse_errors: Vec::new(),
            });
        }

        let config = self.language_configs.get(language);
        if config.is_none() {
            return Ok(ParseResult {
                file_path: path.display().to_string(),
                language,
                content_hash,
                entities: Vec::new(),
                relations: Vec::new(),
                parse_errors: vec![ParseError {
                    message: format!("No parser for language: {:?}", language),
                    range: None,
                }],
            });
        }

        let config = config.unwrap();
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&config.ts_language)?;

        let tree = parser.parse(&content, None)
            .ok_or_else(|| anyhow::anyhow!("Failed to parse"))?;

        let mut extractor = Extractor::new(&config, &content, path);
        let (entities, relations, errors) = extractor.extract(&tree);

        Ok(ParseResult {
            file_path: path.display().to_string(),
            language,
            content_hash,
            entities,
            relations,
            parse_errors: errors,
        })
    }

    /// Parse entire project
    pub fn parse_project(&self, root: &Path) -> anyhow::Result<ProjectParseResult> {
        use walkdir::WalkDir;
        use rayon::prelude::*;

        let files: Vec<_> = WalkDir::new(root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| !self.should_exclude(e.path()))
            .map(|e| e.path().to_path_buf())
            .collect();

        let results: Vec<_> = files.par_iter()
            .map(|f| self.parse_file(f))
            .collect();

        let mut project_result = ProjectParseResult::default();
        for result in results {
            match result {
                Ok(r) => project_result.add(r),
                Err(e) => project_result.errors.push(format!("{}: {}", e, e)),
            }
        }

        Ok(project_result)
    }

    fn detect_language(&self, path: &Path) -> Language {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            self.language_configs.detect(ext)
        } else {
            Language::Unknown
        }
    }

    fn should_exclude(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        self.config.excluded_patterns.iter().any(|pat| {
            glob::Pattern::new(pat).map(|p| p.matches(&path_str)).unwrap_or(false)
        })
    }
}

/// Result of parsing an entire project
#[derive(Debug, Default)]
pub struct ProjectParseResult {
    pub files: Vec<ParseResult>,
    pub errors: Vec<String>,
}

impl ProjectParseResult {
    pub fn add(&mut self, result: ParseResult) {
        self.files.push(result);
    }

    pub fn entity_count(&self) -> usize {
        self.files.iter().map(|f| f.entities.len()).sum()
    }

    pub fn relation_count(&self) -> usize {
        self.files.iter().map(|f| f.relations.len()).sum()
    }
}
