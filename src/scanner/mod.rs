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
  pub items: u32,
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
  let mut id = 0u32;
  // If `/` is provided to scan then set
  // name, relative path and parent path to `/`.
  let name = path.file_name().unwrap_or(OsStr::new("/"));
  let mut relative_path = String::from("/");
  let mut parent_path = "/";
  // If path to scan is not `/` then:
  if let Some(parent) = path.parent() {
    parent_path = parent.to_str().unwrap();
    relative_path = util::path(&path, parent_path);
  }
  let mut root = Node::new(
    Kind::Directory,
    id,
    util::name(name.to_os_string()),
    util::abspath(path.to_path_buf()),
    relative_path,
    0, // 0 because calculated recursively.
    0,
    util::created(&metadata),
    util::modified(&metadata),
    None,
    None,
  );

  let now = timer::start();
  scan(iter, &mut root, &mut id, parent_path);
  let time = timer::end(now);

  let size = root.get_size();
  let json = serde_json::to_string(&root).unwrap();

  Ok(ScanResult {
    time: time,
    size: size,
    items: id,
    json: json,
  })
}

fn scan(iter: ReadDir, node: &mut Node, id: &mut u32, parent_path: &str) {
  for entry in iter {
    let entry = entry.unwrap();
    // Avoid symbolic link.
    if fs::read_link(entry.path()).is_err() {
      // Proceed only when successfully
      // read metadata.
      if let Ok(metadata) = entry.metadata() {
        *id = *id + 1;
        let parent = node.get_id();
        if metadata.is_dir() == true {
          // Proceed only when successfully
          // access dir content.
          if let Ok(iter) = fs::read_dir(entry.path()) {
            node.add_child(Node::new(
              Kind::Directory,
              *id,
              util::name(entry.file_name()),
              util::abspath(entry.path()),
              util::path(&entry.path(), parent_path),
              0,
              0,
              util::created(&metadata),
              util::modified(&metadata),
              Some(parent),
              None,
            ));
            let last_child = node.get_last_child();
            scan(iter, last_child, id, parent_path);
            let size = last_child.get_size();
            node.increment_dir_size(size);
            node.set_value();
          }
        } else {
          node.increment_dir_size(metadata.len());
          node.set_value();
          node.add_child(Node::new(
            Kind::File,
            *id,
            util::name(entry.file_name()),
            util::abspath(entry.path()),
            util::path(&entry.path(), parent_path),
            metadata.len(),
            metadata.len(),
            util::created(&metadata),
            util::modified(&metadata),
            Some(parent),
            None,
          ));
        }
      }
    }
  }
}
