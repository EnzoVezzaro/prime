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
            eprintln!("  [extract] definitions ERROR: {}", e);
            errors.push(crate::ParseError {
                message: format!("Failed to extract definitions: {}", e),
                range: None,
            });
        } else {
            eprintln!("  [extract] definitions: {} entities found", entities.len());
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

        // Extract overrides (methods that override parent methods)
        if let Err(e) = self.extract_overrides(tree.root_node(), &mut relations) {
            errors.push(crate::ParseError {
                message: format!("Failed to extract overrides: {}", e),
                range: None,
            });
        }

        // Extract exports (public symbols)
        if let Err(e) = self.extract_exports(tree.root_node(), &mut relations) {
            errors.push(crate::ParseError {
                message: format!("Failed to extract exports: {}", e),
                range: None,
            });
        }

        // Extract data flows (return value usage)
        if let Err(e) = self.extract_dataflows(tree.root_node(), &mut relations) {
            errors.push(crate::ParseError {
                message: format!("Failed to extract dataflows: {}", e),
                range: None,
            });
        }

        // Extract instantiations (new/construct calls)
        if let Err(e) = self.extract_instantiations(tree.root_node(), &mut relations) {
            errors.push(crate::ParseError {
                message: format!("Failed to extract instantiations: {}", e),
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

    fn extract_overrides(&mut self, node: Node, relations: &mut Vec<crate::ExtractedRelation>) -> anyhow::Result<()> {
        // Look for method definitions that have a parent class with inheritance
        // This is a heuristic: if we see a method in a class that extends another class,
        // and the parent class likely has the same method, we mark it as an override
        
        let mut class_stack: Vec<(String, Vec<String>)> = Vec::new(); // (class_name, methods)
        
        self.extract_overrides_recursive(node, &mut class_stack, relations);
        Ok(())
    }

    fn extract_overrides_recursive(&mut self, node: Node, class_stack: &mut Vec<(String, Vec<String>)>, relations: &mut Vec<crate::ExtractedRelation>) {
        match node.kind() {
            "class_declaration" | "class_definition" | "class_item" | "class_specifier" => {
                if let Some(class_name) = self.extract_name_from_node(node) {
                    let qualified = self.qualify_name(&class_name);
                    let mut methods = Vec::new();
                    
                    // Extract methods in this class
                    let mut cursor = node.walk();
                    for child in node.children(&mut cursor) {
                        if matches!(child.kind(), "method_definition" | "method_declaration" | "function_item") {
                            if let Some(method_name) = self.extract_name_from_node(child) {
                                methods.push(method_name);
                            }
                        }
                    }
                    
                    class_stack.push((qualified, methods));
                }
            }
            "method_definition" | "method_declaration" | "function_item" => {
                if let Some(method_name) = self.extract_name_from_node(node) {
                    // Check if parent class has inheritance
                    if let Some(parent_class) = class_stack.last() {
                        // Heuristic: if method exists in parent, it's likely an override
                        // In a full implementation, we'd resolve the parent class
                        let from_qualified = self.current_function_qualified(node);
                        if !from_qualified.is_empty() {
                            relations.push(crate::ExtractedRelation {
                                from_name: method_name.clone(),
                                from_qualified,
                                to_name: method_name,
                                to_qualified: String::new(), // Would need parent class resolution
                                kind: prime_core::RelationKind::Overrides,
                                confidence: Confidence::Low, // Heuristic, not certain
                            });
                        }
                    }
                }
            }
            _ => {}
        }
        
        // Recurse into children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_overrides_recursive(child, class_stack, relations);
        }
        
        // Pop class from stack when leaving
        if matches!(node.kind(), "class_declaration" | "class_definition" | "class_item" | "class_specifier") {
            class_stack.pop();
        }
    }

    fn extract_dataflows(&mut self, node: Node, relations: &mut Vec<crate::ExtractedRelation>) -> anyhow::Result<()> {
        // Detect data flow patterns by scanning function bodies:
        // 1. bar(foo()) -> foo flows to bar (nested calls)
        // 2. return foo() -> foo flows to return
        // Walk the tree looking for call_expression nodes that contain other call_expression nodes
        self.extract_dataflows_recursive(node, relations);
        Ok(())
    }

    fn extract_dataflows_recursive(&mut self, node: Node, relations: &mut Vec<crate::ExtractedRelation>) {
        match node.kind() {
            // Pattern: bar(foo()) or bar(x, foo()) - nested call as argument
            "call_expression" => {
                self.detect_nested_calls(node, relations);
            }
            "return_statement" => {
                // return foo() -> foo flows to caller's return
                self.detect_return_flow(node, relations);
            }
            _ => {}
        }
        
        // Recurse into children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_dataflows_recursive(child, relations);
        }
    }

    /// Detect nested call patterns: bar(foo()) -> foo FlowsTo bar
    fn detect_nested_calls(&self, outer_call: Node, relations: &mut Vec<crate::ExtractedRelation>) {
        // The outer call has: function expression + arguments
        // We look for call_expression nodes inside the arguments
        let mut cursor = outer_call.walk();
        for child in outer_call.children(&mut cursor) {
            // Arguments node contains the actual arguments
            if child.kind() == "arguments" || child.kind() == "argument_list" {
                let mut arg_cursor = child.walk();
                for arg in child.children(&mut arg_cursor) {
                    if arg.kind() == "call_expression" {
                        // Found inner call inside outer call's arguments
                        if let Some(inner_name) = self.extract_call_name(arg) {
                            if let Some(outer_name) = self.extract_call_name(outer_call) {
                                let from_qualified = self.current_function_qualified(arg);
                                let to_qualified = self.current_function_qualified(outer_call);
                                relations.push(crate::ExtractedRelation {
                                    from_name: inner_name,
                                    from_qualified,
                                    to_name: outer_name,
                                    to_qualified,
                                    kind: prime_core::RelationKind::FlowsTo,
                                    confidence: Confidence::Medium,
                                });
                            }
                        }
                    }
                }
            }
            // Also check for macro invocations with nested calls
            if child.kind() == "macro_invocation" {
                let mut macro_cursor = child.walk();
                for macro_child in child.children(&mut macro_cursor) {
                    if macro_child.kind() == "token_tree" {
                        let mut token_cursor = macro_child.walk();
                        for token in macro_child.children(&mut token_cursor) {
                            if token.kind() == "call_expression" {
                                if let Some(inner_name) = self.extract_call_name(token) {
                                    if let Some(outer_name) = self.extract_macro_name(child) {
                                        let from_qualified = self.current_function_qualified(token);
                                        let to_qualified = self.current_function_qualified(outer_call);
                                        relations.push(crate::ExtractedRelation {
                                            from_name: inner_name,
                                            from_qualified,
                                            to_name: outer_name,
                                            to_qualified,
                                            kind: prime_core::RelationKind::FlowsTo,
                                            confidence: Confidence::Medium,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Detect return flow: return foo() -> foo FlowsTo caller
    fn detect_return_flow(&self, return_node: Node, relations: &mut Vec<crate::ExtractedRelation>) {
        // Walk the return statement looking for call expressions
        let mut cursor = return_node.walk();
        for child in return_node.children(&mut cursor) {
            if child.kind() == "call_expression" {
                if let Some(callee_name) = self.extract_call_name(child) {
                    let from_qualified = self.current_function_qualified(child);
                    let to_qualified = self.current_function_qualified(return_node);
                    relations.push(crate::ExtractedRelation {
                        from_name: callee_name,
                        from_qualified,
                        to_name: "return".to_string(),
                        to_qualified,
                        kind: prime_core::RelationKind::FlowsTo,
                        confidence: Confidence::High,
                    });
                }
            }
        }
    }

    /// Extract the name of a function being called in a call_expression
    fn extract_call_name(&self, call_node: Node) -> Option<String> {
        let mut cursor = call_node.walk();
        for child in call_node.children(&mut cursor) {
            match child.kind() {
                "identifier" | "field_expression" | "scoped_identifier" | "type_identifier" => {
                    if let Some(name) = child.utf8_text(self.source).ok() {
                        return Some(name.to_string());
                    }
                }
                // For field expressions like obj.method(), return the method name
                _ => {}
            }
        }
        // Fallback: try extract_name_from_node
        self.extract_name_from_node(call_node)
    }

    /// Extract name from a macro invocation
    fn extract_macro_name(&self, macro_node: Node) -> Option<String> {
        let mut cursor = macro_node.walk();
        for child in macro_node.children(&mut cursor) {
            if child.kind() == "identifier" || child.kind() == "scoped_identifier" {
                return child.utf8_text(self.source).ok().map(|s| s.to_string());
            }
        }
        None
    }

    /// Extract instantiation patterns: new Foo(), Foo::new(), Foo::default(), etc.
    fn extract_instantiations(&mut self, node: Node, relations: &mut Vec<crate::ExtractedRelation>) -> anyhow::Result<()> {
        self.extract_instantiations_recursive(node, relations);
        Ok(())
    }

    fn extract_instantiations_recursive(&mut self, node: Node, relations: &mut Vec<crate::ExtractedRelation>) {
        match node.kind() {
            // Pattern: new ClassName() in JS/TS/Java/PHP
            "new_expression" => {
                if let Some(class_name) = self.extract_new_class_name(node) {
                    let from_qualified = self.current_function_qualified(node);
                    relations.push(crate::ExtractedRelation {
                        from_name: self.extract_simple_name(&from_qualified),
                        from_qualified,
                        to_name: class_name.clone(),
                        to_qualified: class_name,
                        kind: prime_core::RelationKind::Instantiates,
                        confidence: Confidence::High,
                    });
                }
            }
            // Pattern: ClassName::new() or ClassName::default() in Rust
            "call_expression" => {
                if let Some(type_name) = self.detect_static_constructor(node) {
                    let from_qualified = self.current_function_qualified(node);
                    relations.push(crate::ExtractedRelation {
                        from_name: self.extract_simple_name(&from_qualified),
                        from_qualified,
                        to_name: type_name.clone(),
                        to_qualified: type_name,
                        kind: prime_core::RelationKind::Instantiates,
                        confidence: Confidence::Medium,
                    });
                }
            }
            // Pattern: Python ClassName() 
            "identifier" => {
                // Python: ClassName() looks like a call with identifier callee
                // This is already handled by call_expression above
            }
            _ => {}
        }

        // Recurse into children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_instantiations_recursive(child, relations);
        }
    }

    /// Extract class name from new_expression (JS/TS/Java)
    fn extract_new_class_name(&self, new_node: Node) -> Option<String> {
        let mut cursor = new_node.walk();
        for child in new_node.children(&mut cursor) {
            match child.kind() {
                "identifier" | "type_identifier" | "scoped_identifier" => {
                    return child.utf8_text(self.source).ok().map(|s| s.to_string());
                }
                _ => {}
            }
        }
        None
    }

    /// Detect static constructor patterns like ClassName::new() or ClassName::create()
    fn detect_static_constructor(&self, call_node: Node) -> Option<String> {
        // Check if this is a qualified call like Type::method()
        let mut cursor = call_node.walk();
        for child in call_node.children(&mut cursor) {
            if child.kind() == "scoped_identifier" || child.kind() == "field_expression" {
                if let Some(qualified_name) = child.utf8_text(self.source).ok() {
                    // Check if it ends with known constructor patterns
                    let name_lower = qualified_name.to_lowercase();
                    if name_lower.ends_with("::new") || 
                       name_lower.ends_with("::create") ||
                       name_lower.ends_with("::default") ||
                       name_lower.ends_with("::with") ||
                       name_lower.ends_with("::from") ||
                       name_lower.ends_with(".new") ||
                       name_lower.ends_with(".create") {
                        // Extract the type name (before ::new, etc.)
                        let type_name = if let Some(pos) = qualified_name.rfind("::") {
                            &qualified_name[..pos]
                        } else if let Some(pos) = qualified_name.rfind(".") {
                            &qualified_name[..pos]
                        } else {
                            qualified_name
                        };
                        return Some(type_name.to_string());
                    }
                }
            }
        }
        None
    }

    fn extract_exports(&mut self, node: Node, relations: &mut Vec<crate::ExtractedRelation>) -> anyhow::Result<()> {
        // Look for export statements and public visibility
        let query_str = match self.config.language {
            prime_core::Language::Rust => {
                // Rust: pub items are exported
                "(visibility_modifier) @visibility"
            }
            prime_core::Language::TypeScript | prime_core::Language::JavaScript => {
                // TS/JS: export keyword
                "(export_statement) @export"
            }
            prime_core::Language::Python => {
                // Python: __all__ or non-underscore names
                "(identifier) @name"
            }
            _ => {
                // Default: look for 'public' keyword
                "(visibility_modifier) @visibility"
            }
        };
        
        // For now, use a simple heuristic: look for 'pub' or 'export' keywords
        let source_str = std::str::from_utf8(self.source).unwrap_or("");
        
        // Check for Rust pub items
        if source_str.contains("pub fn ") || source_str.contains("pub struct ") || 
           source_str.contains("pub enum ") || source_str.contains("pub trait ") {
            // Find the current module
            let module_qualified = self.current_module_qualified(node);
            
            // Simple extraction: find all pub items
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if let Some(name) = self.extract_name_from_node(child) {
                    let child_text = child.utf8_text(self.source).unwrap_or("");
                    if child_text.starts_with("pub ") || child_text.contains("\npub ") {
                        let from_qualified = self.qualify_name(&name);
                        relations.push(crate::ExtractedRelation {
                            from_name: self.extract_simple_name(&module_qualified),
                            from_qualified: module_qualified.clone(),
                            to_name: name,
                            to_qualified: from_qualified,
                            kind: prime_core::RelationKind::Exports,
                            confidence: Confidence::High,
                        });
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