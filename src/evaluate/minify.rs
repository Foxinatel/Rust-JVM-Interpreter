use std::collections::HashMap;

use self::{
  mini_attribute::MiniAttribute,
  mini_class_info::MiniClassInfo,
  mini_field_info::MiniFieldInfo,
  mini_method_info::MiniMethodInfo
};
use crate::parser::{classfile::ClassFile, cp_info::CpInfo};

mod mini_attribute;
mod mini_class_info;
mod mini_field_info;
mod mini_method_info;
mod mini_name_and_type_info;

#[derive(Debug)]
pub struct MinifiedClassFile {
  access_flags: u16,
  this_class: MiniClassInfo,
  super_class: MiniClassInfo,
  interfaces: Vec<MiniClassInfo>,
  fields: HashMap<String, MiniFieldInfo>,
  methods: HashMap<String, MiniMethodInfo>,
  attributes: Vec<MiniAttribute>
}

impl MinifiedClassFile {
  pub fn from(cf: ClassFile) -> Self {
    let constant_pool = &cf.constant_pool;

    let access_flags = cf.access_flags;
    let CpInfo::Class { tag:_, name_index: index } = constant_pool[cf.this_class as usize - 1] else { panic!() };
    let CpInfo::Utf8 { tag:_, length:_, bytes: name } = &constant_pool[index as usize - 1] else { panic!() };
    let this_class = MiniClassInfo {
      name: name.to_string()
    };

    let CpInfo::Class { tag:_, name_index: index } = constant_pool[cf.super_class as usize - 1] else { panic!() };
    let CpInfo::Utf8 { tag:_, length:_, bytes: name } = &constant_pool[index as usize - 1] else { panic!() };
    let super_class = MiniClassInfo {
      name: name.to_string()
    };

    let interfaces: Vec<MiniClassInfo> = cf.interfaces.iter()
    .map(|class_index| {
      let CpInfo::Class { tag:_, name_index: index } = &constant_pool[*class_index as usize - 1] else { panic!() };
      let CpInfo::Utf8 { tag:_, length:_, bytes: name } = &constant_pool[*index as usize - 1] else { panic!() };
      MiniClassInfo{ name: name.to_string() }
    }).collect();

    let fields: HashMap<String, MiniFieldInfo> = cf
      .fields
      .iter()
      .map(|val| MiniFieldInfo::from(&val, constant_pool))
      .collect();

    let methods: HashMap<String, MiniMethodInfo> = cf
      .methods
      .iter()
      .map(|val| MiniMethodInfo::from(&val, constant_pool))
      .collect();

    let attributes = cf
      .attributes
      .iter()
      .map(|val| MiniAttribute::from(val, &constant_pool))
      .collect();

    Self {
      access_flags,
      this_class,
      super_class,
      interfaces,
      fields,
      methods,
      attributes
    }
  }
}
