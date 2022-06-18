use crate::stream_reader::StreamReader;

use super::{attribute_info::AttributeInfo, cp_info::CpInfo};

#[derive(Debug)]
pub struct MethodInfo {
  pub access_flags: u16,
  pub name_index: u16,
  pub descriptor_index: u16,
  pub attributes_count: u16,
  pub attributes: Vec<AttributeInfo>,
}

impl MethodInfo {
  pub fn read(sr: &mut StreamReader, constant_pool: &Vec<CpInfo>) -> (String, Self) {
    let access_flags = sr.get_u16();
    let name_index = sr.get_u16();
    let descriptor_index = sr.get_u16();
    let attributes_count = sr.get_u16();
    let attributes: Vec<AttributeInfo> = (0..attributes_count)
      .map(|_| AttributeInfo::read(sr, constant_pool))
      .collect();

    let CpInfo::Utf8 { tag: _, length: _, bytes: name } = &constant_pool[name_index as usize - 1] else {panic!()};
    (name.to_string(), Self {
      access_flags,
      name_index,
      descriptor_index,
      attributes_count,
      attributes,
    })
  }
}
