//! Language definitions and capabilities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Programming language
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
    PHP = 12,
    Swift = 13,
    Kotlin = 14,
    Scala = 15,
    Haskell = 16,
    Zig = 17,
    Ziggy = 18,
    Lua = 19,
    Shell = 20,
    SQL = 21,
    Markdown = 22,
    JSON = 23,
    YAML = 24,
    TOML = 25,
    Protobuf = 26,
    GraphQL = 27,
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
            "php" => Language::PHP,
            "swift" => Language::Swift,
            "kt" | "kts" => Language::Kotlin,
            "scala" => Language::Scala,
            "hs" => Language::Haskell,
            "zig" => Language::Zig,
            "lua" => Language::Lua,
            "sh" | "bash" | "zsh" | "fish" => Language::Shell,
            "sql" => Language::SQL,
            "md" | "markdown" => Language::Markdown,
            "json" => Language::JSON,
            "yaml" | "yml" => Language::YAML,
            "toml" => Language::TOML,
            "proto" => Language::Protobuf,
            "graphql" | "gql" => Language::GraphQL,
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
            Language::PHP => &["php"],
            Language::Swift => &["swift"],
            Language::Kotlin => &["kt", "kts"],
            Language::Scala => &["scala"],
            Language::Haskell => &["hs"],
            Language::Zig => &["zig"],
            Language::Lua => &["lua"],
            Language::Shell => &["sh", "bash", "zsh", "fish"],
            Language::SQL => &["sql"],
            Language::Markdown => &["md", "markdown"],
            Language::JSON => &["json"],
            Language::YAML => &["yaml", "yml"],
            Language::TOML => &["toml"],
            Language::Protobuf => &["proto"],
            Language::GraphQL => &["graphql", "gql"],
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
            Language::Ruby => Some("ruby"),
            Language::PHP => Some("php"),
            Language::Swift => Some("swift"),
            Language::Kotlin => Some("kotlin"),
            Language::Scala => Some("scala"),
            Language::Haskell => Some("haskell"),
            Language::Zig => Some("zig"),
            Language::Lua => Some("lua"),
            Language::Shell => Some("bash"),
            Language::SQL => Some("sql"),
            Language::Markdown => Some("markdown"),
            Language::JSON => Some("json"),
            Language::YAML => Some("yaml"),
            Language::TOML => Some("toml"),
            Language::Protobuf => Some("protobuf"),
            Language::GraphQL => Some("graphql"),
            Language::Dockerfile => Some("dockerfile"),
            Language::Terraform => Some("terraform"),
            _ => None,
        }
    }

    /// Capabilities this language supports for analysis
    pub fn capabilities(&self) -> LanguageCapabilities {
        match self {
            Language::Rust => LanguageCapabilities {
                parsing: Capability::Exact,
                symbols: Capability::Exact,
                references: Capability::Exact,
                types: Capability::Exact,
                calls: Capability::Exact,
                data_flow: Capability::High,
                control_flow: Capability::High,
                architecture: Capability::Inferred,
                runtime_behavior: Capability::Unknown,
            },
            Language::TypeScript => LanguageCapabilities {
                parsing: Capability::Exact,
                symbols: Capability::Exact,
                references: Capability::High,
                types: Capability::High,
                calls: Capability::High,
                data_flow: Capability::Medium,
                control_flow: Capability::Medium,
                architecture: Capability::Inferred,
                runtime_behavior: Capability::Unknown,
            },
            Language::Python => LanguageCapabilities {
                parsing: Capability::Exact,
                symbols: Capability::Exact,
                references: Capability::Medium,
                types: Capability::Medium,
                calls: Capability::Medium,
                data_flow: Capability::Low,
                control_flow: Capability::Medium,
                architecture: Capability::Inferred,
                runtime_behavior: Capability::Unknown,
            },
            Language::Go => LanguageCapabilities {
                parsing: Capability::Exact,
                symbols: Capability::Exact,
                references: Capability::Exact,
                types: Capability::High,
                calls: Capability::Exact,
                data_flow: Capability::High,
                control_flow: Capability::High,
                architecture: Capability::Inferred,
                runtime_behavior: Capability::Unknown,
            },
            Language::Java | Language::CSharp => LanguageCapabilities {
                parsing: Capability::Exact,
                symbols: Capability::Exact,
                references: Capability::Exact,
                types: Capability::Exact,
                calls: Capability::Exact,
                data_flow: Capability::High,
                control_flow: Capability::High,
                architecture: Capability::Inferred,
                runtime_behavior: Capability::Unknown,
            },
            Language::C | Language::Cpp => LanguageCapabilities {
                parsing: Capability::High,
                symbols: Capability::High,
                references: Capability::High,
                types: Capability::Medium,
                calls: Capability::High,
                data_flow: Capability::Medium,
                control_flow: Capability::Medium,
                architecture: Capability::Inferred,
                runtime_behavior: Capability::Unknown,
            },
            Language::Go => LanguageCapabilities {
                parsing: Capability::Exact,
                symbols: Capability::Exact,
                references: Capability::Exact,
                types: Capability::High,
                calls: Capability::Exact,
                data_flow: Capability::High,
                control_flow: Capability::High,
                architecture: Capability::Inferred,
                runtime_behavior: Capability::Unknown,
            },
            _ => LanguageCapabilities::default(),
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::Unknown
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Language::Rust => "Rust",
            Language::TypeScript => "TypeScript",
            Language::JavaScript => "JavaScript",
            Language::Python => "Python",
            Language::Go => "Go",
            Language::Java => "Java",
            Language::C => "C",
            Language::Cpp => "C++",
            Language::CSharp => "C#",
            Language::Python3 => "Python 3",
            Language::Ruby => "Ruby",
            Language::PHP => "PHP",
            Language::Swift => "Swift",
            Language::Kotlin => "Kotlin",
            Language::Scala => "Scala",
            Language::Haskell => "Haskell",
            Language::Zig => "Zig",
            Language::Ziggy => "Zig",
            Language::Lua => "Lua",
            Language::Shell => "Shell",
            Language::SQL => "SQL",
            Language::Markdown => "Markdown",
            Language::JSON => "JSON",
            Language::YAML => "YAML",
            Language::TOML => "TOML",
            Language::Protobuf => "Protobuf",
            Language::GraphQL => "GraphQL",
            Language::Dockerfile => "Dockerfile",
            Language::Terraform => "Terraform",
            Language::Other => "Other",
            Language::Unknown => "Unknown",
        };
        write!(f, "{}", name)
    }
}

/// Capability level for a language feature
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(u8)]
pub enum Capability {
    Unknown = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    Exact = 4,
    Inferred = 5,
}

impl Default for Capability {
    fn default() -> Self {
        Capability::Unknown
    }
}

/// Language capability matrix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageCapabilities {
    pub parsing: Capability,
    pub symbols: Capability,
    pub references: Capability,
    pub types: Capability,
    pub calls: Capability,
    pub data_flow: Capability,
    pub control_flow: Capability,
    pub architecture: Capability,
    pub runtime_behavior: Capability,
}

impl Default for LanguageCapabilities {
    fn default() -> Self {
        Self {
            parsing: Capability::Unknown,
            symbols: Capability::Unknown,
            references: Capability::Unknown,
            types: Capability::Unknown,
            calls: Capability::Unknown,
            data_flow: Capability::Unknown,
            control_flow: Capability::Unknown,
            architecture: Capability::Unknown,
            runtime_behavior: Capability::Unknown,
        }
    }
}

/// Language registry for the project
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LanguageRegistry {
    pub languages: HashMap<Language, LanguageCapabilities>,
    pub file_to_language: HashMap<String, Language>,  // extension -> language
}

impl LanguageRegistry {
    pub fn new() -> Self {
        let mut registry = Self::default();
        registry.register_all();
        registry
    }

    fn register_all(&mut self) {
        for lang in [
            Language::Rust, Language::TypeScript, Language::JavaScript,
            Language::Python, Language::Go, Language::Java, Language::C,
            Language::Cpp, Language::CSharp, Language::Ruby, Language::PHP,
            Language::Swift, Language::Kotlin, Language::Scala, Language::Haskell,
        ] {
            let caps = lang.capabilities();
            for ext in lang.file_extensions() {
                self.file_to_language.insert(ext.to_string(), lang);
            }
            self.languages.insert(lang, caps);
        }
    }

    pub fn detect(&self, path: &str) -> Language {
        if let Some(ext) = std::path::Path::new(path).extension()
            .and_then(|e| e.to_str()) {
            self.file_to_language.get(ext).copied().unwrap_or(Language::Unknown)
        } else {
            Language::Unknown
        }
    }

    pub fn capabilities(&self, lang: Language) -> Option<&LanguageCapabilities> {
        self.languages.get(&lang)
    }
}

impl std::str::FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "unknown" => Ok(Language::Unknown),
            "rust" => Ok(Language::Rust),
            "typescript" => Ok(Language::TypeScript),
            "javascript" => Ok(Language::JavaScript),
            "python" => Ok(Language::Python),
            "go" => Ok(Language::Go),
            "java" => Ok(Language::Java),
            "c" => Ok(Language::C),
            "cpp" => Ok(Language::Cpp),
            "csharp" => Ok(Language::CSharp),
            "python3" => Ok(Language::Python3),
            "ruby" => Ok(Language::Ruby),
            "php" => Ok(Language::PHP),
            "swift" => Ok(Language::Swift),
            "kotlin" => Ok(Language::Kotlin),
            "scala" => Ok(Language::Scala),
            "haskell" => Ok(Language::Haskell),
            "zig" => Ok(Language::Zig),
            "ziggy" => Ok(Language::Ziggy),
            "lua" => Ok(Language::Lua),
            "shell" => Ok(Language::Shell),
            "sql" => Ok(Language::SQL),
            "markdown" => Ok(Language::Markdown),
            "json" => Ok(Language::JSON),
            "yaml" => Ok(Language::YAML),
            "toml" => Ok(Language::TOML),
            "protobuf" => Ok(Language::Protobuf),
            "graphql" => Ok(Language::GraphQL),
            "dockerfile" => Ok(Language::Dockerfile),
            "terraform" => Ok(Language::Terraform),
            "other" => Ok(Language::Other),
            _ => Err(format!("Unknown language: {}", s)),
        }
    }
}