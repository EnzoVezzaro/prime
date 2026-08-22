//! Serialization for the knowledge graph

use prime_core::{KnowledgeGraph, Entity, Relation, File, Module, Project, EntityId, ContentHash, Range, Position, SymbolKind, RelationKind, Confidence, Provenance, Language};
use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};
use std::io::Write;
use indexmap::IndexMap;

// FlatBuffers schema would be defined in a separate .fbs file
// For now, we use bincode for simplicity with optional compression

/// Serialize knowledge graph to bytes with efficient compression
pub fn serialize_graph(graph: &KnowledgeGraph, compress: bool) -> Result<Vec<u8>> {
    // Use a more compact representation that only stores essential data
    let compact_graph = CompactKnowledgeGraph::from(graph);
    let mut buf = Vec::new();
    if compress {
        let mut encoder = zstd::Encoder::new(&mut buf, 9)?; // Higher compression level
        encoder.include_checksum(true)?;
        bincode::serialize_into(&mut encoder, &compact_graph)?;
        encoder.finish().context("Failed to finish zstd encoding")?;
    } else {
        bincode::serialize_into(&mut buf, &compact_graph)?;
    }
    Ok(buf)
}

/// Deserialize knowledge graph from bytes
pub fn deserialize_graph(data: &[u8], compressed: bool) -> Result<KnowledgeGraph> {
    if compressed {
        let mut decoder = zstd::Decoder::new(data)?;
        let compact: CompactKnowledgeGraph = bincode::deserialize_from(&mut decoder)?;
        Ok(compact.into())
    } else {
        let graph = bincode::deserialize(data)?;
        Ok(graph)
    }
}

/// Compact knowledge graph - stores only essential information for agent consumption
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
    pub id: u64,
    pub kind: String,
    pub name: String,
    pub qualified_name: String,
    pub file_id: u64,
    pub range_start: u32,
    pub range_end: u32,
    pub language: String,
    pub confidence: String,
    pub signature: Option<String>,
    pub documentation: Option<String>,
    // SCIP-inspired identity fields
    pub parent_id: Option<u64>,
    pub disambiguator: Option<String>,
    pub stability: String,
    pub aliases: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactRelation {
    pub from: u64,
    pub to: u64,
    pub kind: String,
    pub confidence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactFile {
    pub id: u64,
    pub path: String,
    pub language: String,
    pub size: u32,
    pub content_hash: [u8; 32],
    pub entities: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactModule {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub language: String,
    pub files: Vec<u64>,
    pub parent: Option<u64>,
    pub children: Vec<u64>,
    pub exports: Vec<u64>,
}

impl From<&KnowledgeGraph> for CompactKnowledgeGraph {
    fn from(graph: &KnowledgeGraph) -> Self {
        let project = CompactProject {
            name: graph.project.name.clone(),
            root_path: graph.project.root_path.clone(),
            version: graph.project.version.clone(),
            languages: graph.project.languages.iter().map(|l| format!("{:?}", l)).collect(),
            file_count: graph.project.file_count,
            entity_count: graph.project.entity_count,
            relation_count: graph.project.relation_count,
            created_at: graph.project.created_at,
        };

        let entities: Vec<CompactEntity> = graph.entities.iter().map(|(id, entity)| {
            CompactEntity {
                id: id.0,
                kind: format!("{:?}", entity.kind),
                name: entity.name.clone(),
                qualified_name: entity.qualified_name.clone(),
                file_id: entity.file_id.0,
                range_start: entity.range.start.line as u32,
                range_end: entity.range.end.line as u32,
                language: format!("{:?}", entity.language),
                confidence: format!("{:?}", entity.confidence),
                signature: entity.signature.clone(),
                documentation: entity.documentation.clone(),
                parent_id: entity.parent_id.map(|p| p.0),
                disambiguator: entity.disambiguator.clone(),
                stability: format!("{:?}", entity.stability),
                aliases: entity.aliases.clone(),
            }
        }).collect();

        let relations: Vec<CompactRelation> = graph.relations.iter().map(|rel| {
            CompactRelation {
                from: rel.from.0,
                to: rel.to.0,
                kind: format!("{:?}", rel.kind),
                confidence: format!("{:?}", rel.confidence),
            }
        }).collect();

        let files: Vec<CompactFile> = graph.files.iter().map(|(id, file)| {
            CompactFile {
                id: id.0,
                path: file.path.clone(),
                language: format!("{:?}", file.language),
                size: file.size,
                content_hash: file.content_hash.0,
                entities: file.entities.iter().map(|e| e.0).collect(),
            }
        }).collect();

        let modules: Vec<CompactModule> = graph.modules.iter().map(|(id, module)| {
            CompactModule {
                id: id.0,
                name: module.name.clone(),
                path: module.path.clone(),
                language: format!("{:?}", module.language),
                files: module.files.iter().map(|f| f.0).collect(),
                parent: module.parent.map(|p| p.0),
                children: module.children.iter().map(|c| c.0).collect(),
                exports: module.exports.iter().map(|e| e.0).collect(),
            }
        }).collect();

        Self {
            project,
            entities,
            relations,
            files,
            modules,
        }
    }
}

impl From<CompactKnowledgeGraph> for KnowledgeGraph {
    fn from(compact: CompactKnowledgeGraph) -> Self {
        let project = Project {
            name: compact.project.name,
            root_path: compact.project.root_path,
            version: compact.project.version,
            languages: compact.project.languages.iter().map(|l| l.parse().unwrap_or(Language::Unknown)).collect(),
            file_count: compact.project.file_count,
            entity_count: compact.project.entity_count,
            relation_count: compact.project.relation_count,
            created_at: compact.project.created_at,
            content_hash: ContentHash::new(),
        };

        let mut entities = IndexMap::new();
        for entity in compact.entities {
            let id = EntityId(entity.id);
            let e = Entity {
                id,
                kind: entity.kind.parse().unwrap_or(SymbolKind::Unknown),
                name: entity.name,
                qualified_name: entity.qualified_name,
                file_id: EntityId(entity.file_id),
                range: Range {
                    start: Position { line: entity.range_start, column: 0 },
                    end: Position { line: entity.range_end, column: 0 },
                },
                language: entity.language.parse().unwrap_or(Language::Unknown),
                confidence: entity.confidence.parse().unwrap_or(Confidence::Unknown),
                signature: entity.signature,
                documentation: entity.documentation,
                parent_id: entity.parent_id.map(|p| EntityId(p)),
                disambiguator: entity.disambiguator,
                stability: entity.stability.parse().unwrap_or(prime_core::Stability::Unknown),
                aliases: entity.aliases,
                children: Vec::new(),
                relations: Vec::new(),
                semantic: None,
            };
            entities.insert(id, e);
        }

        let mut relations = Vec::new();
        for rel in compact.relations {
            relations.push(Relation {
                from: EntityId(rel.from),
                to: EntityId(rel.to),
                kind: rel.kind.parse().unwrap_or(RelationKind::References),
                confidence: rel.confidence.parse().unwrap_or(Confidence::Unknown),
                provenance: Provenance::Stored,
            });
        }

        let mut files = IndexMap::new();
        for file in compact.files {
            let id = EntityId(file.id);
            let file_entity = File {
                id,
                path: file.path,
                language: file.language.parse().unwrap_or(Language::Unknown),
                size: file.size,
                content_hash: ContentHash(file.content_hash),
                entities: file.entities.iter().map(|e| EntityId(*e)).collect(),
            };
            files.insert(id, file_entity);
        }

        let mut modules = IndexMap::new();
        for module in compact.modules {
            let id = EntityId(module.id);
            let module_entity = Module {
                id,
                name: module.name,
                path: module.path,
                language: module.language.parse().unwrap_or(Language::Unknown),
                files: module.files.iter().map(|f| EntityId(*f)).collect(),
                parent: module.parent.map(|p| EntityId(p)),
                children: module.children.iter().map(|c| EntityId(*c)).collect(),
                exports: module.exports.iter().map(|e| EntityId(*e)).collect(),
            };
            modules.insert(id, module_entity);
        }

        let mut graph = KnowledgeGraph {
            project,
            entities,
            relations,
            files,
            modules,
            name_index: None,
            file_index: None,
            relation_index: None,
        };
        graph.build_indexes();
        graph
    }
}

/// MessagePack serialization for interoperability
pub mod msgpack {
    use super::*;
    use rmp_serde::{Deserializer, Serializer};
    use serde::{Deserialize, Serialize};

    pub fn serialize<T: Serialize>(value: &T) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        value.serialize(&mut Serializer::new(&mut buf))?;
        Ok(buf)
    }

    pub fn deserialize<'a, T: Deserialize<'a>>(data: &'a [u8]) -> Result<T> {
        let mut de = Deserializer::new(data);
        let value = Deserialize::deserialize(&mut de)?;
        Ok(value)
    }
}

/// JSON serialization for debugging and interoperability
pub mod json {
    use super::*;
    use serde_json;
    use serde::{Serialize, Deserialize};

    pub fn serialize<T: Serialize>(value: &T, pretty: bool) -> Result<String> {
        if pretty {
            Ok(serde_json::to_string_pretty(value)?)
        } else {
            Ok(serde_json::to_string(value)?)
        }
    }

    pub fn deserialize<'a, T: Deserialize<'a>>(data: &'a str) -> Result<T> {
        Ok(serde_json::from_str(data)?)
    }
}

/// Compression utilities
pub mod compression {
    use super::*;

    pub fn compress(data: &[u8], level: i32) -> Result<Vec<u8>> {
        let mut encoder = zstd::Encoder::new(Vec::new(), level)?;
        encoder.include_checksum(true)?;
        std::io::copy(&mut std::io::Cursor::new(data), &mut encoder)?;
        encoder.finish().context("Failed to finish zstd encoding")
    }

    pub fn decompress(data: &[u8]) -> Result<Vec<u8>> {
        let mut decoder = zstd::Decoder::new(data)?;
        let mut out = Vec::new();
        std::io::copy(&mut decoder, &mut out)?;
        Ok(out)
    }

    pub fn compress_lz4(data: &[u8]) -> Result<Vec<u8>> {
        let mut out = Vec::new();
        let mut encoder = lz4::EncoderBuilder::new()
            .build(&mut out)?;
        encoder.write_all(data)?;
        let (_output, result) = encoder.finish();
        result?;
        Ok(out)
    }

    pub fn decompress_lz4(data: &[u8]) -> Result<Vec<u8>> {
        let mut decoder = lz4::Decoder::new(data)?;
        let mut out = Vec::new();
        std::io::copy(&mut decoder, &mut out)?;
        Ok(out)
    }
}