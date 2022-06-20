use crate::stream_reader::StreamReader;

use super::Attribute;

pub fn read(sr: &mut StreamReader, attribute_length: u32) -> Attribute {
  Attribute::SourceDebugExtension { debug_extension: sr.take_n(attribute_length as usize) }
}
