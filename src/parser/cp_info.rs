use crate::helpers::{get_u8, get_u16, get_u32, take_n};

#[derive(Debug)]
pub enum CpInfo {
  Class {tag: u8, name_index: u16},
  Fieldref {tag: u8, class_index: u16, name_and_type_index: u16},
  Methodref {tag: u8, class_index: u16, name_and_type_index: u16},
  InterfaceMethodref {tag: u8, class_index: u16, name_and_type_index: u16},
  String {tag: u8, string_index: u16},
  Integer {tag: u8, bytes: u32},
  Float {tag: u8, bytes: u32},
  Long {tag: u8, high_bytes: u32, low_bytes: u32},
  Double {tag: u8, high_bytes: u32, low_bytes: u32},
  NameAndType {tag: u8, class_index: u16, descriptor_index: u16},
  Utf8 {tag: u8, length: u16, bytes: String},
  MethodHandle {tag: u8, reference_kind: u8, reference_index: u16},
  MethodType {tag: u8, descriptor_index: u16},
  InvokeDynamic {tag: u8, bootstrap_method_attr_index: u16, reference_index: u16},
}

impl CpInfo {
  pub fn read(buf: &mut &[u8]) -> Self {
    let tag = get_u8(buf);
    match tag {
      7 => CpInfo::Class { tag, name_index: get_u16(buf) },
      9 => CpInfo::Fieldref { tag, class_index: get_u16(buf), name_and_type_index: get_u16(buf) },
      10 => CpInfo::Methodref { tag, class_index: get_u16(buf), name_and_type_index: get_u16(buf) },
      11 => CpInfo::InterfaceMethodref { tag, class_index: get_u16(buf), name_and_type_index: get_u16(buf) },
      8 => CpInfo::String { tag, string_index: get_u16(buf) },
      3 => CpInfo::Integer { tag, bytes: get_u32(buf) },
      4 => CpInfo::Float { tag, bytes: get_u32(buf) },
      5 => CpInfo::Long { tag, high_bytes: get_u32(buf), low_bytes: get_u32(buf) },
      6 => CpInfo::Double { tag, high_bytes: get_u32(buf), low_bytes: get_u32(buf) },
      12 => CpInfo::NameAndType { tag, class_index: get_u16(buf), descriptor_index: get_u16(buf) },
      1 => {
          let length = get_u16(buf);
          CpInfo::Utf8 { tag, length, bytes: String::from_utf8(take_n(length as usize, buf)).unwrap() }
      },
      15 => CpInfo::MethodHandle { tag, reference_kind: get_u8(buf), reference_index: get_u16(buf) },
      16 => CpInfo::MethodType { tag, descriptor_index: get_u16(buf) },
      18 => CpInfo::InvokeDynamic { tag, bootstrap_method_attr_index: get_u16(buf), reference_index: get_u16(buf) },
      other => { eprintln!("ERROR, value was {}", other); panic!() }
    }
  }
}
