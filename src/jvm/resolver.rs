use std::collections::{HashMap, HashSet};

use super::static_class::ClassStatics;
use crate::parser::classfile::ClassFile;

pub struct Resolver {
  pub resolved: HashMap<String, ClassStatics>
}

impl Resolver {
  pub fn new() -> Self { Self { resolved: HashMap::new() } }

  pub fn resolve(&mut self, depends: HashSet<String>) {
    for module in depends {
      if module == "java/lang/Object" || self.resolved.contains_key(&module) {
        continue;
      };
      let (name, cf, depends) = ClassFile::read(module.to_string());
      self.resolved.insert(name, ClassStatics::from(cf));
      self.resolve(depends)
    }
  }
}
