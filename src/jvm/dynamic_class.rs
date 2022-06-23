use std::collections::HashMap;

use crate::parser::{field_info::FieldInfo, method_info::MethodInfo};

use super::eval::Eval;

#[derive(Debug, Clone)]
pub struct ClassDynamics {
  pub fields: HashMap<String, FieldInfo>,
  pub methods: HashMap<String, MethodInfo>
  // pub attributes: Vec<Attribute>
}

impl Eval for ClassDynamics {
  fn get_methods(&self) -> &HashMap<String, MethodInfo> {
    &self.methods
  }
}
