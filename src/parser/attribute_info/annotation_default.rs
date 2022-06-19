use super::attribute::{element_value::ElementValue, ATTRIBUTE};
use crate::{parser::cp_info_resolved::ResolvedCpInfo, stream_reader::StreamReader};

pub fn read(sr: &mut StreamReader, constant_pool: &Vec<ResolvedCpInfo>) -> ATTRIBUTE {
  let attribute_name_index = sr.get_u16();
  let ResolvedCpInfo::Utf8 (attribute_name) = &constant_pool[attribute_name_index as usize -1] else {panic!()};
  let attribute_length = sr.get_u32();
  let default_value = ElementValue::read(sr);

  ATTRIBUTE::AnnotationDefault {
    attribute_name: attribute_name.to_string(),
    attribute_length,
    default_value
  }
}
