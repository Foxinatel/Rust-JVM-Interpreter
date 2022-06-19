use super::attribute::ATTRIBUTE;
use crate::stream_reader::StreamReader;

pub fn read(sr: &mut StreamReader) -> ATTRIBUTE {
  ATTRIBUTE::SourceFile {
    sourcefile_index: sr.get_u16()
  }
}
