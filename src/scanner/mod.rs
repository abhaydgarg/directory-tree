use std::{
  ffi::OsStr,
  path::Path,
  fs:: {self, ReadDir}
};
use serde::Serialize;
use serde_json;

mod node;
mod util;

// Paths in `use` statements are relative to the crate root.
// To import items relative to the current and parent modules,
// use the `self::` and `super::` prefixes, respectively.
use super::timer;
use node::Node;

pub struct ScanResult {
  pub time: String,
  pub size: u64,
  pub json: String,
}

#[derive(Serialize, Debug)]
pub enum Kind {
  Directory,
  File,
}

pub fn run(path: &str) -> Result<ScanResult, String> {
  let path = Path::new(path);

  // `read_dir` check:
  // 1. The provided path doesn't exist.
  // 2. The process lacks permissions to view the contents.
  // 3. The path points at a non-directory file.
  let iter = match path.read_dir() {
    Ok(iter) => iter,
    Err(err) => return Err(err.to_string()),
  };

  // Check if symbolic link.
  if path.read_link().is_ok() {
    return Err("The path points at a symbolic link".to_string());
  }

  let metadata = path.metadata().unwrap();

  let mut root = Node::new(
    Kind::Directory,
    util::name(path.file_name().unwrap_or(OsStr::new("/")).to_os_string()),
    util::path(path.to_path_buf()),
    0, // 0 because calculated recursively later.
    util::created(&metadata),
    util::modified(&metadata),
    None,
  );

  let now = timer::start();
  scan(iter, &mut root);
  let time = timer::end(now);

  let size = root.get_size();
  let json = serde_json::to_string(&root).unwrap();

  Ok(ScanResult {
    time: time,
    size: size,
    json: json,
  })
}

fn scan(iter: ReadDir, node: &mut Node) {
  for entry in iter {
    let entry = entry.unwrap();
    // Avoid symbolic link.
    if fs::read_link(entry.path()).is_err() {
      // Proceed only when successfully
      // read metadata.
      if let Ok(metadata) = entry.metadata() {
        if metadata.is_dir() == true {
          // Proceed only when successfully
          // access dir content.
          if let Ok(iter) = fs::read_dir(entry.path()) {
            node.add_child(Node::new(
              Kind::Directory,
              util::name(entry.file_name()),
              util::path(entry.path()),
              0, // 0 because calculated recursively later.
              util::created(&metadata),
              util::modified(&metadata),
              None,
            ));
            let last_child = node.get_last_child();
            scan(iter, last_child);
            let size = last_child.get_size();
            node.increment_dir_size(size);
          }
        } else {
          node.increment_dir_size(metadata.len());
          node.add_child(Node::new(
            Kind::File,
            util::name(entry.file_name()),
            util::path(entry.path()),
            metadata.len(),
            util::created(&metadata),
            util::modified(&metadata),
            None,
          ));
        }
      }
    }
  }
}
