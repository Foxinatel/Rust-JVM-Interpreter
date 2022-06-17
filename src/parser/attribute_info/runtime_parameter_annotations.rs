use crate::stream_reader::StreamReader;

use super::attribute::{parameter_annotation::ParameterAnnotation, ATTRIBUTE};

pub fn read<const VISIBLE: bool>(sr: &mut StreamReader) -> ATTRIBUTE {
  let num_annotations = sr.get_u16();
  let parameter_annotations: Vec<ParameterAnnotation> = (0..num_annotations)
    .map(|_| ParameterAnnotation::read(sr))
    .collect();

  if VISIBLE {
    return ATTRIBUTE::RuntimeVisibleParameterAnnotations {
      num_annotations,
      parameter_annotations,
    };
  } else {
    return ATTRIBUTE::RuntimeInvisibleParameterAnnotations {
      num_annotations,
      parameter_annotations,
    };
  }
}
