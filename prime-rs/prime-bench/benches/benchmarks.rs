//! Benchmarks for Prime

use criterion::{criterion_group, criterion_main, Criterion};
use prime_core::{KnowledgeGraph, Project, Entity, EntityId, Relation, Language, SymbolKind, Confidence, Range, Position, RelationKind, Provenance, ContentHash, File, Module, ToolRequest, ToolIntent};
use prime_parser::{Parser, ParserConfig, ProjectAnalyzer};
use prime_index::{StorageManager, StorageConfig, QueryEngine, QueryOptions, StorageBackend, ToolExecutor};
use std::path::Path;
use std::time::Instant;
use tempfile::TempDir;
use walkdir::WalkDir;
use std::fs;

fn setup_test_project() -> TempDir {
    let dir = TempDir::new().unwrap();
    let root = dir.path();

    // Create a realistic test project structure
    fs::create_dir_all(root.join("src")).unwrap();
    fs::create_dir_all(root.join("src/utils")).unwrap();
    fs::create_dir_all(root.join("src/models")).unwrap();

    // Main file
    fs::write(root.join("src/main.rs"), r#"
mod utils;
mod models;

use models::User;
use utils::config::Config;

fn main() {
    let config = Config::load();
    let user = User::new("test");
    println!("Hello, {}!", user.name);
}
"#).unwrap();

    // Utils module
    fs::create_dir_all(root.join("src/utils")).unwrap();
    fs::write(root.join("src/utils/mod.rs"), r#"
pub mod config;

pub fn helper() -> i32 {
    42
}
"#).unwrap();

    fs::write(root.join("src/utils/config.rs"), r#"
pub struct Config {
    pub debug: bool,
}

impl Config {
    pub fn load() -> Self {
        Self { debug: true }
    }
}
"#).unwrap();

    // Models module
    fs::create_dir_all(root.join("src/models")).unwrap();
    fs::write(root.join("src/models/mod.rs"), r#"
pub struct User {
    pub name: String,
}

impl User {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }

    pub fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}
"#).unwrap();

    // Add more files to simulate larger project
    for i in 0..10 {
        fs::write(root.join(format!("src/module_{}.rs", i)), format!(r#"
pub struct Module{} {{
    pub value: i32,
}}

impl Module{} {{
    pub fn new() -> Self {{
        Self {{ value: {} }}
    }}

    pub fn compute(&self) -> i32 {{
        self.value * 2
    }}
}}
"#, i, i, i)).unwrap();
    }

    dir
}

fn bench_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("parsing");
    let test_dir = setup_test_project();
    let root = test_dir.path();

    let config = ParserConfig::default();

    group.bench_function("parse_project", |b| {
        b.iter(|| {
            let mut analyzer = ProjectAnalyzer::new(config.clone()).unwrap();
            let _graph = analyzer.analyze(root).unwrap();
        });
    });

    group.finish();
}

fn bench_indexing(c: &mut Criterion) {
    let mut group = c.benchmark_group("indexing");
    let test_dir = setup_test_project();
    let root = test_dir.path();

    let config = ParserConfig::default();
    let mut analyzer = ProjectAnalyzer::new(ParserConfig::default()).unwrap();
    let graph = analyzer.analyze(root).unwrap();

    let storage_config = prime_index::StorageConfig {
        path: std::path::PathBuf::from("/tmp/prime_bench"),
        compress: true,
        compression_level: 3,
        use_mmap: true,
        ..Default::default()
    };

    group.bench_function("save_index", |b| {
        b.iter(|| {
            let mut storage = prime_index::StorageManager::new(storage_config.clone());
            storage.save(&graph).unwrap();
        });
    });

    group.bench_function("load_index", |b| {
        let mut storage = prime_index::StorageManager::new(storage_config.clone());
        storage.save(&graph).unwrap();

        b.iter(|| {
            let mut storage = prime_index::StorageManager::new(storage_config.clone());
            storage.load().unwrap();
        });
    });

    group.finish();
}

fn bench_queries(c: &mut Criterion) {
    let mut group = c.benchmark_group("queries");
    let test_dir = setup_test_project();
    let root = test_dir.path();

    let mut analyzer = ProjectAnalyzer::new(ParserConfig::default()).unwrap();
    let graph = analyzer.analyze(root).unwrap();
    let engine = prime_index::QueryEngine::new(graph);

    let opts = prime_index::QueryOptions::for_agent();

    group.bench_function("find_by_name", |b| {
        b.iter(|| {
            engine.find_by_name("User", &prime_index::QueryOptions::for_agent());
        });
    });

    group.bench_function("find_by_prefix", |b| {
        b.iter(|| {
            engine.find_by_prefix("Mod", &prime_index::QueryOptions::for_agent());
        });
    });

    group.bench_function("search", |b| {
        b.iter(|| {
            engine.search("User", &prime_index::QueryOptions::for_agent());
        });
    });

    group.bench_function("get_context", |b| {
        let graph = engine.graph().clone();
        let entity_id = graph.find_by_qualified("src::main").unwrap_or(prime_core::EntityId::new());

        b.iter(|| {
            engine.get_context(entity_id, &prime_index::QueryOptions::for_agent());
        });
    });

    group.bench_function("surrounding_context", |b| {
        let graph = engine.graph().clone();
        let entity_id = graph.find_by_qualified("src::main").unwrap_or(prime_core::EntityId::new());

        b.iter(|| {
            engine.get_surrounding_context(entity_id, &prime_index::QueryOptions::for_agent());
        });
    });

    group.bench_function("progressive_context", |b| {
        let engine_arc = std::sync::Arc::new(engine.clone());
        let graph = engine_arc.graph().clone();
        let entity_id = graph.find_by_qualified("src::main").unwrap_or(prime_core::EntityId::new());

        b.iter(|| {
            let mut builder = prime_index::query::ProgressiveContextBuilder::new(engine_arc.clone(), 8192);
            builder.add_entity(entity_id);
            builder.expand_context(entity_id);
            builder.get_included();
        });
    });

    group.finish();
}

fn bench_storage(c: &mut Criterion) {
    let mut group = c.benchmark_group("storage");

    let test_dir = setup_test_project();
    let root = test_dir.path();
    let mut analyzer = ProjectAnalyzer::new(ParserConfig::default()).unwrap();
    let graph = analyzer.analyze(root).unwrap();

    let mut storage_config = prime_index::StorageConfig {
        path: std::path::PathBuf::from("/tmp/prime_bench"),
        compress: true,
        compression_level: 3,
        use_mmap: true,
        ..Default::default()
    };

    group.bench_function("binary_save", |b| {
        b.iter(|| {
            let mut storage = prime_index::BinaryStorage::new(storage_config.clone());
            storage.save(&graph).unwrap();
        });
    });

    group.bench_function("binary_load", |b| {
        let mut storage = prime_index::BinaryStorage::new(storage_config.clone());
        storage.save(&graph).unwrap();

        b.iter(|| {
            let mut storage = prime_index::BinaryStorage::new(storage_config.clone());
            storage.load().unwrap();
        });
    });

    group.bench_function("mmap_save", |b| {
        b.iter(|| {
            let mut storage = prime_index::MmapStorage::new(storage_config.clone());
            storage.save(&graph).unwrap();
        });
    });

    group.bench_function("mmap_load", |b| {
        let mut storage = prime_index::MmapStorage::new(storage_config.clone());
        storage.save(&graph).unwrap();

        b.iter(|| {
            let mut storage = prime_index::MmapStorage::new(storage_config.clone());
            storage.load().unwrap();
        });
    });

    group.finish();
}

fn bench_incremental(c: &mut Criterion) {
    let mut group = c.benchmark_group("incremental");

    let test_dir = setup_test_project();
    let root = test_dir.path();
    let mut analyzer = ProjectAnalyzer::new(ParserConfig::default()).unwrap();
    let mut graph = analyzer.analyze(root).unwrap();

    let storage_config = prime_index::StorageConfig {
        path: std::path::PathBuf::from("/tmp/prime_bench_incremental"),
        compress: true,
        compression_level: 3,
        use_mmap: true,
        ..Default::default()
    };

    group.bench_function("delta_write", |b| {
        let mut delta = prime_index::StorageDelta::new();
        delta.add_entity(prime_core::Entity {
            id: prime_core::EntityId::new(),
            kind: prime_core::SymbolKind::Function,
            name: "new_func".to_string(),
            qualified_name: "test::new_func".to_string(),
            file_id: prime_core::EntityId::new(),
            range: prime_core::Range { start: prime_core::Position { line: 1, column: 1 }, end: prime_core::Position { line: 10, column: 1 } },
            language: prime_core::Language::Rust,
            confidence: prime_core::Confidence::High,
            signature: Some("fn new_func() -> i32".to_string()),
            documentation: None,
            children: Vec::new(),
            relations: Vec::new(),
        });

        let mut storage = prime_index::IncrementalStorage::new(
            std::path::PathBuf::from("/tmp/prime_bench_incremental")
        );

        b.iter(|| {
            storage.write_delta(&delta).unwrap();
        });
    });

    group.bench_function("delta_apply", |b| {
        let mut delta = prime_index::StorageDelta::new();
        delta.add_entity(prime_core::Entity {
            id: prime_core::EntityId::new(),
            kind: prime_core::SymbolKind::Function,
            name: "new_func".to_string(),
            qualified_name: "test::new_func".to_string(),
            file_id: prime_core::EntityId::new(),
            range: prime_core::Range { start: prime_core::Position { line: 1, column: 1 }, end: prime_core::Position { line: 10, column: 1 } },
            language: prime_core::Language::Rust,
            confidence: prime_core::Confidence::High,
            signature: Some("fn new_func() -> i32".to_string()),
            documentation: None,
            children: Vec::new(),
            relations: Vec::new(),
        });

        let mut storage = prime_index::IncrementalStorage::new(
            std::path::PathBuf::from("/tmp/prime_bench_incremental")
        );

        b.iter(|| {
            let mut graph = graph.clone();
            storage.write_delta(&delta).unwrap();
            storage.apply_delta(&mut graph).unwrap();
        });
    });

    group.finish();
}

fn bench_tool_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("tool_operations");
    let test_dir = setup_test_project();
    let root = test_dir.path();

    let mut analyzer = ProjectAnalyzer::new(ParserConfig::default()).unwrap();
    let graph = analyzer.analyze(root).unwrap();
    let executor = ToolExecutor::from_graph(graph);

    group.bench_function("prime_search", |b| {
        b.iter(|| {
            let req = ToolRequest {
                intent: ToolIntent::Search,
                target: Some("User".to_string()),
                limit: 10,
                ..Default::default()
            };
            executor.execute(&req);
        });
    });

    group.bench_function("prime_lookup", |b| {
        b.iter(|| {
            let req = ToolRequest {
                intent: ToolIntent::Lookup,
                target: Some("src::models::User".to_string()),
                ..Default::default()
            };
            executor.execute(&req);
        });
    });

    group.bench_function("prime_context", |b| {
        b.iter(|| {
            let req = ToolRequest {
                intent: ToolIntent::Context,
                target: Some("src::models::User".to_string()),
                depth: 1,
                token_budget: 8192,
                ..Default::default()
            };
            executor.execute(&req);
        });
    });

    group.bench_function("prime_relationships", |b| {
        b.iter(|| {
            let req = ToolRequest {
                intent: ToolIntent::Relationships,
                target: Some("src::models::User".to_string()),
                dimensions: vec!["callers".to_string(), "callees".to_string()],
                ..Default::default()
            };
            executor.execute(&req);
        });
    });

    group.bench_function("prime_dependencies", |b| {
        b.iter(|| {
            let req = ToolRequest {
                intent: ToolIntent::Dependencies,
                target: Some("src::models::User".to_string()),
                ..Default::default()
            };
            executor.execute(&req);
        });
    });

    group.bench_function("prime_impact", |b| {
        b.iter(|| {
            let req = ToolRequest {
                intent: ToolIntent::Impact,
                target: Some("src::models::User".to_string()),
                ..Default::default()
            };
            executor.execute(&req);
        });
    });

    group.bench_function("prime_architecture", |b| {
        b.iter(|| {
            let req = ToolRequest {
                intent: ToolIntent::Architecture,
                target: Some("".to_string()),
                ..Default::default()
            };
            executor.execute(&req);
        });
    });

    group.finish();
}

fn bench_large_project(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_project");
    group.sample_size(10);

    let dir = TempDir::new().unwrap();
    let root = dir.path();

    // Generate a large project with many files
    fs::create_dir_all(root.join("src")).unwrap();

    // Generate500 Rust files with entities
    for i in 0..500 {
        let code = format!(r#"
pub struct Struct{i} {{
    pub field_a: i32,
    pub field_b: String,
}}

impl Struct{i} {{
    pub fn new(a: i32, b: &str) -> Self {{
        Self {{ field_a: a, field_b: b.to_string() }}
    }}

    pub fn compute(&self, x: i32) -> i32 {{
        self.field_a + x
    }}

    pub fn process(&self, items: &[i32]) -> Vec<i32> {{
        items.iter().map(|&v| self.compute(v)).collect()
    }}
}}

pub fn function_{i}(input: &str) -> Result<String, Box<dyn std::error::Error>> {{
    let result = input.to_uppercase();
    Ok(result)
}}

pub trait Trait{i} {{
    fn method_a(&self) -> i32;
    fn method_b(&self, input: &str) -> String;
}}

impl Trait{i} for Struct{i} {{
    fn method_a(&self) -> i32 {{
        self.field_a
    }}

    fn method_b(&self, input: &str) -> String {{
        format!("{{}}-{{}}", self.field_b, input)
    }}
}}
"#);
        fs::write(root.join(format!("src/file_{}.rs", i)), code).unwrap();
    }

    // Generate mod.rs
    let mod_content: String = (0..500)
        .map(|i| format!("pub mod file_{};", i))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(root.join("src/mod.rs"), mod_content).unwrap();

    let mut analyzer = ProjectAnalyzer::new(ParserConfig::default()).unwrap();

    group.bench_function("parse_500_files", |b| {
        b.iter(|| {
            let mut analyzer = ProjectAnalyzer::new(ParserConfig::default()).unwrap();
            let _graph = analyzer.analyze(root).unwrap();
        });
    });

    let graph = analyzer.analyze(root).unwrap();
    let executor = ToolExecutor::from_graph(graph);

    group.bench_function("search_500_files", |b| {
        b.iter(|| {
            let req = ToolRequest {
                intent: ToolIntent::Search,
                target: Some("Struct".to_string()),
                limit: 50,
                ..Default::default()
            };
            executor.execute(&req);
        });
    });

    group.bench_function("context_500_files", |b| {
        b.iter(|| {
            let req = ToolRequest {
                intent: ToolIntent::Context,
                target: Some("src::file_0::Struct0".to_string()),
                depth: 2,
                token_budget: 16384,
                ..Default::default()
            };
            executor.execute(&req);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_parsing,
    bench_indexing,
    bench_queries,
    bench_storage,
    bench_incremental,
    bench_tool_operations,
    bench_large_project
);
criterion_main!(benches);