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
    /// Path to the project root
    root_path: Option<PathBuf>,
}

impl ProjectAnalyzer {
    pub fn new(config: ParserConfig) -> Result<Self> {
        let parser = crate::Parser::new(config)?;
        Ok(Self {
            parser,
            entity_counter: 1,
            file_id_map: HashMap::new(),
            root_path: None,
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

        self.root_path = Some(root.to_path_buf());

        Ok(graph)
    }

    /// Update the knowledge graph incrementally based on file changes
    ///
    /// This method is more efficient than a full rebuild when only a few files
    /// have changed. It re-parses only the changed files and updates the graph
    /// accordingly.
    pub fn update_incremental(
        &mut self,
        graph: &mut KnowledgeGraph,
        changed_files: &[PathBuf],
        root: &Path,
    ) -> Result<IncrementalUpdateResult> {
        let mut result = IncrementalUpdateResult::default();

        for file_path in changed_files {
            // Get the relative path from root
            let relative_path = if let Ok(rel) = file_path.strip_prefix(root) {
                rel.to_path_buf()
            } else {
                file_path.clone()
            };

            // Check if this is an added or modified file
            let is_new = !self.file_id_map.contains_key(&relative_path.to_string_lossy().to_string());

            // Parse the file
            match self.parser.parse_file(file_path) {
                Ok(parse_result) => {
                    if is_new {
                        // New file - add all entities
                        self.add_file_to_graph(graph, &parse_result, &relative_path.to_string_lossy(), root)?;
                        result.files_added += 1;
                    } else {
                        // Modified file - update entities
                        self.update_file_in_graph(graph, &parse_result, &relative_path.to_string_lossy(), root)?;
                        result.files_modified += 1;
                    }
                }
                Err(e) => {
                    result.errors.push(format!("Failed to parse {}: {}", file_path.display(), e));
                }
            }
        }

        // Rebuild indexes
        graph.build_indexes();

        // Update project metadata
        graph.project.file_count = graph.files.len() as u32;
        graph.project.entity_count = graph.entities.len() as u32;
        graph.project.relation_count = graph.relations.len() as u32;

        Ok(result)
    }

    /// Add a new file and its entities to the graph
    fn add_file_to_graph(
        &mut self,
        graph: &mut KnowledgeGraph,
        parse_result: &ParseResult,
        relative_path: &str,
        _root: &Path,
    ) -> Result<()> {
        let file_id = EntityId::new();
        self.file_id_map.insert(relative_path.to_string(), parse_result.content_hash);

        let file_entity = File {
            id: file_id,
            path: relative_path.to_string(),
            language: parse_result.language,
            size: 0,
            content_hash: parse_result.content_hash,
            entities: Vec::new(),
        };

        // Add entities
        for extracted in &parse_result.entities {
            let entity_id = EntityId::from_str(&extracted.qualified_name);

            let entity = Entity {
                id: entity_id,
                kind: extracted.kind,
                name: extracted.name.clone(),
                qualified_name: extracted.qualified_name.clone(),
                file_id,
                range: extracted.range,
                language: parse_result.language,
                confidence: extracted.confidence,
                signature: extracted.signature.clone(),
                documentation: extracted.documentation.clone(),
                children: Vec::new(),
                relations: Vec::new(),
            };

            graph.add_entity(entity);
        }

        // Add relations
        for relation in &parse_result.relations {
            let from_id = EntityId::from_str(&relation.from_qualified);
            let to_id = EntityId::from_str(&relation.to_qualified);

            let rel = Relation {
                from: from_id,
                to: to_id,
                kind: relation.kind,
                confidence: relation.confidence,
                provenance: Provenance::Discovered,
            };

            graph.add_relation(rel);
        }

        graph.add_file(file_entity);

        Ok(())
    }

    /// Update an existing file's entities in the graph
    fn update_file_in_graph(
        &mut self,
        graph: &mut KnowledgeGraph,
        parse_result: &ParseResult,
        relative_path: &str,
        _root: &Path,
    ) -> Result<()> {
        // Find the existing file entity
        let existing_file_id = graph.files.values()
            .find(|f| f.path == relative_path)
            .map(|f| f.id);

        if let Some(file_id) = existing_file_id {
            // Remove old entities from this file
            let old_entity_ids: Vec<EntityId> = graph.entities.values()
                .filter(|e| e.file_id == file_id)
                .map(|e| e.id)
                .collect();

            for entity_id in &old_entity_ids {
                graph.entities.shift_remove(entity_id);
            }

            // Remove old relations from/to entities in this file
            graph.relations.retain(|r| {
                !old_entity_ids.contains(&r.from) && !old_entity_ids.contains(&r.to)
            });

            // Update file hash
            self.file_id_map.insert(relative_path.to_string(), parse_result.content_hash);

            if let Some(file) = graph.files.get_mut(&file_id) {
                file.content_hash = parse_result.content_hash;
            }

            // Add new entities
            for extracted in &parse_result.entities {
                let entity_id = EntityId::from_str(&extracted.qualified_name);

                let entity = Entity {
                    id: entity_id,
                    kind: extracted.kind,
                    name: extracted.name.clone(),
                    qualified_name: extracted.qualified_name.clone(),
                    file_id,
                    range: extracted.range,
                    language: parse_result.language,
                    confidence: extracted.confidence,
                    signature: extracted.signature.clone(),
                    documentation: extracted.documentation.clone(),
                    children: Vec::new(),
                    relations: Vec::new(),
                };

                graph.add_entity(entity);
            }

            // Add new relations
            for relation in &parse_result.relations {
                let from_id = EntityId::from_str(&relation.from_qualified);
                let to_id = EntityId::from_str(&relation.to_qualified);

                let rel = Relation {
                    from: from_id,
                    to: to_id,
                    kind: relation.kind,
                    confidence: relation.confidence,
                    provenance: Provenance::Discovered,
                };

                graph.add_relation(rel);
            }
        } else {
            // File not found, treat as new
            self.add_file_to_graph(graph, parse_result, relative_path, _root)?;
        }

        Ok(())
    }
}

/// Result of an incremental update
#[derive(Debug, Default)]
pub struct IncrementalUpdateResult {
    pub files_added: usize,
    pub files_modified: usize,
    pub files_removed: usize,
    pub errors: Vec<String>,
}

impl IncrementalUpdateResult {
    pub fn has_changes(&self) -> bool {
        self.files_added > 0 || self.files_modified > 0 || self.files_removed > 0
    }

    pub fn summary(&self) -> String {
        format!(
            "Added: {}, Modified: {}, Removed: {}, Errors: {}",
            self.files_added, self.files_modified, self.files_removed, self.errors.len()
        )
    }
}