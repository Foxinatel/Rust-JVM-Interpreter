use super::annotation::Annotation;
use crate::stream_reader::StreamReader;

#[derive(Debug)]
pub struct ParameterAnnotation {
  pub num_annotations: u16,
  pub annotations: Vec<Annotation>
}

impl ParameterAnnotation {
  pub fn read(sr: &mut StreamReader) -> Self {
    let num_annotations = sr.get_u16();
    let annotations: Vec<Annotation> = (0..num_annotations).map(|_| Annotation::read(sr)).collect();
    ParameterAnnotation { num_annotations, annotations }
  }
}
