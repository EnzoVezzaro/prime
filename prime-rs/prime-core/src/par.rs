//! Prime Agent Representation (PAR) — Canonical semantic representation for agent consumption
//!
//! This module defines the PAR format — a token-efficient, semantically explicit
//! representation of codebase knowledge optimized for agent consumption.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{EntityId, Range};

/// PAR Document — Top-level container for a PAR projection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParDocument {
    /// PAR format version
    pub version: String,
    /// PAR vocabulary used (for tokenizer optimization)
    pub vocabulary: Option<ParVocabulary>,
    /// Alias definitions for entity references
    pub aliases: ParAliases,
    /// Knowledge facts in PAR format
    pub facts: Vec<ParFact>,
    /// Metadata about this projection
    pub metadata: ParMetadata,
}

/// PAR Vocabulary — Controlled vocabulary for token-efficient representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParVocabulary {
    /// Relation type abbreviations (e.g., "CALLS" -> "C")
    pub relations: HashMap<String, String>,
    /// Entity kind abbreviations (e.g., "Function" -> "F")
    pub kinds: HashMap<String, String>,
    /// Property abbreviations (e.g., "signature" -> "sig")
    pub properties: HashMap<String, String>,
}

impl Default for ParVocabulary {
    fn default() -> Self {
        let mut relations = HashMap::new();
        relations.insert("CALLS".to_string(), "C".to_string());
        relations.insert("CALLED_BY".to_string(), "CB".to_string());
        relations.insert("DEPENDS_ON".to_string(), "D".to_string());
        relations.insert("IMPORTS".to_string(), "I".to_string());
        relations.insert("IMPLEMENTS".to_string(), "IM".to_string());
        relations.insert("EXTENDS".to_string(), "E".to_string());
        relations.insert("CONTAINS".to_string(), "CT".to_string());
        relations.insert("PART_OF".to_string(), "PO".to_string());
        relations.insert("RETURNS".to_string(), "R".to_string());
        relations.insert("PARAMETER_OF".to_string(), "P".to_string());
        relations.insert("TYPE_OF".to_string(), "T".to_string());
        relations.insert("READS".to_string(), "RD".to_string());
        relations.insert("WRITES".to_string(), "WR".to_string());
        relations.insert("OVERRIDES".to_string(), "OV".to_string());
        relations.insert("OVERLOADS".to_string(), "OL".to_string());
        relations.insert("EXPORTS".to_string(), "EX".to_string());
        relations.insert("RE_EXPORTS".to_string(), "RX".to_string());
        relations.insert("THROWS".to_string(), "TH".to_string());
        relations.insert("TESTS".to_string(), "TS".to_string());

        let mut kinds = HashMap::new();
        kinds.insert("Module".to_string(), "M".to_string());
        kinds.insert("Namespace".to_string(), "NS".to_string());
        kinds.insert("Package".to_string(), "PK".to_string());
        kinds.insert("Class".to_string(), "C".to_string());
        kinds.insert("Struct".to_string(), "S".to_string());
        kinds.insert("Enum".to_string(), "E".to_string());
        kinds.insert("Trait".to_string(), "T".to_string());
        kinds.insert("Interface".to_string(), "IF".to_string());
        kinds.insert("TypeAlias".to_string(), "TA".to_string());
        kinds.insert("Protocol".to_string(), "PR".to_string());
        kinds.insert("Function".to_string(), "F".to_string());
        kinds.insert("Method".to_string(), "M".to_string());
        kinds.insert("Constructor".to_string(), "CN".to_string());
        kinds.insert("Destructor".to_string(), "DC".to_string());
        kinds.insert("AsyncFunction".to_string(), "AF".to_string());
        kinds.insert("AsyncMethod".to_string(), "AM".to_string());
        kinds.insert("Variable".to_string(), "V".to_string());
        kinds.insert("Field".to_string(), "F".to_string());
        kinds.insert("Constant".to_string(), "K".to_string());
        kinds.insert("StaticVariable".to_string(), "SV".to_string());
        kinds.insert("Parameter".to_string(), "P".to_string());
        kinds.insert("Macro".to_string(), "MC".to_string());
        kinds.insert("Delegate".to_string(), "D".to_string());
        kinds.insert("Event".to_string(), "EV".to_string());
        kinds.insert("Property".to_string(), "PR".to_string());
        kinds.insert("GenericParameter".to_string(), "GP".to_string());
        kinds.insert("ModuleImport".to_string(), "MI".to_string());
        kinds.insert("ModuleExport".to_string(), "ME".to_string());
        kinds.insert("Unknown".to_string(), "?".to_string());

        let mut properties = HashMap::new();
        properties.insert("signature".to_string(), "sig".to_string());
        properties.insert("documentation".to_string(), "doc".to_string());
        properties.insert("confidence".to_string(), "conf".to_string());
        properties.insert("language".to_string(), "lang".to_string());
        properties.insert("range".to_string(), "rng".to_string());
        properties.insert("qualified_name".to_string(), "qname".to_string());
        properties.insert("entity_id".to_string(), "eid".to_string());

        Self {
            relations,
            kinds,
            properties,
        }
    }
}

/// Compression level for PAR output
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CompressionLevel {
    /// Minimal compression - full semantic names, no aliases
    Minimal,
    /// Normal compression - vocabulary abbreviations, entity aliases
    Normal,
    /// Maximum compression - aggressive abbreviation, aggressive aliasing
    Compressed,
}

impl Default for CompressionLevel {
    fn default() -> Self {
        CompressionLevel::Normal
    }
}

/// PAR Aliases — Entity reference aliases for compression
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ParAliases {
    /// Map from alias to entity qualified name
    pub map: HashMap<String, String>,
    /// Reverse map from qualified name to alias
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub reverse: HashMap<String, String>,
}

impl ParAliases {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_or_create(&mut self, qualified_name: &str, vocabulary: &ParVocabulary) -> String {
        if let Some(alias) = self.reverse.get(qualified_name) {
            return alias.clone();
        }

        // Generate alias from vocabulary
        let kind = self.extract_kind(qualified_name);
        let prefix = vocabulary.kinds.get(&kind).cloned().unwrap_or_else(|| "E".to_string());
        let counter = self.map.len() + 1;
        let alias = format!("{}{}", prefix, counter);

        self.map.insert(alias.clone(), qualified_name.to_string());
        self.reverse.insert(qualified_name.to_string(), alias.clone());
        alias
    }

    fn extract_kind(&self, qualified_name: &str) -> String {
        // Extract last component as potential kind hint
        qualified_name.split("::")
            .last()
            .unwrap_or("Entity")
            .to_string()
    }

    pub fn resolve(&self, alias: &str) -> Option<&str> {
        self.map.get(alias).map(|s| s.as_str())
    }
}

/// PAR Fact — A single semantic fact in PAR format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParFact {
    /// Subject entity (alias or qualified name)
    pub subject: String,
    /// Predicate (relation type, possibly abbreviated)
    pub predicate: String,
    /// Object entity (alias or qualified name) or literal value
    pub object: ParObject,
    /// Confidence level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<String>,
    /// Source location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Provenance info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance: Option<String>,
}

/// PAR Object — Can be an entity reference or literal value
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParObject {
    /// Entity reference (alias or qualified name)
    Entity(String),
    /// Literal value (string, number, boolean)
    Literal(serde_json::Value),
    /// Multiple entities
    Entities(Vec<String>),
}

/// PAR Metadata — Projection metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParMetadata {
    /// Source query that generated this projection
    pub query: Option<String>,
    /// Number of entities in projection
    pub entity_count: usize,
    /// Number of facts in projection
    pub fact_count: usize,
    /// Token count estimate
    pub token_estimate: usize,
    /// Projection timestamp
    pub timestamp: u64,
    /// Vocabulary used
    pub vocabulary_version: String,
}

/// PAR Entity — Detailed entity representation in PAR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParEntity {
    /// Entity alias
    pub alias: String,
    /// Full qualified name
    pub qualified_name: String,
    /// Entity kind (possibly abbreviated)
    pub kind: String,
    /// Entity signature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    /// Documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    /// Confidence level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<String>,
    /// Language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Source range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,
    /// Properties
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub properties: HashMap<String, String>,
    /// Relations as facts (inline)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub relations: Vec<ParFact>,
}

/// Convert from internal entity to PAR entity
pub fn entity_to_par_entity(
    entity: &crate::Entity,
    _id: &EntityId,
    aliases: &mut ParAliases,
    vocabulary: &ParVocabulary,
) -> ParEntity {
    let alias = aliases.get_or_create(&entity.qualified_name, vocabulary);
    let kind = vocabulary.kinds.get(&format!("{:?}", entity.kind))
        .cloned()
        .unwrap_or_else(|| format!("{:?}", entity.kind));

    let mut properties = HashMap::new();
    if let Some(sig) = &entity.signature {
        properties.insert("sig".to_string(), sig.clone());
    }
    if let Some(doc) = &entity.documentation {
        properties.insert("doc".to_string(), doc.clone());
    }
    properties.insert("conf".to_string(), format!("{:?}", entity.confidence));
    properties.insert("lang".to_string(), format!("{:?}", entity.language));

    ParEntity {
        alias,
        qualified_name: entity.qualified_name.clone(),
        kind,
        signature: entity.signature.clone(),
        documentation: entity.documentation.clone(),
        confidence: Some(format!("{:?}", entity.confidence)),
        language: Some(format!("{:?}", entity.language)),
        range: Some(entity.range),
        properties,
        relations: Vec::new(),
    }
}

/// Render a PAR document to string with specified compression level
pub fn render_par_compact(doc: &ParDocument, level: CompressionLevel) -> String {
    let mut out = String::new();

    // Header
    out.push_str(&format!("@PAR v{}\n", doc.version));
    if let Some(vocab) = &doc.vocabulary {
        out.push_str("@VOCAB\n");
        for (k, v) in &vocab.relations {
            let (k_out, v_out) = match level {
                CompressionLevel::Minimal => (k.clone(), v.clone()),
                CompressionLevel::Normal => (k.clone(), v.clone()),
                CompressionLevel::Compressed => (vocab.relations.get(k).cloned().unwrap_or(k.clone()), vocab.relations.get(v).cloned().unwrap_or(v.clone())),
            };
            out.push_str(&format!("  {}={}\n", k_out, v_out));
        }
        for (k, v) in &vocab.kinds {
            let (k_out, v_out) = match level {
                CompressionLevel::Minimal => (k.clone(), v.clone()),
                CompressionLevel::Normal => (k.clone(), v.clone()),
                CompressionLevel::Compressed => (vocab.kinds.get(k).cloned().unwrap_or(k.clone()), vocab.kinds.get(v).cloned().unwrap_or(v.clone())),
            };
            out.push_str(&format!("  {}={}\n", k_out, v_out));
        }
    }

    // Aliases
    if !doc.aliases.map.is_empty() && level != CompressionLevel::Minimal {
        out.push_str("@ALIASES\n");
        for (alias, qname) in &doc.aliases.map {
            out.push_str(&format!("  {}={}\n", alias, qname));
        }
    }

    // Entities
    // Group facts by subject
    let mut entity_facts: HashMap<String, Vec<&ParFact>> = HashMap::new();
    for fact in &doc.facts {
        entity_facts.entry(fact.subject.clone()).or_default().push(fact);
    }

    for (entity, facts) in entity_facts {
        // Output entity header
        out.push_str(&format!("\n{} {{\n", entity));
        for fact in facts {
            let pred = &fact.predicate;
            let obj = match &fact.object {
                ParObject::Entity(e) => e.as_str(),
                ParObject::Literal(v) => &v.to_string(),
                ParObject::Entities(es) => &es.join(","),
            };
            out.push_str(&format!("  {} {}\n", pred, obj));
        }
        out.push_str("}\n");
    }

    out
}

/// Render a PAR document with default (Normal) compression
pub fn render_par_compact_default(doc: &ParDocument) -> String {
    render_par_compact(doc, CompressionLevel::Normal)
}

/// Parse PAR from string (basic implementation)
pub fn parse_par(_input: &str) -> Result<ParDocument, String> {
    // TODO: Implement proper parser
    Err("Not implemented".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vocabulary_default() {
        let vocab = ParVocabulary::default();
        assert_eq!(vocab.relations.get("CALLS"), Some(&"C".to_string()));
        assert_eq!(vocab.kinds.get("Function"), Some(&"F".to_string()));
    }

    #[test]
    fn test_aliases() {
        let mut aliases = ParAliases::new();
        let vocab = ParVocabulary::default();

        let a1 = aliases.get_or_create("AuthService.login", &ParVocabulary::default());
        let a2 = aliases.get_or_create("AuthService.login", &ParVocabulary::default());
        assert_eq!(a1, a2);

        let resolved = aliases.resolve(&a1);
        assert_eq!(resolved, Some("AuthService.login"));
    }
}

/// Tokenizer trait for model-specific tokenization
pub trait Tokenizer {
    /// Count tokens in a string
    fn count_tokens(&self, text: &str) -> usize;

    /// Get tokenizer name
    fn name(&self) -> &'static str;
}

/// GPT-4 tokenizer approximation (cl100k_base)
pub struct Gpt4Tokenizer;

impl Tokenizer for Gpt4Tokenizer {
    fn count_tokens(&self, text: &str) -> usize {
        // Rough approximation: ~4 chars per token for English code
        // More accurate: use tiktoken-rs in production
        text.len().div_ceil(4)
    }

    fn name(&self) -> &'static str {
        "gpt-4 (cl100k_base)"
    }
}

/// Claude tokenizer approximation
pub struct ClaudeTokenizer;

impl Tokenizer for ClaudeTokenizer {
    fn count_tokens(&self, text: &str) -> usize {
        // Claude uses a different tokenizer, roughly similar to GPT-4
        text.len().div_ceil(4)
    }

    fn name(&self) -> &'static str {
        "claude"
    }
}

/// Generic tokenizer using character-based approximation
pub struct GenericTokenizer;

impl Tokenizer for GenericTokenizer {
    fn count_tokens(&self, text: &str) -> usize {
        // Conservative estimate: 3.5 chars per token for code
        text.len().div_ceil(3)
    }

    fn name(&self) -> &'static str {
        "generic"
    }
}

/// Token-aware PAR renderer
pub struct ParRenderer<'a> {
    vocab: &'a ParVocabulary,
    aliases: &'a ParAliases,
    tokenizer: std::sync::Arc<dyn Tokenizer>,
}

impl<'a> ParRenderer<'a> {
    pub fn new(vocab: &'a ParVocabulary, aliases: &'a ParAliases, tokenizer: std::sync::Arc<dyn Tokenizer>) -> Self {
        Self {
            vocab,
            aliases,
            tokenizer,
        }
    }

    /// Render a PAR document with token-aware encoding
    pub fn render(&self, doc: &ParDocument) -> String {
        let mut out = String::new();

        // Header
        out.push_str(&format!("@PAR v{}\n", doc.version));
        
        if let Some(vocab) = &doc.vocabulary {
            out.push_str("@VOCAB\n");
            for (k, v) in &vocab.relations {
                out.push_str(&format!("  {}={}\n", k, v));
            }
            for (k, v) in &vocab.kinds {
                out.push_str(&format!("  {}={}\n", k, v));
            }
        }

        // Aliases
        if !doc.aliases.map.is_empty() {
            out.push_str("@ALIASES\n");
            for (alias, qname) in &doc.aliases.map {
                out.push_str(&format!("  {}={}\n", alias, qname));
            }
        }

        // Facts grouped by subject
        let mut entity_facts: HashMap<String, Vec<&ParFact>> = HashMap::new();
        for fact in &doc.facts {
            entity_facts.entry(fact.subject.clone()).or_default().push(fact);
        }

        for (entity, facts) in entity_facts {
            out.push_str(&format!("\n{} {{\n", entity));
            for fact in facts {
                let pred = &fact.predicate;
                let obj = match &fact.object {
                    ParObject::Entity(e) => e.as_str(),
                    ParObject::Literal(v) => &v.to_string(),
                    ParObject::Entities(es) => &es.join(","),
                };
                out.push_str(&format!("  {} {}\n", pred, obj));
            }
            out.push_str("}\n");
        }

        out
    }

    /// Render with token budget - stops when budget exceeded
    pub fn render_with_budget(&self, doc: &ParDocument, token_budget: usize) -> (String, usize) {
        let mut out = String::new();
        let mut tokens_used = 0;

        // Header
        let header = format!("@PAR v{}\n", doc.version);
        let header_tokens = self.tokenizer.count_tokens(&header);
        if tokens_used + header_tokens > token_budget {
            return (out, tokens_used);
        }
        out.push_str(&header);
        tokens_used += header_tokens;

        // Vocabulary
        if let Some(vocab) = &doc.vocabulary {
            let vocab_str = Self::render_vocab_static(&self.vocab);
            let vocab_tokens = self.tokenizer.count_tokens(&vocab_str);
            if tokens_used + vocab_tokens > token_budget {
                return (out, tokens_used);
            }
            out.push_str(&vocab_str);
            tokens_used += vocab_tokens;
        }

        // Aliases
        if !doc.aliases.map.is_empty() {
            let aliases_str = Self::render_aliases_static(&doc.aliases);
            let alias_tokens = self.tokenizer.count_tokens(&aliases_str);
            if tokens_used + alias_tokens > token_budget {
                return (out, tokens_used);
            }
            out.push_str(&aliases_str);
            tokens_used += alias_tokens;
        }

        // Facts grouped by subject
        let mut entity_facts: HashMap<String, Vec<&ParFact>> = HashMap::new();
        for fact in &doc.facts {
            entity_facts.entry(fact.subject.clone()).or_default().push(fact);
        }

        for (entity, facts) in entity_facts {
            let entity_header = format!("\n{} {{\n", entity);
            let header_tokens = self.tokenizer.count_tokens(&entity_header);
            if tokens_used + header_tokens > token_budget {
                return (out, tokens_used);
            }
            out.push_str(&entity_header);
            tokens_used += header_tokens;

            for fact in facts {
                let pred = &fact.predicate;
                let obj = match &fact.object {
                    ParObject::Entity(e) => e.as_str(),
                    ParObject::Literal(v) => &v.to_string(),
                    ParObject::Entities(es) => &es.join(","),
                };
                let fact_line = format!("  {} {}\n", pred, obj);
                let fact_tokens = self.tokenizer.count_tokens(&fact_line);
                if tokens_used + fact_tokens > token_budget {
                    return (out, tokens_used);
                }
                out.push_str(&fact_line);
                tokens_used += fact_tokens;
            }

            let closing = "}\n";
            let closing_tokens = self.tokenizer.count_tokens(closing);
            if tokens_used + closing_tokens > token_budget {
                return (out, tokens_used);
            }
            out.push_str(closing);
            tokens_used += closing_tokens;
        }

        (out, tokens_used)
    }

    fn render_vocab_static(vocab: &ParVocabulary) -> String {
        let mut out = String::new();
        out.push_str("@VOCAB\n");
        for (k, v) in &vocab.relations {
            out.push_str(&format!("  {}={}\n", k, v));
        }
        for (k, v) in &vocab.kinds {
            out.push_str(&format!("  {}={}\n", k, v));
        }
        out
    }

    fn render_aliases_static(aliases: &ParAliases) -> String {
        let mut out = String::new();
        out.push_str("@ALIASES\n");
        for (alias, qname) in &aliases.map {
            out.push_str(&format!("  {}={}\n", alias, qname));
        }
        out
    }
}

/// Model-specific tokenizer registry
pub struct TokenizerRegistry {
    tokenizers: HashMap<String, std::sync::Arc<dyn Tokenizer>>,
}

impl Default for TokenizerRegistry {
    fn default() -> Self {
        let mut tokenizers: HashMap<String, std::sync::Arc<dyn Tokenizer>> = HashMap::new();
        tokenizers.insert("gpt-4".to_string(), std::sync::Arc::new(Gpt4Tokenizer));
        tokenizers.insert("gpt-3.5".to_string(), std::sync::Arc::new(Gpt4Tokenizer));
        tokenizers.insert("claude".to_string(), std::sync::Arc::new(ClaudeTokenizer));
        tokenizers.insert("generic".to_string(), std::sync::Arc::new(GenericTokenizer));
        Self { tokenizers }
    }
}

impl TokenizerRegistry {
    pub fn get(&self, model: &str) -> Option<&std::sync::Arc<dyn Tokenizer>> {
        self.tokenizers.get(model)
    }

    pub fn get_or_default(&self, model: &str) -> &std::sync::Arc<dyn Tokenizer> {
        self.tokenizers.get(model).unwrap_or_else(|| self.tokenizers.get("generic").unwrap())
    }

    pub fn register(&mut self, name: String, tokenizer: std::sync::Arc<dyn Tokenizer>) {
        self.tokenizers.insert(name, tokenizer);
    }
}

/// Token optimization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenOptimizationResult {
    pub original_tokens: usize,
    pub optimized_tokens: usize,
    pub reduction_ratio: f64,
    pub encoding_used: String,
}

/// Token optimizer - finds the most token-efficient encoding
pub struct TokenOptimizer {
    registry: TokenizerRegistry,
}

impl Default for TokenOptimizer {
    fn default() -> Self {
        Self {
            registry: TokenizerRegistry::default(),
        }
    }
}

impl TokenOptimizer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Find the most token-efficient encoding for a document
    pub fn optimize(&self, doc: &ParDocument) -> TokenOptimizationResult {
        let mut best_result = None;
        let mut best_tokens = usize::MAX;
        let mut _best_tokenizer_name = "generic".to_string();

        for (_name, tokenizer) in &self.registry.tokenizers {
            let tokenizer_clone = tokenizer.clone();
            let vocab = ParVocabulary::default();
            let aliases = ParAliases::new();
            let renderer = ParRenderer::new(&vocab, &aliases, tokenizer_clone);
            let (_rendered, tokens) = renderer.render_with_budget(doc, usize::MAX);

            if tokens < best_tokens {
                best_tokens = tokens;
                best_result = Some(TokenOptimizationResult {
                    original_tokens: tokens,
                    optimized_tokens: tokens,
                    reduction_ratio: 1.0,
                    encoding_used: tokenizer.name().to_string(),
                });
            }
        }

        best_result.unwrap_or_else(|| TokenOptimizationResult {
            original_tokens: 0,
            optimized_tokens: 0,
            reduction_ratio: 1.0,
            encoding_used: "generic".to_string(),
        })
    }

    /// Register a custom tokenizer
    pub fn register_tokenizer(&mut self, name: String, tokenizer: std::sync::Arc<dyn Tokenizer>) {
        self.registry.register(name, tokenizer);
    }
}

#[cfg(test)]
mod tokenizer_tests {
    use super::*;

    #[test]
    fn test_gpt4_tokenizer() {
        let tokenizer = Gpt4Tokenizer;
        assert_eq!(tokenizer.count_tokens("hello world"), 3); // 11 chars / 4 = 3
        assert_eq!(tokenizer.count_tokens("fn main() {}"), 3); // 12 chars / 4 = 3
    }

    #[test]
    fn test_token_optimizer() {
        let optimizer = TokenOptimizer::new();
        let doc = ParDocument {
            version: "1.0".to_string(),
            vocabulary: None,
            aliases: ParAliases::new(),
            facts: vec![],
            metadata: ParMetadata {
                query: None,
                entity_count: 0,
                fact_count: 0,
                token_estimate: 0,
                timestamp: 0,
                vocabulary_version: "1.0".to_string(),
            },
        };

        let result = optimizer.optimize(&doc);
        assert!(result.optimized_tokens > 0);
    }
}