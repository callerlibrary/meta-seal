use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectInfo {
  pub name: Option<String>,
  pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitCommit {
  pub hash: String,
  pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitInfo {
  pub branch: Option<String>,
  pub commits: Vec<GitCommit>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildSystemInfo {
  pub os: String,
  pub architecture: String,
  pub build_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionData {
  pub project: Option<ProjectInfo>,
  pub git: Option<GitInfo>,
  pub build: Option<BuildSystemInfo>,
}

pub fn collect_project_info() -> Option<ProjectInfo> {
  let pkg_path = Path::new("package.json");
  if pkg_path.exists() {
    if let Ok(content) = fs::read_to_string(pkg_path) {
      if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
        let name = json["name"].as_str().map(|s| s.to_string());
        let version = json["version"].as_str().map(|s| s.to_string());
        return Some(ProjectInfo { name, version });
      }
    }
  }
  None
}

pub fn collect_git_info(branch: bool, commit: bool, commit_count: usize) -> Option<GitInfo> {
  if !branch && !commit {
    return None;
  }

  let mut info = GitInfo {
    branch: None,
    commits: Vec::new(),
  };

  if branch {
    if let Ok(output) = Command::new("git")
      .args(["rev-parse", "--abbrev-ref", "HEAD"])
      .output()
    {
      if output.status.success() {
        info.branch = Some(String::from_utf8_lossy(&output.stdout).trim().to_string());
      }
    }
  }

  if commit && commit_count > 0 {
    let count_str = format!("-{}", commit_count);
    // %H: commit hash, %s: subject (commit message)
    if let Ok(output) = Command::new("git")
      .args(["log", &count_str, "--format=%H|||%s"])
      .output()
    {
      if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
          if let Some((hash, message)) = line.split_once("|||") {
            info.commits.push(GitCommit {
              hash: hash.trim().to_string(),
              message: message.trim().to_string(),
            });
          }
        }
      }
    }
  }

  Some(info)
}

pub fn collect_build_info() -> Option<BuildSystemInfo> {
  let os = env::consts::OS.to_string();
  let architecture = env::consts::ARCH.to_string();
  let build_time = Utc::now().to_rfc3339();

  Some(BuildSystemInfo {
    os,
    architecture,
    build_time,
  })
}
