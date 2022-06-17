use crate::helpers::{get_u16, get_u32};

use super::attribute::{element_value::ElementValue, ATTRIBUTE};

pub fn read(buf: &mut &[u8]) -> ATTRIBUTE {
  ATTRIBUTE::AnnotationDefault {
    attribute_name_index: get_u16(buf),
    attribute_length: get_u32(buf),
    default_value: ElementValue::read(buf),
  }
}
