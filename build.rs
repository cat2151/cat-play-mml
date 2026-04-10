use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-env-changed=GIT_COMMIT_HASH");

    if let Some(git_dir) = git_dir() {
        let head_path = git_dir.join("HEAD");
        println!("cargo:rerun-if-changed={}", head_path.display());

        if let Ok(head_contents) = fs::read_to_string(&head_path) {
            if let Some(reference) = head_contents.trim().strip_prefix("ref: ") {
                let reference_path = git_dir.join(reference);
                println!("cargo:rerun-if-changed={}", reference_path.display());
            }
        }
    }

    let git_hash = std::env::var("GIT_COMMIT_HASH")
        .ok()
        .filter(|hash| !hash.trim().is_empty())
        .or_else(current_git_hash)
        .unwrap_or_else(|| "unknown".to_string());

    println!("cargo:rustc-env=GIT_COMMIT_HASH={}", git_hash.trim());
}

fn git_dir() -> Option<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let git_dir = String::from_utf8(output.stdout).ok()?;
    let git_dir = git_dir.trim();
    if git_dir.is_empty() {
        return None;
    }

    let path = Path::new(git_dir);
    if path.is_absolute() {
        Some(path.to_path_buf())
    } else {
        // `git rev-parse --git-dir` may return a path relative to the crate root.
        // Resolve it eagerly so `rerun-if-changed` points at a stable location.
        Some(std::env::current_dir().ok()?.join(path))
    }
}

fn current_git_hash() -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    let hash = String::from_utf8(output.stdout).ok()?;
    let hash = hash.trim();
    if hash.is_empty() {
        None
    } else {
        Some(hash.to_string())
    }
}
