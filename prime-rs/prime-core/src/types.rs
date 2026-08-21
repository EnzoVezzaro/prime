//! Core type definitions for the Prime knowledge graph

use std::collections::HashMap;
use crate::confidence::Confidence;
use crate::language::Language;
use crate::hash::{ContentHash, EntityId};
use serde::{Deserialize, Serialize};
use indexmap::IndexMap;
use roaring::RoaringBitmap;

/// A reference to an entity by ID
pub type EntityRef = EntityId;

/// File position (1-indexed, like editors)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub column: u32,
}

impl Position {
    pub fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }
}

/// A range in a source file
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

impl Range {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}

/// Symbol kind - language-agnostic classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum SymbolKind {
    // Module/Package level
    Module = 1,
    Namespace = 2,
    Package = 3,

    // Type definitions
    Class = 10,
    Struct = 11,
    Enum = 12,
    Trait = 13,
    Interface = 14,
    TypeAlias = 15,
    Protocol = 16,

    // Functions/Methods
    Function = 20,
    Method = 21,
    Constructor = 22,
    Destructor = 23,
    AsyncFunction = 24,
    AsyncMethod = 25,

    // Variables/Fields
    Variable = 30,
    Field = 31,
    Constant = 32,
    StaticVariable = 33,
    Parameter = 34,

    // Other
    Macro = 40,
    Delegate = 41,
    Event = 42,
    Property = 43,
    GenericParameter = 44,
    ModuleImport = 45,
    ModuleExport = 46,

    Unknown = 255,
}

impl SymbolKind {
    /// Check if this is a callable entity
    pub fn is_callable(&self) -> bool {
        matches!(self,
            SymbolKind::Function | SymbolKind::Method |
            SymbolKind::Constructor | SymbolKind::AsyncFunction |
            SymbolKind::AsyncMethod
        )
    }

    /// Check if this is a type definition
    pub fn is_type(&self) -> bool {
        matches!(self,
            SymbolKind::Class | SymbolKind::Struct | SymbolKind::Enum |
            SymbolKind::Trait | SymbolKind::Interface | SymbolKind::TypeAlias |
            SymbolKind::Protocol
        )
    }

    /// Check if this is a module/package
    pub fn is_module(&self) -> bool {
        matches!(self, SymbolKind::Module | SymbolKind::Namespace | SymbolKind::Package)
    }
}

impl std::fmt::Display for SymbolKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolKind::Module => write!(f, "Module"),
            SymbolKind::Namespace => write!(f, "Namespace"),
            SymbolKind::Package => write!(f, "Package"),
            SymbolKind::Class => write!(f, "Class"),
            SymbolKind::Struct => write!(f, "Struct"),
            SymbolKind::Enum => write!(f, "Enum"),
            SymbolKind::Trait => write!(f, "Trait"),
            SymbolKind::Interface => write!(f, "Interface"),
            SymbolKind::TypeAlias => write!(f, "TypeAlias"),
            SymbolKind::Protocol => write!(f, "Protocol"),
            SymbolKind::Function => write!(f, "Function"),
            SymbolKind::Method => write!(f, "Method"),
            SymbolKind::Constructor => write!(f, "Constructor"),
            SymbolKind::Destructor => write!(f, "Destructor"),
            SymbolKind::AsyncFunction => write!(f, "AsyncFunction"),
            SymbolKind::AsyncMethod => write!(f, "AsyncMethod"),
            SymbolKind::Variable => write!(f, "Variable"),
            SymbolKind::Field => write!(f, "Field"),
            SymbolKind::Constant => write!(f, "Constant"),
            SymbolKind::StaticVariable => write!(f, "StaticVariable"),
            SymbolKind::Parameter => write!(f, "Parameter"),
            SymbolKind::Macro => write!(f, "Macro"),
            SymbolKind::Delegate => write!(f, "Delegate"),
            SymbolKind::Event => write!(f, "Event"),
            SymbolKind::Property => write!(f, "Property"),
            SymbolKind::GenericParameter => write!(f, "GenericParameter"),
            SymbolKind::ModuleImport => write!(f, "ModuleImport"),
            SymbolKind::ModuleExport => write!(f, "ModuleExport"),
            SymbolKind::Unknown => write!(f, "Unknown"),
        }
    }
}

impl std::str::FromStr for SymbolKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "module" => Ok(SymbolKind::Module),
            "namespace" => Ok(SymbolKind::Namespace),
            "package" => Ok(SymbolKind::Package),
            "class" => Ok(SymbolKind::Class),
            "struct" => Ok(SymbolKind::Struct),
            "enum" => Ok(SymbolKind::Enum),
            "trait" => Ok(SymbolKind::Trait),
            "interface" => Ok(SymbolKind::Interface),
            "typealias" => Ok(SymbolKind::TypeAlias),
            "protocol" => Ok(SymbolKind::Protocol),
            "function" => Ok(SymbolKind::Function),
            "method" => Ok(SymbolKind::Method),
            "constructor" => Ok(SymbolKind::Constructor),
            "destructor" => Ok(SymbolKind::Destructor),
            "asyncfunction" => Ok(SymbolKind::AsyncFunction),
            "asyncmethod" => Ok(SymbolKind::AsyncMethod),
            "variable" => Ok(SymbolKind::Variable),
            "field" => Ok(SymbolKind::Field),
            "constant" => Ok(SymbolKind::Constant),
            "staticvariable" => Ok(SymbolKind::StaticVariable),
            "parameter" => Ok(SymbolKind::Parameter),
            "macro" => Ok(SymbolKind::Macro),
            "delegate" => Ok(SymbolKind::Delegate),
            "event" => Ok(SymbolKind::Event),
            "property" => Ok(SymbolKind::Property),
            "genericparameter" => Ok(SymbolKind::GenericParameter),
            "moduleimport" => Ok(SymbolKind::ModuleImport),
            "moduleexport" => Ok(SymbolKind::ModuleExport),
            "unknown" => Ok(SymbolKind::Unknown),
            _ => Err(format!("Unknown SymbolKind: {}", s)),
        }
    }
}

/// Relationship kinds between entities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum RelationKind {
    // Hierarchical
    Contains = 1,        // Parent contains child
    PartOf = 2,          // Child is part of parent

    // Inheritance/Implementation
    Extends = 10,        // Class extends class
    Implements = 11,     // Class implements interface/trait
    Inherits = 12,       // Generic inheritance

    // Dependencies
    DependsOn = 20,      // Depends on (imports, uses)
    Imports = 21,        // Explicit import
    Requires = 22,       // Requires for compilation

    // Calls/References
    Calls = 30,          // Function calls function
    References = 31,     // References symbol
    Reads = 32,          // Reads variable/field
    Writes = 33,         // Writes variable/field

    // Type relationships
    Returns = 40,        // Function returns type
    ParameterOf = 41,    // Parameter of function
    TypeOf = 42,         // Variable has type
    GenericArgOf = 43,   // Generic argument of

    // Overrides
    Overrides = 50,      // Method overrides parent
    Overloads = 51,      // Function overloads another

    // Instantiation
    Instantiates = 52,   // Creates instance of class/struct
    Factory = 53,        // Factory method creates instance

    // Data flow
    FlowsTo = 55,        // Data flows from producer to consumer
    Yields = 56,         // Generator yields value

    // Module boundaries
    Exports = 60,        // Module exports symbol
    ReExports = 61,      // Module re-exports

    Unknown = 255,
}

impl std::str::FromStr for RelationKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "contains" => Ok(RelationKind::Contains),
            "partof" => Ok(RelationKind::PartOf),
            "extends" => Ok(RelationKind::Extends),
            "implements" => Ok(RelationKind::Implements),
            "inherits" => Ok(RelationKind::Inherits),
            "dependson" => Ok(RelationKind::DependsOn),
            "imports" => Ok(RelationKind::Imports),
            "requires" => Ok(RelationKind::Requires),
            "calls" => Ok(RelationKind::Calls),
            "references" => Ok(RelationKind::References),
            "reads" => Ok(RelationKind::Reads),
            "writes" => Ok(RelationKind::Writes),
            "returns" => Ok(RelationKind::Returns),
            "parameterof" => Ok(RelationKind::ParameterOf),
            "typeof" => Ok(RelationKind::TypeOf),
            "genericargof" => Ok(RelationKind::GenericArgOf),
            "overrides" => Ok(RelationKind::Overrides),
            "overloads" => Ok(RelationKind::Overloads),
            "instantiates" => Ok(RelationKind::Instantiates),
            "factory" => Ok(RelationKind::Factory),
            "flowsto" => Ok(RelationKind::FlowsTo),
            "yields" => Ok(RelationKind::Yields),
            "exports" => Ok(RelationKind::Exports),
            "reexports" => Ok(RelationKind::ReExports),
            "unknown" => Ok(RelationKind::Unknown),
            _ => Err(format!("Unknown RelationKind: {}", s)),
        }
    }
}

/// An entity in the knowledge graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: EntityId,
    pub kind: SymbolKind,
    pub name: String,
    pub qualified_name: String,
    pub file_id: EntityId,
    pub range: Range,
    pub language: Language,
    pub confidence: Confidence,
    pub signature: Option<String>,  // For functions: "fn(a: Type, b: Type) -> Ret"
    pub documentation: Option<String>,
    // Computed/derived fields
    pub children: Vec<EntityId>,
    pub relations: Vec<Relation>,
}

/// A relationship between two entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub from: EntityId,
    pub to: EntityId,
    pub kind: RelationKind,
    pub confidence: Confidence,
    pub provenance: Provenance,
}

/// How this fact was derived
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Provenance {
    Declared = 1,      // Explicit in source (AGENTS.md equivalent)
    Discovered = 2,    // Found by analyzer (imports, calls)
    Inferred = 3,      // Deduced from patterns
    Memory = 4,        // From .acc-memory.md
    Stored = 5,        // From storage (loaded from disk)
}

/// A source file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub id: EntityId,
    pub path: String,           // Relative to project root
    pub language: Language,
    pub size: u32,
    pub content_hash: ContentHash,
    pub entities: Vec<EntityId>,  // Entities defined in this file
}

/// A module/package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub id: EntityId,
    pub name: String,
    pub path: String,
    pub language: Language,
    pub files: Vec<EntityId>,
    pub parent: Option<EntityId>,
    pub children: Vec<EntityId>,
    pub exports: Vec<EntityId>,  // Exported entity IDs
}

/// Project metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub root_path: String,
    pub version: String,
    pub languages: Vec<Language>,
    pub file_count: u32,
    pub entity_count: u32,
    pub relation_count: u32,
    pub created_at: u64,  // Unix timestamp
    pub content_hash: ContentHash,  // Hash of all source for change detection
}

/// The complete knowledge graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGraph {
    pub project: Project,
    pub entities: IndexMap<EntityId, Entity>,
    pub files: IndexMap<EntityId, File>,
    pub modules: IndexMap<EntityId, Module>,
    pub relations: Vec<Relation>,
    // Inverse indexes for fast queries
    #[serde(skip)]
    pub name_index: Option<NameIndex>,
    #[serde(skip)]
    pub file_index: Option<FileIndex>,
    #[serde(skip)]
    pub relation_index: Option<RelationIndex>,
}

/// Index for fast name lookups
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NameIndex {
    // Maps lowercase name to entity IDs
    pub by_name: HashMap<String, Vec<EntityId>>,
    // Maps qualified name to entity ID
    pub by_qualified: HashMap<String, EntityId>,
    // Prefix index for fuzzy search
    pub prefixes: HashMap<String, Vec<EntityId>>,
}

/// Index for file-based lookups
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileIndex {
    pub path_to_id: HashMap<String, EntityId>,
    pub id_to_path: HashMap<EntityId, String>,
}

/// Index for fast relation queries
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RelationIndex {
    // from -> [(kind, to)]
    pub outgoing: HashMap<EntityId, Vec<(RelationKind, EntityId)>>,
    // to -> [(kind, from)]
    pub incoming: HashMap<EntityId, Vec<(RelationKind, EntityId)>>,
    // For bitmap operations on large graphs
    pub dep_bitmaps: HashMap<EntityId, RoaringBitmap>,
}

impl KnowledgeGraph {
    pub fn new(project: Project) -> Self {
        Self {
            project,
            entities: IndexMap::new(),
            files: IndexMap::new(),
            modules: IndexMap::new(),
            relations: Vec::new(),
            name_index: None,
            file_index: None,
            relation_index: None,
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity.id, entity);
    }

    pub fn add_file(&mut self, file: File) {
        self.files.insert(file.id, file);
    }

    pub fn add_module(&mut self, module: Module) {
        self.modules.insert(module.id, module);
    }

    pub fn add_relation(&mut self, relation: Relation) {
        self.relations.push(relation);
    }

    /// Build all indexes after graph construction
    pub fn build_indexes(&mut self) {
        let mut name_index = NameIndex::default();
        let mut file_index = FileIndex::default();
        let mut relation_index = RelationIndex::default();

        for (id, entity) in &self.entities {
            // Name index
            let name_lower = entity.name.to_lowercase();
            name_index.by_name.entry(name_lower.clone()).or_default().push(*id);
            name_index.by_qualified.insert(entity.qualified_name.clone(), *id);

            // Prefix index (first 3 chars minimum)
            if name_lower.len() >= 3 {
                let prefix = &name_lower[..3.min(name_lower.len())];
                name_index.prefixes.entry(prefix.to_string()).or_default().push(*id);
            }
        }

        for (id, file) in &self.files {
            file_index.path_to_id.insert(file.path.clone(), *id);
            file_index.id_to_path.insert(*id, file.path.clone());
        }

        for rel in &self.relations {
            relation_index.outgoing
                .entry(rel.from)
                .or_default()
                .push((rel.kind, rel.to));
            relation_index.incoming
                .entry(rel.to)
                .or_default()
                .push((rel.kind, rel.from));
        }

        self.name_index = Some(name_index);
        self.file_index = Some(file_index);
        self.relation_index = Some(relation_index);
    }

    /// Find entity by qualified name
    pub fn find_by_qualified(&self, name: &str) -> Option<EntityId> {
        self.name_index.as_ref()?.by_qualified.get(name).copied()
    }

    /// Find entities by simple name
    pub fn find_by_name(&self, name: &str) -> Option<&Vec<EntityId>> {
        self.name_index.as_ref()?.by_name.get(&name.to_lowercase())
    }

    /// Find entities by prefix
    pub fn find_by_prefix(&self, prefix: &str) -> Option<&Vec<EntityId>> {
        self.name_index.as_ref()?.prefixes.get(&prefix.to_lowercase())
    }

    /// Get all outgoing relations of a kind
    pub fn outgoing(&self, from: EntityId, kind: RelationKind) -> Vec<EntityId> {
        self.relation_index
            .as_ref()
            .and_then(|idx| idx.outgoing.get(&from))
            .map(|vec| {
                vec.iter()
                    .filter(|(k, _)| *k == kind)
                    .map(|(_, to)| *to)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get all incoming relations of a kind
    pub fn incoming(&self, to: EntityId, kind: RelationKind) -> Vec<EntityId> {
        self.relation_index
            .as_ref()
            .and_then(|idx| idx.incoming.get(&to))
            .map(|vec| {
                vec.iter()
                    .filter(|(k, _)| *k == kind)
                    .map(|(_, from)| *from)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get dependencies of an entity
    pub fn dependencies(&self, entity: EntityId) -> Vec<EntityId> {
        self.outgoing(entity, RelationKind::DependsOn)
    }

    /// Get dependents of an entity
    pub fn dependents(&self, entity: EntityId) -> Vec<EntityId> {
        self.incoming(entity, RelationKind::DependsOn)
    }

    /// Get callers of a function
    pub fn callers(&self, entity: EntityId) -> Vec<EntityId> {
        self.incoming(entity, RelationKind::Calls)
    }

    /// Get callees of a function
    pub fn callees(&self, entity: EntityId) -> Vec<EntityId> {
        self.outgoing(entity, RelationKind::Calls)
    }
}