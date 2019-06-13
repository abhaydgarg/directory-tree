use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Node {
  kind: super::Kind,
  name: String,
  path: String,
  size: u64,
  created: u64,
  modified: u64,
  children: Option<Vec<Node>>,
}

impl Node {
  pub fn new(
    kind: super::Kind,
    name: String,
    path: String,
    size: u64,
    created: u64,
    modified: u64,
    children: Option<Vec<Node>>,
  ) -> Node {
    Node {
      kind: kind,
      name: name,
      path: path,
      size: size,
      created: created,
      modified: modified,
      children: children,
    }
  }

  pub fn get_size(&self) -> u64 {
    self.size
  }

  pub fn increment_dir_size(&mut self, size: u64) {
    self.size += size;
  }

  pub fn add_child(&mut self, node: Node) {
    if let Some(ref mut children) = self.children {
      children.push(node);
    } else {
      self.children = Some(vec![node]);
    }
  }

  pub fn get_last_child(&mut self) -> &mut Node {
    if let Some(ref mut children) = self.children {
      let index = children.len() - 1;
      return &mut children[index];
    } else {
      panic!("Cannot get last child");
    }
  }
}
