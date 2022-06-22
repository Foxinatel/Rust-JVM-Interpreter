use super::Attribute;
use crate::parser::stream_reader::StreamReader;

pub fn read(sr: &mut StreamReader) -> Attribute {
  let number_of_exceptions = sr.get_u16();
  let exception_index_table: Vec<u16> = (0..number_of_exceptions).map(|_| sr.get_u16()).collect();

  Attribute::Exceptions { exception_index_table }
}
