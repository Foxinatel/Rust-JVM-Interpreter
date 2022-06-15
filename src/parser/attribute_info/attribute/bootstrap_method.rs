use crate::parser::{helpers::get_u16, attribute_info::bootstrap_methods};

#[derive(Debug)]
pub struct BootstrapMethod {
  bootstrap_method_ref: u16,
  num_bootstrap_arguments: u16,
  bootstrap_methods: Vec<u16>
}

impl BootstrapMethod {
  pub fn read(buf: &mut &[u8]) -> Self {
    let bootstrap_method_ref = get_u16(buf);
    let num_bootstrap_arguments = get_u16(buf);
    let bootstrap_methods: Vec<u16> = (0..num_bootstrap_arguments).map(|_|
      get_u16(buf)
    ).collect();
    BootstrapMethod { bootstrap_method_ref, num_bootstrap_arguments, bootstrap_methods }
  }
}