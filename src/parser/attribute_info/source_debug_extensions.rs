use crate::stream_reader::StreamReader;

use super::attribute::ATTRIBUTE;

pub fn read(sr: &mut StreamReader, attribute_length: u32) -> ATTRIBUTE {
  ATTRIBUTE::SourceDebugExtension {
    debug_extension: sr.take_n(attribute_length as usize),
  }
}
