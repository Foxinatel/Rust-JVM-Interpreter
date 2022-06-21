use super::{attribute_info::Attribute, cp_info_resolved::{ResolvedCpInfo, NameAndType}};
use crate::stream_reader::StreamReader;
use bitmask::bitmask;

bitmask! {
  #[derive(Debug)]
  pub mask FieldAccessFlags: u16 where flags Flags {
    AccPublic = 0x0001,
    AccPrivate = 0x0002,
    AccProtected = 0x0004,
    AccStatic = 0x0008,
    AccFinal = 0x0010,
    AccVolatile = 0x0040,
    AccTransient = 0x0080,
    AccSynthetic = 0x1000,
    AccEnum = 0x4000,
  }
}

#[derive(Debug)]
pub struct FieldInfo {
  pub access_flags: FieldAccessFlags,
  pub attributes: Vec<Attribute>
}

impl FieldInfo {
  pub fn read(sr: &mut StreamReader, constant_pool: &Vec<ResolvedCpInfo>) -> (String, Self) {
    let access_flags_num = sr.get_u16();
    let access_flags = FieldAccessFlags { mask: access_flags_num };

    let name_index = sr.get_u16();
    let ResolvedCpInfo::Utf8(name) = &constant_pool[name_index as usize -1] else {panic!()};

    let descriptor_index = sr.get_u16();
    let ResolvedCpInfo::Utf8(descriptor) = &constant_pool[descriptor_index as usize -1] else {panic!()};

    let attributes_count = sr.get_u16();
    let attributes: Vec<Attribute> =
      (0..attributes_count).map(|_| Attribute::read(sr, &constant_pool)).collect();

    (NameAndType{ name: name.to_string(), descriptor: descriptor.to_string() }.to_string(), Self { access_flags, attributes })
  }
}
