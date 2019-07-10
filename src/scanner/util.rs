use std::{
  fs::Metadata,
  ffi::OsString,
  path::{Path, PathBuf},
};

pub fn name(name: OsString) -> String {
  name.to_str().unwrap().to_string()
}

pub fn abspath(path: PathBuf) -> String {
  path.to_str().unwrap().to_string()
}

pub fn path(path: &Path, parent_path: Option<&str>) -> String {
  if let Some(parent) = parent_path {
    return path
      .strip_prefix(parent)
      .unwrap()
      .to_str()
      .unwrap()
      .to_string();
  }
  path.to_str().unwrap().to_string()
}

pub fn created(metadata: &Metadata) -> u64 {
  metadata.created().unwrap().elapsed().unwrap().as_secs()
}

pub fn modified(metadata: &Metadata) -> u64 {
  // In some file or dir, `elapsed`
  // gives an error, so in that case
  // 0 is returned.
  match metadata.modified().unwrap().elapsed() {
    Ok(m) => return m.as_secs(),
    Err(_) => return 0,
  }
}
