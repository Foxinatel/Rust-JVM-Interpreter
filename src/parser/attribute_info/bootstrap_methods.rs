use super::attribute::{bootstrap_method::BootstrapMethod, ATTRIBUTE};
use crate::stream_reader::StreamReader;

pub fn read(sr: &mut StreamReader) -> ATTRIBUTE {
  let num_bootstrap_methods = sr.get_u16();
  let bootstrap_methods: Vec<BootstrapMethod> = (0..num_bootstrap_methods)
    .map(|_| BootstrapMethod::read(sr))
    .collect();

  ATTRIBUTE::BootstrapMethods { bootstrap_methods }
}
