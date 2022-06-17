use crate::stream_reader::StreamReader;

use super::element_value::ElementValue;

#[derive(Debug)]
pub struct Annotation {
  pub type_index: u16,
  pub num_element_value_pairs: u16,
  pub element_value_pairs: Vec<(u16, ElementValue)>,
}

impl Annotation {
  pub fn read(sr: &mut StreamReader) -> Self {
    let type_index = sr.get_u16();
    let num_element_value_pairs = sr.get_u16();
    let element_value_pairs: Vec<(u16, ElementValue)> = (0..num_element_value_pairs)
      .map(|_| (sr.get_u16(), ElementValue::read(sr)))
      .collect();

    Annotation {
      type_index,
      num_element_value_pairs,
      element_value_pairs,
    }
  }
}
