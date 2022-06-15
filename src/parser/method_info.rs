use crate::helpers::get_u16;

use super::{
  cp_info::CpInfo,
  attribute_info::AttributeInfo,
};

#[derive(Debug)]
pub struct MethodInfo {
  pub access_flags: u16,
  pub name_index: u16,
  pub descriptor_index: u16,
  pub attributes_count: u16,
  pub attributes: Vec<AttributeInfo>
}

impl MethodInfo {
  pub fn read(buf: &mut &[u8], constant_pool: &Vec<CpInfo>) -> Self {
    let access_flags = get_u16(buf);
    let name_index = get_u16(buf);
    let descriptor_index = get_u16(buf);
    let attributes_count = get_u16(buf);
    let attributes: Vec<AttributeInfo> = (0..attributes_count).map(|_|
        AttributeInfo::read(buf, constant_pool)
    ).collect();

    Self {
      access_flags,
      name_index,
      descriptor_index,
      attributes_count,
      attributes
    }
  }
}
