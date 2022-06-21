use std::collections::{HashMap, HashSet};

use crate::parser::classfile::ClassFile;

pub struct Resolver {
  pub resolved: HashMap<String, ClassFile>
}

impl Resolver {
  pub fn new() -> Self { Self { resolved: HashMap::new() } }

  pub fn resolve(&mut self, depends: HashSet<String>) {
    for module in depends {
      if module == "java/lang/Object" || self.resolved.contains_key(&module) {
        continue;
      };
      let (name, cf, depends) = ClassFile::read(module.to_string());
      self.resolved.insert(name, cf);
      self.resolve(depends)
    }
  }
}
