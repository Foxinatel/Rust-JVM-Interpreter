use std::{collections::HashMap, fs};

use super::{
  cp_info::CpInfo,
  cp_info_resolved::ResolvedCpInfo,
  field_info::FieldInfo,
  method_info::MethodInfo, attribute_info::Attribute
};
use crate::stream_reader::StreamReader;

#[derive(Debug)]
pub struct ClassFile {
  pub access_flags: u16,
  pub super_class: u16,
  pub interfaces: Vec<u16>,
  pub fields: HashMap<String, FieldInfo>,
  pub methods: HashMap<String, MethodInfo>,
  pub attributes: Vec<Attribute>
}

impl ClassFile {
  pub fn read(path: String) -> (String, Self, Vec<String>) {
    let buf = fs::read(path.clone())
      .or(fs::read(path.clone() + ".class"))
      .expect(format!("Could not find a file at {0} or {0}.class", path).as_str());
    let mut sr = &mut StreamReader::from(buf);
    sr.stream =
      sr.stream.strip_prefix(&[0xca, 0xfe, 0xba, 0xbe]).expect("File has invalid header").to_vec();
    let _minor_version = sr.get_u16();
    let _major_version = sr.get_u16();
    let constant_pool_count = sr.get_u16();
    let constant_pool: Vec<CpInfo> = (1..constant_pool_count).map(|_| CpInfo::read(sr)).collect();
    let resolved_constant_pool: Vec<ResolvedCpInfo> =
      constant_pool.iter().map(|val| ResolvedCpInfo::from(val, &constant_pool)).collect();

    let mut depends = Vec::new();
    for constant in resolved_constant_pool.iter() {
      match constant {
        ResolvedCpInfo::Fieldref(a) => {
          depends.push(a.class.name.clone());
        }
        ResolvedCpInfo::Methodref(a) => {
          depends.push(a.class.name.clone());
        }
        ResolvedCpInfo::InterfaceMethodref(a) => {
          depends.push(a.class.name.clone());
        }
        _ => {}
      }
    }

    let access_flags = sr.get_u16();
    let this_class = sr.get_u16();
    let super_class = sr.get_u16();
    let interfaces_count = sr.get_u16();
    let interfaces: Vec<u16> = (0..interfaces_count).map(|_| sr.get_u16()).collect();
    let fields_count = sr.get_u16();
    let fields: HashMap<String, FieldInfo> =
      (0..fields_count).map(|_| FieldInfo::read(sr, &resolved_constant_pool)).collect();
    let methods_count = sr.get_u16();
    let methods: HashMap<String, MethodInfo> =
      (0..methods_count).map(|_| MethodInfo::read(sr, &resolved_constant_pool)).collect();
    let attributes_count = sr.get_u16();
    let attributes: Vec<Attribute> =
      (0..attributes_count).map(|_| Attribute::read(sr, &resolved_constant_pool)).collect();
    if !sr.done() {
      panic!("Extra bytes were found at the end of the classfile")
    }

    let CpInfo::Class { name_index } = &constant_pool[this_class as usize - 1] else {panic!()};
    let CpInfo::Utf8 { bytes: name } = &constant_pool[*name_index as usize - 1] else {panic!()};

    (
      name.to_string(),
      Self { access_flags, super_class, interfaces, fields, methods, attributes },
      depends
    )
  }
}
