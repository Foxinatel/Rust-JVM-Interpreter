use crate::helpers::get_u16;

use super::attribute::{annotation::Annotation, ATTRIBUTE};

pub fn read<const VISIBLE: bool>(buf: &mut &[u8]) -> ATTRIBUTE {
  let num_annotations = get_u16(buf);
  let annotations: Vec<Annotation> = (0..num_annotations)
    .map(|_| Annotation::read(buf))
    .collect();

  if VISIBLE {
    return ATTRIBUTE::RuntimeVisibleAnnotations {
      num_annotations,
      annotations,
    };
  } else {
    return ATTRIBUTE::RuntimeInvisibleAnnotations {
      num_annotations,
      annotations,
    };
  }
}
