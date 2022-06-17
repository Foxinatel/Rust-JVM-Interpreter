use crate::stream_reader::StreamReader;

use super::attribute::{element_value::ElementValue, ATTRIBUTE};

pub fn read(sr: &mut StreamReader) -> ATTRIBUTE {
  ATTRIBUTE::AnnotationDefault {
    attribute_name_index: sr.get_u16(),
    attribute_length: sr.get_u32(),
    default_value: ElementValue::read(sr),
  }
}
