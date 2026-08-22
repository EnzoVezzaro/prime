//! Compact binary serialization for the knowledge graph
//!
//! This module provides a space-efficient binary format for storing the knowledge graph.
//! It uses variable-length encoding for integers, dictionary encoding for strings,
//! and compact relation encoding to minimize storage size.

use prime_core::{
    Entity, EntityId,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use indexmap::IndexMap;
use byteorder::{WriteBytesExt, ReadBytesExt};
use std::io::{Write, Read};

/// Compact representation of the knowledge graph for efficient storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactKnowledgeGraph {
    pub project: CompactProject,
    pub entities: Vec<CompactEntity>,
    pub relations: Vec<CompactRelation>,
    pub files: Vec<CompactFile>,
    pub modules: Vec<CompactModule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactProject {
    pub name: String,
    pub root_path: String,
    pub version: String,
    pub languages: Vec<String>,
    pub file_count: u32,
    pub entity_count: u32,
    pub relation_count: u32,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactEntity {
    pub id: EntityId,
    pub kind: u8,           // SymbolKind as u8
    pub name: String,
    pub qualified_name: String,
    pub file_id: EntityId,
    pub range_start: u32,
    pub range_end: u32,
    pub language: u8,       // Language as u8
    pub confidence: u8,     // Confidence as u8
    pub signature: Option<String>,
    pub documentation: Option<String>,
    // Relations stored as indices into the relations array
    pub relation_indices: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactRelation {
    pub from: u32,      // Index into entities array
    pub to: u32,        // Index into entities array
    pub kind: u8,       // RelationKind as u8
    pub confidence: u8, // Confidence as u8
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactFile {
    pub id: EntityId,
    pub path: String,
    pub language: u8,
    pub size: u32,
    pub content_hash: [u8; 32],
    pub entity_indices: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactModule {
    pub id: EntityId,
    pub name: String,
    pub path: String,
    pub language: u8,
    pub file_indices: Vec<u32>,
    pub parent: Option<EntityId>,
    pub children: Vec<EntityId>,
    pub exports: Vec<EntityId>,
}

/// Writer for compact binary format
pub struct CompactWriter<W: Write> {
    writer: W,
    string_table: HashMap<String, u32>,
    next_string_id: u32,
}

impl<W: Write> CompactWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            string_table: HashMap::new(),
            next_string_id: 0,
        }
    }

    /// Write a variable-length encoded unsigned integer (LEB128)
    fn write_varint(&mut self, mut value: u64) -> std::io::Result<()> {
        while value >= 0x80 {
            self.writer.write_u8((value & 0x7F) as u8 | 0x80)?;
            value >>= 7;
        }
        self.writer.write_u8(value as u8)?;
        Ok(())
    }

    /// Write a string, using string interning for deduplication
    fn write_string(&mut self, s: &str) -> std::io::Result<u32> {
        if let Some(&id) = self.string_table.get(s) {
            return Ok(id);
        }
        let id = self.next_string_id;
        self.next_string_id += 1;
        self.string_table.insert(s.to_string(), id);
        self.write_varint(id as u64)?;
        self.write_varint(s.len() as u64)?;
        self.writer.write_all(s.as_bytes())?;
        Ok(id)
    }

    /// Write a CompactKnowledgeGraph
    pub fn write_graph(&mut self, graph: &CompactKnowledgeGraph) -> std::io::Result<()> {
        // Magic number and version
        self.writer.write_all(b"PRME")?; // Prime magic
        self.write_varint(1)?; // Version 1

        // Write project
        self.write_string(&graph.project.name)?;
        self.write_string(&graph.project.root_path)?;
        self.write_string(&graph.project.version)?;
        self.write_varint(graph.project.languages.len() as u64)?;
        for lang in &graph.project.languages {
            self.write_string(lang)?;
        }
        self.write_varint(graph.project.file_count as u64)?;
        self.write_varint(graph.project.entity_count as u64)?;
        self.write_varint(graph.project.relation_count as u64)?;
        self.write_varint(graph.project.created_at as u64)?;

        // Write entities
        self.write_varint(graph.entities.len() as u64)?;
        for entity in &graph.entities {
            self.write_varint(entity.id.0)?;
            self.writer.write_u8(entity.kind)?;
            self.write_string(&entity.name)?;
            self.write_string(&entity.qualified_name)?;
            self.write_varint(entity.file_id.0)?;
            self.write_varint(entity.range_start as u64)?;
            self.write_varint(entity.range_end as u64)?;
            self.writer.write_u8(entity.language)?;
            self.writer.write_u8(entity.confidence)?;
            self.write_string(entity.signature.as_deref().unwrap_or(""))?;
            self.write_string(entity.documentation.as_deref().unwrap_or(""))?;
            self.write_varint(entity.relation_indices.len() as u64)?;
            for &idx in &entity.relation_indices {
                self.write_varint(idx as u64)?;
            }
        }

        // Write relations
        self.write_varint(graph.relations.len() as u64)?;
        for rel in &graph.relations {
            self.write_varint(rel.from as u64)?;
            self.write_varint(rel.to as u64)?;
            self.writer.write_u8(rel.kind)?;
            self.writer.write_u8(rel.confidence)?;
        }

        // Write files
        self.write_varint(graph.files.len() as u64)?;
        for file in &graph.files {
            self.write_varint(file.id.0)?;
            self.write_string(&file.path)?;
            self.writer.write_u8(file.language)?;
            self.write_varint(file.size as u64)?;
            self.writer.write_all(&file.content_hash)?;
            self.write_varint(file.entity_indices.len() as u64)?;
            for &idx in &file.entity_indices {
                self.write_varint(idx as u64)?;
            }
        }

        // Write modules
        self.write_varint(graph.modules.len() as u64)?;
        for module in &graph.modules {
            self.write_varint(module.id.0)?;
            self.write_string(&module.name)?;
            self.write_string(&module.path)?;
            self.writer.write_u8(module.language)?;
            self.write_varint(module.file_indices.len() as u64)?;
            for &idx in &module.file_indices {
                self.write_varint(idx as u64)?;
            }
            self.write_varint(module.parent.map(|p| p.0).unwrap_or(0))?;
            self.write_varint(module.children.len() as u64)?;
            for &child in &module.children {
                self.write_varint(child.0)?;
            }
            self.write_varint(module.exports.len() as u64)?;
            for &exp in &module.exports {
                self.write_varint(exp.0)?;
            }
        }

        Ok(())
    }

    pub fn finish(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

/// Reader for compact binary format
pub struct CompactReader<R: Read> {
    reader: R,
    string_table: Vec<String>,
}

impl<R: Read> CompactReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            string_table: Vec::new(),
        }
    }

    fn read_varint(&mut self) -> std::io::Result<u64> {
        let mut value = 0u64;
        let mut shift = 0;
        loop {
            let mut byte = [0u8; 1];
            self.reader.read_exact(&mut byte)?;
            let byte = byte[0];
            value |= ((byte & 0x7F) as u64) << shift;
            if byte & 0x80 == 0 {
                break;
            }
            shift += 7;
        }
        Ok(value)
    }

    fn read_string(&mut self) -> std::io::Result<String> {
        let id = self.read_varint()? as usize;
        if id < self.string_table.len() {
            return Ok(self.string_table[id].clone());
        }
        let len = self.read_varint()? as usize;
        let mut buf = vec![0u8; len];
        self.reader.read_exact(&mut buf)?;
        let s = String::from_utf8(buf).map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8"))?;
        if id == self.string_table.len() {
            self.string_table.push(s.clone());
        }
        Ok(s)
    }

    pub fn read_graph(&mut self) -> std::io::Result<CompactKnowledgeGraph> {
        // Read magic number
        let mut magic = [0u8; 4];
        self.reader.read_exact(&mut magic)?;
        if &magic != b"PRME" {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid magic number"));
        }

        let version = self.read_varint()?;
        if version != 1 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Unsupported version: {}", version)));
        }

        // Read project
        let name = self.read_string()?;
        let root_path = self.read_string()?;
        let version = self.read_string()?;
        let lang_count = self.read_varint()? as usize;
        let mut languages = Vec::with_capacity(lang_count);
        for _ in 0..lang_count {
            languages.push(self.read_string()?);
        }
        let file_count = self.read_varint()? as u32;
        let entity_count = self.read_varint()? as u32;
        let relation_count = self.read_varint()? as u32;
        let created_at = self.read_varint()?;

        let project = CompactProject {
            name,
            root_path,
            version,
            languages,
            file_count,
            entity_count,
            relation_count,
            created_at,
        };

        // Read entities
        let entity_count = self.read_varint()? as usize;
        let mut entities = Vec::with_capacity(entity_count);
        for _ in 0..entity_count {
            let id = EntityId(self.read_varint()?);
            let kind = self.reader.read_u8()?;
            let name = self.read_string()?;
            let qualified_name = self.read_string()?;
            let file_id = EntityId(self.read_varint()?);
            let range_start = self.read_varint()? as u32;
            let range_end = self.read_varint()? as u32;
            let language = self.reader.read_u8()?;
            let confidence = self.reader.read_u8()?;
            let signature = self.read_string()?;
            let documentation = self.read_string()?;
            let rel_count = self.read_varint()? as usize;
            let mut relation_indices = Vec::with_capacity(rel_count);
            for _ in 0..rel_count {
                relation_indices.push(self.read_varint()? as u32);
            }

            entities.push(CompactEntity {
                id,
                kind,
                name,
                qualified_name,
                file_id,
                range_start,
                range_end,
                language,
                confidence,
                signature: if signature.is_empty() { None } else { Some(signature) },
                documentation: if documentation.is_empty() { None } else { Some(documentation) },
                relation_indices,
            });
        }

        // Read relations
        let relation_count = self.read_varint()? as usize;
        let mut relations = Vec::with_capacity(relation_count);
        for _ in 0..relation_count {
            let from = self.read_varint()? as u32;
            let to = self.read_varint()? as u32;
            let kind = self.reader.read_u8()?;
            let confidence = self.reader.read_u8()?;
            relations.push(CompactRelation { from, to, kind, confidence });
        }

        // Read files
        let file_count = self.read_varint()? as usize;
        let mut files = Vec::with_capacity(file_count);
        for _ in 0..file_count {
            let id = self.read_varint()?;
            let path = self.read_string()?;
            let language = self.reader.read_u8()?;
            let size = self.read_varint()? as u32;
            let mut content_hash = [0u8; 32];
            self.reader.read_exact(&mut content_hash)?;
            let entity_count = self.read_varint()? as usize;
            let mut entity_indices = Vec::with_capacity(entity_count);
            for _ in 0..entity_count {
                entity_indices.push(self.read_varint()? as u32);
            }
            files.push(CompactFile { id: EntityId(id), path, language, size, content_hash, entity_indices });
        }

        // Read modules
        let module_count = self.read_varint()? as usize;
        let mut modules = Vec::with_capacity(module_count);
        for _ in 0..module_count {
            let id = self.read_varint()?;
            let name = self.read_string()?;
            let path = self.read_string()?;
            let language = self.reader.read_u8()?;
            let file_count = self.read_varint()? as usize;
            let mut file_indices = Vec::with_capacity(file_count);
            for _ in 0..file_count {
                file_indices.push(self.read_varint()? as u32);
            }
            let parent = if self.read_varint()? == 0 { None } else { Some(EntityId(self.read_varint()?)) };
            let child_count = self.read_varint()? as usize;
            let mut children = Vec::with_capacity(child_count);
            for _ in 0..child_count {
                children.push(EntityId(self.read_varint()?));
            }
            let export_count = self.read_varint()? as usize;
            let mut exports = Vec::with_capacity(export_count);
            for _ in 0..export_count {
                exports.push(EntityId(self.read_varint()?));
            }

            modules.push(CompactModule {
                id: EntityId(id),
                name,
                path,
                language,
                file_indices,
                parent,
                children,
                exports,
            });
        }

        Ok(CompactKnowledgeGraph {
            project,
            entities,
            relations,
            files,
            modules,
        })
    }
}

/// Convert from KnowledgeGraph to CompactKnowledgeGraph
impl From<&prime_core::KnowledgeGraph> for CompactKnowledgeGraph {
    fn from(graph: &prime_core::KnowledgeGraph) -> Self {
        // Build entity ID to index mapping
        let mut entity_id_to_index = HashMap::new();
        for (idx, (id, _)) in graph.entities.iter().enumerate() {
            entity_id_to_index.insert(id.clone(), idx as u32);
        }

        // Convert entities
        let entities: Vec<CompactEntity> = graph.entities.values().enumerate().map(|(_idx, entity)| {
            let relation_indices: Vec<u32> = graph.relations.iter()
                .enumerate()
                .filter(|(_, rel)| rel.from == entity.id)
                .map(|(idx, _)| idx as u32)
                .collect();

            CompactEntity {
                id: entity.id,
                kind: entity.kind as u8,
                name: entity.name.clone(),
                qualified_name: entity.qualified_name.clone(),
                file_id: entity.file_id,
                range_start: entity.range.start.line as u32,
                range_end: entity.range.end.line as u32,
                language: entity.language as u8,
                confidence: entity.confidence as u8,
                signature: entity.signature.clone(),
                documentation: entity.documentation.clone(),
                relation_indices,
            }
        }).collect();

        // Relations with indices instead of EntityIds
        let relations: Vec<CompactRelation> = graph.relations.iter().map(|rel| {
            CompactRelation {
                from: *entity_id_to_index.get(&rel.from).unwrap(),
                to: *entity_id_to_index.get(&rel.to).unwrap(),
                kind: rel.kind as u8,
                confidence: rel.confidence as u8,
            }
        }).collect();

let files: Vec<CompactFile> = graph.files.iter().map(|(id, file)| {
                CompactFile {
                    id: *id,
                path: file.path.clone(),
                language: file.language as u8,
                size: file.size,
                content_hash: file.content_hash.0,
                entity_indices: file.entities.iter().map(|e| *entity_id_to_index.get(&e).unwrap()).collect(),
            }
        }).collect();

let modules: Vec<CompactModule> = graph.modules.iter().map(|(id, module)| {
                CompactModule {
                    id: *id,
                name: module.name.clone(),
                path: module.path.clone(),
                language: module.language as u8,
                file_indices: module.files.iter().map(|f| *entity_id_to_index.get(f).unwrap() as u32).collect(),
                parent: module.parent,
                children: module.children.iter().map(|c| *c).collect(),
                exports: module.exports.iter().copied().collect(),
            }
        }).collect();

        Self {
            project: CompactProject {
                name: graph.project.name.clone(),
                root_path: graph.project.root_path.clone(),
                version: graph.project.version.clone(),
                languages: graph.project.languages.iter().map(|l| format!("{:?}", l)).collect(),
                file_count: graph.project.file_count,
                entity_count: graph.project.entity_count,
                relation_count: graph.project.relation_count,
                created_at: graph.project.created_at,
            },
            entities,
            relations,
            files,
            modules,
        }
    }
}

/// Serialize a KnowledgeGraph to compact binary format with zstd compression
pub fn serialize_compact(graph: &prime_core::KnowledgeGraph, _compression_level: i32) -> anyhow::Result<Vec<u8>> {
    let _compact = CompactKnowledgeGraph::from(graph);
    let mut buf = Vec::new();
    let mut encoder = zstd::Encoder::new(&mut buf, 9)?; // Higher compression level
    encoder.include_checksum(true)?;
    {
        let mut writer = CompactWriter::new(&mut encoder);
        writer.write_graph(&CompactKnowledgeGraph::from(graph))?;
        writer.finish()?;
    }
    encoder.finish()?;
    Ok(buf)
}

/// Deserialize a KnowledgeGraph from compact binary format
pub fn deserialize_compact(data: &[u8]) -> anyhow::Result<prime_core::KnowledgeGraph> {
    let mut decoder = zstd::Decoder::new(data)?;
    let mut reader = CompactReader::new(&mut decoder);
    let compact = reader.read_graph()?;
    Ok(compact.into())
}

impl From<CompactKnowledgeGraph> for prime_core::KnowledgeGraph {
    fn from(compact: CompactKnowledgeGraph) -> Self {
        // Reverse conversion - rebuild KnowledgeGraph from compact form
        let project = prime_core::Project {
            name: compact.project.name,
            root_path: compact.project.root_path,
            version: compact.project.version,
            languages: compact.project.languages.iter().map(|l| l.parse().unwrap_or(prime_core::Language::Unknown)).collect(),
            file_count: compact.project.file_count,
            entity_count: compact.project.entity_count,
            relation_count: compact.project.relation_count,
            created_at: compact.project.created_at,
            content_hash: prime_core::ContentHash::new(),
        };

let mut entities: IndexMap<EntityId, Entity> = IndexMap::new();
for entity in &compact.entities {
            let kind = match entity.kind {
                1 => prime_core::SymbolKind::Module,
                2 => prime_core::SymbolKind::Namespace,
                3 => prime_core::SymbolKind::Package,
                10 => prime_core::SymbolKind::Class,
                11 => prime_core::SymbolKind::Struct,
                12 => prime_core::SymbolKind::Enum,
                13 => prime_core::SymbolKind::Trait,
                14 => prime_core::SymbolKind::Interface,
                15 => prime_core::SymbolKind::TypeAlias,
                16 => prime_core::SymbolKind::Protocol,
                20 => prime_core::SymbolKind::Function,
                21 => prime_core::SymbolKind::Method,
                22 => prime_core::SymbolKind::Constructor,
                23 => prime_core::SymbolKind::Destructor,
                24 => prime_core::SymbolKind::AsyncFunction,
                25 => prime_core::SymbolKind::AsyncMethod,
                30 => prime_core::SymbolKind::Variable,
                31 => prime_core::SymbolKind::Field,
                32 => prime_core::SymbolKind::Constant,
                33 => prime_core::SymbolKind::StaticVariable,
                34 => prime_core::SymbolKind::Parameter,
                40 => prime_core::SymbolKind::Macro,
                41 => prime_core::SymbolKind::Delegate,
                42 => prime_core::SymbolKind::Event,
                43 => prime_core::SymbolKind::Property,
                44 => prime_core::SymbolKind::GenericParameter,
                45 => prime_core::SymbolKind::ModuleImport,
                46 => prime_core::SymbolKind::ModuleExport,
                _ => prime_core::SymbolKind::Unknown,
            };
            let language = match entity.language {
                1 => prime_core::Language::Rust,
                2 => prime_core::Language::TypeScript,
                3 => prime_core::Language::JavaScript,
                4 => prime_core::Language::Python,
                5 => prime_core::Language::Go,
                6 => prime_core::Language::Java,
                7 => prime_core::Language::C,
                8 => prime_core::Language::Cpp,
                _ => prime_core::Language::Unknown,
            };
            let confidence = match entity.confidence {
                1 => prime_core::Confidence::Exact,
                2 => prime_core::Confidence::High,
                3 => prime_core::Confidence::Medium,
                4 => prime_core::Confidence::Low,
                _ => prime_core::Confidence::Unknown,
            };
            let e = prime_core::Entity {
                id: entity.id,
                kind,
                name: entity.name.clone(),
                qualified_name: entity.qualified_name.clone(),
                file_id: entity.file_id,
                range: prime_core::Range {
                    start: prime_core::Position { line: entity.range_start, column: 0 },
                    end: prime_core::Position { line: entity.range_end, column: 0 },
                },
                language,
                confidence,
                signature: entity.signature.clone().filter(|s| !s.is_empty()),
                documentation: entity.documentation.clone().filter(|s| !s.is_empty()),
                parent_id: None,
                disambiguator: None,
                stability: prime_core::Stability::Unknown,
                aliases: Vec::new(),
                children: Vec::new(),
                relations: Vec::new(),
                semantic: None,
            };
            entities.insert(e.id, e);
        }

        let mut relations = Vec::new();
        for rel in compact.relations {
            let kind = match rel.kind {
                1 => prime_core::RelationKind::Contains,
                2 => prime_core::RelationKind::PartOf,
                10 => prime_core::RelationKind::Extends,
                11 => prime_core::RelationKind::Implements,
                12 => prime_core::RelationKind::Inherits,
                20 => prime_core::RelationKind::DependsOn,
                21 => prime_core::RelationKind::Imports,
                22 => prime_core::RelationKind::Requires,
                30 => prime_core::RelationKind::Calls,
                31 => prime_core::RelationKind::References,
                32 => prime_core::RelationKind::Reads,
                33 => prime_core::RelationKind::Writes,
                40 => prime_core::RelationKind::Returns,
                41 => prime_core::RelationKind::ParameterOf,
                42 => prime_core::RelationKind::TypeOf,
                43 => prime_core::RelationKind::GenericArgOf,
                50 => prime_core::RelationKind::Overrides,
                51 => prime_core::RelationKind::Overloads,
                52 => prime_core::RelationKind::Instantiates,
                53 => prime_core::RelationKind::Factory,
                55 => prime_core::RelationKind::FlowsTo,
                56 => prime_core::RelationKind::Yields,
                60 => prime_core::RelationKind::Exports,
                61 => prime_core::RelationKind::ReExports,
                _ => prime_core::RelationKind::Unknown,
            };
            let confidence = match rel.confidence {
                1 => prime_core::Confidence::Exact,
                2 => prime_core::Confidence::High,
                3 => prime_core::Confidence::Medium,
                4 => prime_core::Confidence::Low,
                _ => prime_core::Confidence::Unknown,
            };
            relations.push(prime_core::Relation {
                from: compact.entities[rel.from as usize].id,
                to: compact.entities[rel.to as usize].id,
                kind,
                confidence,
                provenance: prime_core::Provenance::Stored,
            });
        }

        let mut files = indexmap::IndexMap::new();
for file in &compact.files {
            let lang = match file.language {
                1 => prime_core::Language::Rust,
                2 => prime_core::Language::TypeScript,
                3 => prime_core::Language::JavaScript,
                4 => prime_core::Language::Python,
                5 => prime_core::Language::Go,
                6 => prime_core::Language::Java,
                7 => prime_core::Language::C,
                8 => prime_core::Language::Cpp,
                _ => prime_core::Language::Unknown,
            };
            let file_obj = prime_core::File {
                id: file.id,
                path: file.path.clone(),
                language: lang,
                size: file.size,
                content_hash: prime_core::ContentHash(file.content_hash),
                entities: file.entity_indices.iter().map(|&idx| {
                    compact.entities[idx as usize].id
                }).collect(),
            };
            files.insert(file_obj.id, file_obj);
        }

let mut modules = indexmap::IndexMap::new();        for module in &compact.modules {
            let lang = match module.language {
                1 => prime_core::Language::Rust,
                2 => prime_core::Language::TypeScript,
                3 => prime_core::Language::JavaScript,
                4 => prime_core::Language::Python,
                5 => prime_core::Language::Go,
                6 => prime_core::Language::Java,
                7 => prime_core::Language::C,
                8 => prime_core::Language::Cpp,
                _ => prime_core::Language::Unknown,
            };
            let module_obj = prime_core::Module {
                id: module.id,
                name: module.name.clone(),
                path: module.path.clone(),
                language: lang,
                files: module.file_indices.iter().map(|&idx| compact.files[idx as usize].id).collect(),
                parent: module.parent,
                children: module.children.iter().map(|&c| c).collect(),
                exports: module.exports.iter().copied().collect(),
            };
            modules.insert(module_obj.id, module_obj);
        }

        let mut graph = prime_core::KnowledgeGraph {
            project: project,
            entities: IndexMap::new(),
            relations: Vec::new(),
            files: IndexMap::new(),
            modules: IndexMap::new(),
            name_index: None,
            file_index: None,
            relation_index: None,
        };

        for (id, entity) in entities {
            graph.entities.insert(id, entity);
        }
        graph.relations = relations;
        for (id, file) in files {
            graph.files.insert(id, file);
        }
        for (id, module) in modules {
            graph.modules.insert(id, module);
        }
        graph.build_indexes();
        graph
    }
}