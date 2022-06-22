use std::{
  collections::{HashMap, HashSet},
  fs
};

use bitmask::bitmask;

use super::{
  attribute_info::Attribute,
  cp_info::CpInfo,
  cp_info_resolved::{Class, ResolvedCpInfo},
  field_info::FieldInfo,
  method_info::MethodInfo,
  stream_reader::StreamReader
};

bitmask! {
  #[derive(Debug)]
  pub mask ClassAccessFlags: u16 where flags Flags {
    AccPublic = 0x0001,
    AccPrivate = 0x0002,
    AccProtected = 0x0004,
    AccStatic = 0x0008,
    AccFinal = 0x0010,
    AccVolatile = 0x0040,
    AccTransient = 0x0080,
    AccSynthetic = 0x1000,
    AccEnum = 0x4000
  }
}

#[derive(Debug, Clone)]
pub struct ClassFile {
  pub access_flags: ClassAccessFlags,
  pub super_class: Class,
  pub interfaces: Vec<u16>,
  pub fields: HashMap<String, FieldInfo>,
  pub methods: HashMap<String, MethodInfo>,
  pub attributes: Vec<Attribute>
}

impl ClassFile {
  pub fn read(path: String) -> (String, Self, HashSet<String>) {
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

    let mut depends = HashSet::new();
    for constant in resolved_constant_pool.iter() {
      match constant {
        ResolvedCpInfo::Fieldref(a) => {
          depends.insert(a.class.name.clone());
        }
        ResolvedCpInfo::Methodref(a) => {
          depends.insert(a.class.name.clone());
        }
        ResolvedCpInfo::InterfaceMethodref(a) => {
          depends.insert(a.class.name.clone());
        }
        _ => {}
      }
    }

    let access_flags_num = sr.get_u16();
    let access_flags = ClassAccessFlags { mask: access_flags_num };

    let this_class = sr.get_u16();
    let super_class_index = sr.get_u16();
    let ResolvedCpInfo::Class(super_class) = resolved_constant_pool[super_class_index as usize -1].clone() else {panic!()};
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
