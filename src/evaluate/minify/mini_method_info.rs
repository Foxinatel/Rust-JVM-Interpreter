use super::mini_attribute::MiniAttribute;
use crate::parser::{cp_info::CpInfo, method_info::MethodInfo};

#[derive(Debug)]
pub struct MiniMethodInfo {
  pub access_flags: u16,
  pub attributes: Vec<MiniAttribute>
}

impl MiniMethodInfo {
  pub fn from(fi: &MethodInfo, constant_pool: &Vec<CpInfo>) -> (String, Self) {
    let access_flags = fi.access_flags;

    let attributes = fi
      .attributes
      .iter()
      .map(|val| MiniAttribute::from(val, &constant_pool))
      .collect();

    let CpInfo::Utf8 { tag:_, length:_, bytes: name } = &constant_pool[fi.name_index as usize - 1] else { panic!() };
    let CpInfo::Utf8 { tag:_, length:_, bytes: descriptor } = &constant_pool[fi.descriptor_index as usize - 1] else { panic!() };
    (name.to_string() + descriptor, Self {
      access_flags,
      attributes
    })
  }
}
