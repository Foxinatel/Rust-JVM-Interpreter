use crate::stream_reader::StreamReader;

use super::Annotation;

#[derive(Debug)]
pub enum ElementValues {
  ConstValueIndex(u16),
  EnumConstValue {
    type_name_index: u16,
    const_name_index: u16,
  },
  ClassInfoIndex(u16),
  AnnotationValue(Annotation),
  ArrayValue {
    num_values: u16,
    values: Vec<ElementValue>,
  },
}

#[derive(Debug)]
pub struct ElementValue {
  pub tag: u8,
  pub value: ElementValues,
}

impl ElementValue {
  pub fn read(sr: &mut StreamReader) -> Self {
    let tag = sr.get_u8();
    let value = match tag as char {
      'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z' | 's' => {
        ElementValues::ConstValueIndex(sr.get_u16())
      }
      'e' => ElementValues::EnumConstValue {
        type_name_index: sr.get_u16(),
        const_name_index: sr.get_u16(),
      },
      'c' => ElementValues::ClassInfoIndex(sr.get_u16()),
      '@' => ElementValues::AnnotationValue(Annotation::read(sr)),
      '[' => {
        let num_values = sr.get_u16();
        let values: Vec<ElementValue> = (0..num_values).map(|_| ElementValue::read(sr)).collect();
        ElementValues::ArrayValue { num_values, values }
      }
      _ => panic!(),
    };
    ElementValue { tag, value }
  }
}
