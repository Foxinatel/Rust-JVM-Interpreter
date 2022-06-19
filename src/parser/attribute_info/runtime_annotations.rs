use super::attribute::{annotation::Annotation, Attribute};
use crate::stream_reader::StreamReader;

pub fn read<const VISIBLE: bool>(sr: &mut StreamReader) -> Attribute {
  let num_annotations = sr.get_u16();
  let annotations: Vec<Annotation> = (0..num_annotations).map(|_| Annotation::read(sr)).collect();

  if VISIBLE {
    return Attribute::RuntimeVisibleAnnotations { annotations };
  } else {
    return Attribute::RuntimeInvisibleAnnotations { annotations };
  }
}
