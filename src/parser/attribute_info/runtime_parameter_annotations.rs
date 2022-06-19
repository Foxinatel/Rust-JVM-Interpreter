use super::attribute::{parameter_annotation::ParameterAnnotation, Attribute};
use crate::stream_reader::StreamReader;

pub fn read<const VISIBLE: bool>(sr: &mut StreamReader) -> Attribute {
  let num_annotations = sr.get_u16();
  let parameter_annotations: Vec<ParameterAnnotation> =
    (0..num_annotations).map(|_| ParameterAnnotation::read(sr)).collect();

  if VISIBLE {
    return Attribute::RuntimeVisibleParameterAnnotations { parameter_annotations };
  } else {
    return Attribute::RuntimeInvisibleParameterAnnotations { parameter_annotations };
  }
}
