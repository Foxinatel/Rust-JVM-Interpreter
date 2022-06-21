use super::{attribute::bootstrap_method::BootstrapMethod, Attribute};
use crate::stream_reader::StreamReader;

pub fn read(sr: &mut StreamReader) -> Attribute {
  let num_bootstrap_methods = sr.get_u16();
  let bootstrap_methods: Vec<BootstrapMethod> =
    (0..num_bootstrap_methods).map(|_| BootstrapMethod::read(sr)).collect();

  Attribute::BootstrapMethods { bootstrap_methods }
}
