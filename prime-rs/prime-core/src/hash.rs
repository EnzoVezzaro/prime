//! Hashing utilities for entity IDs and content

use blake3;
use xxhash_rust::xxh3::xxh3_64;
use serde::{Deserialize, Serialize};
use std::fmt;
use hex;

/// A 64-bit entity identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct EntityId(pub u64);

impl EntityId {
    /// Generate a new entity ID from a string (for stable IDs)
    pub fn from_str(s: &str) -> Self {
        EntityId(xxh3_64(s.as_bytes()))
    }

    /// Generate a new entity ID from bytes
    pub fn from_bytes(bytes: &[u8]) -> Self {
        EntityId(xxh3_64(bytes))
    }

    /// Create a new random entity ID
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        EntityId(COUNTER.fetch_add(1, Ordering::Relaxed))
    }

    /// Generate from namespace + name (for stable cross-run IDs)
    pub fn from_parts(namespace: &str, name: &str) -> Self {
        EntityId::from_str(&format!("{}::{}", namespace, name))
    }
}

impl Default for EntityId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:016x}", self.0)
    }
}

impl From<u64> for EntityId {
    fn from(v: u64) -> Self {
        EntityId(v)
    }
}

impl From<EntityId> for u64 {
    fn from(id: EntityId) -> Self {
        id.0
    }
}

/// Content hash using BLAKE3 (256-bit)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContentHash(pub [u8; 32]);

impl ContentHash {
    pub fn new() -> Self {
        ContentHash([0; 32])
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let hash = blake3::hash(bytes);
        ContentHash(*hash.as_bytes())
    }

    pub fn from_str(s: &str) -> Self {
        Self::from_bytes(s.as_bytes())
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn as_hex(&self) -> String {
        hex::encode(self.0)
    }

    pub fn short(&self) -> String {
        hex::encode(&self.0[..8])
    }
}

impl Default for ContentHash {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ContentHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_hex())
    }
}

impl From<[u8; 32]> for ContentHash {
    fn from(v: [u8; 32]) -> Self {
        ContentHash(v)
    }
}

impl From<ContentHash> for [u8; 32] {
    fn from(h: ContentHash) -> Self {
        h.0
    }
}

/// Hash chain for incremental invalidation (Merkle tree)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashChain {
    pub root: ContentHash,
    pub leaves: Vec<ContentHash>,
}

impl HashChain {
    pub fn new(leaves: Vec<ContentHash>) -> Self {
        let root = Self::compute_root(&leaves);
        Self { root, leaves }
    }

    fn compute_root(leaves: &[ContentHash]) -> ContentHash {
        if leaves.is_empty() {
            return ContentHash::new();
        }
        if leaves.len() == 1 {
            return leaves[0];
        }

        let mut current: Vec<ContentHash> = leaves.to_vec();
        while current.len() > 1 {
            let mut next = Vec::with_capacity((current.len() + 1) / 2);
            for chunk in current.chunks(2) {
                let combined = if chunk.len() == 2 {
                    let mut data = Vec::with_capacity(64);
                    data.extend_from_slice(&chunk[0].0);
                    data.extend_from_slice(&chunk[1].0);
                    ContentHash::from_bytes(&data)
                } else {
                    chunk[0]
                };
                next.push(combined);
            }
            current = next;
        }
        current[0]
    }

    /// Check if this chain matches another (for change detection)
    pub fn matches(&self, other: &HashChain) -> bool {
        self.root == other.root
    }
}

/// Incremental hash tracker for files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileHashTracker {
    pub file_hashes: std::collections::HashMap<String, ContentHash>,
    pub chain: HashChain,
}

impl FileHashTracker {
    pub fn new() -> Self {
        Self {
            file_hashes: std::collections::HashMap::new(),
            chain: HashChain::new(Vec::new()),
        }
    }

    pub fn update(&mut self, path: &str, content: &[u8]) -> bool {
        let hash = ContentHash::from_bytes(content);
        let changed = self.file_hashes.get(path) != Some(&hash);
        self.file_hashes.insert(path.to_string(), hash);
        self.rebuild_chain();
        changed
    }

    pub fn remove(&mut self, path: &str) -> bool {
        let removed = self.file_hashes.remove(path).is_some();
        if removed {
            self.rebuild_chain();
        }
        removed
    }

    fn rebuild_chain(&mut self) {
        let mut leaves: Vec<ContentHash> = self.file_hashes.values().cloned().collect();
        leaves.sort_by_key(|h| h.0); // Deterministic ordering
        self.chain = HashChain::new(leaves);
    }

    pub fn root_hash(&self) -> ContentHash {
        self.chain.root
    }

    pub fn has_changed(&self, other: &Self) -> bool {
        !self.chain.matches(&other.chain)
    }
}