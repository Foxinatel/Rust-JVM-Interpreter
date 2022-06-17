use crate::stream_reader::StreamReader;

use super::attribute::ATTRIBUTE;

pub fn read(sr: &mut StreamReader) -> ATTRIBUTE {
  ATTRIBUTE::SourceFile {
    sourcefile_index: sr.get_u16(),
  }
}
