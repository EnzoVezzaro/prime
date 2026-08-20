//! Directory walking and file discovery

use ignore::WalkBuilder;
use std::path::{Path, PathBuf};
use prime_core::Language;

/// Walk a directory and collect source files
pub fn walk_project(root: &Path, excluded_patterns: &[String]) -> Vec<PathBuf> {
    let mut files = Vec::new();

    let walker = WalkBuilder::new(root)
        .hidden(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .build();

    for entry in walker {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() {
                    if !is_excluded(path, excluded_patterns) {
if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        let lang = Language::from_extension(ext);
                        if lang != Language::Unknown {
                            files.push(path.to_path_buf());
                        }
                    }
                    }
                }
            }
            Err(_) => {} // Skip errors
        }
    }

    files
}

fn is_excluded(path: &Path, patterns: &[String]) -> bool {
    let path_str = path.to_string_lossy();
    patterns.iter().any(|pat| {
        glob::Pattern::new(pat).map(|p| p.matches(&path_str)).unwrap_or(false)
    })
}

/// Get all files grouped by language
pub fn group_files_by_language(root: &Path, excluded_patterns: &[String]) -> std::collections::HashMap<Language, Vec<PathBuf>> {
    let mut grouped = std::collections::HashMap::new();

    let walker = WalkBuilder::new(root)
        .hidden(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .build();

    for entry in walker {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() && !is_excluded(path, excluded_patterns) {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    let lang = crate::Language::from_extension(ext);
                    if lang != crate::Language::Unknown {
                        grouped.entry(lang).or_insert_with(Vec::new).push(path.to_path_buf());
                    }
                }
            }
        }
    }

    grouped
}