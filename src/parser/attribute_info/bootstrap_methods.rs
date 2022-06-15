use super::attribute::{ATTRIBUTE, bootstrap_method::BootstrapMethod};
use crate::helpers::get_u16;

pub fn read(buf: &mut &[u8]) -> ATTRIBUTE {
  let num_bootstrap_methods = get_u16(buf);
  let bootstrap_methods: Vec<BootstrapMethod> = (0..num_bootstrap_methods).map(|_|
    BootstrapMethod::read(buf)
  ).collect();

  ATTRIBUTE::BootstrapMethods {
    num_bootstrap_methods,
    bootstrap_methods
  }
}
