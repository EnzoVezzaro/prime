/// Prime CLI - Build and query codebase knowledge graphs

use clap::{Parser as ClapParser, Subcommand};
use prime_core::{KnowledgeGraph, Language, Confidence, RelationKind, EntityId, SymbolKind, ToolRequest, ToolIntent, RelationScope, DetailLevel};
use prime_parser::{Parser as PrimeParser, ParserConfig};
use prime_index::{StorageManager, StorageConfig, QueryEngine, QueryOptions, EntitySummary, ToolExecutor};
use std::path::{Path, PathBuf};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use std::thread;
use std::fs;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use chrono;

#[derive(ClapParser)]
#[command(name = "prime")]
#[command(about = "Prime - Ultra-light, ultra-fast codebase knowledge graph for agents", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Project root directory
    #[arg(short, long, default_value = ".")]
    root: PathBuf,

    /// Storage directory
    #[arg(short, long, default_value = ".prime")]
    storage: PathBuf,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the knowledge graph from source code
    Build {
        /// Force rebuild (ignore existing index)
        #[arg(short, long)]
        force: bool,

        /// Languages to parse (comma-separated)
        #[arg(long)]
        languages: Option<String>,

        /// Show progress bar
        #[arg(short, long, default_value = "true")]
        progress: bool,
    },

    /// Update the knowledge graph incrementally
    Update {
        /// Specific files to update (if empty, detects all changes)
        #[arg(short, long)]
        files: Vec<PathBuf>,

        /// Show progress bar
        #[arg(short, long, default_value = "true")]
        progress: bool,
    },

    /// Query the knowledge graph
    Query {
        /// Query string (symbol name, prefix, or search term)
        query: String,

        /// Query type: name, prefix, search, context
        #[arg(short, long, default_value = "search")]
        query_type: String,

        /// Output format: json, text, minimal
        #[arg(short, long, default_value = "text")]
        format: String,

        /// Maximum results
        #[arg(short, long, default_value = "20")]
        limit: usize,

        /// Include relations
        #[arg(long, default_value = "true")]
        relations: bool,

        /// Max depth for context expansion
        #[arg(long, default_value = "2")]
        depth: usize,

        /// Minimum confidence level
        #[arg(long, default_value = "medium")]
        confidence: String,

        /// Token budget for agent context
        #[arg(long, default_value = "8192")]
        token_budget: usize,
    },

    /// Show graph statistics
    Stats {},

    /// Check for drift between code and index
    Check {},

    /// Initialize storage directory
    Init {
        /// Force reinitialize
        #[arg(short, long)]
        force: bool,
    },

    /// Show dependencies for a symbol
    Deps {
        symbol: String,
        #[arg(short, long, default_value = "true")]
        transitive: bool,
    },

    /// Show dependents (reverse deps) for a symbol
    Dependents {
        symbol: String,
        #[arg(short, long, default_value = "true")]
        transitive: bool,
    },

    /// Show call graph for a symbol
    Calls {
        symbol: String,
        #[arg(short, long, default_value = "callers")]
        direction: String,
    },

    /// Export graph to various formats
    Export {
        #[arg(short, long, default_value = "json")]
        format: String,

        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Start the MCP server (stdio transport)
    Serve {},

    /// Inspect an entity with agent-native envelope
    Inspect {
        /// Entity qualified name
        entity: String,

        /// Output format: json, text
        #[arg(short, long, default_value = "text")]
        format: String,
    },

    /// Run comprehensive benchmark producing canonical result JSON
    Benchmark {
        /// Benchmark corpus: pr, nightly, or path to repos dir
        #[arg(long, default_value = "pr")]
        corpus: String,

        /// Output path for result JSON
        #[arg(short, long, default_value = "benchmarks/results/latest.json")]
        output: PathBuf,

        /// Storage directory for built indexes
        #[arg(long, default_value = "/tmp/prime-bench-storage")]
        bench_storage: PathBuf,
    },

    /// Prepare benchmark corpus (clone repositories at pinned commits)
    Prepare {
        /// Benchmark corpus: pr, nightly, or all
        #[arg(long, default_value = "pr")]
        corpus: String,

        /// Output directory for repositories
        #[arg(long, default_value = "benchmarks/repos")]
        output: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build { force, languages, progress } => {
            cmd_build(&cli.root, &cli.storage, force, languages, progress, cli.verbose)
        }
        Commands::Update { files, progress } => {
            cmd_update(&cli.root, &cli.storage, &files, progress)
        }
        Commands::Query { query, query_type, format, limit, relations, depth, confidence, token_budget } => {
            cmd_query(&cli.storage, &query, &query_type, &format, limit, relations, depth, &confidence, token_budget)
        }
        Commands::Stats {} => cmd_stats(&cli.storage),
        Commands::Check {} => cmd_check(&cli.storage),
        Commands::Init { force } => cmd_init(&cli.storage, force),
        Commands::Deps { symbol, transitive } => cmd_deps(&cli.storage, &symbol, transitive),
        Commands::Dependents { symbol, transitive } => cmd_dependents(&cli.storage, &symbol, transitive),
        Commands::Calls { symbol, direction } => cmd_calls(&cli.storage, &symbol, &direction),
        Commands::Export { format, output } => cmd_export(&cli.storage, &format, output.as_deref()),
        Commands::Serve {} => cmd_serve(&cli.storage),
        Commands::Inspect { entity, format } => cmd_inspect(&cli.storage, &entity, &format),
        Commands::Benchmark { corpus, output, bench_storage } => cmd_benchmark(&cli.root, &cli.storage, &corpus, &output, &bench_storage),
        Commands::Prepare { corpus, output } => cmd_prepare(&corpus, &output),
    }
}

fn cmd_build(root: &Path, storage: &Path, force: bool, _languages: Option<String>, progress: bool, _verbose: bool) -> anyhow::Result<()> {
    let storage_path = storage.join("graph.bin");
    if storage_path.exists() && !force {
        println!("Index already exists. Use --force to rebuild.");
        return Ok(());
    }

    let config = ParserConfig {
        max_file_size: 1024 * 1024,
        ..Default::default()
    };

    let _parser = PrimeParser::new(config.clone())?;
    let mut analyzer = prime_parser::ProjectAnalyzer::new(config)?;

    println!("Building knowledge graph for: {}", root.display());

    let start = Instant::now();

    let progress_bar = if progress {
        Some(ProgressBar::new_spinner().with_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap()
        ))
    } else {
        None
    };

    if let Some(pb) = &progress_bar {
        pb.set_message("Parsing project...");
    }

    let graph = analyzer.analyze(root)?;

    if let Some(pb) = &progress_bar {
        pb.set_message("Saving index...");
    }

    let storage_config = StorageConfig {
        path: storage.to_path_buf(),
        compress: true,
        compression_level: 3,
        use_mmap: true,
        ..Default::default()
    };

    let mut storage_mgr = StorageManager::new(storage_config);
    storage_mgr.save(&graph)?;

    if let Some(pb) = &progress_bar {
        pb.finish_with_message("Done!");
    }

    let elapsed = start.elapsed();
    println!("Build completed in {:.2}s", elapsed.as_secs_f64());
    println!("  Files: {}", graph.project.file_count);
    println!("  Entities: {}", graph.project.entity_count);
    println!("  Relations: {}", graph.project.relation_count);
    println!("  Languages: {:?}", graph.project.languages);
    println!("  Index size: {} MB", storage_mgr.size() as f64 / 1024.0 / 1024.0);

    Ok(())
}

fn cmd_update(root: &Path, storage: &Path, files: &[PathBuf], progress: bool) -> anyhow::Result<()> {
    use prime_index::IncrementalIndexer;
    use prime_core::Language;
    use prime_parser::ParserConfig;

    let storage_path = storage.join("graph.bin");
    if !storage_path.exists() {
        println!("No existing index found. Run 'prime build' first.");
        return Ok(());
    }

    let start = Instant::now();

    let progress_bar = if progress {
        Some(ProgressBar::new_spinner().with_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap()
        ))
    } else {
        None
    };

    // Initialize incremental indexer
    let mut indexer = IncrementalIndexer::new(root.to_path_buf());
    indexer.init()?;

    // Detect changes or use specified files
    let changes = if files.is_empty() {
        if let Some(pb) = &progress_bar {
            pb.set_message("Detecting changes...");
        }
        indexer.detect_changes()?
    } else {
        // Use specified files
        files.iter().map(|f| {
            let hash = prime_core::ContentHash::from_bytes(&std::fs::read(f).unwrap_or_default());
            prime_index::FileChange {
                path: f.clone(),
                change_type: prime_index::ChangeType::Modified,
                old_hash: None,
                new_hash: hash,
            }
        }).collect()
    };

    if changes.is_empty() {
        if let Some(pb) = &progress_bar {
            pb.finish_with_message("No changes detected");
        }
        println!("No changes detected.");
        return Ok(());
    }

    println!("Detected {} changes:", changes.len());
    for change in &changes {
        println!("  {:?}: {}", change.change_type, change.path.display());
    }

    // Load existing graph
    if let Some(pb) = &progress_bar {
        pb.set_message("Loading existing index...");
    }

    let storage_config = StorageConfig {
        path: storage.to_path_buf(),
        compress: true,
        compression_level: 3,
        use_mmap: true,
        ..Default::default()
    };

    let mut storage_mgr = StorageManager::new(storage_config);
    let mut graph = storage_mgr.load()?;

    // Build invalidation index
    indexer.build_invalidation_index(&graph);

    // Get files to update
    let files_to_update: Vec<PathBuf> = changes.iter()
        .filter(|c| c.change_type != prime_index::ChangeType::Removed)
        .map(|c| c.path.clone())
        .collect();

    // Also update dependent files
    let mut all_files_to_update = files_to_update.clone();
    for file in &files_to_update {
        let dependents = indexer.dependent_files(file, &graph);
        for dep in dependents {
            if !all_files_to_update.contains(&dep) {
                all_files_to_update.push(dep);
            }
        }
    }

    println!("Updating {} files (including dependents)...", all_files_to_update.len());

    // Create parser and analyzer
    let config = ParserConfig {
        max_file_size: 1024 * 1024,
        ..Default::default()
    };
    let mut analyzer = prime_parser::ProjectAnalyzer::new(config)?;

    // Perform incremental update
    if let Some(pb) = &progress_bar {
        pb.set_message("Updating graph...");
    }

    let update_result = analyzer.update_incremental(&mut graph, &all_files_to_update, root)?;

    // Save updated graph
    if let Some(pb) = &progress_bar {
        pb.set_message("Saving updated index...");
    }

    storage_mgr.save(&graph)?;

    if let Some(pb) = &progress_bar {
        pb.finish_with_message("Done!");
    }

    let elapsed = start.elapsed();
    println!("\nUpdate completed in {:.2}s", elapsed.as_secs_f64());
    println!("  {}", update_result.summary());
    println!("  Total entities: {}", graph.project.entity_count);
    println!("  Total relations: {}", graph.project.relation_count);

    if !update_result.errors.is_empty() {
        println!("\nErrors:");
        for error in &update_result.errors {
            println!("  {}", error);
        }
    }

    Ok(())
}

fn cmd_query(storage: &Path, query: &str, qtype: &str, format: &str, limit: usize, relations: bool, depth: usize, confidence: &str, token_budget: usize) -> anyhow::Result<()> {
    let storage_config = StorageConfig {
        path: storage.to_path_buf(),
        use_mmap: true,
        ..Default::default()
    };

    let mut storage_mgr = StorageManager::new(storage_config);
    let graph = storage_mgr.load()?;

    let engine = QueryEngine::new(graph);

    let confidence = match confidence.to_lowercase().as_str() {
        "exact" => prime_core::Confidence::Exact,
        "high" => prime_core::Confidence::High,
        "medium" => prime_core::Confidence::Medium,
        "low" => prime_core::Confidence::Low,
        _ => prime_core::Confidence::Medium,
    };

    let opts = QueryOptions {
        max_results: limit,
        include_relations: relations,
        max_depth: depth,
        min_confidence: confidence,
        token_budget,
        ..Default::default()
    };

    let results = match qtype {
        "name" => engine.find_by_name(query, &opts),
        "prefix" => engine.find_by_prefix(query, &opts),
        "search" => engine.search(query, &opts),
        "context" => {
            if let Some(entity_id) = engine.graph().find_by_qualified(query) {
                vec![engine.get_context(entity_id, &opts).unwrap().entity]
            } else {
                vec![]
            }
        },
        _ => {
            eprintln!("Unknown query type: {}", qtype);
            return Ok(());
        }
    };

    match format {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&results)?);
        }
        "minimal" => {
            for r in results {
                println!("{} ({}) [{}]", r.qualified_name, r.kind, r.confidence);
            }
        }
        _ => {
            for r in results {
                println!("{} ({}) [{}]", r.qualified_name, r.kind, r.confidence);
                if let Some(sig) = &r.signature {
                    println!("  Signature: {}", sig);
                }
                if let Some(doc) = &r.documentation {
                    println!("  Doc: {}", doc);
                }
            }
        }
    }

    Ok(())
}

fn cmd_stats(storage: &Path) -> anyhow::Result<()> {
    let storage_config = StorageConfig {
        path: storage.to_path_buf(),
        use_mmap: true,
        ..Default::default()
    };

    let mut storage_mgr = StorageManager::new(storage_config);
    let graph = storage_mgr.load()?;

    println!("Knowledge Graph Statistics");
    println!("=========================");
    println!("Project: {}", graph.project.name);
    println!("Root: {}", graph.project.root_path);
    println!("Files: {}", graph.project.file_count);
    println!("Entities: {}", graph.project.entity_count);
    println!("Relations: {}", graph.project.relation_count);
    println!("Languages: {:?}", graph.project.languages);

    // Entity kinds
    let mut kind_counts = std::collections::HashMap::new();
    for entity in graph.entities.values() {
        *kind_counts.entry(entity.kind).or_insert(0) += 1;
    }
    println!("\nEntity kinds:");
    for (kind, count) in kind_counts {
        println!("  {:?}: {}", kind, count);
    }

    // Relation kinds
    let mut rel_counts = std::collections::HashMap::new();
    for rel in &graph.relations {
        *rel_counts.entry(rel.kind).or_insert(0) += 1;
    }
    println!("\nRelation kinds:");
    for (kind, count) in rel_counts {
        println!("  {:?}: {}", kind, count);
    }

    // Index size
    let storage_config = StorageConfig {
        path: std::path::PathBuf::from("."),
        ..Default::default()
    };
    let mut storage_mgr = StorageManager::new(storage_config);
    println!("\nIndex size: {} MB", storage_mgr.size() as f64 / 1024.0 / 1024.0);

    Ok(())
}

fn cmd_check(storage: &Path) -> anyhow::Result<()> {
    use prime_index::IncrementalIndexer;

    println!("Checking for drift...");

    // Get the root path from storage
    let root = storage.parent().unwrap_or(Path::new("."));

    // Initialize incremental indexer
    let mut indexer = IncrementalIndexer::new(root.to_path_buf());
    indexer.init()?;

    // Detect changes
    let changes = indexer.detect_changes()?;

    if changes.is_empty() {
        println!("No drift detected.");
        println!("  Tracked files: {}", indexer.file_count());
    } else {
        println!("Drift detected!");
        println!("  Changed files: {}", changes.len());
        for change in &changes {
            println!("    {:?}: {}", change.change_type, change.path.display());
        }
        println!("\nRun 'prime update' to update the index.");
    }

    Ok(())
}

fn cmd_init(storage: &Path, force: bool) -> anyhow::Result<()> {
    let storage_path = storage.join("graph.bin");
    if storage_path.exists() && !force {
        println!("Storage already initialized. Use --force to reinitialize.");
        return Ok(());
    }

    std::fs::create_dir_all(storage)?;
    println!("Initialized storage at: {}", storage.display());
    Ok(())
}

fn cmd_deps(storage: &Path, symbol: &str, transitive: bool) -> anyhow::Result<()> {
    let storage_config = StorageConfig {
        path: storage.to_path_buf(),
        use_mmap: true,
        ..Default::default()
    };

    let mut storage_mgr = StorageManager::new(storage_config);
    let graph = storage_mgr.load()?;

    if let Some(entity_id) = graph.find_by_qualified(symbol) {
        let deps = if transitive {
            // Would need transitive closure
            graph.dependencies(entity_id)
        } else {
            graph.dependencies(entity_id)
        };

        println!("Dependencies of {}:", symbol);
        for dep_id in deps {
            if let Some(entity) = graph.entities.get(&dep_id) {
                println!("  {} ({})", entity.qualified_name, entity.kind);
            }
        }
    } else {
        println!("Symbol not found: {}", symbol);
    }

    Ok(())
}

fn cmd_dependents(storage: &Path, symbol: &str, transitive: bool) -> anyhow::Result<()> {
    let storage_config = StorageConfig {
        path: storage.to_path_buf(),
        use_mmap: true,
        ..Default::default()
    };

    let mut storage_mgr = StorageManager::new(storage_config);
    let graph = storage_mgr.load()?;

    if let Some(entity_id) = graph.find_by_qualified(symbol) {
        let deps = if transitive {
            graph.dependents(entity_id)
        } else {
            graph.dependents(entity_id)
        };

        println!("Dependents of {}:", symbol);
        for dep_id in deps {
            if let Some(entity) = graph.entities.get(&dep_id) {
                println!("  {} ({})", entity.qualified_name, entity.kind);
            }
        }
    } else {
        println!("Symbol not found: {}", symbol);
    }

    Ok(())
}

fn cmd_calls(storage: &Path, symbol: &str, direction: &str) -> anyhow::Result<()> {
    let storage_config = StorageConfig {
        path: storage.to_path_buf(),
        use_mmap: true,
        ..Default::default()
    };

    let mut storage_mgr = StorageManager::new(storage_config);
    let graph = storage_mgr.load()?;

    if let Some(entity_id) = graph.find_by_qualified(symbol) {
        let results = match direction {
            "callers" => graph.callers(entity_id),
            "callees" => graph.callees(entity_id),
            "both" => {
                let mut v = graph.callers(entity_id);
                v.extend(graph.callees(entity_id));
                v
            },
            _ => {
                eprintln!("Unknown direction: {} (use callers, callees, or both)", direction);
                return Ok(());
            }
        };

        println!("{} of {}:", direction, symbol);
        for id in results {
            if let Some(entity) = graph.entities.get(&id) {
                println!("  {} ({})", entity.qualified_name, entity.kind);
            }
        }
    } else {
        println!("Symbol not found: {}", symbol);
    }

    Ok(())
}

fn cmd_export(storage: &Path, format: &str, output: Option<&Path>) -> anyhow::Result<()> {
    let storage_config = StorageConfig {
        path: storage.to_path_buf(),
        use_mmap: true,
        ..Default::default()
    };

    let mut storage_mgr = StorageManager::new(storage_config);
    let graph = storage_mgr.load()?;

    let default_output = PathBuf::from("graph_export");
    let output = output.unwrap_or(&default_output);

    match format {
        "json" => {
            let json = serde_json::to_string_pretty(&graph)?;
            std::fs::write(output.with_extension("json"), json)?;
        }
        "msgpack" => {
            let data = rmp_serde::to_vec(&graph)?;
            std::fs::write(output.with_extension("msgpack"), data)?;
        }
        "dot" => {
            // Generate GraphViz DOT format
            let mut dot = String::new();
            dot.push_str("digraph prime {\n");
            for rel in &graph.relations {
                if let (Some(from), Some(to)) = (graph.entities.get(&rel.from), graph.entities.get(&rel.to)) {
                    dot.push_str(&format!("  \"{}\" -> \"{}\" [label=\"{:?}\"];\n",
                        from.qualified_name, to.qualified_name, rel.kind));
                }
            }
            dot.push_str("}\n");
            std::fs::write(output.with_extension("dot"), dot)?;
        }
        _ => {
            eprintln!("Unknown format: {} (use json, msgpack, or dot)", format);
        }
    }

    println!("Exported to: {}", output.display());
    Ok(())
}

fn cmd_serve(storage: &Path) -> anyhow::Result<()> {
    let storage_config = StorageConfig {
        path: storage.to_path_buf(),
        use_mmap: true,
        ..Default::default()
    };

    let mut storage_mgr = StorageManager::new(storage_config);
    let graph = storage_mgr.load()?;

    eprintln!("Starting Prime MCP server on stdio...");
    eprintln!("Entities: {}, Relations: {}", graph.project.entity_count, graph.project.relation_count);

    let server = prime_mcp::PrimeMcpServer::new(graph);

    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(server.serve_stdio())?;

    Ok(())
}

fn cmd_inspect(storage: &Path, entity_name: &str, format: &str) -> anyhow::Result<()> {
    let storage_config = StorageConfig {
        path: storage.to_path_buf(),
        use_mmap: true,
        ..Default::default()
    };

    let mut storage_mgr = StorageManager::new(storage_config);
    let graph = storage_mgr.load()?;

    let executor = ToolExecutor::from_graph(graph);

    let request = prime_core::ToolRequest {
        intent: prime_core::ToolIntent::Context,
        target: Some(entity_name.to_string()),
        depth: 1,
        token_budget: 8192,
        ..Default::default()
    };

    let result = executor.execute(&request);

    match format {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        _ => {
            if let Some(ctx) = result.get("result").and_then(|v| v.as_object()) {
                println!("Entity: {}", entity_name);
                if let Some(status) = result.get("status") {
                    println!("Status: {}", status);
                }
                if let Some(coverage) = result.get("coverage") {
                    println!("Coverage: {}", coverage);
                }
                if let Some(deps) = ctx.get("dependencies").and_then(|v| v.as_array()) {
                    println!("Dependencies ({}):", deps.len());
                    for dep in deps {
                        if let Some(name) = dep.get("qualified_name").and_then(|v| v.as_str()) {
                            println!("  {}", name);
                        }
                    }
                }
                if let Some(callers) = ctx.get("callers").and_then(|v| v.as_array()) {
                    println!("Callers ({}):", callers.len());
                    for c in callers {
                        if let Some(name) = c.get("qualified_name").and_then(|v| v.as_str()) {
                            println!("  {}", name);
                        }
                    }
                }
                if let Some(callees) = ctx.get("callees").and_then(|v| v.as_array()) {
                    println!("Callees ({}):", callees.len());
                    for c in callees {
                        if let Some(name) = c.get("qualified_name").and_then(|v| v.as_str()) {
                            println!("  {}", name);
                        }
                    }
                }
            } else if let Some(warnings) = result.get("warnings").and_then(|v| v.as_array()) {
                for w in warnings {
                    eprintln!("Warning: {}", w);
                }
            }
        }
    }

    Ok(())
}

fn cmd_benchmark(root: &Path, _storage: &Path, corpus: &str, output: &PathBuf, bench_storage: &PathBuf) -> anyhow::Result<()> {
    use std::collections::HashMap;
    use std::fs;
    use std::thread;

    println!("=== Prime Comprehensive Benchmark ===");
    println!("Corpus: {}", corpus);
    println!("Output: {}", output.display());
    println!("Bench storage: {}", bench_storage.display());
    println!();

    // 1. Collect environment info
    let env_info = collect_environment();
    println!("Environment: {} / {} / {} cores", env_info.os, env_info.arch, env_info.cpu_cores);

    // 2. Get git info
    let git_info = collect_git_info();
    println!("Git commit: {}", git_info.commit);

    // 3. Load corpus
    let repos = load_corpus(corpus)?;
    println!("Loaded {} repositories for benchmarking", repos.len());

    // 4. Run benchmarks
    let mut benchmark_results = BenchmarkResult::new(
        git_info.commit.clone(),
        env_info,
        repos,
    );

    let prime_version = env!("CARGO_PKG_VERSION");
    benchmark_results.prime.version = prime_version.to_string();
    benchmark_results.prime.git_commit = git_info.commit.clone();
    benchmark_results.benchmark_version = "1.0.0".to_string();
    benchmark_results.timestamp = chrono::Utc::now().to_rfc3339();

    // Benchmark each repository
    for repo in &benchmark_results.corpus {
        println!("\n--- Benchmarking: {} ---", repo.name);
        let result = benchmark_repo(repo, bench_storage, root)?;
        benchmark_results.benchmarks.push(result);
    }

    // 5. Compute aggregate metrics
    benchmark_results.compute_aggregates();

    // 6. Generate BMF metrics
    benchmark_results.generate_bmf();

    // 7. Compute integrity
    benchmark_results.compute_integrity();

    // 8. Set final status based on integrity
    benchmark_results.status = if benchmark_results.integrity.valid {
        "complete".to_string()
    } else if benchmark_results.integrity.repositories_completed > 0 {
        "partial".to_string()
    } else {
        "failed".to_string()
    };

    // 9. Write result
    write_result(&benchmark_results, output)?;

    // 10. Print summary
    print_summary(&benchmark_results);

    Ok(())
}

fn cmd_prepare(corpus: &str, output: &PathBuf) -> anyhow::Result<()> {
    use std::process::Command;
    use std::fs;
    use std::path::Path;

    println!("=== Prime Corpus Preparation ===");
    println!("Corpus: {}", corpus);
    println!("Output: {}", output.display());
    println!();

    // Load corpus config
    let corpus_config = fs::read_to_string("benchmarks/corpus/repositories.json")?;
    let data: serde_json::Value = serde_json::from_str(&corpus_config)?;

    let repos: Vec<(String, serde_json::Value)> = if corpus == "pr" {
        let arr_opt = data["pr_corpus"]["repositories"].as_array();
        if arr_opt.is_none() {
            anyhow::bail!("pr_corpus.repositories not found");
        }
        let arr = arr_opt.unwrap();
        // pr_corpus may contain just names or full objects
        arr.iter()
            .filter_map(|v| {
                let name = if v.is_string() {
                    v.as_str()?.to_string()
                } else {
                    v["name"].as_str()?.to_string()
                };
                // Look up full repo info from main repositories list
                let main_repos = data["repositories"].as_array().unwrap_or_else(|| {
        static EMPTY: Vec<serde_json::Value> = Vec::new();
        &EMPTY
    });
                let repo_info = main_repos.iter().find(|r| r["name"].as_str() == Some(name.as_str()))?;
                Some((name, repo_info.clone()))
            })
            .collect::<Vec<_>>()
    } else if corpus == "nightly" || corpus == "all" {
        let nightly_arr = data["nightly_corpus"]["repositories"].as_array().unwrap_or_else(|| data["repositories"].as_array().unwrap());
        let repos_arr = if nightly_arr.len() == 1 && nightly_arr[0].as_str() == Some("*") {
            data["repositories"].as_array().unwrap()
        } else {
            nightly_arr
        };
        repos_arr.iter()
            .filter_map(|v| {
                let name = v["name"].as_str()?;
                Some((name.to_string(), v.clone()))
            })
            .collect::<Vec<_>>()
    } else if corpus == "stress" {
        let stress_arr = data["corpus_tiers"]["stress"]["repositories"].as_array()
            .ok_or_else(|| anyhow::anyhow!("stress_corpus.repositories not found"))?;
        stress_arr.iter()
            .filter_map(|v| {
                let name = v.as_str()?;
                let repo = data["repositories"].as_array()?.iter().find(|r| r["name"].as_str() == Some(name))?;
                Some((name.to_string(), repo.clone()))
            })
            .collect::<Vec<_>>()
    } else {
        anyhow::bail!("Unknown corpus: {}", corpus)
    };

    let total_repos = repos.len();
    println!("Preparing {} repositories...", total_repos);
    println!();

    fs::create_dir_all(output)?;

    let mut success = 0;
    for (name, repo) in repos {
        println!("--- Preparing: {} ---", name);
        
        let repo_path = Path::new(output).join(&name);
        let url = repo["url"].as_str().unwrap_or("");
        let commit = repo["commit"].as_str().unwrap_or("");

        if repo_path.exists() {
            println!("  Repository exists, checking commit...");
            let current = Command::new("git")
                .args(["rev-parse", "HEAD"])
                .current_dir(&repo_path)
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_default();
            
            let expected = Command::new("git")
                .args(["rev-parse", commit])
                .current_dir(&repo_path)
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_else(|_| commit.to_string());

            if current == expected {
                println!("  Already at correct commit");
                success += 1;
                continue;
            }
        }

        if !repo_path.exists() {
            println!("  Cloning from {}...", url);
            let status = Command::new("git")
                .args(["clone", url, &name])
                .current_dir(output)
                .status()?;
            if !status.success() {
                eprintln!("  Clone failed");
                continue;
            }
        }

        println!("  Checking out {}...", commit);
        let status = Command::new("git")
            .args(["checkout", commit])
            .current_dir(&repo_path)
            .status()?;
        
        if !status.success() {
            eprintln!("  Checkout failed");
            continue;
        }

        println!("  Verified at commit {}", commit);
        success += 1;
        println!();
    }

    println!("=== Preparation Complete ===");
    println!("Successful: {}/{}", success, total_repos);
    
    if success < total_repos {
        anyhow::bail!("Some repositories failed to prepare");
    }

    Ok(())
}

fn collect_environment() -> EnvironmentInfo {
    use std::env::consts;

    let cpu_cores = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    let memory_bytes = get_memory_bytes();

    EnvironmentInfo {
        os: consts::OS.to_string(),
        arch: consts::ARCH.to_string(),
        cpu: get_cpu_model(),
        cpu_cores,
        memory_bytes,
        runtime: "rustc".to_string(),
        runtime_version: rustc_version_runtime::version().to_string(),
        compiler_version: rustc_version_runtime::version().to_string(),
        kernel_version: get_kernel_version(),
    }
}

fn get_cpu_model() -> String {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("sysctl")
            .arg("-n")
            .arg("machdep.cpu.brand_string")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "Unknown".to_string())
    }
    #[cfg(target_os = "linux")]
    {
        std::fs::read_to_string("/proc/cpuinfo")
            .ok()
            .and_then(|s| s.lines().find(|l| l.starts_with("model name")))
            .and_then(|l| l.split(':').nth(1))
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "Unknown".to_string())
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        "Unknown".to_string()
    }
}

fn get_memory_bytes() -> u64 {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("sysctl")
            .arg("-n")
            .arg("hw.memsize")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .and_then(|s| s.trim().parse().ok())
            .unwrap_or(0)
    }
    #[cfg(target_os = "linux")]
    {
        std::fs::read_to_string("/proc/meminfo")
            .ok()
            .and_then(|s| s.lines().find(|l| l.starts_with("MemTotal:")))
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|k| k.parse::<u64>().ok())
            .map(|k| k * 1024)
            .unwrap_or(0)
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        0
    }
}

fn get_kernel_version() -> String {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("uname")
            .arg("-r")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "unknown".to_string())
    }
    #[cfg(target_os = "linux")]
    {
        std::fs::read_to_string("/proc/version")
            .ok()
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "unknown".to_string())
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        "unknown".to_string()
    }
}

fn collect_git_info() -> GitInfo {
    let commit = std::process::Command::new("git")
        .args(["rev-parse", "--short=12", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let dirty = std::process::Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .ok()
        .map(|o| !o.stdout.is_empty())
        .unwrap_or(false);

    GitInfo { commit, dirty }
}

fn repo_config(name: &str, path: &str, language: &str, size_category: &str, commit: &str, url: &str) -> RepoConfig {
    RepoConfig {
        name: name.to_string(),
        path: path.to_string(),
        language: language.to_string(),
        size_category: size_category.to_string(),
        commit: commit.to_string(),
        url: url.to_string(),
        files: 0,
        source_bytes: 0,
        lines_of_code: 0,
    }
}

fn load_corpus(corpus: &str) -> anyhow::Result<Vec<RepoConfig>> {
    // Load from repositories.json - use absolute path from workspace root
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let corpus_path = std::path::Path::new(manifest_dir).join("../../benchmarks/corpus/repositories.json");
    let corpus_config = fs::read_to_string(&corpus_path)?;
    let data: serde_json::Value = serde_json::from_str(&corpus_config)?;

    let repo_names = if corpus == "pr" {
        data["corpus_tiers"]["pr"]["repositories"].as_array()
            .ok_or_else(|| anyhow::anyhow!("pr_corpus.repositories not found"))?
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect::<Vec<_>>()
    } else if corpus == "nightly" {
        let nightly_arr = data["corpus_tiers"]["nightly"]["repositories"].as_array()
            .unwrap_or_else(|| data["repositories"].as_array().unwrap());
        if nightly_arr.len() == 1 && nightly_arr[0].as_str() == Some("*") {
            data["repositories"].as_array().unwrap()
                .iter()
                .filter_map(|v| v["name"].as_str().map(|s| s.to_string()))
                .collect()
        } else {
            nightly_arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        }
    } else if corpus == "stress" {
        data["corpus_tiers"]["stress"]["repositories"].as_array()
            .ok_or_else(|| anyhow::anyhow!("stress_corpus.repositories not found"))?
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect()
    } else {
        anyhow::bail!("Unknown corpus: {}. Use 'pr', 'nightly', 'stress', or a directory path", corpus)
    };

    let mut repos = Vec::new();
    for name in repo_names {
        if let Some(repo) = data["repositories"].as_array()
            .and_then(|arr| arr.iter().find(|r| r["name"].as_str() == Some(&name)))
        {
            repos.push(RepoConfig {
                name: name.clone(),
                path: format!("../benchmarks/repos/{}", name),
                language: repo["language"].as_str().unwrap_or("unknown").to_string(),
                size_category: repo["size_category"].as_str().unwrap_or("unknown").to_string(),
                commit: repo["commit"].as_str().unwrap_or("").to_string(),
                url: repo["url"].as_str().unwrap_or("").to_string(),
                files: repo["file_count_estimate"].as_u64().unwrap_or(0) as u32,
                source_bytes: repo["lines_of_code_estimate"].as_u64().unwrap_or(0) * 50,
                lines_of_code: repo["lines_of_code_estimate"].as_u64().unwrap_or(0) as u32,
            });
        }
    }
    Ok(repos)
}

fn benchmark_repo(repo: &RepoConfig, bench_storage: &PathBuf, _root: &Path) -> anyhow::Result<RepoBenchmark> {
    let repo_path = std::path::Path::new(&repo.path);
    eprintln!("DEBUG: Checking repo path: {}", repo_path.display());
    eprintln!("DEBUG: CWD: {}", std::env::current_dir().unwrap().display());
    if !repo_path.exists() {
        eprintln!("  Repo not found at {}, skipping", repo.path);
        return Ok(RepoBenchmark {
            repo_name: repo.name.clone(),
            status: "skipped".to_string(),
            error: Some("Repository not found locally".to_string()),
            ..Default::default()
        });
    }
    eprintln!("DEBUG: Repo exists, creating storage at: {}", bench_storage.join(&repo.name).display());
    let storage_path = bench_storage.join(&repo.name);
    fs::create_dir_all(&storage_path)?;
    eprintln!("DEBUG: Storage dir created");

    // 1. Discover and measure corpus metrics (single source of truth)
    let corpus_info = discover_corpus_info(repo_path)?;
    println!("  Corpus: {} files, {} bytes, {} LOC", corpus_info.files, corpus_info.source_bytes, corpus_info.lines_of_code);

    println!("  Building index...");
    let derivation = benchmark_derivation(repo_path, &storage_path, &corpus_info)?;
    println!("    Derivation: {:.2}s, {} entities, {} relations", derivation.time_ms / 1000.0, derivation.entities, derivation.relations);

    println!("  Measuring artifact...");
    let artifact = benchmark_artifact(&storage_path, corpus_info.source_bytes)?;
    println!("    Artifact: {}, ratio: {:.3}", artifact.artifact_bytes, artifact.artifact_to_source_ratio);

    println!("  Running retrieval benchmarks...");
    let retrieval = benchmark_retrieval(&storage_path)?;
    println!("    Cold p50: {:.0}µs, Warm p50: {:.0}µs, Search p50: {:.0}µs, Lookup p50: {:.0}µs, Context p50: {:.0}µs",
        retrieval.cold.median, retrieval.warm.median, retrieval.search.median, retrieval.lookup.median, retrieval.context.median);

    println!("  Running knowledge benchmarks...");
    let knowledge = benchmark_knowledge(&storage_path)?;
    println!("    Accuracy: {:.1}%, Source-free accuracy: {:.1}%",
        knowledge.accuracy * 100.0, knowledge.source_free_accuracy * 100.0);

    println!("  Source savings: not measured (requires controlled baseline experiment)");

    Ok(RepoBenchmark {
        repo_name: repo.name.clone(),
        status: "complete".to_string(),
        error: None,
        derivation,
        artifact,
        retrieval,
        knowledge,
        source_savings: None,
    })
}

#[derive(Debug, Clone)]
struct CorpusInfo {
    files: u32,
    source_bytes: u64,
    lines_of_code: u32,
    non_empty_lines: u32,
}

fn discover_corpus_info(repo_path: &Path) -> anyhow::Result<CorpusInfo> {
    use walkdir::WalkDir;
    use std::fs;

    let mut files = 0u32;
    let mut source_bytes = 0u64;
    let mut lines_of_code = 0u32;
    let mut non_empty_lines = 0u32;

    let source_extensions = [
        "rs", "py", "js", "ts", "jsx", "tsx", "go", "java", "kt", "scala",
        "c", "h", "cpp", "cc", "cxx", "hpp", "hxx", "cs", "rb", "php",
        "swift", "lua", "dart", "zig", "toml", "json", "yaml", "yml",
        "md", "txt", "sh", "bash", "zsh", "fish", "ps1", "bat",
        "sql", "graphql", "proto", "dockerfile", "gradle", "maven",
    ];

    for entry in WalkDir::new(repo_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
            if source_extensions.contains(&ext.as_str()) {
                if let Ok(meta) = fs::metadata(path) {
                    files += 1;
                    source_bytes += meta.len();

                    // Count lines
                    if let Ok(content) = fs::read_to_string(path) {
                        for line in content.lines() {
                            lines_of_code += 1;
                            if !line.trim().is_empty() {
                                non_empty_lines += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(CorpusInfo { files, source_bytes, lines_of_code, non_empty_lines })
}

fn benchmark_derivation(repo_path: &Path, storage_path: &Path, corpus: &CorpusInfo) -> anyhow::Result<DerivationMetrics> {
    let config = ParserConfig {
        max_file_size: 1024 * 1024,
        ..Default::default()
    };

    let mut analyzer = prime_parser::ProjectAnalyzer::new(config)?;

    // Phase 1: Discovery + parsing
    let parse_start = Instant::now();
    let graph = analyzer.analyze(repo_path)?;
    let parse_time = parse_start.elapsed().as_micros() as f64;

    // Phase 2: Index building
    let storage_config = StorageConfig {
        path: storage_path.to_path_buf(),
        compress: true,
        compression_level: 3,
        use_mmap: true,
        ..Default::default()
    };

    let index_start = Instant::now();
    let mut storage = StorageManager::new(storage_config);
    storage.save(&graph)?;
    let index_time = index_start.elapsed().as_micros() as f64;

    let total_time = parse_time + index_time;

    let entities = graph.project.entity_count as usize;
    let relations = graph.project.relation_count as usize;
    let symbols = graph.entities.values().filter(|e| e.kind != prime_core::SymbolKind::Unknown).count();

    Ok(DerivationMetrics {
        time_ms: total_time / 1000.0,
        files: corpus.files,
        source_bytes: corpus.source_bytes,
        lines_of_code: corpus.lines_of_code,
        non_empty_lines: corpus.non_empty_lines,
        entities,
        symbols,
        relations,
        files_per_second: (corpus.files as f64) / (total_time / 1_000_000.0),
        loc_per_second: (corpus.lines_of_code as f64) / (total_time / 1_000_000.0),
        symbols_per_second: (symbols as f64) / (total_time / 1_000_000.0),
        relationships_per_second: (relations as f64) / (total_time / 1_000_000.0),
        peak_memory_bytes: get_peak_memory_bytes(),
        parse_time_us: parse_time,
        index_time_us: index_time,
        serialization_time_us: 0.0,
    })
}

fn get_peak_memory_bytes() -> u64 {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("ps").args(["-o", "rss=", "-p", &std::process::id().to_string()]).output() {
            if let Ok(s) = String::from_utf8(output.stdout) {
                if let Ok(kb) = s.trim().parse::<u64>() {
                    return kb * 1024; // RSS is in KB on macOS
                }
            }
        }
    }
    #[cfg(target_os = "linux")]
    {
        if let Ok(content) = fs::read_to_string("/proc/self/status") {
            for line in content.lines() {
                if line.starts_with("VmHWM:") {
                    if let Some(kb_str) = line.split_whitespace().nth(1) {
                        if let Ok(kb) = kb_str.parse::<u64>() {
                            return kb * 1024;
                        }
                    }
                }
            }
        }
    }
    0 // Not available
}

fn benchmark_artifact(storage_path: &Path, source_bytes: u64) -> anyhow::Result<ArtifactMetrics> {
    let graph_bin = storage_path.join("graph.bin");
    let artifact_bytes = fs::metadata(&graph_bin).map(|m| m.len()).unwrap_or(0);

    let artifact_to_source_ratio = if source_bytes > 0 {
        artifact_bytes as f64 / source_bytes as f64
    } else {
        0.0
    };

    let source_reduction_ratio = 1.0 - artifact_to_source_ratio;

    let artifact_bytes_per_kloc = if source_bytes > 0 {
        // Use 100 bytes per LOC as rough estimate for KLLOC calculation
        let estimated_kloc = (source_bytes as f64 / 100.0) / 1000.0;
        if estimated_kloc > 0.0 {
            artifact_bytes as f64 / estimated_kloc
        } else {
            0.0
        }
    } else {
        0.0
    };

    Ok(ArtifactMetrics {
        source_bytes,
        artifact_bytes,
        artifact_to_source_ratio,
        source_reduction_ratio,
        artifact_bytes_per_kloc,
    })
}

fn benchmark_retrieval(storage_path: &Path) -> anyhow::Result<RetrievalMetrics> {
    let storage_config = StorageConfig {
        path: storage_path.to_path_buf(),
        use_mmap: true,
        ..Default::default()
    };

    let mut storage = StorageManager::new(storage_config);
    let graph = storage.load()?;

    // Use actual symbols from the graph for realistic queries (extract before moving graph)
    let symbols: Vec<String> = graph.entities.values()
        .filter(|e| !e.name.is_empty())
        .map(|e| e.qualified_name.clone())
        .take(20)
        .collect();

    if symbols.is_empty() {
        // Fallback queries
        return Ok(RetrievalMetrics::default());
    }

    let executor = ToolExecutor::from_graph(graph);

    // Cold: first query after fresh load (simulate by creating new executor)
    let cold_queries = symbols.iter().take(5).cloned().collect::<Vec<_>>();
    let mut cold_latencies = Vec::new();
    for q in &cold_queries {
        let mut cold_storage = StorageManager::new(StorageConfig {
            path: storage_path.to_path_buf(),
            use_mmap: true,
            ..Default::default()
        });
        let cold_graph = cold_storage.load()?;
        let cold_executor = ToolExecutor::from_graph(cold_graph);
        let start = Instant::now();
        let req = ToolRequest {
            intent: ToolIntent::Search,
            target: Some(q.clone()),
            limit: 10,
            ..Default::default()
        };
        let _ = cold_executor.execute(&req);
        cold_latencies.push(start.elapsed().as_nanos() as f64 / 1000.0);
    }

    // Warm: repeated queries on same executor
    let warm_iterations = 50;
    let mut warm_search = Vec::new();
    let mut warm_lookup = Vec::new();
    let mut warm_context = Vec::new();

    for _ in 0..warm_iterations {
        for q in &symbols {
            // Search
            let start = Instant::now();
            let req = ToolRequest {
                intent: ToolIntent::Search,
                target: Some(q.clone()),
                limit: 10,
                ..Default::default()
            };
            let _ = executor.execute(&req);
            warm_search.push(start.elapsed().as_nanos() as f64 / 1000.0);

            // Lookup
            let start = Instant::now();
            let req = ToolRequest {
                intent: ToolIntent::Lookup,
                target: Some(q.clone()),
                ..Default::default()
            };
            let _ = executor.execute(&req);
            warm_lookup.push(start.elapsed().as_nanos() as f64 / 1000.0);

            // Context
            let start = Instant::now();
            let req = ToolRequest {
                intent: ToolIntent::Context,
                target: Some(q.clone()),
                depth: 1,
                token_budget: 8192,
                ..Default::default()
            };
            let _ = executor.execute(&req);
            warm_context.push(start.elapsed().as_nanos() as f64 / 1000.0);
        }
    }

    // Repeated: same query repeated many times
    let repeated_query = &symbols[0];
    let repeated_iterations = 100;
    let mut repeated_latencies = Vec::new();
    for _ in 0..repeated_iterations {
        let start = Instant::now();
        let req = ToolRequest {
            intent: ToolIntent::Search,
            target: Some(repeated_query.clone()),
            limit: 10,
            ..Default::default()
        };
        let _ = executor.execute(&req);
        repeated_latencies.push(start.elapsed().as_nanos() as f64 / 1000.0);
    }

    Ok(RetrievalMetrics {
        cold: stats_with_samples(&cold_latencies, "cold"),
        warm: stats_with_samples(&warm_search, "warm"),
        search: stats_with_samples(&warm_search, "search"),
        lookup: stats_with_samples(&warm_lookup, "lookup"),
        context: stats_with_samples(&warm_context, "context"),
        repeated: stats_with_samples(&repeated_latencies, "repeated"),
    })
}

fn stats_with_samples(values: &[f64], _name: &str) -> LatencyStats {
    if values.is_empty() {
        return LatencyStats::default();
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    LatencyStats {
        samples: sorted.len(),
        min: sorted[0],
        max: sorted[sorted.len() - 1],
        mean: sorted.iter().sum::<f64>() / sorted.len() as f64,
        median: percentile(&sorted, 0.50),
        p95: percentile(&sorted, 0.95),
        p99: percentile(&sorted, 0.99),
    }
}

fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() { return 0.0; }
    let idx = ((sorted.len() - 1) as f64 * p).round() as usize;
    sorted[idx.min(sorted.len() - 1)]
}

// Helper function to map question category/evaluation to appropriate ToolIntent
fn question_to_tool_intent(q: &KnowledgeQuestion) -> ToolIntent {
    match q.category.as_str() {
        "architecture" => {
            // Architecture questions about module hierarchy, public APIs -> Architecture tool
            if q.evaluation == "relationship_recall" || q.evaluation == "relationship_precision" {
                ToolIntent::Architecture
            } else {
                ToolIntent::Search
            }
        }
        "symbols" => ToolIntent::Search,
        "calls" => ToolIntent::Context,  // Context gives callers/callees
        "imports" => ToolIntent::Dependencies,
        "exports" => ToolIntent::Architecture,
        "flows_to" => ToolIntent::Context,  // Context gives data flow info
        "instantiates" => ToolIntent::Relationships,
        "relationships" => ToolIntent::Relationships,
        "dependencies" => ToolIntent::Dependencies,
        "impact" => ToolIntent::Impact,
        "dataflow" => ToolIntent::Context,  // Context gives callers/callees/dependencies
        "entrypoints" => ToolIntent::Search,
        "configuration" => ToolIntent::Architecture,
        "testing" => ToolIntent::Search,
        "polyglot" => ToolIntent::Architecture,
        _ => ToolIntent::Search,
    }
}

fn benchmark_knowledge(storage_path: &Path) -> anyhow::Result<KnowledgeMetrics> {
    let storage_config = StorageConfig {
        path: storage_path.to_path_buf(),
        use_mmap: true,
        ..Default::default()
    };

    let mut storage = StorageManager::new(storage_config);
    let graph = storage.load()?;
    let executor = ToolExecutor::from_graph(graph);

    let questions = load_questions()?;
    let mut total = 0;
    let mut correct = 0;
    let mut source_free_correct = 0;
    let mut source_required_correct = 0;
    
    // Retrieval metrics
    let mut entity_tp = 0;
    let mut entity_fp = 0;
    let mut entity_fn = 0;
    let mut rel_tp = 0;
    let mut rel_fp = 0;
    let mut rel_fn = 0;
    
    // Ranking metrics
    let mut reciprocal_ranks = Vec::new();
    let mut hits_at_1 = 0;
    let mut hits_at_3 = 0;
    let mut hits_at_5 = 0;
    let mut hits_at_10 = 0;
    
    let mut by_category: HashMap<String, CategoryResult> = HashMap::new();

    for q in &questions {
        total += 1;
        // For context/dependencies/impact, first search to find the entity, then use its qualified name
        let tool_intent = question_to_tool_intent(q);
        let target = if matches!(tool_intent, ToolIntent::Context | ToolIntent::Dependencies | ToolIntent::Impact | ToolIntent::Relationships) {
            // First search to find the entity
            let search_req = ToolRequest {
                intent: ToolIntent::Search,
                target: Some(q.search_query.clone()),
                limit: 1,
                ..Default::default()
            };
            let search_result = executor.execute(&search_req);
            let search_array = search_result.get("result").and_then(|v| v.as_array()).cloned().unwrap_or_default();
            // Use the first found entity's qualified name
            search_array.first()
                .and_then(|e| e.get("qualified_name").and_then(|v| v.as_str()))
                .map(|s| s.to_string())
                .unwrap_or_else(|| q.search_query.clone())
        } else {
            q.search_query.clone()
        };
        
        let req = ToolRequest {
            intent: tool_intent,
            target: Some(target),
            limit: 10,
            ..Default::default()
        };
        let result = executor.execute(&req);

        let status = result.get("status").and_then(|v| v.as_str()).unwrap_or("Unknown");
        let result_value = result.get("result").cloned().unwrap_or(serde_json::Value::Null);
        
        // Handle both array results (Search) and object results (Context, Relationships, etc.)
        let result_array = if let Some(arr) = result_value.as_array() {
            arr.clone()
        } else if let Some(obj) = result_value.as_object() {
            // For Context/Relationships/Dependencies, extract entities from nested arrays
            let mut entities = Vec::new();
            for key in &["callers", "callees", "dependencies", "dependents", "relations"] {
                if let Some(arr) = obj.get(*key).and_then(|v| v.as_array()) {
                    for item in arr {
                        entities.push(item.clone());
                    }
                }
            }
            entities
        } else {
            Vec::new()
        };
        let has_results = !result_array.is_empty() || !result_value.is_null();

        // Extract returned entities
        let returned_entities: Vec<String> = result_array.iter()
            .filter_map(|e| e.get("qualified_name").and_then(|v| v.as_str()))
            .map(|s| s.to_string())
            .collect();
        
        let returned_kinds: Vec<String> = result_array.iter()
            .filter_map(|e| e.get("kind").and_then(|v| v.as_str()))
            .map(|s| s.to_string())
            .collect();

        // Evaluate based on question type - pass both flattened array AND original result for relationship extraction
        let (is_correct, entity_tp_q, entity_fp_q, entity_fn_q, rel_tp_q, rel_fp_q, rel_fn_q, rank) = 
            evaluate_question(&q, &result_array, &returned_entities, &returned_kinds, tool_intent, &result_value);

        entity_tp += entity_tp_q;
        entity_fp += entity_fp_q;
        entity_fn += entity_fn_q;
        rel_tp += rel_tp_q;
        rel_fp += rel_fp_q;
        rel_fn += rel_fn_q;
        
        if let Some(r) = rank {
            reciprocal_ranks.push(1.0 / r as f64);
            if r <= 1 { hits_at_1 += 1; }
            if r <= 3 { hits_at_3 += 1; }
            if r <= 5 { hits_at_5 += 1; }
            if r <= 10 { hits_at_10 += 1; }
        }

        let is_correct = entity_tp_q > 0 || (q.expected_entities.is_empty() && q.expected_relationships.is_empty() && !returned_entities.is_empty());
        
        // Use envelope's actual source_required field
        let envelope_source_required = result.get("source_required").and_then(|v| v.as_bool()).unwrap_or(false);
        
        if is_correct {
            correct += 1;
            if !envelope_source_required {
                source_free_correct += 1;
            } else {
                source_required_correct += 1;
            }
        }

        let cat = by_category.entry(q.category.clone()).or_default();
        cat.total += 1;
        if is_correct { cat.correct += 1; }
        if !envelope_source_required && is_correct { cat.source_free_correct += 1; }
        if envelope_source_required && is_correct { cat.source_required_correct += 1; }
    }

    let accuracy = if total > 0 { correct as f64 / total as f64 } else { 0.0 };
    let source_free_accuracy = if total > 0 { source_free_correct as f64 / total as f64 } else { 0.0 };

    // Compute retrieval metrics
    let entity_precision = if entity_tp + entity_fp > 0 { entity_tp as f64 / (entity_tp + entity_fp) as f64 } else { 0.0 };
    let entity_recall = if entity_tp + entity_fn > 0 { entity_tp as f64 / (entity_tp + entity_fn) as f64 } else { 0.0 };
    let entity_f1 = if entity_precision + entity_recall > 0.0 { 2.0 * entity_precision * entity_recall / (entity_precision + entity_recall) } else { 0.0 };
    
    let relationship_precision = if rel_tp + rel_fp > 0 { rel_tp as f64 / (rel_tp + rel_fp) as f64 } else { 0.0 };
    let relationship_recall = if rel_tp + rel_fn > 0 { rel_tp as f64 / (rel_tp + rel_fn) as f64 } else { 0.0 };
    let relationship_f1 = if relationship_precision + relationship_recall > 0.0 { 2.0 * relationship_precision * relationship_recall / (relationship_precision + relationship_recall) } else { 0.0 };

    let mrr = if !reciprocal_ranks.is_empty() { reciprocal_ranks.iter().sum::<f64>() / reciprocal_ranks.len() as f64 } else { 0.0 };
    let recall_at_1 = if total > 0 { hits_at_1 as f64 / total as f64 } else { 0.0 };
    let recall_at_3 = if total > 0 { hits_at_3 as f64 / total as f64 } else { 0.0 };
    let recall_at_5 = if total > 0 { hits_at_5 as f64 / total as f64 } else { 0.0 };
    let recall_at_10 = if total > 0 { hits_at_10 as f64 / total as f64 } else { 0.0 };

    let mut category_map = HashMap::new();
    for (cat, res) in by_category {
        let cat_precision = if res.correct as f64 > 0.0 { res.correct as f64 / res.total as f64 } else { 0.0 };
        category_map.insert(cat, CategoryStats {
            total: res.total,
            correct: res.correct,
            source_free_correct: res.source_free_correct,
            source_required_correct: res.source_required_correct,
            accuracy: if res.total > 0 { res.correct as f64 / res.total as f64 } else { 0.0 },
            source_free_accuracy: if res.total > 0 { res.source_free_correct as f64 / res.total as f64 } else { 0.0 },
            entity_precision: res.entity_precision,
            entity_recall: res.entity_recall,
            entity_f1: res.entity_f1,
            relationship_precision: res.relationship_precision,
            relationship_recall: res.relationship_recall,
            relationship_f1: res.relationship_f1,
            mrr: res.mrr,
            recall_at_1: res.recall_at_1,
            recall_at_3: res.recall_at_3,
            recall_at_5: res.recall_at_5,
            recall_at_10: res.recall_at_10,
        });
    }

    Ok(KnowledgeMetrics {
        total_questions: total,
        correct,
        incorrect: total - correct,
        source_free_correct,
        source_required_correct,
        accuracy,
        source_free_accuracy,
        entity_precision,
        entity_recall,
        entity_f1,
        relationship_precision,
        relationship_recall,
        relationship_f1,
        mrr,
        recall_at_1,
        recall_at_3,
        recall_at_5,
        recall_at_10,
        by_category: category_map,
    })
}

// Evaluate a single question and return (is_correct, entity_tp, entity_fp, entity_fn, rel_tp, rel_fp, rel_fn, rank)
fn evaluate_question(q: &KnowledgeQuestion, results: &[serde_json::Value], returned: &[String], kinds: &[String], tool_intent: ToolIntent, raw_result: &serde_json::Value) -> (bool, usize, usize, usize, usize, usize, usize, Option<usize>) {
    let mut entity_tp = 0;
    let mut entity_fp = 0;
    let mut entity_fn = 0;
    let mut rel_tp = 0;
    let mut rel_fp = 0;
    let mut rel_fn = 0;
    let mut rank = None;

    // Check entity retrieval - use substring/suffix matching on qualified names
    let expected_entities: Vec<String> = q.expected_entities.iter().map(|s| s.to_lowercase()).collect();
    let returned_lower: Vec<String> = returned.iter().map(|s| s.to_lowercase()).collect();

    for (i, ret) in returned_lower.iter().enumerate() {
        if expected_entities.iter().any(|e| {
            // Match if: exact match, or returned ends with expected, or expected is suffix of last component
            ret == e || e == ret ||
            ret.ends_with(&format!("::{}", e)) ||
            ret.ends_with(&format!(".{}", e)) ||
            ret.split("::").last().map_or(false, |last| last == e.as_str()) ||
            ret.split(".").last().map_or(false, |last| last == e.as_str())
        }) {
            entity_tp += 1;
            if rank.is_none() { rank = Some(i + 1); }
        } else {
            entity_fp += 1;
        }
    }
    entity_fn = expected_entities.len().saturating_sub(entity_tp);

    // Check relationship retrieval - extract from structured response based on tool
    let expected_rels: Vec<Vec<String>> = q.expected_relationships.iter()
        .map(|rels| rels.iter().map(|s| s.to_lowercase()).collect())
        .collect();

    if !expected_rels.is_empty() {
        // Extract relationships from structured response based on tool
        let extracted_rels = extract_relationships_from_response(&[raw_result.clone()], tool_intent);
        
        for expected_rel in &expected_rels {
            let mut found = false;
            for expected_kw in expected_rel {
                if extracted_rels.iter().any(|r| r.to_lowercase().contains(expected_kw.as_str())) {
                    found = true;
                    break;
                }
            }
            if found { rel_tp += 1; } else { rel_fn += 1; }
        }
        // Note: rel_fp is hard to measure without ground truth negatives
        rel_fp = 0;
    }

    // Filter by expected kind if specified
    if let Some(expected_kind) = &q.expected_kind {
        let kind_matches = kinds.iter().filter(|k| k.to_lowercase() == expected_kind.to_lowercase()).count();
        if kind_matches == 0 && !returned.is_empty() {
            // All returned entities are wrong kind
            entity_fp += returned.len();
            entity_tp = 0;
        }
    }

    let is_correct = match q.evaluation.as_str() {
        "entity_match" => entity_tp > 0,
        "entity_recall" => entity_tp > 0 || (q.expected_entities.is_empty() && !returned.is_empty()),
        "relationship_recall" => rel_tp > 0,
        "relationship_precision" => rel_tp > 0,
        _ => entity_tp > 0 || (!q.expected_entities.is_empty() && entity_tp > 0),
    };

    (is_correct, entity_tp, entity_fp, entity_fn, rel_tp, rel_fp, rel_fn, rank)
}

// Extract relationships from tool response based on tool intent
fn extract_relationships_from_response(results: &[serde_json::Value], tool_intent: ToolIntent) -> Vec<String> {
    let mut rels = Vec::new();
    
    for result in results {
        match tool_intent {
            ToolIntent::Context => {
                // Context returns: callers, callees, dependencies, dependents
                if let Some(callees) = result.get("callees").and_then(|v| v.as_array()) {
                    for callee in callees {
                        if let Some(name) = callee.get("qualified_name").and_then(|v| v.as_str()) {
                            rels.push(format!("calls:{}", name));
                        }
                    }
                }
                if let Some(callers) = result.get("callers").and_then(|v| v.as_array()) {
                    for caller in callers {
                        if let Some(name) = caller.get("qualified_name").and_then(|v| v.as_str()) {
                            rels.push(format!("called_by:{}", name));
                        }
                    }
                }
                if let Some(deps) = result.get("dependencies").and_then(|v| v.as_array()) {
                    for dep in deps {
                        if let Some(name) = dep.get("qualified_name").and_then(|v| v.as_str()) {
                            rels.push(format!("depends_on:{}", name));
                        }
                    }
                }
                if let Some(deps) = result.get("dependents").and_then(|v| v.as_array()) {
                    for dep in deps {
                        if let Some(name) = dep.get("qualified_name").and_then(|v| v.as_str()) {
                            rels.push(format!("depended_by:{}", name));
                        }
                    }
                }
                // Also check for nested result field (PrimeEnvelope wrapping)
                if let Some(inner) = result.get("result").and_then(|v| v.as_object()) {
                    for key in &["callers", "callees", "dependencies", "dependents"] {
                        if let Some(arr) = inner.get(*key).and_then(|v| v.as_array()) {
                            for item in arr {
                                if let Some(name) = item.get("qualified_name").and_then(|v| v.as_str()) {
                                    rels.push(format!("{}:{}", key, name));
                                }
                            }
                        }
                    }
                }
            }
            ToolIntent::Relationships => {
                // Relationships returns: entity, relations array
                if let Some(relations) = result.get("relations").and_then(|v| v.as_array()) {
                    for rel in relations {
                        if let Some(kind) = rel.get("kind").and_then(|v| v.as_str()) {
                            if let Some(target) = rel.get("target").and_then(|v| v.as_str()) {
                                rels.push(format!("{}:{}", kind.to_lowercase(), target));
                            }
                        }
                    }
                }
            }
            ToolIntent::Dependencies => {
                // Dependencies returns: entity, dependencies array, dependents array
                if let Some(deps) = result.get("dependencies").and_then(|v| v.as_array()) {
                    for dep in deps {
                        if let Some(target) = dep.get("target").and_then(|v| v.as_str()) {
                            if let Some(kind) = dep.get("kind").and_then(|v| v.as_str()) {
                                rels.push(format!("{}:{}", kind.to_lowercase(), target));
                            } else {
                                rels.push(format!("depends_on:{}", target));
                            }
                        }
                    }
                }
                if let Some(dependents) = result.get("dependents").and_then(|v| v.as_array()) {
                    for dep in dependents {
                        if let Some(s) = dep.as_str() { rels.push(format!("depended_by:{}", s)); }
                    }
                }
            }
            ToolIntent::Impact => {
                // Impact returns: entity, direct_impact, transitive_impact, tests_affected
                if let Some(direct) = result.get("direct_impact").and_then(|v| v.as_array()) {
                    for d in direct { if let Some(s) = d.as_str() { rels.push(format!("directly_affects:{}", s)); } }
                }
                if let Some(transitive) = result.get("transitive_impact").and_then(|v| v.as_array()) {
                    for t in transitive { if let Some(s) = t.as_str() { rels.push(format!("transitively_affects:{}", s)); } }
                }
                if let Some(tests) = result.get("tests_affected").and_then(|v| v.as_array()) {
                    for t in tests { if let Some(s) = t.as_str() { rels.push(format!("tests:{}", s)); } }
                }
            }
            ToolIntent::Architecture => {
                // Architecture returns: modules, boundaries, layers
                if let Some(modules) = result.get("modules").and_then(|v| v.as_array()) {
                    for m in modules {
                        if let Some(name) = m.get("name").and_then(|v| v.as_str()) {
                            if let Some(deps) = m.get("deps").and_then(|v| v.as_array()) {
                                for d in deps { if let Some(s) = d.as_str() { rels.push(format!("module:{}:depends_on:{}", name, s)); } }
                            }
                        }
                    }
                }
                if let Some(boundaries) = result.get("boundaries").and_then(|v| v.as_array()) {
                    for b in boundaries {
                        if let Some(from) = b.get("from").and_then(|v| v.as_str()) {
                            if let Some(to) = b.get("to").and_then(|v| v.as_str()) {
                                rels.push(format!("boundary_violation:{}:{}", from, to));
                            }
                        }
                    }
                }
            }
            _ => {
                // For Search, Lookup - no relationship data in response
            }
        }
    }
    
    rels
}

fn load_questions() -> anyhow::Result<Vec<KnowledgeQuestion>> {
    // Try to load from file first - use absolute path from workspace root
    let questions_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../benchmarks/corpus/questions/knowledge.json");
    
    if let Ok(content) = fs::read_to_string(&questions_path) {
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(arr) = parsed.get("questions").and_then(|v| v.as_array()) {
                let mut questions = Vec::new();
                for q in arr {
                    if let (Some(id), Some(category), Some(question), Some(search_query)) = (
                        q.get("id").and_then(|v| v.as_str()),
                        q.get("category").and_then(|v| v.as_str()),
                        q.get("question").and_then(|v| v.as_str()),
                        q.get("search_query").and_then(|v| v.as_str()),
                    ) {
                        let expected_entities = q.get("expected_entities")
                            .and_then(|v| v.as_array())
                            .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
                            .unwrap_or_default();
                        // Fix: Parse expected_relationships as Vec<Vec<String>> - inner elements are strings, not arrays
                        let expected_relationships = q.get("expected_relationships")
                            .and_then(|v| v.as_array())
                            .map(|arr| arr.iter().filter_map(|v| v.as_array()).map(|a| a.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect()).collect())
                            .unwrap_or_default();
                        let expected_kind = q.get("expected_kind").and_then(|v| v.as_str()).map(|s| s.to_string());
                        let evaluation = q.get("evaluation").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or("keyword".to_string());
                        
                        questions.push(KnowledgeQuestion {
                            id: id.to_string(),
                            category: category.to_string(),
                            question: question.to_string(),
                            search_query: search_query.to_string(),
                            expected_entities,
                            expected_relationships,
                            expected_kind,
                            evaluation,
                            source_allowed: q.get("source_allowed").and_then(|v| v.as_bool()).unwrap_or(false),
                        });
                    }
                }
                if !questions.is_empty() {
                    return Ok(questions);
                }
            }
        }
    }

    // Built-in fallback questions (minimal set for when file not found)
    Ok(vec![
        KnowledgeQuestion { id: "arch-001".to_string(), category: "architecture".to_string(), question: "What is the main entry point?".to_string(), search_query: "main".to_string(), expected_entities: vec!["main".to_string()], expected_relationships: vec![], expected_kind: None, evaluation: "entity_match".to_string(), source_allowed: false },
        KnowledgeQuestion { id: "arch-002".to_string(), category: "architecture".to_string(), question: "What are the top-level modules?".to_string(), search_query: "mod".to_string(), expected_entities: vec![], expected_relationships: vec![vec!["contains".to_string()]], expected_kind: None, evaluation: "relationship_recall".to_string(), source_allowed: false },
        KnowledgeQuestion { id: "sym-001".to_string(), category: "symbols".to_string(), question: "List all public functions".to_string(), search_query: "fn".to_string(), expected_entities: vec![], expected_relationships: vec![], expected_kind: Some("function".to_string()), evaluation: "entity_recall".to_string(), source_allowed: false },
    ])
}

fn benchmark_source_savings(_storage_path: &Path, _knowledge: &KnowledgeMetrics) -> anyhow::Result<Option<SourceSavings>> {
    // Source savings measurement requires a controlled baseline experiment:
    // 1. Run agent tasks WITHOUT Prime, measure source reads/bytes
    // 2. Run same agent tasks WITH Prime, measure source reads/bytes
    // 3. Compare the two
    //
    // Current implementation estimates savings from knowledge questions,
    // which is synthetic. Return None until a controlled experiment is implemented.
    Ok(None)
}

fn write_result(result: &BenchmarkResult, output: &PathBuf) -> anyhow::Result<()> {
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(result)?;
    fs::write(output, json)?;
    println!("\nResult written to: {}", output.display());
    Ok(())
}

fn print_summary(result: &BenchmarkResult) {
    println!("\n=== BENCHMARK SUMMARY ===");
    println!("Status: {}", result.status);
    println!("Repos benchmarked: {}", result.corpus.len());

    let complete = result.benchmarks.iter().filter(|b| b.status == "complete").count();
    let skipped = result.benchmarks.iter().filter(|b| b.status == "skipped").count();
    let failed = result.benchmarks.iter().filter(|b| b.status == "failed").count();
    println!("Completed: {}, Skipped: {}, Failed: {}", complete, skipped, failed);

    println!("\nIntegrity:");
    println!("  Valid: {}", result.integrity.valid);
    println!("  Expected: {}, Completed: {}, Failed: {}, Skipped: {}",
        result.integrity.repositories_expected,
        result.integrity.repositories_completed,
        result.integrity.repositories_failed,
        result.integrity.repositories_skipped);
    if !result.integrity.warnings.is_empty() {
        println!("  Warnings: {}", result.integrity.warnings.join(", "));
    }
    if !result.integrity.errors.is_empty() {
        println!("  Errors: {}", result.integrity.errors.join(", "));
    }

    if let Some(der) = result.aggregate_derivation() {
        println!("\nDerivation (aggregate):");
        println!("  Mean time: {:.2}s", der.time_ms / 1000.0);
        println!("  Files/sec: {:.0}", der.files_per_second);
        println!("  LOC/sec: {:.0}", der.loc_per_second);
        println!("  Symbols/sec: {:.0}", der.symbols_per_second);
        println!("  Relations/sec: {:.0}", der.relationships_per_second);
        println!("  Peak memory: {:.1} MB", der.peak_memory_bytes as f64 / (1024.0 * 1024.0));
        println!("  Parse: {:.0}ms, Index: {:.0}ms", der.parse_time_us / 1000.0, der.index_time_us / 1000.0);
    }

    if let Some(art) = result.aggregate_artifact() {
        println!("\nArtifact (aggregate):");
        println!("  Mean size: {:.1} KB", art.artifact_bytes as f64 / 1024.0);
        println!("  Artifact/Source ratio: {:.3}", art.artifact_to_source_ratio);
        println!("  Source reduction: {:.1}%", art.source_reduction_ratio * 100.0);
    }

    if let Some(ret) = result.aggregate_retrieval() {
        println!("\nRetrieval (aggregate):");
        println!("  Cold p50: {:.0}µs, p95: {:.0}µs", ret.cold.median, ret.cold.p95);
        println!("  Warm p50: {:.0}µs, p95: {:.0}µs", ret.warm.median, ret.warm.p95);
        println!("  Search p50: {:.0}µs, p95: {:.0}µs", ret.search.median, ret.search.p95);
        println!("  Lookup p50: {:.0}µs, p95: {:.0}µs", ret.lookup.median, ret.lookup.p95);
        println!("  Context p50: {:.0}µs, p95: {:.0}µs", ret.context.median, ret.context.p95);
        println!("  Repeated p50: {:.0}µs, p95: {:.0}µs", ret.repeated.median, ret.repeated.p95);
    }

    if let Some(know) = result.aggregate_knowledge() {
        println!("\nKnowledge (aggregate):");
        println!("  Total questions: {}", know.total_questions);
        println!("  Accuracy: {:.1}%", know.accuracy * 100.0);
        println!("  Source-free accuracy: {:.1}%", know.source_free_accuracy * 100.0);
    }

    if let Some(savings) = result.aggregate_source_savings() {
        println!("\nSource Savings (aggregate):");
        println!("  Baseline files: {}, Prime files: {}", savings.source_files_retrieved_baseline, savings.source_files_retrieved_prime);
        println!("  Baseline bytes: {:.1} KB, Prime bytes: {:.1} KB",
            savings.source_bytes_retrieved_baseline as f64 / 1024.0,
            savings.source_bytes_retrieved_prime as f64 / 1024.0);
        println!("  Reduction: {:.1}%", savings.reduction_ratio * 100.0);
    }
}

// --- Data structures ---

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct EnvironmentInfo {
    os: String,
    arch: String,
    cpu: String,
    cpu_cores: usize,
    memory_bytes: u64,
    runtime: String,
    runtime_version: String,
    compiler_version: String,
    kernel_version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct GitInfo {
    commit: String,
    dirty: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct RepoConfig {
    name: String,
    path: String,
    language: String,
    size_category: String,
    commit: String,
    url: String,
    files: u32,
    source_bytes: u64,
    lines_of_code: u32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct PrimeInfo {
    version: String,
    git_commit: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Integrity {
    valid: bool,
    repositories_expected: usize,
    repositories_completed: usize,
    repositories_failed: usize,
    repositories_skipped: usize,
    metrics_valid: bool,
    warnings: Vec<String>,
    errors: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct BenchmarkResult {
    schema: String,
    schema_version: String,
    benchmark: BenchmarkInfo,
    prime: PrimeInfo,
    benchmark_version: String,
    timestamp: String,
    environment: EnvironmentInfo,
    corpus: Vec<RepoConfig>,
    benchmarks: Vec<RepoBenchmark>,
    bmf: serde_json::Value,
    integrity: Integrity,
    status: String,
    reason: Option<String>,
}

impl BenchmarkResult {
    fn new(git_commit: String, env: EnvironmentInfo, corpus: Vec<RepoConfig>) -> Self {
        Self {
            schema: "prime-benchmark-result".to_string(),
            schema_version: "1.0.0".to_string(),
            benchmark: BenchmarkInfo {
                name: "Prime Benchmark".to_string(),
                version: "1.0.0".to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
                git_commit: git_commit.clone(),
                dirty: false,
            },
            prime: PrimeInfo {
                version: String::new(),
                git_commit: git_commit.clone(),
            },
            benchmark_version: String::new(),
            timestamp: String::new(),
            environment: env,
            corpus,
            benchmarks: Vec::new(),
            bmf: serde_json::Value::Object(serde_json::Map::new()),
            integrity: Integrity::default(),
            status: "partial".to_string(),
            reason: None,
        }
    }

    fn compute_aggregates(&mut self) {
        // Nothing to do here, aggregates computed on demand for printing
    }

    fn compute_integrity(&mut self) {
        let expected = self.corpus.len();
        let completed = self.benchmarks.iter().filter(|b| b.status == "complete").count();
        let failed = self.benchmarks.iter().filter(|b| b.status == "failed").count();
        let skipped = self.benchmarks.iter().filter(|b| b.status == "skipped").count();

        let mut warnings = Vec::new();
        let mut errors = Vec::new();
        let mut metrics_valid = true;

        // Check if all expected repos were completed
        if completed != expected {
            if skipped > 0 {
                warnings.push(format!("{} repositories skipped (not found locally)", skipped));
            }
            if failed > 0 {
                errors.push(format!("{} repositories failed", failed));
                metrics_valid = false;
            }
        }

        // Check derivation metrics validity
        for b in &self.benchmarks {
            if b.status == "complete" {
                if b.derivation.time_ms <= 0.0 {
                    errors.push(format!("{}: invalid derivation time", b.repo_name));
                    metrics_valid = false;
                }
                if b.derivation.files == 0 {
                    warnings.push(format!("{}: zero files in derivation", b.repo_name));
                }
                if b.derivation.source_bytes == 0 {
                    warnings.push(format!("{}: zero source bytes", b.repo_name));
                }
                if b.derivation.lines_of_code == 0 {
                    warnings.push(format!("{}: zero lines of code", b.repo_name));
                }
                if b.derivation.peak_memory_bytes == 0 {
                    warnings.push(format!("{}: peak memory not measured", b.repo_name));
                }
            }
        }

        // Check knowledge metrics
        for b in &self.benchmarks {
            if b.status == "complete" && b.knowledge.total_questions == 0 {
                warnings.push(format!("{}: no knowledge questions evaluated", b.repo_name));
            }
        }

        // Check source savings (not measured, so just warn)
        for b in &self.benchmarks {
            if b.status == "complete" && b.source_savings.is_none() {
                warnings.push(format!("{}: source savings not measured (requires controlled baseline)", b.repo_name));
            }
        }

        let valid = errors.is_empty() && completed == expected;

        self.integrity = Integrity {
            valid,
            repositories_expected: expected,
            repositories_completed: completed,
            repositories_failed: failed,
            repositories_skipped: skipped,
            metrics_valid,
            warnings,
            errors,
        };
    }

    fn aggregate_derivation(&self) -> Option<DerivationMetrics> {
        let complete: Vec<&RepoBenchmark> = self.benchmarks.iter().filter(|b| b.derivation.time_ms > 0.0).collect();
        if complete.is_empty() { return None; }
        let n = complete.len() as f64;
        Some(DerivationMetrics {
            time_ms: complete.iter().map(|b| b.derivation.time_ms).sum::<f64>() / n,
            files: (complete.iter().map(|b| b.derivation.files as usize).sum::<usize>() / complete.len()) as u32,
            source_bytes: (complete.iter().map(|b| b.derivation.source_bytes).sum::<u64>() / complete.len() as u64),
            lines_of_code: (complete.iter().map(|b| b.derivation.lines_of_code as usize).sum::<usize>() / complete.len()) as u32,
            non_empty_lines: (complete.iter().map(|b| b.derivation.non_empty_lines as usize).sum::<usize>() / complete.len()) as u32,
            entities: complete.iter().map(|b| b.derivation.entities).sum::<usize>() / complete.len(),
            symbols: complete.iter().map(|b| b.derivation.symbols).sum::<usize>() / complete.len(),
            relations: complete.iter().map(|b| b.derivation.relations).sum::<usize>() / complete.len(),
            files_per_second: complete.iter().map(|b| b.derivation.files_per_second).sum::<f64>() / n,
            loc_per_second: complete.iter().map(|b| b.derivation.loc_per_second).sum::<f64>() / n,
            symbols_per_second: complete.iter().map(|b| b.derivation.symbols_per_second).sum::<f64>() / n,
            relationships_per_second: complete.iter().map(|b| b.derivation.relationships_per_second).sum::<f64>() / n,
            peak_memory_bytes: complete.iter().map(|b| b.derivation.peak_memory_bytes).sum::<u64>() / complete.len() as u64,
            parse_time_us: complete.iter().map(|b| b.derivation.parse_time_us).sum::<f64>() / n,
            index_time_us: complete.iter().map(|b| b.derivation.index_time_us).sum::<f64>() / n,
            serialization_time_us: complete.iter().map(|b| b.derivation.serialization_time_us).sum::<f64>() / n,
        })
    }

    fn aggregate_artifact(&self) -> Option<ArtifactMetrics> {
        let complete: Vec<&RepoBenchmark> = self.benchmarks.iter().filter(|b| b.artifact.artifact_bytes > 0).collect();
        if complete.is_empty() { return None; }
        let n = complete.len() as u64;
        Some(ArtifactMetrics {
            source_bytes: complete.iter().map(|b| b.artifact.source_bytes).sum::<u64>() / n,
            artifact_bytes: complete.iter().map(|b| b.artifact.artifact_bytes).sum::<u64>() / n,
            artifact_to_source_ratio: complete.iter().map(|b| b.artifact.artifact_to_source_ratio).sum::<f64>() / n as f64,
            source_reduction_ratio: complete.iter().map(|b| b.artifact.source_reduction_ratio).sum::<f64>() / n as f64,
            artifact_bytes_per_kloc: complete.iter().map(|b| b.artifact.artifact_bytes_per_kloc).sum::<f64>() / n as f64,
        })
    }

    fn aggregate_retrieval(&self) -> Option<RetrievalMetrics> {
        let complete: Vec<&RepoBenchmark> = self.benchmarks.iter().filter(|b| b.retrieval.warm.mean > 0.0).collect();
        if complete.is_empty() { return None; }
        let n = complete.len() as f64;
        Some(RetrievalMetrics {
            cold: LatencyStats {
                samples: (complete.iter().map(|b| b.retrieval.cold.samples).sum::<usize>() / complete.len()),
                mean: complete.iter().map(|b| b.retrieval.cold.mean).sum::<f64>() / n,
                median: complete.iter().map(|b| b.retrieval.cold.median).sum::<f64>() / n,
                p95: complete.iter().map(|b| b.retrieval.cold.p95).sum::<f64>() / n,
                p99: complete.iter().map(|b| b.retrieval.cold.p99).sum::<f64>() / n,
                min: complete.iter().map(|b| b.retrieval.cold.min).sum::<f64>() / n,
                max: complete.iter().map(|b| b.retrieval.cold.max).sum::<f64>() / n,
            },
            warm: LatencyStats {
                samples: (complete.iter().map(|b| b.retrieval.warm.samples).sum::<usize>() / complete.len()),
                mean: complete.iter().map(|b| b.retrieval.warm.mean).sum::<f64>() / n,
                median: complete.iter().map(|b| b.retrieval.warm.median).sum::<f64>() / n,
                p95: complete.iter().map(|b| b.retrieval.warm.p95).sum::<f64>() / n,
                p99: complete.iter().map(|b| b.retrieval.warm.p99).sum::<f64>() / n,
                min: complete.iter().map(|b| b.retrieval.warm.min).sum::<f64>() / n,
                max: complete.iter().map(|b| b.retrieval.warm.max).sum::<f64>() / n,
            },
            search: LatencyStats {
                samples: (complete.iter().map(|b| b.retrieval.search.samples).sum::<usize>() / complete.len()),
                mean: complete.iter().map(|b| b.retrieval.search.mean).sum::<f64>() / n,
                median: complete.iter().map(|b| b.retrieval.search.median).sum::<f64>() / n,
                p95: complete.iter().map(|b| b.retrieval.search.p95).sum::<f64>() / n,
                p99: complete.iter().map(|b| b.retrieval.search.p99).sum::<f64>() / n,
                min: complete.iter().map(|b| b.retrieval.search.min).sum::<f64>() / n,
                max: complete.iter().map(|b| b.retrieval.search.max).sum::<f64>() / n,
            },
            lookup: LatencyStats {
                samples: (complete.iter().map(|b| b.retrieval.lookup.samples).sum::<usize>() / complete.len()),
                mean: complete.iter().map(|b| b.retrieval.lookup.mean).sum::<f64>() / n,
                median: complete.iter().map(|b| b.retrieval.lookup.median).sum::<f64>() / n,
                p95: complete.iter().map(|b| b.retrieval.lookup.p95).sum::<f64>() / n,
                p99: complete.iter().map(|b| b.retrieval.lookup.p99).sum::<f64>() / n,
                min: complete.iter().map(|b| b.retrieval.lookup.min).sum::<f64>() / n,
                max: complete.iter().map(|b| b.retrieval.lookup.max).sum::<f64>() / n,
            },
            context: LatencyStats {
                samples: (complete.iter().map(|b| b.retrieval.context.samples).sum::<usize>() / complete.len()),
                mean: complete.iter().map(|b| b.retrieval.context.mean).sum::<f64>() / n,
                median: complete.iter().map(|b| b.retrieval.context.median).sum::<f64>() / n,
                p95: complete.iter().map(|b| b.retrieval.context.p95).sum::<f64>() / n,
                p99: complete.iter().map(|b| b.retrieval.context.p99).sum::<f64>() / n,
                min: complete.iter().map(|b| b.retrieval.context.min).sum::<f64>() / n,
                max: complete.iter().map(|b| b.retrieval.context.max).sum::<f64>() / n,
            },
            repeated: LatencyStats {
                samples: (complete.iter().map(|b| b.retrieval.repeated.samples).sum::<usize>() / complete.len()),
                mean: complete.iter().map(|b| b.retrieval.repeated.mean).sum::<f64>() / n,
                median: complete.iter().map(|b| b.retrieval.repeated.median).sum::<f64>() / n,
                p95: complete.iter().map(|b| b.retrieval.repeated.p95).sum::<f64>() / n,
                p99: complete.iter().map(|b| b.retrieval.repeated.p99).sum::<f64>() / n,
                min: complete.iter().map(|b| b.retrieval.repeated.min).sum::<f64>() / n,
                max: complete.iter().map(|b| b.retrieval.repeated.max).sum::<f64>() / n,
            },
        })
    }

    fn aggregate_knowledge(&self) -> Option<KnowledgeMetrics> {
        let complete: Vec<&RepoBenchmark> = self.benchmarks.iter().filter(|b| b.knowledge.total_questions > 0).collect();
        if complete.is_empty() { return None; }
        let total: usize = complete.iter().map(|b| b.knowledge.total_questions).sum();
        let correct: usize = complete.iter().map(|b| b.knowledge.correct).sum();
        let source_free_correct: usize = complete.iter().map(|b| b.knowledge.source_free_correct).sum();
        let source_required_correct: usize = complete.iter().map(|b| b.knowledge.source_required_correct).sum();
        
        // Properly aggregate retrieval metrics
        let entity_precision = if complete.iter().map(|b| b.knowledge.entity_precision as f64).sum::<f64>() > 0.0 {
            complete.iter().map(|b| b.knowledge.entity_precision as f64).sum::<f64>() / complete.len() as f64
        } else { 0.0 };
        let entity_recall = if complete.iter().map(|b| b.knowledge.entity_recall as f64).sum::<f64>() > 0.0 {
            complete.iter().map(|b| b.knowledge.entity_recall as f64).sum::<f64>() / complete.len() as f64
        } else { 0.0 };
        let entity_f1 = if complete.iter().map(|b| b.knowledge.entity_f1 as f64).sum::<f64>() > 0.0 {
            complete.iter().map(|b| b.knowledge.entity_f1 as f64).sum::<f64>() / complete.len() as f64
        } else { 0.0 };
        let relationship_precision = if complete.iter().map(|b| b.knowledge.relationship_precision as f64).sum::<f64>() > 0.0 {
            complete.iter().map(|b| b.knowledge.relationship_precision as f64).sum::<f64>() / complete.len() as f64
        } else { 0.0 };
        let relationship_recall = if complete.iter().map(|b| b.knowledge.relationship_recall as f64).sum::<f64>() > 0.0 {
            complete.iter().map(|b| b.knowledge.relationship_recall as f64).sum::<f64>() / complete.len() as f64
        } else { 0.0 };
        let relationship_f1 = if complete.iter().map(|b| b.knowledge.relationship_f1 as f64).sum::<f64>() > 0.0 {
            complete.iter().map(|b| b.knowledge.relationship_f1 as f64).sum::<f64>() / complete.len() as f64
        } else { 0.0 };
        let mrr = if complete.iter().map(|b| b.knowledge.mrr as f64).sum::<f64>() > 0.0 {
            complete.iter().map(|b| b.knowledge.mrr as f64).sum::<f64>() / complete.len() as f64
        } else { 0.0 };
        let recall_at_1 = if complete.iter().map(|b| b.knowledge.recall_at_1 as f64).sum::<f64>() > 0.0 {
            complete.iter().map(|b| b.knowledge.recall_at_1 as f64).sum::<f64>() / complete.len() as f64
        } else { 0.0 };
        let recall_at_3 = if complete.iter().map(|b| b.knowledge.recall_at_3 as f64).sum::<f64>() > 0.0 {
            complete.iter().map(|b| b.knowledge.recall_at_3 as f64).sum::<f64>() / complete.len() as f64
        } else { 0.0 };
        let recall_at_5 = if complete.iter().map(|b| b.knowledge.recall_at_5 as f64).sum::<f64>() > 0.0 {
            complete.iter().map(|b| b.knowledge.recall_at_5 as f64).sum::<f64>() / complete.len() as f64
        } else { 0.0 };
        let recall_at_10 = if complete.iter().map(|b| b.knowledge.recall_at_10 as f64).sum::<f64>() > 0.0 {
            complete.iter().map(|b| b.knowledge.recall_at_10 as f64).sum::<f64>() / complete.len() as f64
        } else { 0.0 };
        
        // Aggregate by_category
        let mut by_category = HashMap::new();
        for b in &complete {
            for (cat, stats) in &b.knowledge.by_category {
                let entry = by_category.entry(cat.clone()).or_insert(CategoryStats::default());
                entry.total += stats.total;
                entry.correct += stats.correct;
                entry.source_free_correct += stats.source_free_correct;
                entry.source_required_correct += stats.source_required_correct;
                entry.accuracy = if entry.total > 0 { entry.correct as f64 / entry.total as f64 } else { 0.0 };
                entry.source_free_accuracy = if entry.total > 0 { entry.source_free_correct as f64 / entry.total as f64 } else { 0.0 };
                // Aggregate retrieval metrics per category (weighted average)
                let weight = stats.total as f64;
                entry.entity_precision = (entry.entity_precision * (entry.total - stats.total) as f64 + stats.entity_precision * weight) / entry.total as f64;
                entry.entity_recall = (entry.entity_recall * (entry.total - stats.total) as f64 + stats.entity_recall * weight) / entry.total as f64;
                entry.entity_f1 = (entry.entity_f1 * (entry.total - stats.total) as f64 + stats.entity_f1 * weight) / entry.total as f64;
                entry.relationship_precision = (entry.relationship_precision * (entry.total - stats.total) as f64 + stats.relationship_precision * weight) / entry.total as f64;
                entry.relationship_recall = (entry.relationship_recall * (entry.total - stats.total) as f64 + stats.relationship_recall * weight) / entry.total as f64;
                entry.relationship_f1 = (entry.relationship_f1 * (entry.total - stats.total) as f64 + stats.relationship_f1 * weight) / entry.total as f64;
                entry.mrr = (entry.mrr * (entry.total - stats.total) as f64 + stats.mrr * weight) / entry.total as f64;
                entry.recall_at_1 = (entry.recall_at_1 * (entry.total - stats.total) as f64 + stats.recall_at_1 * weight) / entry.total as f64;
                entry.recall_at_3 = (entry.recall_at_3 * (entry.total - stats.total) as f64 + stats.recall_at_3 * weight) / entry.total as f64;
                entry.recall_at_5 = (entry.recall_at_5 * (entry.total - stats.total) as f64 + stats.recall_at_5 * weight) / entry.total as f64;
                entry.recall_at_10 = (entry.recall_at_10 * (entry.total - stats.total) as f64 + stats.recall_at_10 * weight) / entry.total as f64;
            }
        }
        
        Some(KnowledgeMetrics {
            total_questions: total,
            correct,
            incorrect: total - correct,
            source_free_correct,
            source_required_correct,
            accuracy: if total > 0 { correct as f64 / total as f64 } else { 0.0 },
            source_free_accuracy: if total > 0 { source_free_correct as f64 / total as f64 } else { 0.0 },
            entity_precision,
            entity_recall,
            entity_f1,
            relationship_precision,
            relationship_recall,
            relationship_f1,
            mrr,
            recall_at_1,
            recall_at_3,
            recall_at_5,
            recall_at_10,
            by_category,
        })
    }

    fn generate_bmf(&mut self) {
        let mut bmf = serde_json::Map::new();

        if let Some(der) = self.aggregate_derivation() {
            bmf.insert("prime::derivation".to_string(), serde_json::json!({
                "latency": { "value": der.time_ms, "unit": "ms" }
            }));
        }
        if let Some(art) = self.aggregate_artifact() {
            bmf.insert("prime::artifact".to_string(), serde_json::json!({
                "file-size": { "value": art.artifact_bytes, "unit": "bytes" }
            }));
        }
        if let Some(ret) = self.aggregate_retrieval() {
            bmf.insert("prime::retrieval::cold".to_string(), serde_json::json!({
                "latency": { "value": ret.cold.median, "unit": "us" }
            }));
            bmf.insert("prime::retrieval::warm".to_string(), serde_json::json!({
                "latency": { "value": ret.warm.median, "unit": "us" }
            }));
            bmf.insert("prime::retrieval::search".to_string(), serde_json::json!({
                "latency": { "value": ret.search.median, "unit": "us" }
            }));
            bmf.insert("prime::retrieval::lookup".to_string(), serde_json::json!({
                "latency": { "value": ret.lookup.median, "unit": "us" }
            }));
            bmf.insert("prime::retrieval::context".to_string(), serde_json::json!({
                "latency": { "value": ret.context.median, "unit": "us" }
            }));
            bmf.insert("prime::retrieval::repeated".to_string(), serde_json::json!({
                "latency": { "value": ret.repeated.median, "unit": "us" }
            }));
        }
        if let Some(know) = self.aggregate_knowledge() {
            bmf.insert("prime::knowledge::accuracy".to_string(), serde_json::json!({
                "ratio": { "value": know.accuracy }
            }));
            bmf.insert("prime::knowledge::source_free_accuracy".to_string(), serde_json::json!({
                "ratio": { "value": know.source_free_accuracy }
            }));
        }
        if let Some(savings) = self.aggregate_source_savings() {
            bmf.insert("prime::source_savings::reduction".to_string(), serde_json::json!({
                "ratio": { "value": savings.reduction_ratio }
            }));
        }

        self.bmf = serde_json::Value::Object(bmf);
    }

    fn aggregate_source_savings(&self) -> Option<SourceSavings> {
        let complete: Vec<&RepoBenchmark> = self.benchmarks.iter().filter(|b| b.source_savings.as_ref().map(|s| s.reduction_ratio).unwrap_or(0.0) > 0.0).collect();
        if complete.is_empty() { return None; }
        let n = complete.len() as usize;
        Some(SourceSavings {
            source_files_retrieved_baseline: complete.iter().map(|b| b.source_savings.as_ref().map(|s| s.source_files_retrieved_baseline).unwrap_or(0)).sum::<usize>() / n,
            source_files_retrieved_prime: complete.iter().map(|b| b.source_savings.as_ref().map(|s| s.source_files_retrieved_prime).unwrap_or(0)).sum::<usize>() / n,
            source_bytes_retrieved_baseline: complete.iter().map(|b| b.source_savings.as_ref().map(|s| s.source_bytes_retrieved_baseline).unwrap_or(0)).sum::<u64>() / n as u64,
            source_bytes_retrieved_prime: complete.iter().map(|b| b.source_savings.as_ref().map(|s| s.source_bytes_retrieved_prime).unwrap_or(0)).sum::<u64>() / n as u64,
            reduction_ratio: complete.iter().map(|b| b.source_savings.as_ref().map(|s| s.reduction_ratio).unwrap_or(0.0)).sum::<f64>() / n as f64,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct BenchmarkInfo {
    name: String,
    version: String,
    timestamp: String,
    git_commit: String,
    dirty: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct RepoBenchmark {
    repo_name: String,
    status: String,
    error: Option<String>,
    derivation: DerivationMetrics,
    artifact: ArtifactMetrics,
    retrieval: RetrievalMetrics,
    knowledge: KnowledgeMetrics,
    source_savings: Option<SourceSavings>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct DerivationMetrics {
    time_ms: f64,
    files: u32,
    source_bytes: u64,
    lines_of_code: u32,
    non_empty_lines: u32,
    entities: usize,
    symbols: usize,
    relations: usize,
    files_per_second: f64,
    loc_per_second: f64,
    symbols_per_second: f64,
    relationships_per_second: f64,
    peak_memory_bytes: u64,
    parse_time_us: f64,
    index_time_us: f64,
    serialization_time_us: f64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ArtifactMetrics {
    source_bytes: u64,
    artifact_bytes: u64,
    artifact_to_source_ratio: f64,
    source_reduction_ratio: f64,
    artifact_bytes_per_kloc: f64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct RetrievalMetrics {
    cold: LatencyStats,
    warm: LatencyStats,
    search: LatencyStats,
    lookup: LatencyStats,
    context: LatencyStats,
    repeated: LatencyStats,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct LatencyStats {
    samples: usize,
    min: f64,
    mean: f64,
    median: f64,
    p95: f64,
    p99: f64,
    max: f64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct KnowledgeMetrics {
    total_questions: usize,
    correct: usize,
    incorrect: usize,
    source_free_correct: usize,
    source_required_correct: usize,
    accuracy: f64,
    source_free_accuracy: f64,
    // Retrieval metrics
    entity_precision: f64,
    entity_recall: f64,
    entity_f1: f64,
    relationship_precision: f64,
    relationship_recall: f64,
    relationship_f1: f64,
    mrr: f64,
    recall_at_1: f64,
    recall_at_3: f64,
    recall_at_5: f64,
    recall_at_10: f64,
    by_category: HashMap<String, CategoryStats>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct CategoryResult {
    total: usize,
    correct: usize,
    source_free_correct: usize,
    source_required_correct: usize,
    // Retrieval metrics per category
    entity_precision: f64,
    entity_recall: f64,
    entity_f1: f64,
    relationship_precision: f64,
    relationship_recall: f64,
    relationship_f1: f64,
    mrr: f64,
    recall_at_1: f64,
    recall_at_3: f64,
    recall_at_5: f64,
    recall_at_10: f64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct CategoryStats {
    total: usize,
    correct: usize,
    source_free_correct: usize,
    source_required_correct: usize,
    accuracy: f64,
    source_free_accuracy: f64,
    // Retrieval metrics
    entity_precision: f64,
    entity_recall: f64,
    entity_f1: f64,
    relationship_precision: f64,
    relationship_recall: f64,
    relationship_f1: f64,
    mrr: f64,
    recall_at_1: f64,
    recall_at_3: f64,
    recall_at_5: f64,
    recall_at_10: f64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct SourceSavings {
    source_files_retrieved_baseline: usize,
    source_files_retrieved_prime: usize,
    source_bytes_retrieved_baseline: u64,
    source_bytes_retrieved_prime: u64,
    reduction_ratio: f64,
}

/// I/O Metrics for benchmark tracking
#[derive(Serialize, Deserialize, Debug, Default)]
struct IoMetrics {
    /// Total bytes read from storage
    pub bytes_read: u64,
    /// Number of cache hits
    pub cache_hits: u64,
    /// Number of cache misses
    pub cache_misses: u64,
    /// Number of pages touched in memory
    pub pages_touched: u64,
    /// Number of mmap reads
    pub mmap_reads: u64,
    /// Number of file system reads
    pub fs_reads: u64,
    /// Time spent in I/O operations (microseconds)
    pub io_time_us: u64,
    /// Peak memory usage during operation (bytes)
    pub peak_memory_bytes: u64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct IoMetricsAggregate {
    pub total_bytes_read: u64,
    pub total_cache_hits: u64,
    pub total_cache_misses: u64,
    pub total_pages_touched: u64,
    pub total_mmap_reads: u64,
    pub total_fs_reads: u64,
    pub total_io_time_us: u64,
    pub peak_memory_bytes: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct KnowledgeQuestion {
    id: String,
    category: String,
    question: String,
    search_query: String,
    expected_entities: Vec<String>,
    expected_relationships: Vec<Vec<String>>,
    expected_kind: Option<String>,
    evaluation: String,
    source_allowed: bool,
}