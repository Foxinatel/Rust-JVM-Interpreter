use std::collections::HashMap;

use super::minify::MinifiedClassFile;
use crate::parser::{classfile::ClassFile, cp_info::CpInfo};

pub struct Resolver {
  pub resolved: HashMap<String, MinifiedClassFile>
}

impl Resolver {
  pub fn new() -> Self {
    Self {
      resolved: HashMap::new()
    }
  }

  pub fn resolve(&mut self, (name, cf): (String, ClassFile)) {
    let constant_pool = cf.constant_pool.clone();
    let (newname, newcf) = MinifiedClassFile::from(cf);
    self.resolved.insert(newname, newcf);
    for i in constant_pool.as_slice() {
      match i {
        CpInfo::Fieldref {
          tag: _,
          class_index,
          name_and_type_index: _
        }
        | CpInfo::Methodref {
          tag: _,
          class_index,
          name_and_type_index: _
        }
        | CpInfo::InterfaceMethodref {
          tag: _,
          class_index,
          name_and_type_index: _
        } => {
          let CpInfo::Class { tag:_, name_index } = &constant_pool[*class_index as usize - 1] else {panic!()};
          let CpInfo::Utf8 { tag: _, length: _, bytes: name } = &constant_pool[*name_index as usize - 1] else {panic!()};
          if name == "java/lang/Object" || self.resolved.contains_key(name) {
            continue;
          };
          self.resolve(ClassFile::read(name.to_string()));
        }
        _ => {}
      }
    }
  }
}
