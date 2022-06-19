use crate::stream_reader::StreamReader;

#[derive(Debug)]
pub enum CpInfo {
  Class { name_index: u16 },
  Fieldref { class_index: u16, name_and_type_index: u16 },
  Methodref { class_index: u16, name_and_type_index: u16 },
  InterfaceMethodref { class_index: u16, name_and_type_index: u16 },
  String { string_index: u16 },
  Integer { bytes: u32 },
  Float { bytes: u32 },
  Long { high_bytes: u32, low_bytes: u32 },
  Double { high_bytes: u32, low_bytes: u32 },
  NameAndType { name_index: u16, descriptor_index: u16 },
  Utf8 { bytes: String },
  MethodHandle { reference_kind: u8, reference_index: u16 },
  MethodType { descriptor_index: u16 },
  InvokeDynamic { bootstrap_method_attr_index: u16, reference_index: u16 }
}

impl CpInfo {
  pub fn read(sr: &mut StreamReader) -> Self {
    let tag = sr.get_u8();
    match tag {
      7 => CpInfo::Class { name_index: sr.get_u16() },
      9 => CpInfo::Fieldref { class_index: sr.get_u16(), name_and_type_index: sr.get_u16() },
      10 => CpInfo::Methodref { class_index: sr.get_u16(), name_and_type_index: sr.get_u16() },
      11 => {
        CpInfo::InterfaceMethodref { class_index: sr.get_u16(), name_and_type_index: sr.get_u16() }
      }
      8 => CpInfo::String { string_index: sr.get_u16() },
      3 => CpInfo::Integer { bytes: sr.get_u32() },
      4 => CpInfo::Float { bytes: sr.get_u32() },
      5 => CpInfo::Long { high_bytes: sr.get_u32(), low_bytes: sr.get_u32() },
      6 => CpInfo::Double { high_bytes: sr.get_u32(), low_bytes: sr.get_u32() },
      12 => CpInfo::NameAndType { name_index: sr.get_u16(), descriptor_index: sr.get_u16() },
      1 => {
        let length = sr.get_u16();
        CpInfo::Utf8 { bytes: String::from_utf8(sr.take_n(length as usize)).unwrap() }
      }
      15 => CpInfo::MethodHandle { reference_kind: sr.get_u8(), reference_index: sr.get_u16() },
      16 => CpInfo::MethodType { descriptor_index: sr.get_u16() },
      18 => CpInfo::InvokeDynamic {
        bootstrap_method_attr_index: sr.get_u16(),
        reference_index: sr.get_u16()
      },
      other => {
        eprintln!("ERROR, value was {}", other);
        panic!()
      }
    }
  }
}
