use std::collections::HashMap;

use crate::parser::{attribute_info::Attribute, field_info::FieldInfo, method_info::MethodInfo};

#[derive(Debug, Clone)]
pub struct ClassDynamics {
  pub fields: HashMap<String, FieldInfo>,
  pub methods: HashMap<String, MethodInfo>,
  // pub attributes: Vec<Attribute>
}
