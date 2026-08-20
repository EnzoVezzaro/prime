//! Prime Core Types - Core type definitions for the Prime knowledge interface

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unique identifier for entities in the knowledge graph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId(pub u64);

impl EntityId {
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        EntityId(std::sync::atomic::AtomicU64::new(1).fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }

    pub fn from_u64(id: u64) -> Self {
        EntityId(id)
    }
}

impl Default for EntityId {
    fn default() -> Self {
        EntityId::new()
    }
}

impl From<u64> for EntityId {
    fn from(id: u64) -> Self {
        EntityId(id)
    }
}

impl From<EntityId> for u64 {
    fn from(id: EntityId) -> Self {
        id.0
    }
}

impl std::fmt::Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Symbol kind - language-agnostic classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum SymbolKind {
    // Module/Package level
    Module = 1,
    Namespace = 2,
    Package = 3,

    // Type definitions
    Class = 10,
    Struct = 11,
    Enum = 12,
    Trait = 13,
    Interface = 14,
    TypeAlias = 15,
    Protocol = 16,

    // Functions/Methods
    Function = 20,
    Method = 21,
    Constructor = 22,
    Destructor = 23,
    AsyncFunction = 24,
    AsyncMethod = 25,

    // Variables/Fields
    Variable = 30,
    Field = 31,
    Constant = 32,
    StaticVariable = 33,
    Parameter = 34,

    // Other
    Macro = 40,
    Delegate = 41,
    Event = 42,
    Property = 43,
    GenericParameter = 44,
    ModuleImport = 45,
    ModuleExport = 46,

    Unknown = 255,
}

impl SymbolKind {
    pub fn is_callable(&self) -> bool {
        matches!(self,
            SymbolKind::Function | SymbolKind::Method |
            SymbolKind::Constructor | SymbolKind::AsyncFunction |
            SymbolKind::AsyncMethod
        )
    }

    pub fn is_type(&self) -> bool {
        matches!(self,
            SymbolKind::Class | SymbolKind::Struct | SymbolKind::Enum |
            SymbolKind::Trait | SymbolKind::Interface | SymbolKind::TypeAlias |
            SymbolKind::Protocol
        )
    }

    pub fn is_module(&self) -> bool {
        matches!(self, SymbolKind::Module | SymbolKind::Namespace | SymbolKind::Package)
    }
}

impl std::fmt::Display for SymbolKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SymbolKind::Module => "Module",
            SymbolKind::Namespace => "Namespace",
            SymbolKind::Package => "Package",
            SymbolKind::Class => "Class",
            SymbolKind::Struct => "Struct",
            SymbolKind::Enum => "Enum",
            SymbolKind::Trait => "Trait",
            SymbolKind::Interface => write!(f, "Interface"),
            SymbolKind::TypeAlias => write!(f, "TypeAlias"),
            SymbolKind::Protocol => write!(f, "Protocol"),
            SymbolKind::Function => write!(f, "Function"),
            SymbolKind::Method => write!(f, "Method"),
            SymbolKind::Constructor => write!(f, "Constructor"),
            SymbolKind::Destructor => write!(f, "Destructor"),
            SymbolKind::AsyncFunction => write!(f, "AsyncFunction"),
            SymbolKind::AsyncMethod => write!(f, "AsyncMethod"),
            SymbolKind::Variable => write!(f, "Variable"),
            SymbolKind::Field => write!(f, "Field"),
            SymbolKind::Constant => write!(f, "Constant"),
            SymbolKind::StaticVariable => write!(f, "StaticVariable"),
            SymbolKind::Parameter => write!(f, "Parameter"),
            SymbolKind::Macro => write!(f, "Macro"),
            SymbolKind::Delegate => write!(f, "Delegate"),
            SymbolKind::Event => write!(f, "Event"),
            SymbolKind::Property => write!(f, "Property"),
            SymbolKind::GenericParameter => write!(f, "GenericParameter"),
            SymbolKind::ModuleImport => write!(f, "ModuleImport"),
            SymbolKind::ModuleExport => write!(f, "ModuleExport"),
            SymbolKind::Unknown => write!(f, "Unknown"),
        };
        write!(f, "{}", s)
    }
}

/// Relationship kinds between entities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum RelationKind {
    // Hierarchical
    Contains = 1,
    PartOf = 2,

    // Inheritance/Implementation
    Extends = 10,
    Implements = 11,
    Inherits = 12,

    // Dependencies
    DependsOn = 20,
    Imports = 21,
    Requires = 22,

    // Calls/References
    Calls = 30,
    References = 31,
    Reads = 32,
    Writes = 33,

    // Type relationships
    Returns = 40,
    ParameterOf = 41,
    TypeOf = 42,
    GenericArgOf = 43,

    // Overrides
    Overrides = 50,
    Overloads = 51,

    // Module boundaries
    Exports = 60,
    ReExports = 61,

    Unknown = 255,
}

impl std::fmt::Display for RelationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            RelationKind::Contains => "Contains",
            RelationKind::PartOf => "PartOf",
            RelationKind::Extends => "Extends",
            RelationKind::Implements => "Implements",
            RelationKind::Inherits => "Inherits",
            RelationKind::DependsOn => "DependsOn",
            RelationKind::Imports => "Imports",
            RelationKind::Requires => "Requires",
            RelationKind::Calls => "Calls",
            RelationKind::References => "References",
            RelationKind::Reads => "Reads",
            RelationKind::Writes => "Writes",
            RelationKind::Returns => "Returns",
            RelationKind::ParameterOf => "ParameterOf",
            RelationKind::TypeOf => "TypeOf",
            RelationKind::GenericArgOf => "GenericArgOf",
            RelationKind::Overrides => "Overrides",
            RelationKind::Overloads => "Overloads",
            RelationKind::Exports => "Exports",
            RelationKind::ReExports => "ReExports",
            RelationKind::Unknown => "Unknown",
        };
        write!(f, "{}", s)
    }
}

/// Confidence levels for derived knowledge
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Confidence {
    Exact = 4,
    Observation = 3,
    Hypothesis = 2,
    Inference = 1,
    Unknown = 0,
}

impl Confidence {
    pub fn as_str(&self) -> &'static str {
        match self {
            Confidence::Exact => "exact",
            Confidence::Observation => "observation",
            Confidence::Hypothesis => "hypothesis",
            Confidence::Inference => "inference",
            Confidence::Unknown => "unknown",
        }
    }

    pub fn is_reliable(&self) -> bool {
        *self >= Confidence::Observation
    }
}

impl std::fmt::Display for Confidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for Confidence {
    fn default() -> Self {
        Confidence::Unknown
    }
}

/// Provenance of a derived fact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provenance {
    pub kind: ProvenanceKind,
    pub source: String,
    pub detail: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProvenanceKind {
    Declared,
    Discovered,
    Inferred,
    Memory,
}

/// Programming language identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Language {
    Unknown = 0,
    Rust = 1,
    TypeScript = 2,
    JavaScript = 3,
    Python = 4,
    Go = 5,
    Java = 6,
    C = 7,
    Cpp = 8,
    CSharp = 9,
    Python3 = 10,
    Ruby = 11,
    Php = 12,
    Swift = 13,
    Kotlin = 14,
    Scala = 15,
    Haskell = 16,
    Zig = 17,
    Lua = 17,
    Shell = 18,
    Sql = 19,
    Markdown = 22,
    Json = 23,
    Yaml = 24,
    Toml = 25,
    Protobuf = 26,
    Graphql = 27,
    Dockerfile = 28,
    Terraform = 29,
    Other = 255,
}

impl Language {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "rs" => Language::Rust,
            "ts" | "tsx" => Language::TypeScript,
            "js" | "jsx" | "mjs" | "cjs" => Language::JavaScript,
            "py" | "pyw" | "pyi" => Language::Python,
            "go" => Language::Go,
            "java" => Language::Java,
            "c" | "h" => Language::C,
            "cpp" | "cc" | "cxx" | "hpp" | "hxx" => Language::Cpp,
            "cs" => Language::CSharp,
            "rb" => Language::Ruby,
            "php" => Language::Php,
            "swift" => Language::Swift,
            "kt" | "kts" => Language::Kotlin,
            "scala" => Language::Scala,
            "hs" => Language::Haskell,
            "zig" => Language::Zig,
            "lua" => Language::Lua,
            "sh" | "bash" | "zsh" | "fish" => Language::Shell,
            "sql" => Language::Sql,
            "md" | "markdown" => Language::Markdown,
            "json" => Language::Json,
            "yaml" | "yml" => Language::Yaml,
            "toml" => Language::Toml,
            "proto" => Language::Protobuf,
            "graphql" | "gql" => Language::Graphql,
            "dockerfile" => Language::Dockerfile,
            "tf" | "tfvars" => Language::Terraform,
            _ => Language::Other,
        }
    }

    pub fn file_extensions(&self) -> &'static [&'static str] {
        match self {
            Language::Rust => &["rs"],
            Language::TypeScript => &["ts", "tsx"],
            Language::JavaScript => &["js", "jsx", "mjs", "cjs"],
            Language::Python => &["py", "pyw", "pyi"],
            Language::Go => &["go"],
            Language::Java => &["java"],
            Language::C => &["c", "h"],
            Language::Cpp => &["cpp", "cc", "cxx", "hpp", "hxx"],
            Language::CSharp => &["cs"],
            Language::Ruby => &["rb"],
            Language::Php => &["php"],
            Language::Swift => &["swift"],
            Language::Kotlin => &["kt", "kts"],
            Language::Scala => &["scala"],
            Language::Haskell => &["hs"],
            Language::Zig => &["zig"],
            Language::Lua => &["lua"],
            Language::Shell => &["sh", "bash", "zsh", "fish"],
            Language::Sql => &["sql"],
            Language::Markdown => &["md", "markdown"],
            Language::Json => &["json"],
            Language::Yaml => &["yaml", "yml"],
            Language::Toml => &["toml"],
            Language::Protobuf => &["proto"],
            Language::Graphql => &["graphql", "gql"],
            Language::Dockerfile => &["dockerfile"],
            Language::Terraform => &["tf", "tfvars"],
            _ => &[],
        }
    }

    pub fn tree_sitter_language(&self) -> Option<&'static str> {
        match self {
            Language::Rust => Some("rust"),
            Language::TypeScript => Some("typescript"),
            Language::JavaScript => Some("javascript"),
            Language::Python => Some("python"),
            Language::Go => Some("go"),
            Language::Java => Some("java"),
            Language::C => Some("c"),
            Language::Cpp => Some("cpp"),
            Language::CSharp => Some("c_sharp"),
            Language::Python => Some("python"),
            Language::Go => Some("go"),
            Language::Java => Some("java"),
            Language::Rust => Some("rust"),
            _ => None,
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Language::Rust => "Rust",
            Language::TypeScript => "TypeScript",
            Language::JavaScript => "JavaScript",
            Language::Python => "Python",
            Language::Go => "Go",
            Language::Java => "Java",
            Language::C => "C",
            Language::Cpp => "C++",
            Language::CSharp => "C#",
            Language::Python3 => "Python",
            Language::Ruby => "Ruby",
            Language::Php => "PHP",
            Language::Swift => "Swift",
            Language::Kotlin => "Kotlin",
            Language::Scala => "Scala",
            Language::Haskell => "Haskell",
            Language::Zig => "Zig",
            Language::Lua => "Lua",
            Language::Shell => "Shell",
            Language::Sql => "SQL",
            Language::Markdown => "Markdown",
            Language::Json => "JSON",
            Language::Yaml => "YAML",
            Language::Toml => "TOML",
            Language::Protobuf => "Protobuf",
            Language::Graphql => "GraphQL",
            Language::Dockerfile => "Dockerfile",
            Language::Terraform => "Terraform",
            Language::Other => "Other",
            Language::Unknown => "Unknown",
        };
        write!(f, "{}", s)
    }
}

/// File position (1-indexed, like editors)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub column: u32,
}

impl Position {
    pub fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }
}

/// A range in a source file
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

impl Range {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}

/// Content hash for integrity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContentHash(pub [u8; 32]);

impl ContentHash {
    pub fn new() -> Self {
        ContentHash([0; 32])
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        use blake3::Hasher;
        let mut hasher = Hasher::new();
        hasher.update(bytes);
        let hash = hasher.finalize();
        ContentHash(*hash.as_bytes())
    }

    pub fn as_hex(&self) -> String {
        hex::encode(self.0)
    }

    pub fn short(&self) -> String {
        hex::encode(&self.0[..8])
    }
}

impl Default for ContentHash {
    fn default() -> Self {
        ContentHash::new()
    }
}

impl std::fmt::Display for ContentHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_hex())
    }
}

/// Entity summary for agent consumption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySummary {
    pub id: EntityId,
    pub kind: SymbolKind,
    pub name: String,
    pub qualified_name: String,
    pub language: Language,
    pub range: Option<Range>,
    pub signature: Option<String>,
    pub documentation: Option<String>,
    pub confidence: Confidence,
    pub relation_counts: std::collections::HashMap<RelationKind, usize>,
}

/// Relation summary for agent context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationSummary {
    pub kind: RelationKind,
    pub target_id: EntityId,
    pub target_name: String,
    pub target_kind: SymbolKind,
    pub confidence: Confidence,
}

/// Query options for controlling retrieval
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryOptions {
    pub max_results: usize,
    pub include_relations: bool,
    pub relation_kinds: Option<Vec<RelationKind>>,
    pub max_depth: usize,
    pub min_confidence: Confidence,
    pub include_documentation: bool,
    pub include_signatures: bool,
    pub token_budget: usize,
}

impl QueryOptions {
    pub fn for_agent() -> Self {
        Self {
            max_results: 50,
            include_relations: true,
            max_depth: 2,
            min_confidence: Confidence::Medium,
            include_documentation: false,
            include_signatures: true,
            token_budget: 8192,
            ..Default::default()
        }
    }

    pub fn for_exploration() -> Self {
        Self {
            max_results: 100,
            include_relations: true,
            max_depth: 3,
            min_confidence: Confidence::Low,
            include_documentation: true,
            include_signatures: true,
            token_budget: 32768,
            ..Default::default()
        }
    }
}

/// Query result for agent consumption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub entity: EntitySummary,
    pub relations: Vec<RelationSummary>,
    pub score: f32,
}

/// Lightweight entity summary for agent context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySummary {
    pub id: EntityId,
    pub kind: SymbolKind,
    pub name: String,
    pub qualified_name: String,
    pub language: Language,
    pub range: Option<Range>,
    pub signature: Option<String>,
    pub documentation: Option<String>,
    pub confidence: Confidence,
    pub relation_counts: std::collections::HashMap<RelationKind, usize>,
}

/// Lightweight relation summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationSummary {
    pub kind: RelationKind,
    pub target_id: EntityId,
    pub target_name: String,
    pub target_kind: SymbolKind,
    pub confidence: Confidence,
}

/// Query result for agent consumption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub entity: EntitySummary,
    pub relations: Vec<RelationSummary>,
    pub score: f32,
}

/// Symbol kind - language-agnostic classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum SymbolKind {
    Module = 1,
    Namespace = 2,
    Package = 3,
    Class = 10,
    Struct = 11,
    Enum = 12,
    Trait = 13,
    Interface = 14,
    TypeAlias = 15,
    Protocol = 16,
    Function = 20,
    Method = 21,
    Constructor = 22,
    Destructor = 23,
    AsyncFunction = 24,
    AsyncMethod = 25,
    Variable = 30,
    Field = 31,
    Constant = 32,
    StaticVariable = 33,
    Parameter = 34,
    Macro = 40,
    Delegate = 41,
    Event = 42,
    Property = 43,
    GenericParameter = 44,
    ModuleImport = 45,
    ModuleExport = 46,
    Unknown = 255,
}

impl SymbolKind {
    pub fn is_callable(&self) -> bool {
        matches!(self,
            SymbolKind::Function | SymbolKind::Method |
            SymbolKind::Constructor | SymbolKind::AsyncFunction |
            SymbolKind::AsyncMethod
        )
    }

    pub fn is_type(&self) -> bool {
        matches!(self,
            SymbolKind::Class | SymbolKind::Struct | SymbolKind::Enum |
            SymbolKind::Trait | SymbolKind::Interface | SymbolKind::TypeAlias |
            SymbolKind::Protocol
        )
    }

    pub fn is_module(&self) -> bool {
        matches!(self, SymbolKind::Module | SymbolKind::Namespace | SymbolKind::Package)
    }
}

impl std::fmt::Display for SymbolKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SymbolKind::Module => "Module",
            SymbolKind::Namespace => "Namespace",
            SymbolKind::Package => "Package",
            SymbolKind::Class => "Class",
            SymbolKind::Struct => "Struct",
            SymbolKind::Enum => "Enum",
            SymbolKind::Trait => "Trait",
            SymbolKind::Interface => "Interface",
            SymbolKind::TypeAlias => "TypeAlias",
            SymbolKind::Protocol => "Protocol",
            SymbolKind::Function => "Function",
            SymbolKind::Method => "Method",
            SymbolKind::Constructor => "Constructor",
            SymbolKind::Destructor => "Destructor",
            SymbolKind::AsyncFunction => "AsyncFunction",
            SymbolKind::AsyncMethod => "AsyncMethod",
            SymbolKind::Variable => "Variable",
            SymbolKind::Field => "Field",
            SymbolKind::Constant => "Constant",
            SymbolKind::StaticVariable => "StaticVariable",
            SymbolKind::Parameter => "Parameter",
            SymbolKind::Macro => "Macro",
            SymbolKind::Delegate => "Delegate",
            SymbolKind::Event => "Event",
            SymbolKind::Property => "Property",
            SymbolKind::GenericParameter => "GenericParameter",
            SymbolKind::ModuleImport => "ModuleImport",
            SymbolKind::ModuleExport => "ModuleExport",
            SymbolKind::Unknown => "Unknown",
        };
        write!(f, "{}", s)
    }
}