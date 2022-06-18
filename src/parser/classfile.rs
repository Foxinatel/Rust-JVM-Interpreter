use std::{collections::HashMap, fs};

use crate::stream_reader::StreamReader;

use super::{
  attribute_info::AttributeInfo, cp_info::CpInfo, field_info::FieldInfo, method_info::MethodInfo,
};

#[derive(Debug)]
pub struct ClassFile {
  pub minor_version: u16,
  pub major_version: u16,
  pub constant_pool_count: u16,
  pub constant_pool: Vec<CpInfo>, //cp_info constant_pool[constant_pool_count-1];
  pub access_flags: u16,
  pub this_class: u16,
  pub super_class: u16,
  pub interfaces_count: u16,
  pub interfaces: Vec<u16>, //u16 interfaces[interfaces_count]
  pub fields_count: u16,
  pub fields: Vec<FieldInfo>, //field_info fields[fields_count];
  pub methods_count: u16,
  pub methods: HashMap<String, MethodInfo>, //method_info methods[methods_count];
  pub attributes_count: u16,
  pub attributes: Vec<AttributeInfo>, //attribute_info attributes[attributes_count];
}

impl ClassFile {
  pub fn read(path: String) -> (String,Self) {
    let buf = fs::read(path.clone()).or(fs::read(path.clone() + ".class")).expect(
      format!("Could not find a file at {0} or {0}.class", path).as_str()
    );
    let mut sr = &mut StreamReader::from(buf);
    sr.stream = sr.stream
      .strip_prefix(&[0xca, 0xfe, 0xba, 0xbe])
      .expect("File has invalid header").to_vec();
    let minor_version = sr.get_u16();
    let major_version = sr.get_u16();
    let constant_pool_count = sr.get_u16();
    let constant_pool: Vec<CpInfo> = (1..constant_pool_count)
      .map(|_| CpInfo::read(sr))
      .collect();
    let access_flags = sr.get_u16();
    let this_class = sr.get_u16();
    let super_class = sr.get_u16();
    let interfaces_count = sr.get_u16();
    let interfaces: Vec<u16> = (0..interfaces_count).map(|_| sr.get_u16()).collect();
    let fields_count = sr.get_u16();
    let fields: Vec<FieldInfo> = (0..fields_count)
      .map(|_| FieldInfo::read(sr, &constant_pool))
      .collect();
    let methods_count = sr.get_u16();
    let methods: HashMap<String, MethodInfo> = (0..methods_count)
      .map(|_| MethodInfo::read(sr, &constant_pool))
      .collect();
    let attributes_count = sr.get_u16();
    let attributes: Vec<AttributeInfo> = (0..attributes_count)
      .map(|_| AttributeInfo::read(sr, &constant_pool))
      .collect();
    if !sr.done() {panic!("Extra bytes were found at the end of the classfile")}

    let CpInfo::Class { tag:_, name_index } = &constant_pool[this_class as usize - 1] else {panic!()};
    let CpInfo::Utf8 { tag: _, length: _, bytes: name } = &constant_pool[*name_index as usize - 1] else {panic!()};

    (name.to_string(), Self {
      minor_version,
      major_version,
      constant_pool_count,
      constant_pool,
      access_flags,
      this_class,
      super_class,
      interfaces_count,
      interfaces,
      fields_count,
      fields,
      methods_count,
      methods,
      attributes_count,
      attributes,
    })
  }
}
