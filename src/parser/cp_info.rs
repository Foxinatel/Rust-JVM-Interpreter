use crate::stream_reader::StreamReader;

#[derive(Debug)]
pub enum CpInfo {
  Class {
    tag: u8,
    name_index: u16,
  },
  Fieldref {
    tag: u8,
    class_index: u16,
    name_and_type_index: u16,
  },
  Methodref {
    tag: u8,
    class_index: u16,
    name_and_type_index: u16,
  },
  InterfaceMethodref {
    tag: u8,
    class_index: u16,
    name_and_type_index: u16,
  },
  String {
    tag: u8,
    string_index: u16,
  },
  Integer {
    tag: u8,
    bytes: u32,
  },
  Float {
    tag: u8,
    bytes: u32,
  },
  Long {
    tag: u8,
    high_bytes: u32,
    low_bytes: u32,
  },
  Double {
    tag: u8,
    high_bytes: u32,
    low_bytes: u32,
  },
  NameAndType {
    tag: u8,
    class_index: u16,
    descriptor_index: u16,
  },
  Utf8 {
    tag: u8,
    length: u16,
    bytes: String,
  },
  MethodHandle {
    tag: u8,
    reference_kind: u8,
    reference_index: u16,
  },
  MethodType {
    tag: u8,
    descriptor_index: u16,
  },
  InvokeDynamic {
    tag: u8,
    bootstrap_method_attr_index: u16,
    reference_index: u16,
  },
}

impl CpInfo {
  pub fn read(sr: &mut StreamReader) -> Self {
    let tag = sr.get_u8();
    match tag {
      7 => CpInfo::Class {
        tag,
        name_index: sr.get_u16(),
      },
      9 => CpInfo::Fieldref {
        tag,
        class_index: sr.get_u16(),
        name_and_type_index: sr.get_u16(),
      },
      10 => CpInfo::Methodref {
        tag,
        class_index: sr.get_u16(),
        name_and_type_index: sr.get_u16(),
      },
      11 => CpInfo::InterfaceMethodref {
        tag,
        class_index: sr.get_u16(),
        name_and_type_index: sr.get_u16(),
      },
      8 => CpInfo::String {
        tag,
        string_index: sr.get_u16(),
      },
      3 => CpInfo::Integer {
        tag,
        bytes: sr.get_u32(),
      },
      4 => CpInfo::Float {
        tag,
        bytes: sr.get_u32(),
      },
      5 => CpInfo::Long {
        tag,
        high_bytes: sr.get_u32(),
        low_bytes: sr.get_u32(),
      },
      6 => CpInfo::Double {
        tag,
        high_bytes: sr.get_u32(),
        low_bytes: sr.get_u32(),
      },
      12 => CpInfo::NameAndType {
        tag,
        class_index: sr.get_u16(),
        descriptor_index: sr.get_u16(),
      },
      1 => {
        let length = sr.get_u16();
        CpInfo::Utf8 {
          tag,
          length,
          bytes: String::from_utf8(sr.take_n(length as usize)).unwrap(),
        }
      }
      15 => CpInfo::MethodHandle {
        tag,
        reference_kind: sr.get_u8(),
        reference_index: sr.get_u16(),
      },
      16 => CpInfo::MethodType {
        tag,
        descriptor_index: sr.get_u16(),
      },
      18 => CpInfo::InvokeDynamic {
        tag,
        bootstrap_method_attr_index: sr.get_u16(),
        reference_index: sr.get_u16(),
      },
      other => {
        eprintln!("ERROR, value was {}", other);
        panic!()
      }
    }
  }
}
