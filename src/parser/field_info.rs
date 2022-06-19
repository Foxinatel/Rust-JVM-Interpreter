use super::{attribute_info::AttributeInfo, cp_info_resolved::ResolvedCpInfo};
use crate::stream_reader::StreamReader;

#[derive(Debug)]
pub struct FieldInfo {
  pub access_flags: u16,
  pub attributes_count: u16,
  pub attributes: Vec<AttributeInfo>
}

impl FieldInfo {
  pub fn read(sr: &mut StreamReader, constant_pool: &Vec<ResolvedCpInfo>) -> (String, Self) {
    let access_flags = sr.get_u16();

    let name_index = sr.get_u16();
    let ResolvedCpInfo::Utf8(name) = &constant_pool[name_index as usize -1] else {panic!()};

    let descriptor_index = sr.get_u16();
    let ResolvedCpInfo::Utf8(descriptor) = &constant_pool[descriptor_index as usize -1] else {panic!()};

    let attributes_count = sr.get_u16();
    let attributes: Vec<AttributeInfo> = (0..attributes_count)
      .map(|_| AttributeInfo::read(sr, &constant_pool))
      .collect();

    (name.to_owned() + descriptor, Self {
      access_flags,
      attributes_count,
      attributes
    })
  }
}
