mod config;
mod crypto;
mod info;

use config::MetaSealConfig;
use info::{collect_build_info, collect_git_info, collect_project_info, VersionData};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::env;
use std::fs;
use std::path::Path;

#[napi]
pub fn run_generate(config_path: Option<String>, cli_key: Option<String>) -> Result<()> {
  let config = MetaSealConfig::load(config_path.as_deref())
    .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

  let key = cli_key
    .or_else(|| config.encryption_key.clone())
    .or_else(|| {
      config
        .encryption_key_env
        .as_ref()
        .and_then(|env_var| env::var(env_var).ok())
    })
    .or_else(|| env::var("META_SEAL_KEY").ok());

  let key = match key {
    Some(k) => k,
    None => {
      return Err(Error::new(
        Status::InvalidArg,
        "Encryption key not provided. Use --key or META_SEAL_KEY env variable.".to_string(),
      ))
    }
  };

  if key.len() != 32 {
    return Err(Error::new(
      Status::InvalidArg,
      format!("Key must be 32 bytes, got {}", key.len()),
    ));
  }

  let mut version_data = VersionData {
    project: None,
    git: None,
    build: None,
  };

  if config.basic_info {
    version_data.project = collect_project_info();
  }

  if config.git_branch || config.git_commit {
    version_data.git = collect_git_info(
      config.git_branch,
      config.git_commit,
      config.git_commit_count,
    );
  }

  if config.build_system {
    version_data.build = collect_build_info();
  }

  let json_data = serde_json::to_string(&version_data)
    .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

  let encrypted_data = crypto::encrypt_data(&json_data, &key)
    .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

  let out_dir = Path::new(&config.output_dir);
  if !out_dir.exists() {
    fs::create_dir_all(out_dir).map_err(|e| {
      Error::new(
        Status::GenericFailure,
        format!("Failed to create output dir: {}", e),
      )
    })?;
  }

  let out_file = out_dir.join("VERSION");
  fs::write(&out_file, encrypted_data).map_err(|e| {
    Error::new(
      Status::GenericFailure,
      format!("Failed to write VERSION file: {}", e),
    )
  })?;

  println!(
    "Successfully generated VERSION file at {}",
    out_file.display()
  );
  Ok(())
}

#[napi]
pub fn run_read(
  file_path: String,
  cli_key: Option<String>,
  config_path: Option<String>,
) -> Result<String> {
  let config = MetaSealConfig::load(config_path.as_deref())
    .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

  let key = cli_key
    .or_else(|| config.encryption_key.clone())
    .or_else(|| {
      config
        .encryption_key_env
        .as_ref()
        .and_then(|env_var| env::var(env_var).ok())
    })
    .or_else(|| env::var("META_SEAL_KEY").ok());

  let key = match key {
    Some(k) => k,
    None => {
      return Err(Error::new(
        Status::InvalidArg,
        "Encryption key not provided. Use --key or META_SEAL_KEY env variable.".to_string(),
      ))
    }
  };

  if key.len() != 32 {
    return Err(Error::new(
      Status::InvalidArg,
      format!("Key must be 32 bytes, got {}", key.len()),
    ));
  }

  let content = fs::read_to_string(&file_path).map_err(|e| {
    Error::new(
      Status::GenericFailure,
      format!("Failed to read file {}: {}", file_path, e),
    )
  })?;

  let decrypted_data = crypto::decrypt_data(&content, &key)
    .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

  let parsed: serde_json::Value = serde_json::from_str(&decrypted_data)
    .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

  let pretty_json = serde_json::to_string_pretty(&parsed)
    .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

  Ok(pretty_json)
}
