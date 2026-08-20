//! Serialization for the knowledge graph

use prime_core::{KnowledgeGraph, Entity, Relation, File, Module, Project, EntityId, ContentHash};
use flatbuffers::{FlatBufferBuilder, WIPOffset as FbWIPOffset};
use anyhow::{Result, Context};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::io::Write;

// FlatBuffers schema would be defined in a separate .fbs file
// For now, we use bincode for simplicity with optional compression

/// Serialize knowledge graph to bytes
pub fn serialize_graph(graph: &KnowledgeGraph, compress: bool) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    if compress {
        let mut encoder = zstd::Encoder::new(&mut buf, 3)?;
        encoder.include_checksum(true)?;
        bincode::serialize_into(&mut encoder, graph)?;
        encoder.finish().context("Failed to finish zstd encoding")?;
    } else {
        bincode::serialize_into(&mut buf, graph)?;
    }
    Ok(buf)
}

/// Deserialize knowledge graph from bytes
pub fn deserialize_graph(data: &[u8], compressed: bool) -> Result<KnowledgeGraph> {
    if compressed {
        let mut decoder = zstd::Decoder::new(data)?;
        let graph = bincode::deserialize_from(&mut decoder)?;
        Ok(graph)
    } else {
        let graph = bincode::deserialize(data)?;
        Ok(graph)
    }
}

/// FlatBuffers serialization (for zero-copy access)
pub mod fb {
    use super::*;
    use flatbuffers::{FlatBufferBuilder, WIPOffset};

    // This would use a generated FlatBuffers schema
    // For now, we provide a placeholder structure

    pub struct GraphSerializer<'a> {
        builder: FlatBufferBuilder<'a>,
    }

    impl<'a> GraphSerializer<'a> {
        pub fn new() -> Self {
            Self {
                builder: FlatBufferBuilder::new(),
            }
        }

        pub fn serialize(&mut self, graph: &KnowledgeGraph) -> Vec<u8> {
            // Serialize project
            let project_offset = self.serialize_project(&graph.project);

            // Serialize entities
            let entity_offsets: Vec<_> = graph.entities.values()
                .map(|e| self.serialize_entity(e))
                .collect();

            // Serialize relations
            let relation_offsets: Vec<_> = graph.relations.iter()
                .map(|r| self.serialize_relation(r))
                .collect();

            // Build vectors
            let entities_vec = self.builder.create_vector(&entity_offsets);
            let relations_vec = self.builder.create_vector(&relation_offsets);

            // Create root table
            // This would use generated FlatBuffers code
            // For now, return placeholder
            self.builder.finished_data().to_vec()
        }

        fn serialize_project(&mut self, project: &Project) -> flatbuffers::WIPOffset<u32> {
            // Placeholder
            flatbuffers::WIPOffset::new(0)
        }

        fn serialize_entity(&mut self, entity: &Entity) -> flatbuffers::WIPOffset<u32> {
            // Placeholder
            flatbuffers::WIPOffset::new(0)
        }

        fn serialize_relation(&mut self, relation: &Relation) -> flatbuffers::WIPOffset<u32> {
            // Placeholder
            flatbuffers::WIPOffset::new(0)
        }
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