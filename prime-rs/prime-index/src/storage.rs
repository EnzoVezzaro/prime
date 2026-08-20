//! Storage layer for the Prime knowledge graph

use prime_core::{KnowledgeGraph, EntityId, Entity, Relation, File, Module, Project, ContentHash, Range, Position, SymbolKind, RelationKind, Confidence, Provenance, Language};
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use indexmap::IndexMap;
use roaring::RoaringBitmap;

/// Storage configuration
#[derive(Debug, Clone)]
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

/// Storage backend trait
pub trait StorageBackend: Send + Sync {
    fn save(&mut self, graph: &KnowledgeGraph) -> Result<()>;
    fn load(&mut self) -> Result<KnowledgeGraph>;
    fn exists(&self) -> bool;
    fn size(&self) -> u64;
}

/// Binary storage using bincode + zstd
pub struct BinaryStorage {
    config: StorageConfig,
    path: PathBuf,
}

impl BinaryStorage {
    pub fn new(config: StorageConfig) -> Self {
        let path = config.path.join("graph.bin");
        Self { config, path }
    }
}

impl StorageBackend for BinaryStorage {
    fn save(&mut self, graph: &KnowledgeGraph) -> Result<()> {
        use std::fs::File;
        use std::io::BufWriter;

        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let file = File::create(&self.path)?;
        let writer = BufWriter::new(file);

        if self.config.compress {
            let mut encoder = zstd::Encoder::new(writer, self.config.compression_level)?;
            encoder.include_checksum(true)?;
            bincode::serialize_into(&mut encoder, graph)?;
            encoder.finish()?;
        } else {
            bincode::serialize_into(writer, graph)?;
        }

        Ok(())
    }

    fn load(&mut self) -> Result<KnowledgeGraph> {
        use std::fs::File;
        use std::io::BufReader;

        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);

        if self.config.compress {
            let decoder = zstd::Decoder::new(reader)?;
            let graph: KnowledgeGraph = bincode::deserialize_from(decoder)?;
            Ok(graph)
        } else {
            let graph: KnowledgeGraph = bincode::deserialize_from(reader)?;
            Ok(graph)
        }
    }

    fn exists(&self) -> bool {
        self.path.exists()
    }

    fn size(&self) -> u64 {
        std::fs::metadata(&self.path).map(|m| m.len()).unwrap_or(0)
    }
}

/// FlatBuffers storage for zero-copy access
pub struct FlatBufferStorage {
    config: StorageConfig,
    path: PathBuf,
}

impl FlatBufferStorage {
    pub fn new(config: StorageConfig) -> Self {
        let path = config.path.join("graph.fbs");
        Self { config, path }
    }
}

impl StorageBackend for FlatBufferStorage {
    fn save(&mut self, graph: &KnowledgeGraph) -> Result<()> {
        // FlatBuffers serialization would go here
        // For now, delegate to binary storage
        let mut binary = BinaryStorage::new(StorageConfig {
            path: self.path.clone(),
            ..self.config.clone()
        });
        binary.save(graph)
    }

    fn load(&mut self) -> Result<KnowledgeGraph> {
        let mut binary = BinaryStorage::new(StorageConfig {
            path: self.path.clone(),
            ..self.config.clone()
        });
        binary.load()
    }

    fn exists(&self) -> bool {
        self.path.exists()
    }

    fn size(&self) -> u64 {
        std::fs::metadata(&self.path).map(|m| m.len()).unwrap_or(0)
    }
}

/// Memory-mapped storage for zero-copy reads
pub struct MmapStorage {
    config: StorageConfig,
    path: PathBuf,
    mmap: Option<memmap2::Mmap>,
}

impl MmapStorage {
    pub fn new(config: StorageConfig) -> Self {
        let path = config.path.join("graph.bin");
        Self {
            config,
            path,
            mmap: None,
        }
    }

    fn ensure_mmap(&mut self) -> Result<()> {
        if self.mmap.is_none() && self.path.exists() {
            let file = std::fs::File::open(&self.path)?;
            let mmap = unsafe { memmap2::Mmap::map(&file)? };
            self.mmap = Some(mmap);
        }
        Ok(())
    }
}

impl StorageBackend for MmapStorage {
    fn save(&mut self, graph: &KnowledgeGraph) -> Result<()> {
        use std::fs::File;
        use std::io::BufWriter;

        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Save uncompressed for zero-copy mmap reads
        let file = File::create(&self.path)?;
        let writer = BufWriter::new(file);
        bincode::serialize_into(writer, graph)?;

        self.mmap = None; // Invalidate mmap
        self.ensure_mmap()?;
        Ok(())
    }

    fn load(&mut self) -> Result<KnowledgeGraph> {
        self.ensure_mmap()?;
        if let Some(mmap) = &self.mmap {
            let graph: KnowledgeGraph = bincode::deserialize(&mmap)?;
            Ok(graph)
        } else {
            Err(anyhow::anyhow!("No data available"))
        }
    }

    fn exists(&self) -> bool {
        self.path.exists()
    }

    fn size(&self) -> u64 {
        std::fs::metadata(&self.path).map(|m| m.len()).unwrap_or(0)
    }
}

/// Composite storage with multiple backends
pub struct CompositeStorage {
    primary: Box<dyn StorageBackend>,
    secondary: Option<Box<dyn StorageBackend>>,
}

impl CompositeStorage {
    pub fn new(primary: Box<dyn StorageBackend>) -> Self {
        Self { primary, secondary: None }
    }

    pub fn with_secondary(mut self, secondary: Box<dyn StorageBackend>) -> Self {
        self.secondary = Some(secondary);
        self
    }
}

impl StorageBackend for CompositeStorage {
    fn save(&mut self, graph: &KnowledgeGraph) -> Result<()> {
        self.primary.save(graph)?;
        if let Some(sec) = &mut self.secondary {
            sec.save(graph)?;
        }
        Ok(())
    }

    fn load(&mut self) -> Result<KnowledgeGraph> {
        // Try primary first, fallback to secondary
        match self.primary.load() {
            Ok(graph) => Ok(graph),
            Err(_) => {
                if let Some(sec) = &mut self.secondary {
                    sec.load()
                } else {
                    Err(anyhow::anyhow!("No data available"))
                }
            }
        }
    }

    fn exists(&self) -> bool {
        self.primary.exists() || self.secondary.as_ref().map(|s| s.exists()).unwrap_or(false)
    }

    fn size(&self) -> u64 {
        self.primary.size() + self.secondary.as_ref().map(|s| s.size()).unwrap_or(0)
    }
}

/// Storage manager for the knowledge graph
pub struct StorageManager {
    backend: Box<dyn StorageBackend>,
    config: StorageConfig,
}

impl StorageManager {
    pub fn new(config: StorageConfig) -> Self {
        let backend: Box<dyn StorageBackend> = if config.use_mmap {
            Box::new(MmapStorage::new(config.clone()))
        } else {
            Box::new(BinaryStorage::new(config.clone()))
        };

        Self { backend, config }
    }

    pub fn with_composite(config: StorageConfig) -> Self {
        let primary = Box::new(MmapStorage::new(config.clone()));
        let secondary = Box::new(BinaryStorage::new(config.clone()));
        let composite = CompositeStorage::new(primary).with_secondary(secondary);
        Self { backend: Box::new(composite), config }
    }

    pub fn save(&mut self, graph: &KnowledgeGraph) -> Result<()> {
        self.backend.save(graph)
    }

    pub fn load(&mut self) -> Result<KnowledgeGraph> {
        self.backend.load()
    }

    pub fn exists(&self) -> bool {
        self.backend.exists()
    }

    pub fn size(&self) -> u64 {
        self.backend.size()
    }
}

/// Incremental storage for partial updates
pub struct IncrementalStorage {
    base_path: PathBuf,
    delta_path: PathBuf,
    base_version: u64,
}

impl IncrementalStorage {
    pub fn new(base_path: PathBuf) -> Self {
        let delta_path = base_path.with_extension("delta.bin");
        Self {
            base_path,
            delta_path,
            base_version: 0,
        }
    }

    pub fn write_delta(&mut self, changes: &StorageDelta) -> Result<()> {
        use std::fs::File;
        use std::io::BufWriter;

        let file = File::create(&self.delta_path)?;
        let writer = BufWriter::new(file);
        bincode::serialize_into(writer, changes)?;
        self.base_version += 1;
        Ok(())
    }

    pub fn apply_delta(&mut self, base: &mut KnowledgeGraph) -> Result<()> {
        use std::fs::File;
        use std::io::BufReader;

        if !self.delta_path.exists() {
            return Ok(());
        }

        let file = File::open(&self.delta_path)?;
        let reader = BufReader::new(file);
        let delta: StorageDelta = bincode::deserialize_from(reader)?;

        for entity in delta.added_entities {
            base.add_entity(entity);
        }

        for entity_id in delta.removed_entities {
            base.entities.shift_remove(&entity_id);
        }

        for relation in delta.added_relations {
            base.add_relation(relation);
        }

        for (from, to, kind) in delta.removed_relations {
            base.relations.retain(|r| !(r.from == from && r.to == to && r.kind == kind));
        }

        base.build_indexes();
        Ok(())
    }
}

/// Delta for incremental updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageDelta {
    pub added_entities: Vec<Entity>,
    pub removed_entities: Vec<EntityId>,
    pub added_relations: Vec<Relation>,
    pub removed_relations: Vec<(EntityId, EntityId, RelationKind)>,
}

impl StorageDelta {
    pub fn new() -> Self {
        Self {
            added_entities: Vec::new(),
            removed_entities: Vec::new(),
            added_relations: Vec::new(),
            removed_relations: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.added_entities.push(entity);
    }

    pub fn remove_entity(&mut self, id: EntityId) {
        self.removed_entities.push(id);
    }

    pub fn add_relation(&mut self, relation: Relation) {
        self.added_relations.push(relation);
    }

    pub fn remove_relation(&mut self, from: EntityId, to: EntityId, kind: RelationKind) {
        self.removed_relations.push((from, to, kind));
    }

    pub fn is_empty(&self) -> bool {
        self.added_entities.is_empty()
            && self.removed_entities.is_empty()
            && self.added_relations.is_empty()
            && self.removed_relations.is_empty()
    }
}