use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MetaSealConfig {
  #[serde(default = "default_true")]
  pub basic_info: bool,
  #[serde(default = "default_true")]
  pub git_commit: bool,
  #[serde(default = "default_git_commit_count")]
  pub git_commit_count: usize,
  #[serde(default = "default_true")]
  pub git_branch: bool,
  #[serde(default = "default_true")]
  pub build_system: bool,
  #[serde(default = "default_output_dir")]
  pub output_dir: String,
  pub encryption_key_env: Option<String>,
  pub encryption_key: Option<String>,
}

impl Default for MetaSealConfig {
  fn default() -> Self {
    Self {
      basic_info: true,
      git_commit: true,
      git_commit_count: 3,
      git_branch: true,
      build_system: true,
      output_dir: "./dist".to_string(),
      encryption_key_env: None,
      encryption_key: None,
    }
  }
}

fn default_true() -> bool {
  true
}

fn default_git_commit_count() -> usize {
  3
}

fn default_output_dir() -> String {
  "./dist".to_string()
}

impl MetaSealConfig {
  pub fn load(path: Option<&str>) -> Result<Self, Box<dyn std::error::Error>> {
    let config_path = path.unwrap_or(".meta-sealrc");

    if Path::new(config_path).exists() {
      let content = fs::read_to_string(config_path)?;
      let config: MetaSealConfig = serde_json::from_str(&content)?;
      Ok(config)
    } else {
      if path.is_none() {
        Ok(MetaSealConfig::default())
      } else {
        Err(format!("Configuration file not found: {}", config_path).into())
      }
    }
  }
}
