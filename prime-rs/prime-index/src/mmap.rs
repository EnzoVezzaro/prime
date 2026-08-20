//! Memory-mapped file access for zero-copy reads

use memmap2::{Mmap, MmapOptions, MmapMut};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::Path;
use anyhow::Result;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use libc;

// POSIX_FADV_* constants - defined here since they may not be exposed by libc
const POSIX_FADV_SEQUENTIAL: libc::c_int = 2;
const POSIX_FADV_RANDOM: libc::c_int = 1;
const POSIX_FADV_WILLNEED: libc::c_int = 3;
const POSIX_FADV_DONTNEED: libc::c_int = 4;

// Declare posix_fadvise since it may not be exposed by libc
extern "C" {
    fn posix_fadvise(fd: libc::c_int, offset: libc::off_t, len: libc::off_t, advice: libc::c_int) -> libc::c_int;
}

/// Read-only memory-mapped file
pub struct ReadOnlyMmap {
    mmap: Mmap,
    file: File,
    path: std::path::PathBuf,
}

impl ReadOnlyMmap {
    /// Open a file for memory-mapped reading
    pub fn open<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let file = File::open(&path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        Ok(Self { mmap, file, path })
    }

    /// Get a slice of the mmap
    pub fn slice(&self, offset: usize, len: usize) -> Option<&[u8]> {
        if offset + len <= self.mmap.len() {
            Some(&self.mmap[offset..offset + len])
        } else {
            None
        }
    }

    /// Get the entire mmap as bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.mmap
    }

    /// Get the length of the mmap
    pub fn len(&self) -> usize {
        self.mmap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.mmap.is_empty()
    }

    /// Advise the kernel about access pattern
    pub fn advise_sequential(&self) -> Result<()> {
        #[cfg(unix)]
        {
            use std::os::unix::io::AsRawFd;
            let fd = self.file.as_raw_fd();
            unsafe {
                posix_fadvise(fd, 0, 0, POSIX_FADV_SEQUENTIAL);
            }
        }
        #[cfg(windows)]
        {
            // Windows doesn't have posix_fadvise
        }
        Ok(())
    }

    pub fn advise_random(&self) -> Result<()> {
        #[cfg(unix)]
        {
            use std::os::unix::io::AsRawFd;
            let fd = self.file.as_raw_fd();
            unsafe {
                posix_fadvise(fd, 0, 0, POSIX_FADV_RANDOM);
            }
        }
        Ok(())
    }

    pub fn advise_willneed(&self, offset: usize, len: usize) -> Result<()> {
        #[cfg(unix)]
        {
            use std::os::unix::io::AsRawFd;
            let fd = self.file.as_raw_fd();
            unsafe {
                posix_fadvise(fd, offset as libc::off_t, len as libc::off_t, POSIX_FADV_WILLNEED);
            }
        }
        Ok(())
    }

    pub fn advise_dontneed(&self, offset: usize, len: usize) -> Result<()> {
        #[cfg(unix)]
        {
            use std::os::unix::io::AsRawFd;
            let fd = self.file.as_raw_fd();
            unsafe {
                posix_fadvise(fd, offset as libc::off_t, len as libc::off_t, POSIX_FADV_DONTNEED);
            }
        }
        Ok(())
    }
}

/// Read-write memory-mapped file
pub struct ReadWriteMmap {
    mmap: MmapMut,
    file: File,
    path: std::path::PathBuf,
}

impl ReadWriteMmap {
    /// Create or open a file for memory-mapped read/write
    pub fn create<P: AsRef<std::path::Path>>(path: P, size: usize) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)?;

        file.set_len(size as u64)?;
        let mmap = unsafe { MmapMut::map_mut(&file)? };

        Ok(Self { mmap, file, path })
    }

    /// Open existing file for read/write
    pub fn open<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)?;
        let mmap = unsafe { MmapMut::map_mut(&file)? };
        Ok(Self { mmap, file, path })
    }

    /// Get mutable slice
    pub fn slice_mut(&mut self, offset: usize, len: usize) -> Option<&mut [u8]> {
        if offset + len <= self.mmap.len() {
            Some(&mut self.mmap[offset..offset + len])
        } else {
            None
        }
    }

    /// Get immutable slice
    pub fn slice(&self, offset: usize, len: usize) -> Option<&[u8]> {
        if offset + len <= self.mmap.len() {
            Some(&self.mmap[offset..offset + len])
        } else {
            None
        }
    }

    /// Flush changes to disk
    pub fn flush(&mut self) -> Result<()> {
        self.mmap.flush()?;
        self.file.flush()?;
        Ok(())
    }

    /// Resize the mmap
    pub fn resize(&mut self, new_size: usize) -> Result<()> {
        self.file.set_len(new_size as u64)?;
        self.mmap = unsafe { MmapMut::map_mut(&self.file)? };
        Ok(())
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.mmap
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.mmap
    }

    pub fn len(&self) -> usize {
        self.mmap.len()
    }
}

/// Thread-safe shared memory-mapped file
pub struct SharedMmap {
    mmap: Arc<Mutex<Mmap>>,
    path: std::path::PathBuf,
}

impl SharedMmap {
    pub fn open<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let file = File::open(&path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        Ok(Self {
            mmap: Arc::new(Mutex::new(mmap)),
            path,
        })
    }

    pub fn read(&self, offset: usize, len: usize) -> Option<Vec<u8>> {
        let mmap = self.mmap.lock().ok()?;
        if offset + len <= mmap.len() {
            Some(mmap[offset..offset + len].to_vec())
        } else {
            None
        }
    }

    pub fn read_exact(&self, offset: usize, buf: &mut [u8]) -> Result<()> {
        let mmap = self.mmap.lock().map_err(|_| anyhow::anyhow!("Lock poisoned"))?;
        if offset + buf.len() <= mmap.len() {
            buf.copy_from_slice(&mmap[offset..offset + buf.len()]);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Read out of bounds"))
        }
    }

    pub fn len(&self) -> usize {
        self.mmap.lock().map(|m| m.len()).unwrap_or(0)
    }
}

/// Mmap-based index for fast lookups
pub struct MmapIndex<T> {
    mmap: ReadOnlyMmap,
    index_offset: usize,
    data_offset: usize,
    entry_size: usize,
    count: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> MmapIndex<T> {
    pub fn open<P: AsRef<std::path::Path>>(path: P, entry_size: usize) -> Result<Self> {
        let mmap = ReadOnlyMmap::open(path)?;
        let count = mmap.len() / entry_size;
        Ok(Self {
            mmap,
            index_offset: 0,
            data_offset: 0,
            entry_size,
            count,
            _phantom: std::marker::PhantomData,
        })
    }

    pub fn get(&self, index: usize) -> Option<&[u8]> {
        if index < self.count {
            let offset = index * self.entry_size;
            self.mmap.slice(offset, self.entry_size)
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

/// Mmap-based key-value store
pub struct MmapKV {
    mmap: ReadWriteMmap,
    index: HashMap<Vec<u8>, (usize, usize)>, // key -> (offset, len)
}

impl MmapKV {
    pub fn create<P: AsRef<std::path::Path>>(path: P, estimated_size: usize) -> Result<Self> {
        let mmap = ReadWriteMmap::create(path, estimated_size)?;
        Ok(Self {
            mmap,
            index: HashMap::new(),
        })
    }

    pub fn open<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let mmap = ReadWriteMmap::open(path)?;
        let mut kv = Self {
            mmap,
            index: HashMap::new(),
        };
        kv.rebuild_index()?;
        Ok(kv)
    }

    fn rebuild_index(&mut self) -> Result<()> {
        // Simple format: [key_len: u32][key: bytes][value_len: u32][value: bytes]...
        // For simplicity, we'll skip full implementation here
        Ok(())
    }

    pub fn put(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        let key_len = (key.len() as u32).to_le_bytes();
        let value_len = (value.len() as u32).to_le_bytes();

        let current_len = self.mmap.len();
        let needed = 4 + key.len() + 4 + value.len();

        if current_len + needed > self.mmap.mmap.len() {
            // Resize
            let new_size = (current_len + needed) * 2;
            self.mmap.resize(new_size)?;
        }

        let slice = self.mmap.slice_mut(current_len, needed)
            .ok_or_else(|| anyhow::anyhow!("Failed to get slice"))?;

        slice[0..4].copy_from_slice(&key_len);
        slice[4..4 + key.len()].copy_from_slice(key);
        slice[4 + key.len()..4 + key.len() + 4].copy_from_slice(&value_len);
        slice[4 + key.len() + 4..].copy_from_slice(value);

        self.mmap.flush()?;
        Ok(())
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        // Simplified - would need proper index
        None
    }

    pub fn flush(&mut self) -> Result<()> {
        self.mmap.flush()
    }
}