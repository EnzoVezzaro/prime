//! High-level analysis orchestration

use prime_core::{Language, KnowledgeGraph, Project, Entity, Relation, File, Module, ContentHash, Range, Position, SymbolKind, RelationKind, Confidence, Provenance, EntityId};
use crate::{Parser, ParserConfig, ParseResult, ProjectParseResult};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use anyhow::Result;

/// High-level project analyzer
pub struct ProjectAnalyzer {
    parser: crate::Parser,
    entity_counter: u64,
    file_id_map: HashMap<String, ContentHash>,
}

impl ProjectAnalyzer {
    pub fn new(config: ParserConfig) -> Result<Self> {
        let parser = crate::Parser::new(config)?;
        Ok(Self {
            parser,
            entity_counter: 1,
            file_id_map: HashMap::new(),
        })
    }

    /// Analyze a project and build the knowledge graph
    pub fn analyze(&mut self, root: &Path) -> Result<KnowledgeGraph> {
        let parse_result = self.parser.parse_project(root)?;

        let mut graph = KnowledgeGraph::new(Project {
            name: root.file_name().unwrap_or_default().to_string_lossy().to_string(),
            root_path: root.display().to_string(),
            version: "0.1.0".to_string(),
            languages: Vec::new(),
            file_count: 0,
            entity_count: 0,
            relation_count: 0,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            content_hash: ContentHash::new(),
        });

        let mut languages = std::collections::HashSet::new();
        let mut entity_id = EntityId::new();

        for file_result in &parse_result.files {
            if file_result.language != prime_core::Language::Unknown {
                languages.insert(file_result.language);
            }

            // Create file entity
            let file_id = entity_id;
            entity_id = EntityId(entity_id.0 + 1);
            self.file_id_map.insert(file_result.file_path.clone(), file_result.content_hash);

            let file_entity = File {
                id: file_id,
                path: file_result.file_path.clone(),
                language: file_result.language,
                size: 0,
                content_hash: file_result.content_hash,
                entities: Vec::new(),
            };

            // Convert extracted entities
            let mut file_entity_ids = Vec::new();
            for extracted in &file_result.entities {
                let entity_id = EntityId::from_str(&extracted.qualified_name);

                let entity = Entity {
                    id: entity_id,
                    kind: extracted.kind,
                    name: extracted.name.clone(),
                    qualified_name: extracted.qualified_name.clone(),
                    file_id,
                    range: extracted.range,
                    language: file_result.language,
                    confidence: extracted.confidence,
                    signature: extracted.signature.clone(),
                    documentation: extracted.documentation.clone(),
                    children: Vec::new(),
                    relations: Vec::new(),
                };

                file_entity_ids.push(entity_id);
                graph.add_entity(entity);
            }

            graph.add_file(file_entity);
        }

        // Add relations
        for file_result in &parse_result.files {
            for relation in &file_result.relations {
                let from_id = EntityId::from_str(&relation.from_qualified);
                let to_id = EntityId::from_str(&relation.to_qualified);

                let relation = Relation {
                    from: from_id,
                    to: to_id,
                    kind: relation.kind,
                    confidence: relation.confidence,
                    provenance: prime_core::Provenance::Discovered,
                };

                graph.add_relation(relation);
            }
        }

        // Build indexes
        graph.build_indexes();

        // Update project metadata
        graph.project.languages = languages.into_iter().collect();
        graph.project.file_count = parse_result.files.len() as u32;
        graph.project.entity_count = graph.entities.len() as u32;
        graph.project.relation_count = graph.relations.len() as u32;

        Ok(graph)
    }
}