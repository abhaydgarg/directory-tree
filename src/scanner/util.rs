use std::{
  fs::Metadata,
  ffi::OsString,
  path::PathBuf
};

pub fn name(name: OsString) -> String {
  name.to_str().unwrap().to_string()
}

pub fn abspath(path: PathBuf) -> String {
  path.to_str().unwrap().to_string()
}

pub fn path(name: OsString, parent_path: &String) -> String {
  let mut path = name.into_string().unwrap();
  path.insert_str(0, "/");
  path.insert_str(0, parent_path);
  path
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
