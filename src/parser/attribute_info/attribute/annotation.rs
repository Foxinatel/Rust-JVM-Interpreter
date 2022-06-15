use crate::parser::helpers::get_u16;

use super::element_value::ElementValue;

#[derive(Debug)]
pub struct Annotation {
  type_index: u16,
  num_element_value_pairs: u16,
  element_value_pairs: Vec<(u16,ElementValue)>
}

impl Annotation {
  pub fn read(buf: &mut &[u8]) -> Self {
    let type_index = get_u16(buf);
    let num_element_value_pairs = get_u16(buf);
    let element_value_pairs: Vec<(u16,ElementValue)> = (0..num_element_value_pairs).map(|_|
      (get_u16(buf),ElementValue::read(buf))
    ).collect();
   Annotation { type_index, num_element_value_pairs, element_value_pairs }
  }
}