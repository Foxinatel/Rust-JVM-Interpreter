use crate::helpers::{get_u8, get_u16};

use super::Annotation;

#[derive(Debug)]
pub enum ElementValues {
  ConstValueIndex(u16),
  EnumConstValue{type_name_index: u16, const_name_index: u16},
  ClassInfoIndex(u16),
  AnnotationValue(Annotation),
  ArrayValue{num_values: u16, values: Vec<ElementValue>}
}

#[derive(Debug)]
pub struct ElementValue {
  pub tag: u8,
  pub value: ElementValues
}

impl ElementValue {
  pub fn read(buf: &mut &[u8]) -> Self {
    let tag = get_u8(buf);
    let value = match tag as char {
      'B'|'C'|'D'|'F'|'I'|'J'|'S'|'Z'|'s' => ElementValues::ConstValueIndex(get_u16(buf)),
      'e' => ElementValues::EnumConstValue {
        type_name_index: get_u16(buf),
        const_name_index: get_u16(buf)
      },
      'c' => ElementValues::ClassInfoIndex(get_u16(buf)),
      '@' => ElementValues::AnnotationValue(Annotation::read(buf)),
      '[' => {
        let num_values = get_u16(buf);
        let values: Vec<ElementValue> = (0..num_values).map(|_|
          ElementValue::read(buf)
        ).collect();
        ElementValues::ArrayValue { num_values, values }
      },
       _  => panic!()
    };
    ElementValue { tag, value }
  }
}
