//! Incremental indexing for Prime
//!
//! Provides file-level change detection and entity-level invalidation
//! for efficient updates without full rebuilds.

use prime_core::{ContentHash, EntityId, Entity, KnowledgeGraph, File, Range, Position};
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};

/// File change detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    pub path: PathBuf,
    pub change_type: ChangeType,
    pub old_hash: Option<ContentHash>,
    pub new_hash: ContentHash,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeType {
    Added,
    Modified,
    Removed,
}

/// Tracks file content hashes for change detection
pub struct FileChangeDetector {
    /// Map of file path to content hash
    hashes: HashMap<PathBuf, ContentHash>,
    /// Root directory being tracked
    root: PathBuf,
    /// File extensions to track
    tracked_extensions: HashSet<String>,
}

impl FileChangeDetector {
    /// Create a new change detector
    pub fn new(root: PathBuf) -> Self {
        let tracked_extensions = HashSet::from([
            "rs".to_string(),
            "py".to_string(),
            "js".to_string(),
            "ts".to_string(),
            "tsx".to_string(),
            "jsx".to_string(),
            "go".to_string(),
            "java".to_string(),
            "c".to_string(),
            "cpp".to_string(),
            "h".to_string(),
            "hpp".to_string(),
            "cs".to_string(),
            "rb".to_string(),
            "php".to_string(),
        ]);

        Self {
            hashes: HashMap::new(),
            root,
            tracked_extensions,
        }
    }

    /// Create a new change detector with custom tracked extensions
    pub fn with_extensions(root: PathBuf, extensions: HashSet<String>) -> Self {
        Self {
            hashes: HashMap::new(),
            root,
            tracked_extensions: extensions,
        }
    }

    /// Check if a file should be tracked based on extension
    fn should_track(&self, path: &Path) -> bool {
        match path.extension().and_then(|e| e.to_str()) {
            Some(ext) => self.tracked_extensions.contains(ext),
            None => false,
        }
    }

    /// Scan the root directory and detect changes
    pub fn detect_changes(&mut self) -> Result<Vec<FileChange>> {
        let mut current_hashes = HashMap::new();
        let mut changes = Vec::new();

        // Walk the directory and compute hashes
        self.walk_directory(&self.root, &mut current_hashes)?;

        // Detect modifications and removals
        for (path, old_hash) in &self.hashes {
            match current_hashes.get(path) {
                Some(new_hash) => {
                    if old_hash != new_hash {
                        changes.push(FileChange {
                            path: path.clone(),
                            change_type: ChangeType::Modified,
                            old_hash: Some(old_hash.clone()),
                            new_hash: new_hash.clone(),
                        });
                    }
                }
                None => {
                    changes.push(FileChange {
                        path: path.clone(),
                        change_type: ChangeType::Removed,
                        old_hash: Some(old_hash.clone()),
                        new_hash: ContentHash::new(),
                    });
                }
            }
        }

        // Detect additions
        for (path, hash) in &current_hashes {
            if !self.hashes.contains_key(path) {
                changes.push(FileChange {
                    path: path.clone(),
                    change_type: ChangeType::Added,
                    old_hash: None,
                    new_hash: hash.clone(),
                });
            }
        }

        // Update stored hashes
        self.hashes = current_hashes;

        Ok(changes)
    }

    /// Recursively walk directory and compute content hashes
    fn walk_directory(&self, dir: &Path, hashes: &mut HashMap<PathBuf, ContentHash>) -> Result<()> {
        if !dir.is_dir() {
            return Ok(());
        }

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Skip hidden directories and target directories
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with('.') || name == "target" || name == "node_modules" {
                        continue;
                    }
                }
                self.walk_directory(&path, hashes)?;
            } else if self.should_track(&path) {
                let content = std::fs::read(&path)?;
                let hash = ContentHash::from_bytes(&content);
                hashes.insert(path, hash);
            }
        }

        Ok(())
    }

    /// Load previously stored hashes from a file
    pub fn load_hashes(&mut self, path: &Path) -> Result<()> {
        if path.exists() {
            let data = std::fs::read(path)?;
            self.hashes = bincode::deserialize(&data)?;
        }
        Ok(())
    }

    /// Save current hashes to a file
    pub fn save_hashes(&self, path: &Path) -> Result<()> {
        let data = bincode::serialize(&self.hashes)?;
        std::fs::write(path, data)?;
        Ok(())
    }

    /// Get the number of tracked files
    pub fn file_count(&self) -> usize {
        self.hashes.len()
    }
}

/// Entity-level invalidation tracker
pub struct EntityInvalidator {
    /// Map from file path to entity IDs in that file
    file_entities: HashMap<PathBuf, Vec<EntityId>>,
    /// Map from file path to relations originating from that file
    file_relations: HashMap<PathBuf, Vec<usize>>,
}

impl EntityInvalidator {
    pub fn new() -> Self {
        Self {
            file_entities: HashMap::new(),
            file_relations: HashMap::new(),
        }
    }

    /// Build the invalidation index from a knowledge graph
    pub fn build_index(&mut self, graph: &KnowledgeGraph) {
        self.file_entities.clear();
        self.file_relations.clear();

        // Index entities by file
        for entity in graph.entities.values() {
            if let Some(file) = graph.files.get(&entity.file_id) {
                self.file_entities
                    .entry(file.path.clone().into())
                    .or_default()
                    .push(entity.id);
            }
        }

        // Index relations by source entity's file
        for (idx, relation) in graph.relations.iter().enumerate() {
            if let Some(entity) = graph.entities.get(&relation.from) {
                if let Some(file) = graph.files.get(&entity.file_id) {
                    self.file_relations
                        .entry(file.path.clone().into())
                        .or_default()
                        .push(idx);
                }
            }
        }
    }

    /// Get entity IDs affected by a file change
    pub fn affected_entities(&self, file_path: &Path) -> Vec<EntityId> {
        self.file_entities
            .get(file_path)
            .cloned()
            .unwrap_or_default()
    }

    /// Get relation indices affected by a file change
    pub fn affected_relations(&self, file_path: &Path) -> Vec<usize> {
        self.file_relations
            .get(file_path)
            .cloned()
            .unwrap_or_default()
    }

    /// Get all files that reference entities from a given file
    pub fn dependent_files(&self, file_path: &Path, graph: &KnowledgeGraph) -> HashSet<PathBuf> {
        let mut dependents = HashSet::new();

        // Get entities in the changed file
        let affected_entities = self.affected_entities(file_path);

        // Find relations that reference these entities
        for relation in &graph.relations {
            if affected_entities.contains(&relation.to) {
                // This relation points to an entity in the changed file
                if let Some(source_entity) = graph.entities.get(&relation.from) {
                    if let Some(source_file) = graph.files.get(&source_entity.file_id) {
                        dependents.insert(source_file.path.clone().into());
                    }
                }
            }
        }

        dependents
    }
}

/// Incremental indexer for efficient updates
pub struct IncrementalIndexer {
    change_detector: FileChangeDetector,
    invalidator: EntityInvalidator,
    /// Path to store hash state
    state_path: PathBuf,
}

impl IncrementalIndexer {
    pub fn new(root: PathBuf) -> Self {
        let state_path = root.join(".prime").join("change_state.bin");
        let change_detector = FileChangeDetector::new(root.clone());

        Self {
            change_detector,
            invalidator: EntityInvalidator::new(),
            state_path,
        }
    }

    /// Initialize the indexer, loading any previous state
    pub fn init(&mut self) -> Result<()> {
        if self.state_path.exists() {
            self.change_detector.load_hashes(&self.state_path)?;
        }
        Ok(())
    }

    /// Detect changes and return files that need updating
    pub fn detect_changes(&mut self) -> Result<Vec<FileChange>> {
        let changes = self.change_detector.detect_changes()?;

        // Save state for next run
        if let Some(parent) = self.state_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        self.change_detector.save_hashes(&self.state_path)?;

        Ok(changes)
    }

    /// Build invalidation index from a knowledge graph
    pub fn build_invalidation_index(&mut self, graph: &KnowledgeGraph) {
        self.invalidator.build_index(graph);
    }

    /// Get entities affected by file changes
    pub fn affected_entities(&self, file_path: &Path) -> Vec<EntityId> {
        self.invalidator.affected_entities(file_path)
    }

    /// Get dependent files that may need re-computation
    pub fn dependent_files(&self, file_path: &Path, graph: &KnowledgeGraph) -> HashSet<PathBuf> {
        self.invalidator.dependent_files(file_path, graph)
    }

    /// Get the number of tracked files
    pub fn file_count(&self) -> usize {
        self.change_detector.file_count()
    }
}#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_file_change_detection() {
        let temp_dir = std::env::temp_dir().join("prime_test_".to_string() + &std::process::id().to_string());
        fs::create_dir_all(&temp_dir).unwrap();
        let root = temp_dir.clone();

        // Create a test file
        fs::write(root.join("test.rs"), "fn main() {}" ).unwrap();

        let mut detector = FileChangeDetector::new(root.clone());
        
        // First scan - should detect addition
        let changes = detector.detect_changes().unwrap();
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].change_type, ChangeType::Added);

        // Second scan - no changes
        let changes = detector.detect_changes().unwrap();
        assert_eq!(changes.len(), 0);

        // Modify file
        fs::write(root.join("test.rs"), "fn main() { println!(\"hello\"); }").unwrap();

        // Third scan - should detect modification
        let changes = detector.detect_changes().unwrap();
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].change_type, ChangeType::Modified);

        // Cleanup
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_entity_invalidation() {
        let mut invalidator = EntityInvalidator::new();
        
        // Create a minimal graph
        let mut graph = KnowledgeGraph::new(prime_core::Project {
            name: "test".to_string(),
            root_path: "/test".to_string(),
            version: "0.1.0".to_string(),
            languages: vec![],
            file_count: 0,
            entity_count: 0,
            relation_count: 0,
            created_at: 0,
            content_hash: ContentHash::new(),
        });

        // Add a file
        let file_id = EntityId::new();
        graph.add_file(File {
            id: file_id,
            path: "test.rs".to_string(),
            language: prime_core::Language::Rust,
            size: 100,
            content_hash: ContentHash::new(),
            entities: vec![],
        });

        // Add an entity in that file
        let entity_id = EntityId::from_str("test::main");
        graph.add_entity(Entity {
            id: entity_id,
            kind: prime_core::SymbolKind::Function,
            name: "main".to_string(),
            qualified_name: "test::main".to_string(),
            file_id,
            range: Range::new(Position::new(1, 0), Position::new(1, 10)),
            language: prime_core::Language::Rust,
            confidence: prime_core::Confidence::Medium,
            signature: None,
            documentation: None,
            children: vec![],
            relations: vec![],
        });

        invalidator.build_index(&graph);

        // Check affected entities
        let affected = invalidator.affected_entities(&PathBuf::from("test.rs"));
        assert_eq!(affected.len(), 1);
        assert!(affected.contains(&entity_id));
    }
}
