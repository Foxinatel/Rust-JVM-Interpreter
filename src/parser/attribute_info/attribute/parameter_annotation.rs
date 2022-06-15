use crate::parser::helpers::get_u16;

use super::annotation::Annotation;

#[derive(Debug)]
pub struct ParameterAnnotation {
  num_annotations: u16,
  annotations: Vec<Annotation>
}

impl ParameterAnnotation {
  pub fn read(buf: &mut &[u8]) -> Self {
    let num_annotations = get_u16(buf);
    let annotations: Vec<Annotation> = (0..num_annotations).map(|_|
      Annotation::read(buf)
    ).collect();
    ParameterAnnotation { num_annotations, annotations }
  }
}