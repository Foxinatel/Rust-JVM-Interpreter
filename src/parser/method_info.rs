use super::{attribute_info::Attribute, cp_info_resolved::{ResolvedCpInfo, NameAndType}};
use crate::stream_reader::StreamReader;
use bitmask::bitmask;

bitmask! {
  #[derive(Debug)]
  pub mask MethodAccessFlags: u16 where flags Flags {
    AccPublic = 0x0001,
    AccPrivate = 0x0002,
    AccProtected = 0x0004,
    AccStatic = 0x0008,
    AccFinal = 0x0010,
    AccSynchronised = 0x0020,
    AccBridge = 0x0040,
    AccVarArgs = 0x0080,
    AccNative = 0x0100,
    AccAbstract = 0x0400,
    AccStrict = 0x0800,
    AccSynthetic = 0x1000,
  }
}

#[derive(Debug)]
pub struct MethodInfo {
  pub access_flags: MethodAccessFlags,
  pub attributes: Vec<Attribute>
}

impl MethodInfo {
  pub fn read(sr: &mut StreamReader, constant_pool: &Vec<ResolvedCpInfo>) -> (String, Self) {
    let access_flags_num = sr.get_u16();
    let access_flags = MethodAccessFlags { mask: access_flags_num };

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
