use crate::parser::helpers::get_u16;

use super::attribute::{ATTRIBUTE, parameter_annotation::ParameterAnnotation};

pub fn read<const VISIBLE: bool>(buf: &mut &[u8]) -> ATTRIBUTE {
  let num_annotations = get_u16(buf);
  let parameter_annotations: Vec<ParameterAnnotation> = (0..num_annotations).map(|_|
    ParameterAnnotation::read(buf)
  ).collect();
  if VISIBLE {
    return ATTRIBUTE::RuntimeVisibleParameterAnnotations { num_annotations, parameter_annotations }
  } else {
    return ATTRIBUTE::RuntimeInvisibleParameterAnnotations { num_annotations, parameter_annotations }
  }
}