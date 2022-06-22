use super::{attribute::stack_map_frame::StackMapFrame, Attribute};
use crate::parser::stream_reader::StreamReader;

pub fn read(sr: &mut StreamReader) -> Attribute {
  let number_of_entries = sr.get_u16();
  let entries: Vec<StackMapFrame> =
    (0..number_of_entries).map(|_| StackMapFrame::read(sr)).collect();

  Attribute::StackMapTable { entries }
}
