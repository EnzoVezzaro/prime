//! Language-specific Tree-sitter configuration

use tree_sitter::Language as TSLanguage;
use prime_core::Language;
use std::collections::HashMap;
use std::sync::Arc;

/// Language-specific parser configuration
pub struct LanguageConfig {
    pub language: Language,
    pub ts_language: TSLanguage,
    pub queries: LanguageQueries,
}

/// Tree-sitter queries for extracting entities and relations
pub struct LanguageQueries {
    pub definitions: String,
    pub calls: String,
    pub references: String,
    pub imports: String,
    pub types: String,
}

/// Registry of all language configurations
pub struct LanguageConfigRegistry {
    configs: HashMap<prime_core::Language, Arc<LanguageConfig>>,
}

impl LanguageConfigRegistry {
    pub fn new() -> anyhow::Result<Self> {
        let mut configs = HashMap::new();

        // Rust
        configs.insert(
            prime_core::Language::Rust,
            Arc::new(Self::rust_config()?),
        );

        // TypeScript
        configs.insert(
            prime_core::Language::TypeScript,
            Arc::new(Self::typescript_config()?),
        );

        // JavaScript
        configs.insert(
            prime_core::Language::JavaScript,
            Arc::new(Self::javascript_config()?),
        );

        // Python
        configs.insert(
            prime_core::Language::Python,
            Arc::new(Self::python_config()?),
        );

        // Go
        configs.insert(
            prime_core::Language::Go,
            Arc::new(Self::go_config()?),
        );

        // Java
        configs.insert(
            prime_core::Language::Java,
            Arc::new(Self::java_config()?),
        );

        // C
        configs.insert(
            prime_core::Language::C,
            Arc::new(Self::c_config()?),
        );

        // C++
        configs.insert(
            prime_core::Language::Cpp,
            Arc::new(Self::cpp_config()?),
        );

        // C#
        configs.insert(
            prime_core::Language::CSharp,
            Arc::new(Self::csharp_config()?),
        );

        // Python3 (alias)
        configs.insert(
            prime_core::Language::Python3,
            configs.get(&prime_core::Language::Python).unwrap().clone(),
        );

        Ok(Self { configs })
    }

    pub fn get(&self, lang: prime_core::Language) -> Option<Arc<LanguageConfig>> {
        self.configs.get(&lang).cloned()
    }

    pub fn detect(&self, ext: &str) -> prime_core::Language {
        match ext.to_lowercase().as_str() {
            "rs" => prime_core::Language::Rust,
            "ts" | "tsx" => prime_core::Language::TypeScript,
            "js" | "jsx" | "mjs" | "cjs" => prime_core::Language::JavaScript,
            "py" | "pyw" | "pyi" => prime_core::Language::Python,
            "go" => prime_core::Language::Go,
            "java" => prime_core::Language::Java,
            "c" | "h" => prime_core::Language::C,
            "cpp" | "cc" | "cxx" | "hpp" | "hxx" => prime_core::Language::Cpp,
            "cs" => prime_core::Language::CSharp,
            "rb" => prime_core::Language::Ruby,
            "php" => prime_core::Language::PHP,
            "swift" => prime_core::Language::Swift,
            "kt" | "kts" => prime_core::Language::Kotlin,
            "scala" => prime_core::Language::Scala,
            "hs" => prime_core::Language::Haskell,
            "zig" => prime_core::Language::Zig,
            "lua" => prime_core::Language::Lua,
            "sh" | "bash" | "zsh" | "fish" => prime_core::Language::Shell,
            "sql" => prime_core::Language::SQL,
            "md" | "markdown" => prime_core::Language::Markdown,
            "json" => prime_core::Language::JSON,
            "yaml" | "yml" => prime_core::Language::YAML,
            "toml" => prime_core::Language::TOML,
            "proto" => prime_core::Language::Protobuf,
            "graphql" | "gql" => prime_core::Language::GraphQL,
            "dockerfile" => prime_core::Language::Dockerfile,
            "tf" | "tfvars" => prime_core::Language::Terraform,
            _ => prime_core::Language::Unknown,
        }
    }

    fn rust_config() -> anyhow::Result<LanguageConfig> {
        let language = tree_sitter_rust::LANGUAGE.into();
        Ok(LanguageConfig {
            language: prime_core::Language::Rust,
            ts_language: language,
            queries: LanguageQueries {
                definitions: include_str!("queries/rust_definitions.scm").to_string(),
                calls: include_str!("queries/rust_calls.scm").to_string(),
                references: include_str!("queries/rust_references.scm").to_string(),
                imports: include_str!("queries/rust_imports.scm").to_string(),
                types: include_str!("queries/rust_types.scm").to_string(),
            },
        })
    }

    fn typescript_config() -> anyhow::Result<LanguageConfig> {
        let language = tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into();
        Ok(LanguageConfig {
            language: prime_core::Language::TypeScript,
            ts_language: language,
            queries: LanguageQueries {
                definitions: include_str!("queries/typescript_definitions.scm").to_string(),
                calls: include_str!("queries/typescript_calls.scm").to_string(),
                references: include_str!("queries/typescript_references.scm").to_string(),
                imports: include_str!("queries/typescript_imports.scm").to_string(),
                types: include_str!("queries/typescript_types.scm").to_string(),
            },
        })
    }

    fn javascript_config() -> anyhow::Result<LanguageConfig> {
        let language = tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into();
        Ok(LanguageConfig {
            language: prime_core::Language::JavaScript,
            ts_language: language,
            queries: LanguageQueries {
                definitions: include_str!("queries/javascript_definitions.scm").to_string(),
                calls: include_str!("queries/javascript_calls.scm").to_string(),
                references: include_str!("queries/javascript_references.scm").to_string(),
                imports: include_str!("queries/javascript_imports.scm").to_string(),
                types: include_str!("queries/javascript_types.scm").to_string(),
            },
        })
    }

    fn python_config() -> anyhow::Result<LanguageConfig> {
        let language = tree_sitter_python::LANGUAGE.into();
        Ok(LanguageConfig {
            language: prime_core::Language::Python,
            ts_language: language,
            queries: LanguageQueries {
                definitions: include_str!("queries/python_definitions.scm").to_string(),
                calls: include_str!("queries/python_calls.scm").to_string(),
                references: include_str!("queries/python_references.scm").to_string(),
                imports: include_str!("queries/python_imports.scm").to_string(),
                types: include_str!("queries/python_types.scm").to_string(),
            },
        })
    }

    fn go_config() -> anyhow::Result<LanguageConfig> {
        let language = tree_sitter_go::LANGUAGE.into();
        Ok(LanguageConfig {
            language: prime_core::Language::Go,
            ts_language: language,
            queries: LanguageQueries {
                definitions: include_str!("queries/go_definitions.scm").to_string(),
                calls: include_str!("queries/go_calls.scm").to_string(),
                references: include_str!("queries/go_references.scm").to_string(),
                imports: include_str!("queries/go_imports.scm").to_string(),
                types: include_str!("queries/go_types.scm").to_string(),
            },
        })
    }

    fn java_config() -> anyhow::Result<LanguageConfig> {
        let language = tree_sitter_java::LANGUAGE.into();
        Ok(LanguageConfig {
            language: prime_core::Language::Java,
            ts_language: language,
            queries: LanguageQueries {
                definitions: include_str!("queries/java_definitions.scm").to_string(),
                calls: include_str!("queries/java_calls.scm").to_string(),
                references: include_str!("queries/java_references.scm").to_string(),
                imports: include_str!("queries/java_imports.scm").to_string(),
                types: include_str!("queries/java_types.scm").to_string(),
            },
        })
    }

    fn c_config() -> anyhow::Result<LanguageConfig> {
        let language = tree_sitter_c::LANGUAGE.into();
        Ok(LanguageConfig {
            language: prime_core::Language::C,
            ts_language: language,
            queries: LanguageQueries {
                definitions: include_str!("queries/c_definitions.scm").to_string(),
                calls: include_str!("queries/c_calls.scm").to_string(),
                references: include_str!("queries/c_references.scm").to_string(),
                imports: include_str!("queries/c_imports.scm").to_string(),
                types: include_str!("queries/c_types.scm").to_string(),
            },
        })
    }

    fn cpp_config() -> anyhow::Result<LanguageConfig> {
        let language = tree_sitter_cpp::LANGUAGE.into();
        Ok(LanguageConfig {
            language: prime_core::Language::Cpp,
            ts_language: language,
            queries: LanguageQueries {
                definitions: include_str!("queries/cpp_definitions.scm").to_string(),
                calls: include_str!("queries/cpp_calls.scm").to_string(),
                references: include_str!("queries/cpp_references.scm").to_string(),
                imports: include_str!("queries/cpp_imports.scm").to_string(),
                types: include_str!("queries/cpp_types.scm").to_string(),
            },
        })
    }

    fn csharp_config() -> anyhow::Result<LanguageConfig> {
        // tree-sitter-c-sharp would be needed
        // For now, create a minimal config
        let language = tree_sitter_c::LANGUAGE.into(); // placeholder
        Ok(LanguageConfig {
            language: prime_core::Language::CSharp,
            ts_language: language,
            queries: LanguageQueries {
                definitions: String::new(),
                calls: String::new(),
                references: String::new(),
                imports: String::new(),
                types: String::new(),
            },
        })
    }
}