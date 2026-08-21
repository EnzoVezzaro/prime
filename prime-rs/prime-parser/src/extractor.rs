//! Entity and relation extraction from Tree-sitter parse trees

use tree_sitter::{Node, Query, QueryCursor, Tree, Language as TSLanguage};
use tree_sitter::{StreamingIterator, StreamingIteratorMut};
use prime_core::{SymbolKind, RelationKind, Confidence, Language as PrimeLanguage, Range, Position, HashEntityId};
use crate::{ExtractedEntity, ExtractedRelation, ParseError, LanguageConfig};
use std::collections::HashMap;

/// Entity extractor using Tree-sitter queries
pub struct Extractor<'a> {
    config: &'a LanguageConfig,
    source: &'a [u8],
    file_path: &'a std::path::Path,
    entity_counter: u64,
    entity_map: HashMap<String, u64>,  // qualified_name -> EntityId (u64)
}

impl<'a> Extractor<'a> {
    pub fn new(config: &'a LanguageConfig, source: &'a [u8], file_path: &'a std::path::Path) -> Self {
        Self {
            config,
            source,
            file_path,
            entity_counter: 0,
            entity_map: HashMap::new(),
        }
    }

    pub fn extract(&mut self, tree: &Tree) -> (Vec<crate::ExtractedEntity>, Vec<crate::ExtractedRelation>, Vec<crate::ParseError>) {
        let mut entities = Vec::new();
        let mut relations = Vec::new();
        let mut errors = Vec::new();

        // Extract definitions
        if let Err(e) = self.extract_definitions(tree.root_node(), &mut entities) {
            errors.push(crate::ParseError {
                message: format!("Failed to extract definitions: {}", e),
                range: None,
            });
        }

        // Extract calls
        if let Err(e) = self.extract_calls(tree.root_node(), &mut relations) {
            errors.push(crate::ParseError {
                message: format!("Failed to extract calls: {}", e),
                range: None,
            });
        }

        // Extract references
        if let Err(e) = self.extract_references(tree.root_node(), &mut relations) {
            errors.push(crate::ParseError {
                message: format!("Failed to extract references: {}", e),
                range: None,
            });
        }

        // Extract imports
        if let Err(e) = self.extract_imports(tree.root_node(), &mut relations) {
            errors.push(crate::ParseError {
                message: format!("Failed to extract imports: {}", e),
                range: None,
            });
        }

        // Extract types
        if let Err(e) = self.extract_types(tree.root_node(), &mut relations) {
            errors.push(crate::ParseError {
                message: format!("Failed to extract types: {}", e),
                range: None,
            });
        }

        (entities, relations, errors)
    }

    fn extract_definitions(&mut self, node: Node, entities: &mut Vec<crate::ExtractedEntity>) -> anyhow::Result<()> {
        let query = Query::new(&self.config.ts_language, &self.config.queries.definitions)?;
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&query, node, self.source);

        while let Some(match_) = matches.next() {
            let mut name = None;
            let mut def_node = None;
            let mut kind = SymbolKind::Unknown;

            for capture in match_.captures {
                let capture_name = query.capture_names()[capture.index as usize];
                let text = capture.node.utf8_text(self.source).unwrap_or("");

                match capture_name {
                    "name" => name = Some(text.to_string()),
                    "kind" => kind = self.parse_symbol_kind(text),
                    "definition" => def_node = Some(capture.node),
                    _ => {}
                }
            }

            // If no explicit kind was captured, infer from the definition node's type
            if kind == SymbolKind::Unknown {
                if let Some(dn) = def_node {
                    kind = self.parse_symbol_kind(dn.kind());
                }
            }

            if let Some(name) = name {
                let qualified = self.qualify_name(&name);
                let id = HashEntityId::from_str(&qualified).0;

                let range = self.node_to_range(def_node.unwrap_or(node));

                let entity = crate::ExtractedEntity {
                    kind,
                    name,
                    qualified_name: qualified.clone(),
                    range,
                    signature: self.extract_signature(def_node.unwrap_or(node)),
                    documentation: self.extract_documentation(def_node.unwrap_or(node)),
                    confidence: Confidence::High,
                };

                self.entity_map.insert(qualified, id);
                entities.push(entity);
            }
        }
        Ok(())
    }

    fn extract_calls(&mut self, node: Node, relations: &mut Vec<crate::ExtractedRelation>) -> anyhow::Result<()> {
        let query = Query::new(&self.config.ts_language, &self.config.queries.calls)?;
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&query, node, self.source);

        while let Some(match_) = matches.next() {
            let mut callee = None;
            let mut call_node = None;

            for capture in match_.captures {
                let capture_name = query.capture_names()[capture.index as usize];
                let text = capture.node.utf8_text(self.source).unwrap_or("");

                match capture_name {
                    "callee" => callee = Some(text.to_string()),
                    "call" => call_node = Some(capture.node),
                    _ => {}
                }
            }

            if let Some(callee) = callee {
                let caller_qualified = self.current_function_qualified(call_node.unwrap_or(node));
                if !caller_qualified.is_empty() {
                    relations.push(crate::ExtractedRelation {
                        from_name: self.extract_simple_name(&caller_qualified),
                        from_qualified: caller_qualified,
                        to_name: callee.clone(),
                        to_qualified: self.qualify_callee(&callee),
                        kind: prime_core::RelationKind::Calls,
                        confidence: Confidence::High,
                    });
                }
            }
        }
        Ok(())
    }

    fn extract_references(&mut self, node: Node, relations: &mut Vec<crate::ExtractedRelation>) -> anyhow::Result<()> {
        let query = Query::new(&self.config.ts_language, &self.config.queries.references)?;
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&query, node, self.source);

        while let Some(match_) = matches.next() {
            for capture in match_.captures {
                let capture_name = query.capture_names()[capture.index as usize];
                if capture_name == "reference" {
                    let text = capture.node.utf8_text(self.source).unwrap_or("");
                    if !text.is_empty() && text.len() > 1 {
                        let from_qualified = self.current_function_qualified(capture.node);
                        if !from_qualified.is_empty() {
                            relations.push(crate::ExtractedRelation {
                                from_name: self.extract_simple_name(&from_qualified),
                                from_qualified,
                                to_name: text.to_string(),
                                to_qualified: text.to_string(),
                                kind: prime_core::RelationKind::References,
                                confidence: Confidence::Medium,
                            });
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn extract_imports(&mut self, node: Node, relations: &mut Vec<crate::ExtractedRelation>) -> anyhow::Result<()> {
        let query = Query::new(&self.config.ts_language, &self.config.queries.imports)?;
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&query, node, self.source);

        while let Some(match_) = matches.next() {
            let mut module = None;
            let mut item = None;

            for capture in match_.captures {
                let capture_name = query.capture_names()[capture.index as usize];
                let text = capture.node.utf8_text(self.source).unwrap_or("");

                match capture_name {
                    "module" => module = Some(text.to_string()),
                    "item" => item = Some(text.to_string()),
                    "import" => {} // the import node itself
                    _ => {}
                }
            }

            if let Some(module) = module {
                let from_qualified = self.current_module_qualified(node);
                relations.push(crate::ExtractedRelation {
                    from_name: self.extract_simple_name(&from_qualified),
                    from_qualified,
                    to_name: item.unwrap_or(module.clone()),
                    to_qualified: module,
                    kind: prime_core::RelationKind::Imports,
                    confidence: Confidence::High,
                });
            }
        }
        Ok(())
    }

    fn extract_types(&mut self, node: Node, relations: &mut Vec<crate::ExtractedRelation>) -> anyhow::Result<()> {
        let query = Query::new(&self.config.ts_language, &self.config.queries.types)?;
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&query, node, self.source);

        while let Some(match_) = matches.next() {
            for capture in match_.captures {
                let capture_name = query.capture_names()[capture.index as usize];
                if capture_name == "type" {
                    let text = capture.node.utf8_text(self.source).unwrap_or("");
                    if !text.is_empty() {
                        let from_qualified = self.current_function_qualified(capture.node);
                        if !from_qualified.is_empty() {
                            relations.push(crate::ExtractedRelation {
                                from_name: self.extract_simple_name(&from_qualified),
                                from_qualified,
                                to_name: text.to_string(),
                                to_qualified: text.to_string(),
                                kind: prime_core::RelationKind::TypeOf,
                                confidence: Confidence::Medium,
                            });
                        }
                    }
                }
            }
        }
        Ok(())
    }

    // Helper methods
    fn node_to_range(&self, node: Node) -> Range {
        Range {
            start: Position::new(node.start_position().row as u32 + 1, node.start_position().column as u32),
            end: Position::new(node.end_position().row as u32 + 1, node.end_position().column as u32),
        }
    }

    fn extract_signature(&self, node: Node) -> Option<String> {
        // Extract function signature
        node.utf8_text(self.source).ok().map(|s| s.to_string())
    }

    fn extract_documentation(&self, node: Node) -> Option<String> {
        // Look for preceding comments
        let mut cursor = node.walk();
        if cursor.goto_previous_sibling() {
            if cursor.node().kind() == "comment" || cursor.node().kind() == "line_comment" {
                return cursor.node().utf8_text(self.source).ok().map(|s| s.to_string());
            }
        }
        None
    }

    fn current_function_qualified(&self, node: Node) -> String {
        // Walk up to find containing function/class
        let mut current = Some(node);
        while let Some(n) = current {
            if matches!(n.kind(), "function_definition" | "function_declaration" | "method_definition" | "method_declaration" | "function_item" | "class_declaration" | "class_specifier") {
                if let Some(name) = self.extract_name_from_node(n) {
                    return self.qualify_name(&name);
                }
            }
            current = n.parent();
        }
        // Fallback to file-level
        self.qualify_name("file")
    }

    fn current_module_qualified(&self, node: Node) -> String {
        self.file_path.to_string_lossy().to_string()
    }

    fn qualify_name(&self, name: &str) -> String {
        let file_qualified = self.file_path.to_string_lossy().replace('/', "::");
        format!("{}::{}", file_qualified, name)
    }

    fn qualify_callee(&self, callee: &str) -> String {
        // Try to resolve to qualified name
        if callee.contains("::") || callee.contains('.') {
            callee.to_string()
        } else {
            let file_qualified = self.file_path.to_string_lossy().replace('/', "::");
            format!("{}::{}", file_qualified, callee)
        }
    }

    fn extract_simple_name(&self, qualified: &str) -> String {
        qualified.split("::").last().unwrap_or(qualified).to_string()
    }

    fn extract_name_from_node(&self, node: Node) -> Option<String> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if matches!(child.kind(), "identifier" | "type_identifier" | "property_identifier" | "field_identifier") {
                return child.utf8_text(self.source).ok().map(|s| s.to_string());
            }
        }
        None
    }

    fn parse_symbol_kind(&self, kind_str: &str) -> SymbolKind {
        match kind_str {
            // Functions
            "function" | "function_item" | "function_declaration" | "function_definition" | "async_function_definition" => SymbolKind::Function,
            // Methods
            "method" | "method_definition" | "method_declaration" | "method_declaration" => SymbolKind::Method,
            // Classes/Structs/Enums
            "class" | "class_declaration" | "class_item" | "class_specifier" | "class_definition" => SymbolKind::Class,
            "struct" | "struct_item" | "struct_specifier" | "struct_declaration" => SymbolKind::Struct,
            "enum" | "enum_item" | "enum_declaration" | "enum_specifier" => SymbolKind::Enum,
            // Traits/Interfaces
            "trait" | "trait_item" | "trait_declaration" | "interface_declaration" => SymbolKind::Trait,
            "interface" => SymbolKind::Interface,
            "impl" | "impl_item" => SymbolKind::Trait,
            // Types
            "type_alias" | "type_item" | "type_declaration" => SymbolKind::TypeAlias,
            // Macros
            "macro" | "macro_definition" | "macro_rules" => SymbolKind::Macro,
            // Modules
            "module" | "mod_item" | "namespace_definition" => SymbolKind::Module,
            // Constants/Variables
            "const" | "const_item" | "const_declaration" => SymbolKind::Constant,
            "static" | "static_item" => SymbolKind::StaticVariable,
            "field" | "field_declaration" => SymbolKind::Field,
            "parameter" => SymbolKind::Parameter,
            "variable" | "var_declaration" => SymbolKind::Variable,
            // Python-specific
            "function_definition" => SymbolKind::Function,
            "async_function_definition" => SymbolKind::Function,
            "class_definition" => SymbolKind::Class,
            "async_function_definition" => SymbolKind::Function,
            // JavaScript/TypeScript
            "function_declaration" => SymbolKind::Function,
            "function_expression" => SymbolKind::Function,
            "arrow_function" => SymbolKind::Function,
            "method_definition" => SymbolKind::Method,
            "class_declaration" => SymbolKind::Class,
            "interface_declaration" => SymbolKind::Interface,
            "type_alias_declaration" => SymbolKind::TypeAlias,
            "enum_declaration" => SymbolKind::Enum,
            // Go
            "function_declaration" => SymbolKind::Function,
            "method_declaration" => SymbolKind::Method,
            "struct_declaration" => SymbolKind::Struct,
            "interface_declaration" => SymbolKind::Interface,
            "type_declaration" => SymbolKind::TypeAlias,
            // Python-specific
            "async_function_definition" => SymbolKind::Function,
            "class_definition" => SymbolKind::Class,
            "type_alias_declaration" => SymbolKind::TypeAlias,
            // Variables
            "variable_declaration" => SymbolKind::Variable,
            _ => SymbolKind::Unknown,
        }
    }
}