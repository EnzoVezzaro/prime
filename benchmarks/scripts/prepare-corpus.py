#!/usr/bin/env python3
"""
Prime Corpus Preparation Script

Prepares benchmark repositories by cloning them at pinned commits.
Usage: python3 scripts/prepare-corpus.py [--corpus pr|nightly|all] [--output-dir DIR]
"""

import json
import os
import subprocess
import sys
import shutil
from pathlib import Path

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
ROOT_DIR = os.path.dirname(os.path.dirname(SCRIPT_DIR))
CORPUS_FILE = os.path.join(SCRIPT_DIR, "..", "corpus", "repositories.json")

def load_corpus():
    with open(CORPUS_FILE, "r") as f:
        return json.load(f)

def get_repos_for_corpus(corpus_name, data):
    if corpus_name == "pr":
        return data.get("pr_corpus", {}).get("repositories", [])
    elif corpus_name in ("nightly", "all"):
        return data.get("nightly_corpus", {}).get("repositories", ["*"])
    else:
        # Custom corpus name - treat as repo list
        return corpus_name.split(",")

def prepare_repo(repo_name, repo_info, output_dir):
    repo_path = output_dir / repo_name
    
    if repo_path.exists():
        print(f"  {repo_name}: already exists, checking commit...")
        try:
            # Verify it's at the right commit
            result = subprocess.run(
                ["git", "rev-parse", "HEAD"],
                cwd=repo_path,
                capture_output=True,
                text=True,
                check=True
            )
            current_commit = result.stdout.strip()
            expected_commit = repo_info["commit"]
            
            # Try to resolve tag to commit
            result = subprocess.run(
                ["git", "rev-parse", expected_commit],
                cwd=repo_path,
                capture_output=True,
                text=True
            )
            if result.returncode == 0:
                expected_commit = result.stdout.strip()
            
            if current_commit == expected_commit:
                print(f"  {repo_name}: already at correct commit")
                return True
            else:
                print(f"  {repo_name}: wrong commit, resetting...")
        except subprocess.CalledProcessError:
            print(f"  {repo_name}: not a valid git repo, removing...")
            shutil.rmtree(repo_path)
    
    if not repo_path.exists():
        print(f"  {repo_name}: cloning...")
        try:
            subprocess.run(
                ["git", "clone", repo_info["url"], str(repo_path)],
                check=True,
                capture_output=True
            )
        except subprocess.CalledProcessError as e:
            print(f"  {repo_name}: clone failed: {e}")
            return False
    
    # Checkout the pinned commit
    print(f"  {repo_name}: checking out {repo_info['commit']}...")
    try:
        subprocess.run(
            ["git", "checkout", repo_info["commit"]],
            cwd=repo_path,
            check=True,
            capture_output=True
        )
    except subprocess.CalledProcessError as e:
        print(f"  {repo_name}: checkout failed: {e}")
        return False
    
    # Verify
    result = subprocess.run(
        ["git", "rev-parse", "HEAD"],
        cwd=repo_path,
        capture_output=True,
        text=True,
        check=True
    )
    actual_commit = result.stdout.strip()
    print(f"  {repo_name}: at commit {actual_commit[:12]}")
    
    # Calculate source stats
    source_bytes = 0
    lines_of_code = 0
    file_count = 0
    
    source_extensions = {
        "rs", "py", "js", "ts", "jsx", "tsx", "go", "java", "kt", "scala",
        "c", "h", "cpp", "cc", "cxx", "hpp", "hxx", "cs", "rb", "php",
        "swift", "lua", "dart", "zig", "toml", "json", "yaml", "yml",
        "md", "txt", "sh", "bash", "zsh", "fish", "ps1", "bat",
        "sql", "graphql", "proto", "dockerfile", "gradle", "maven",
    }
    
    for entry in Path(repo_path).rglob("*"):
        if entry.is_file() and entry.suffix[1:] in source_extensions:
            try:
                file_count += 1
                source_bytes += entry.stat().st_size
                content = entry.read_text(errors="ignore")
                lines_of_code += len(content.splitlines())
            except Exception:
                pass
    
    print(f"  {repo_name}: {file_count} files, {source_bytes/1024:.1f} KB, {lines_of_code} LOC")
    
    return True

def main():
    import argparse
    parser = argparse.ArgumentParser(description="Prepare Prime benchmark corpus")
    parser.add_argument("--corpus", default="pr", help="Corpus to prepare (pr, nightly, all)")
    parser.add_argument("--output-dir", default=None, help="Output directory (default: benchmarks/repos)")
    args = parser.parse_args()
    
    data = load_corpus()
    repos_to_prepare = get_repos_for_corpus(args.corpus, data)
    
    if args.output_dir:
        output_dir = Path(args.output_dir)
    else:
        output_dir = Path(ROOT_DIR) / "benchmarks" / "repos"
    
    output_dir.mkdir(parents=True, exist_ok=True)
    
    print(f"Preparing {args.corpus} corpus...")
    print(f"Output directory: {output_dir}")
    print(f"Repositories: {len(repos_to_prepare)}")
    
    success_count = 0
    for repo_name in repos_to_prepare:
        repo_info = next((r for r in data["repositories"] if r["name"] == repo_name), None)
        if not repo_info:
            print(f"  {repo_name}: not found in corpus, skipping")
            continue
        
        if prepare_repo(repo_name, repo_info, output_dir):
            success_count += 1
    
    print(f"\nDone: {success_count}/{len(repos_to_prepare)} repositories prepared")
    
    if success_count < len(repos_to_prepare):
        sys.exit(1)

if __name__ == "__main__":
    main()