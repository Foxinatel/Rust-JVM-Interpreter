use crate::parser::stream_reader::StreamReader;

#[derive(Debug, Clone)]
pub struct BootstrapMethod {
  pub bootstrap_method_ref: u16,
  pub bootstrap_methods: Vec<u16>
}

impl BootstrapMethod {
  pub fn read(sr: &mut StreamReader) -> Self {
    let bootstrap_method_ref = sr.get_u16();
    let num_bootstrap_arguments = sr.get_u16();
    let bootstrap_methods: Vec<u16> = (0..num_bootstrap_arguments).map(|_| sr.get_u16()).collect();

    BootstrapMethod { bootstrap_method_ref, bootstrap_methods }
  }
}
